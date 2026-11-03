Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -like '*time.sleep(300)*' -or ($_.Name -match '^(bun|node)' -and $_.CommandLine -like '*cast*tmp*') } | ForEach-Object { "$($_.ProcessId) $($_.Name)"; if ($args -contains 'kill') { Stop-Process -Id $_.ProcessId -Force } }
"scan done"
