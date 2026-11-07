$ErrorActionPreference = 'Stop'
Get-CimInstance Win32_Process | Where-Object { $_.ExecutablePath -like '*proctor-*.exe' -and $_.CommandLine -like '*setInterval*' } | Select-Object ProcessId, ParentProcessId, CreationDate, ExecutablePath, CommandLine | ConvertTo-Json -Compress
