$ErrorActionPreference = 'Stop'
Get-CimInstance Win32_Process | Where-Object { $_.ProcessId -in @(9332,28384,17180) -or ($_.CommandLine -like '*teaching-proctor*' -and $_.CommandLine -like '*json-render-diagnostics*') } | Select-Object ProcessId, ParentProcessId, CreationDate, ExecutablePath, CommandLine | ConvertTo-Json -Compress
