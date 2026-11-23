Get-Process link,rustc,cargo -ErrorAction SilentlyContinue | Group-Object Name | ForEach-Object { '{0} n={1} GB={2:N1}' -f $_.Name,$_.Count,(($_.Group | Measure-Object WorkingSet64 -Sum).Sum/1GB) }
'{0:N1} GB free' -f ((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory/1MB)
'pages/s {0:N0}' -f (Get-Counter '\Memory\Pages/sec').CounterSamples[0].CookedValue
