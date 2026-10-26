$ErrorActionPreference = 'Stop'
$renderPid = 29216
$output = Join-Path $PSScriptRoot '🗑️generated\taxonomy-fixed'
$processes = @(Get-CimInstance Win32_Process)
$rootProcess = $processes | Where-Object { $_.ProcessId -eq $renderPid }
if (!$rootProcess -or $rootProcess.CommandLine -notmatch 'taxonomy-fixed') { throw 'Owned controller identity did not match' }
$ownedIds = [System.Collections.Generic.HashSet[uint32]]::new()
[void]$ownedIds.Add($renderPid)
do {
    $added = $false
    foreach ($item in $processes) { if ($ownedIds.Contains([uint32]$item.ParentProcessId) -and $ownedIds.Add([uint32]$item.ProcessId)) { $added = $true } }
} while ($added)
$owned = @($processes | Where-Object { $ownedIds.Contains([uint32]$_.ProcessId) } | Select-Object ProcessId,ParentProcessId,Name,CommandLine,CreationDate)
$owned | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $output 'owned-before-completion.json') -Encoding utf8
Write-Output ('[DEBUG] Live owned render tree inventory: ' + $owned.Count)
