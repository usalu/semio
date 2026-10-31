param([string]$Pattern)
Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -and $_.CommandLine -like "*$Pattern*" -and $_.ProcessId -ne $PID } | ForEach-Object {
  '{0} {1}' -f $_.ProcessId, $_.Name
  Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue
}
