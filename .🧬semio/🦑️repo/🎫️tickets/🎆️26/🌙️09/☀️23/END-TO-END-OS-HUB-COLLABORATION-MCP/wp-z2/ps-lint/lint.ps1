# Z2 PowerShell oracle: parse + PSScriptAnalyzer 5.1/7 syntax compatibility of the bootstrap scripts (input via /in).
$ErrorActionPreference = "Stop"
Install-PSResource -Name PSScriptAnalyzer -Version 1.24.0 -TrustRepository -Scope CurrentUser -Quiet
Import-Module PSScriptAnalyzer
foreach ($file in Get-ChildItem -LiteralPath /in -Filter *.ps1) {
    $bytes = [System.IO.File]::ReadAllBytes($file.FullName)
    $bom = $bytes.Length -ge 3 -and $bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF
    $tokens = $null; $errors = $null
    [System.Management.Automation.Language.Parser]::ParseFile($file.FullName, [ref]$tokens, [ref]$errors) | Out-Null
    Write-Output ("[z2-ps] {0} bom={1} parseErrors={2}" -f $file.Name, $bom, $errors.Count)
    foreach ($e in $errors) { Write-Output ("[z2-ps]   parse {0}: {1}" -f $e.Extent.StartLineNumber, $e.Message) }
    $settings = @{ Rules = @{ PSUseCompatibleSyntax = @{ Enable = $true; TargetVersions = @("5.1", "7.0") } }; IncludeRules = @("PSUseCompatibleSyntax", "PSAvoidUsingCmdletAliases", "PSUseBOMForUnicodeEncodedFile") }
    $findings = Invoke-ScriptAnalyzer -Path $file.FullName -Settings $settings
    Write-Output ("[z2-ps] {0} analyzerFindings={1}" -f $file.Name, @($findings).Count)
    foreach ($f in $findings) { Write-Output ("[z2-ps]   {0} L{1} {2}" -f $f.RuleName, $f.Line, $f.Message) }
    $ansi = [System.Text.Encoding]::GetEncoding(1252)
    $asAnsi = $ansi.GetString($bytes)
    $tokens2 = $null; $errors2 = $null
    [System.Management.Automation.Language.Parser]::ParseInput($asAnsi, [ref]$tokens2, [ref]$errors2) | Out-Null
    Write-Output ("[z2-ps] {0} decodedAsCp1252WithoutBomHandling parseErrors={1} emojiPathLiteralIntact={2}" -f $file.Name, $errors2.Count, $asAnsi.Contains(".🧬semio"))
}
