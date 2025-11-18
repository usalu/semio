# validate-i18n.ps1
# Validates i18n setup by checking if all UI element IDs have corresponding locale entries
#
# This script scans all .tsx files in the sketchpad directory for UI element IDs
# (elements with id="semio.sketchpad.*" attributes) and validates that:
# 1. All IDs have corresponding entries in locale files (en.json, de.json)
# 2. Locale entries have the correct structure (label.normal, label.beginner, etc.)
# 3. No locale keys are unused (exist but not referenced in code)
#
# Usage:
#   .\scripts\validate-i18n.ps1
#   .\scripts\validate-i18n.ps1 -OutputFile "custom-report.md"
#
# Output:
#   - Console summary of validation results
#   - Detailed markdown report in agents/i18n.md

param(
    [string]$LocalesDir = "js\js\sketchpad\locales",
    [string]$SourceDir = "js\js\sketchpad",
    [string]$OutputFile = "agents\i18n.md"
)

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "🔍 Validating i18n setup..." -ForegroundColor Cyan
Write-Host ""

# Region: Load Locale Files
Write-Host "📂 Loading locale files..." -ForegroundColor Yellow

$localeFiles = Get-ChildItem -Path $LocalesDir -Filter "*.json"
$locales = @{}

foreach ($file in $localeFiles) {
    $langCode = [System.IO.Path]::GetFileNameWithoutExtension($file.Name)
    try {
        $locales[$langCode] = Get-Content $file.FullName -Raw | ConvertFrom-Json
        Write-Host "  ✓ Loaded $langCode.json" -ForegroundColor Green
    } catch {
        Write-Host "  ✗ Failed to parse $langCode.json: $_" -ForegroundColor Red
        exit 1
    }
}

if ($locales.Count -eq 0) {
    Write-Host "  ✗ No locale files found in $LocalesDir" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Region: Extract UI Element IDs from Source Files
Write-Host "🔎 Scanning source files for UI element IDs..." -ForegroundColor Yellow

$tsxFiles = Get-ChildItem -Path $SourceDir -Filter "*.tsx" -Recurse
$foundIds = @{}

# Patterns to detect i18n key usage
$patterns = @(
    @{ Regex = 'id="(semio\.sketchpad\.[^"]+)"'; Kind = "id" }
    @{ Regex = 'id:\s*"(semio\.sketchpad\.[^"]+)"'; Kind = "id" }
    @{ Regex = "id:\s*'(semio\.sketchpad\.[^']+)'"; Kind = "id" }
    @{ Regex = 'placeholderId="([^"]+)"'; Kind = "placeholderId" }
    @{ Regex = 'i18nPressed="([^"]+)"'; Kind = "i18nPressed" }
    @{ Regex = 'actionId="([^"]+)"'; Kind = "actionId" }
    @{ Regex = 't\("(semio\.sketchpad\.[^"]+)"\)'; Kind = "t" }
    @{ Regex = "t\('(semio\.sketchpad\.[^']+)'\)"; Kind = "t" }
    @{ Regex = 'i18n\.t\("(semio\.sketchpad\.[^"]+)"\)'; Kind = "t" }
    @{ Regex = "i18n\.t\('(semio\.sketchpad\.[^']+)'\)"; Kind = "t" }
)

foreach ($file in $tsxFiles) {
    $content = Get-Content $file.FullName -Raw
    
    foreach ($pattern in $patterns) {
        $matches = [regex]::Matches($content, $pattern.Regex)
        foreach ($match in $matches) {
            $id = $match.Groups[1].Value
            
            # Filter placeholderId to only include semio.sketchpad ones
            if ($pattern.Kind -eq "placeholderId" -and $id -notmatch '^semio\.sketchpad\.') {
                continue
            }
            
            if (-not $foundIds.ContainsKey($id)) {
                $foundIds[$id] = @{
                    Files = @()
                    Kind = $pattern.Kind
                }
            }
            
            $relPath = $file.FullName.Replace("$PWD\", "")
            if ($foundIds[$id].Files -notcontains $relPath) {
                $foundIds[$id].Files += $relPath
            }
        }
    }
}

Write-Host "  ✓ Found $($foundIds.Count) unique UI element IDs" -ForegroundColor Green
Write-Host ""

# Region: Check Locale Completeness
Write-Host "🔍 Validating locale entries..." -ForegroundColor Yellow

function Get-NestedProperty($obj, $path) {
    $parts = $path -split '\.'
    $current = $obj
    foreach ($part in $parts) {
        if ($null -eq $current -or -not ($current.PSObject.Properties.Name -contains $part)) {
            return $null
        }
        $current = $current.$part
    }
    return $current
}

function Test-I18nEntry($locale, $id, $kind) {
    $value = Get-NestedProperty $locale $id
    
    if ($null -eq $value) {
        return @{
            Status = "Missing"
            Details = "Key does not exist"
        }
    }
    
    # Check structure based on expected format
    if ($value -is [string]) {
        # Simple string value - valid for placeholderId
        if ($kind -eq "placeholderId") {
            return @{
                Status = "Valid"
                Details = "String value"
            }
        } else {
            return @{
                Status = "Warning"
                Details = "Expected object with label/hotkey, found string"
            }
        }
    }
    
    $issues = @()
    $hasLabel = $null -ne $value.label
    $hasHotkey = $null -ne $value.hotkey
    $hasManual = $null -ne $value.manual
    $hasTutorial = $null -ne $value.tutorial
    
    # Check label structure
    if ($hasLabel) {
        if ($value.label -is [string]) {
            $issues += "label is string (expected object with normal/beginner)"
        } elseif ($value.label -is [PSCustomObject]) {
            $hasNormal = $null -ne $value.label.normal
            $hasBeginner = $null -ne $value.label.beginner
            
            if (-not $hasNormal -and -not $hasBeginner) {
                $issues += "label object is empty"
            }
            if ($hasNormal -and $value.label.normal -eq "") {
                $issues += "label.normal is empty string"
            }
            if ($hasBeginner -and $value.label.beginner -eq "") {
                $issues += "label.beginner is empty string"
            }
        }
    } else {
        $issues += "missing label property"
    }
    
    # Check hotkey (optional but should be string if present)
    if ($hasHotkey -and ($value.hotkey -isnot [string] -or $value.hotkey -eq "")) {
        $issues += "hotkey is not a valid string"
    }
    
    if ($issues.Count -gt 0) {
        return @{
            Status = "Incomplete"
            Details = $issues -join "; "
        }
    }
    
    return @{
        Status = "Valid"
        Details = "Complete entry"
    }
}

$results = @{}

foreach ($langCode in $locales.Keys) {
    $results[$langCode] = @{
        Missing = @()
        Incomplete = @()
        Valid = @()
        Warnings = @()
    }
    
    foreach ($id in $foundIds.Keys) {
        $kind = $foundIds[$id].Kind
        $check = Test-I18nEntry $locales[$langCode] $id $kind
        
        switch ($check.Status) {
            "Missing" {
                $results[$langCode].Missing += @{
                    Id = $id
                    Kind = $kind
                    Details = $check.Details
                    Files = $foundIds[$id].Files
                }
            }
            "Incomplete" {
                $results[$langCode].Incomplete += @{
                    Id = $id
                    Kind = $kind
                    Details = $check.Details
                    Files = $foundIds[$id].Files
                }
            }
            "Warning" {
                $results[$langCode].Warnings += @{
                    Id = $id
                    Kind = $kind
                    Details = $check.Details
                    Files = $foundIds[$id].Files
                }
            }
            "Valid" {
                $results[$langCode].Valid += @{
                    Id = $id
                    Kind = $kind
                }
            }
        }
    }
    
    $missing = $results[$langCode].Missing.Count
    $incomplete = $results[$langCode].Incomplete.Count
    $warnings = $results[$langCode].Warnings.Count
    $valid = $results[$langCode].Valid.Count
    $total = $missing + $incomplete + $warnings + $valid
    
    Write-Host "  $langCode.json:" -ForegroundColor White
    Write-Host "    ✓ Valid:      $valid / $total" -ForegroundColor Green
    if ($warnings -gt 0) {
        Write-Host "    ⚠ Warnings:   $warnings / $total" -ForegroundColor Yellow
    }
    if ($incomplete -gt 0) {
        Write-Host "    ⚠ Incomplete: $incomplete / $total" -ForegroundColor Yellow
    }
    if ($missing -gt 0) {
        Write-Host "    ✗ Missing:    $missing / $total" -ForegroundColor Red
    }
}

Write-Host ""

# Region: Check for Unused Locale Keys
Write-Host "🔍 Checking for unused locale keys..." -ForegroundColor Yellow

function Get-AllKeys($obj, $prefix = "") {
    $keys = @()
    if ($null -eq $obj) { return $keys }
    
    foreach ($prop in $obj.PSObject.Properties) {
        $key = if ($prefix -eq "") { $prop.Name } else { "$prefix.$($prop.Name)" }
        
        # Skip metadata keys
        if ($prop.Name -in @("label", "hotkey", "manual", "tutorial", "beginner", "normal")) {
            continue
        }
        
        $keys += $key
        
        if ($prop.Value -is [PSCustomObject]) {
            $keys += Get-AllKeys $prop.Value $key
        }
    }
    
    return $keys
}

$unusedKeys = @{}

foreach ($langCode in $locales.Keys) {
    $allKeys = Get-AllKeys $locales[$langCode]
    $semioKeys = $allKeys | Where-Object { $_ -match '^semio\.sketchpad\.' }
    
    $unused = @()
    foreach ($key in $semioKeys) {
        if (-not $foundIds.ContainsKey($key)) {
            $unused += $key
        }
    }
    
    $unusedKeys[$langCode] = $unused
    
    if ($unused.Count -gt 0) {
        Write-Host "  $langCode.json: $($unused.Count) potentially unused keys" -ForegroundColor Yellow
    } else {
        Write-Host "  $langCode.json: No unused keys" -ForegroundColor Green
    }
}

Write-Host ""

# Region: Generate Report
Write-Host "📝 Generating report..." -ForegroundColor Yellow

$reportDir = Split-Path $OutputFile -Parent
if (-not (Test-Path $reportDir)) {
    New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
}

$report = @"
# i18n Validation Report

Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Summary

Total UI elements scanned: **$($foundIds.Count)**

"@

foreach ($langCode in $locales.Keys | Sort-Object) {
    $missing = $results[$langCode].Missing.Count
    $incomplete = $results[$langCode].Incomplete.Count
    $warnings = $results[$langCode].Warnings.Count
    $valid = $results[$langCode].Valid.Count
    $total = $missing + $incomplete + $warnings + $valid
    $percentage = if ($total -gt 0) { [math]::Round(($valid / $total) * 100, 1) } else { 0 }
    
    $report += @"


### $langCode.json

- ✓ Valid: $valid / $total ($percentage%)
- ⚠ Warnings: $warnings / $total
- ⚠ Incomplete: $incomplete / $total
- ✗ Missing: $missing / $total

"@
}

$report += @"


## Details

"@

foreach ($langCode in $locales.Keys | Sort-Object) {
    $report += @"

### $langCode Missing Entries

"@
    
    if ($results[$langCode].Missing.Count -eq 0) {
        $report += "`nNo missing entries.`n"
    } else {
        $report += "`n| ID | Kind | Files | Details |`n"
        $report += "|---|---|---|---|`n"
        
        foreach ($item in $results[$langCode].Missing | Sort-Object { $_.Id }) {
            $filesStr = ($item.Files | Select-Object -First 2) -join ", "
            if ($item.Files.Count -gt 2) {
                $filesStr += " (+ $($item.Files.Count - 2) more)"
            }
            $report += "| ``$($item.Id)`` | $($item.Kind) | $filesStr | $($item.Details) |`n"
        }
    }
    
    $report += @"

#### Incomplete Entries

"@
    
    if ($results[$langCode].Incomplete.Count -eq 0) {
        $report += "`nNo incomplete entries.`n"
    } else {
        $report += "`n| ID | Kind | Files | Issues |`n"
        $report += "|---|---|---|---|`n"
        
        foreach ($item in $results[$langCode].Incomplete | Sort-Object { $_.Id }) {
            $filesStr = ($item.Files | Select-Object -First 2) -join ", "
            if ($item.Files.Count -gt 2) {
                $filesStr += " (+ $($item.Files.Count - 2) more)"
            }
            $report += "| ``$($item.Id)`` | $($item.Kind) | $filesStr | $($item.Details) |`n"
        }
    }
    
    $report += @"

#### Warnings

"@
    
    if ($results[$langCode].Warnings.Count -eq 0) {
        $report += "`nNo warnings.`n"
    } else {
        $report += "`n| ID | Kind | Files | Details |`n"
        $report += "|---|---|---|---|`n"
        
        foreach ($item in $results[$langCode].Warnings | Sort-Object { $_.Id }) {
            $filesStr = ($item.Files | Select-Object -First 2) -join ", "
            if ($item.Files.Count -gt 2) {
                $filesStr += " (+ $($item.Files.Count - 2) more)"
            }
            $report += "| ``$($item.Id)`` | $($item.Kind) | $filesStr | $($item.Details) |`n"
        }
    }
    
    $report += @"

#### Unused Locale Keys

"@
    
    if ($unusedKeys[$langCode].Count -eq 0) {
        $report += "`nNo unused keys detected.`n"
    } else {
        $report += "`nThese keys exist in the locale file but are not referenced in the codebase:`n`n"
        foreach ($key in $unusedKeys[$langCode] | Sort-Object) {
            $report += "- ``$key```n"
        }
    }
}

$report += @"


## i18n System Documentation

### Expected Structure

UI elements with IDs should have corresponding locale entries with this structure:

``````json
{
  "semio.sketchpad.element.id": {
    "label": {
      "normal": "Label text",
      "beginner": "Beginner-friendly description"
    },
    "hotkey": "Ctrl+K",
    "manual": "path/to/manual",
    "tutorial": "path/to/tutorial"
  }
}
``````

### Property Usage

- **label.normal**: Standard label text (required)
- **label.beginner**: Beginner-friendly description (optional, shown in beginner mode)
- **hotkey**: Keyboard shortcut (optional)
- **manual**: Path to manual page (optional)
- **tutorial**: Path to tutorial (optional)

### ID Attribute Types

- **id**: Standard UI element ID (expects full structure)
- **placeholderId**: Placeholder text (can be simple string or use .label)
- **i18nPressed**: Pressed state label for toggles
- **actionId**: Action button IDs

"@

Set-Content -Path $OutputFile -Value $report -Encoding UTF8
Write-Host "  ✓ Report saved to $OutputFile" -ForegroundColor Green
Write-Host ""

# Region: Summary
$totalIssues = 0
foreach ($langCode in $locales.Keys) {
    $totalIssues += $results[$langCode].Missing.Count + $results[$langCode].Incomplete.Count
}

if ($totalIssues -eq 0) {
    Write-Host "✅ All i18n entries are valid!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "⚠️  Found $totalIssues issues across all locales. Check the report for details." -ForegroundColor Yellow
    exit 0
}
