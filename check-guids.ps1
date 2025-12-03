# Script to find duplicate GUIDs in Semio.Grasshopper.cs
$filePath = "c:\git\semio.tech\semio\net\Semio.Grasshopper\Semio.Grasshopper.cs"
$content = Get-Content $filePath

# Extract all GUIDs with their line numbers
$guidPattern = '[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}'
$guids = @{}
$lineNumber = 0

foreach ($line in $content) {
    $lineNumber++
    if ($line -match $guidPattern) {
        $guid = $matches[0].ToUpper()
        if (-not $guids.ContainsKey($guid)) {
            $guids[$guid] = @()
        }
        $guids[$guid] += $lineNumber
    }
}

# Find duplicates
$duplicates = $guids.GetEnumerator() | Where-Object { $_.Value.Count -gt 1 }

if ($duplicates) {
    Write-Host "Found duplicate GUIDs:" -ForegroundColor Red
    foreach ($dup in $duplicates) {
        Write-Host "`nGUID: $($dup.Key)" -ForegroundColor Yellow
        Write-Host "Appears on lines: $($dup.Value -join ', ')" -ForegroundColor Cyan
    }
    Write-Host "`nTotal duplicates: $($duplicates.Count)" -ForegroundColor Red
} else {
    Write-Host "No duplicate GUIDs found!" -ForegroundColor Green
}

# Show total GUID count
Write-Host "`nTotal GUIDs: $($guids.Count)" -ForegroundColor White
