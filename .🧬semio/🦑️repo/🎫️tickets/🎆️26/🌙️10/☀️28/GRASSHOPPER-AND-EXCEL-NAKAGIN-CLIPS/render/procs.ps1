Get-CimInstance Win32_Process -Filter "Name='python.exe'" | ForEach-Object { $c = $_.CommandLine -replace '.*render[/\\]preview\.py"?\s*', 'preview.py '; "{0} {1}" -f $_.ProcessId, $c }
