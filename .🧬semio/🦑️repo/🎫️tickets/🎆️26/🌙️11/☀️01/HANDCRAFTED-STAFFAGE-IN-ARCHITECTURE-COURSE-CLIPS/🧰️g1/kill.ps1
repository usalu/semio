param([string]$Pattern)
Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -like "*$Pattern*" -and $_.ProcessId -ne $PID } | ForEach-Object { Write-Output "$($_.ProcessId) $($_.Name)"; Stop-Process -Id $_.ProcessId -Force }
