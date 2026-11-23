$all = Get-CimInstance Win32_Process; $byId=@{}; $all | % { $byId[[int]$_.ProcessId]=$_ }
foreach ($id in 33996,57316) { $p=$byId[$id]; for($i=0;$i -lt 10 -and $p;$i++){ '{0}:{1} {2}' -f $p.ProcessId,$p.Name,("$($p.CommandLine)".Substring(0,[Math]::Min(220,"$($p.CommandLine)".Length))); $p=$byId[[int]$p.ParentProcessId] }; '---' }
