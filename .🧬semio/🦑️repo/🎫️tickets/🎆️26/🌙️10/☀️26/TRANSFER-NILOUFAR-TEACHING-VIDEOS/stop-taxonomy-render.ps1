$ErrorActionPreference = 'Stop'
$renderPid = 6892
$output = Join-Path $PSScriptRoot '🗑️generated\taxonomy-render'
$processes = @(Get-CimInstance Win32_Process)
$ownedRoot = $processes | Where-Object { $_.ProcessId -eq $renderPid }
if (!$ownedRoot -or $ownedRoot.Name -notmatch '^python.*\.exe$' -or $ownedRoot.CommandLine -notmatch 'taxonomy-render') { throw 'Owned render process identity did not match' }
$ownedIds = [System.Collections.Generic.HashSet[uint32]]::new()
[void]$ownedIds.Add($renderPid)
do {
    $added = $false
    foreach ($item in $processes) { if ($ownedIds.Contains([uint32]$item.ParentProcessId) -and $ownedIds.Add([uint32]$item.ProcessId)) { $added = $true } }
} while ($added)
$owned = @($processes | Where-Object { $ownedIds.Contains([uint32]$_.ProcessId) } | Select-Object ProcessId, ParentProcessId, Name, CommandLine, CreationDate)
$owned | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $output 'cancel-before.json') -Encoding utf8
Write-Output ('[DEBUG] Verified owned render tree before cancellation: ' + $owned.Count)
& taskkill /PID $renderPid /T /F
if ($LASTEXITCODE -ne 0) { throw ('Owned tree cancellation failed: ' + $LASTEXITCODE) }
