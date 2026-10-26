$ErrorActionPreference = 'Stop'
$output = Join-Path $PSScriptRoot '🗑️generated\taxonomy-render'
$before = @(Get-Content -LiteralPath (Join-Path $output 'cancel-before.json') -Raw -Encoding utf8 | ConvertFrom-Json)
$current = @(Get-CimInstance Win32_Process)
$remaining = @($current | Where-Object { $candidate = $_; ($before | Where-Object { $_.ProcessId -eq $candidate.ProcessId }) -or ($candidate.Name -match '^python.*\.exe$' -and $candidate.CommandLine -match 'taxonomy-render' -and $candidate.CommandLine -notmatch 'check-taxonomy-render-stopped') })
$remaining | Select-Object ProcessId,ParentProcessId,Name,CommandLine,CreationDate | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $output 'cancel-after.json') -Encoding utf8
Write-Output ('[DEBUG] Owned remaining process inventory: ' + $remaining.Count)
if ($remaining.Count) { throw 'Owned render descendants remain' }
