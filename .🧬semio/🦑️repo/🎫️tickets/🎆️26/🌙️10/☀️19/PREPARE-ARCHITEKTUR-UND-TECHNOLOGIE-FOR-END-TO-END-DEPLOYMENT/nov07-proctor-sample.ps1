$first = @(Get-CimInstance Win32_Process)
$matches = @($first | Where-Object { $_.Name -match '^bun(\.exe)?$' -and $_.CommandLine -match '@teaching/proctor:build' -and $_.CommandLine -match 'D:[\\/]semio' })
if ($matches.Count -ne 1) { Write-Output ("AMBIGUOUS_MATCH_COUNT=" + $matches.Count); exit 2 }
$root = $matches[0]; $ids = @([int]$root.ProcessId)
for ($i=0; $i -lt 8; $i++) { $ids = @($ids + @($first | Where-Object { $ids -contains [int]$_.ParentProcessId } | ForEach-Object { [int]$_.ProcessId }) | Select-Object -Unique) }
$selected = @($first | Where-Object { $ids -contains [int]$_.ProcessId })
Write-Output ('FIRST=' + (ConvertTo-Json -InputObject @($selected | Select-Object ProcessId,ParentProcessId,CreationDate,ExecutablePath,CommandLine,KernelModeTime,UserModeTime,ReadTransferCount,WriteTransferCount) -Depth 4 -Compress))
Start-Sleep -Seconds 10
$second = @(Get-CimInstance Win32_Process); $checked = @()
foreach ($old in $selected) { $new = @($second | Where-Object { $_.ProcessId -eq $old.ProcessId -and $_.CreationDate -eq $old.CreationDate -and $_.ExecutablePath -eq $old.ExecutablePath }); if ($new.Count -eq 1) { $checked += $new[0] } else { Write-Output ('IDENTITY_CHANGED=' + $old.ProcessId) } }
Write-Output ('SECOND=' + (ConvertTo-Json -InputObject @($checked | Select-Object ProcessId,ParentProcessId,CreationDate,ExecutablePath,KernelModeTime,UserModeTime,ReadTransferCount,WriteTransferCount) -Depth 4 -Compress))
