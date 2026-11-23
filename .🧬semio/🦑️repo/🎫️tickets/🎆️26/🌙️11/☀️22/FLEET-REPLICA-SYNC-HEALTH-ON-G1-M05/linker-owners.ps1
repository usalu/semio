$all = Get-CimInstance Win32_Process
$byId = @{}; $all | ForEach-Object { $byId[[int]$_.ProcessId] = $_ }
$all | Where-Object Name -eq 'link.exe' | ForEach-Object {
  $chain = @(); $p = $_
  for ($i = 0; $i -lt 8 -and $p; $i++) { $chain += ('{0}:{1}' -f $p.ProcessId, $p.Name); $p = $byId[[int]$p.ParentProcessId] }
  '{0:N1}GB {1}' -f ($_.WorkingSetSize/1GB), ($chain -join ' < ')
}
