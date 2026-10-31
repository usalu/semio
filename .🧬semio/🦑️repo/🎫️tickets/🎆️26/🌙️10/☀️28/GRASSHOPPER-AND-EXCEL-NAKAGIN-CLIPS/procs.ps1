Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'python.exe' } | ForEach-Object {
  $p = Get-Process -Id $_.ProcessId -ErrorAction SilentlyContinue
  '{0} {1:HH:mm} cpu={2} mb={3} {4}' -f $_.ProcessId, $p.StartTime, [int]$p.CPU, [int]($p.WorkingSet64 / 1MB), (($_.CommandLine -replace '.*python\.exe"?\s*', '') -replace '^(.{0,150}).*', '$1')
}
