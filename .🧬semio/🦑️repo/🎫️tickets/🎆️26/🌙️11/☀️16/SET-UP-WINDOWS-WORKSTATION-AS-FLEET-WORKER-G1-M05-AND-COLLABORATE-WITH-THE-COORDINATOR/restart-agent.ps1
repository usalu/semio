<#
.SYNOPSIS
🔄 Ends the build agent (the sidecar the replica daemon supervises), its role processes and the owner pool's worker tree so that the daemon starts them again with the code on disk. The ready stage wrapper and the graph read are not touched.
#>
$ErrorActionPreference = 'SilentlyContinue'
$all = Get-CimInstance Win32_Process
$sidecar = $all | Where-Object { $_.Name -eq 'bun.exe' -and $_.CommandLine -match 'sidecar\\\S+\.ts"? (agent|graph|pool|ready)\s*$' }
$pool = $sidecar | Where-Object { $_.CommandLine -match ' pool\s*$' }
foreach ($p in $pool) { "pool tree $($p.ProcessId)"; & taskkill.exe /PID $p.ProcessId /T /F | Out-Null }
foreach ($p in ($sidecar | Where-Object { $_.CommandLine -notmatch ' pool\s*$' })) { "end $($p.ProcessId) $(($p.CommandLine -split ' ')[-1])"; Stop-Process -Id $p.ProcessId -Force }
$all | Where-Object { $_.Name -eq 'bun.exe' -and $_.CommandLine -match 'owner-pool\\\S+child\\' } | ForEach-Object { "pool child tree $($_.ProcessId)"; & taskkill.exe /PID $_.ProcessId /T /F | Out-Null }
Start-Sleep 5
"left: $(@(Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'bun.exe' -and $_.CommandLine -match 'sidecar\\\S+\.ts"? (agent|graph|pool|ready)\s*$' }).Count) sidecar processes, $(@(Get-CimInstance Win32_Process -Filter "Name='OpenConsole.exe'").Count) console hosts"
