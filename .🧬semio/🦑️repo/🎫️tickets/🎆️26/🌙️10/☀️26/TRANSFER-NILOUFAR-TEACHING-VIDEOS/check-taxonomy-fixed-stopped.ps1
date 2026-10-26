$ErrorActionPreference = 'Stop'
$output = Join-Path $PSScriptRoot '🗑️generated\taxonomy-fixed'
$before = @(Get-Content -LiteralPath (Join-Path $output 'owned-before-completion.json') -Raw -Encoding utf8 | ConvertFrom-Json)
$current = @(Get-CimInstance Win32_Process)
$remaining = @($current | Where-Object { $candidate = $_; ($before | Where-Object { $_.ProcessId -eq $candidate.ProcessId -and $_.CommandLine -eq $candidate.CommandLine }) -or ($candidate.Name -match '(?i)^(python|ffmpeg).*\.exe$' -and $candidate.CommandLine -match 'taxonomy-fixed' -and $candidate.CommandLine -notmatch 'check-taxonomy-fixed-stopped') })
ConvertTo-Json -InputObject @($remaining | Select-Object ProcessId,ParentProcessId,Name,CommandLine,CreationDate) -Depth 3 | Set-Content -LiteralPath (Join-Path $output 'owned-after-completion.json') -Encoding utf8
Write-Output ('[DEBUG] Owned remaining process inventory: ' + $remaining.Count)
if ($remaining.Count) { throw 'Owned render descendants remain' }
