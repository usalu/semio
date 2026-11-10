$rows = @(Get-CimInstance Win32_Process)
$ids = @(33624,25344,23616)
for ($i=0; $i -lt 8; $i++) { $next = @($rows | Where-Object { $ids -contains [int]$_.ParentProcessId } | ForEach-Object { [int]$_.ProcessId }); $ids = @($ids + $next | Select-Object -Unique) }
$selected = @($rows | Where-Object { $ids -contains [int]$_.ProcessId } | Select-Object ProcessId,ParentProcessId,CreationDate,ExecutablePath,CommandLine)
Write-Output (ConvertTo-Json -InputObject $selected -Depth 4 -Compress)
