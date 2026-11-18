<#
.SYNOPSIS
👀 Read-only watch: for the given seconds prints every new terminal host process (WindowsTerminal, OpenConsole, visible conhost) and every new top-level window, then the last result of the semio tasks.
#>
param([int]$Seconds = 150)
$ErrorActionPreference = 'SilentlyContinue'
$seenHosts = @{}; $seenWindows = @{}
Get-CimInstance Win32_Process | Where-Object { $_.Name -match '^(WindowsTerminal|OpenConsole)\.exe$' } | ForEach-Object { $seenHosts[$_.ProcessId] = 1 }
Get-Process | Where-Object { $_.MainWindowHandle -ne 0 } | ForEach-Object { $seenWindows[$_.Id] = 1 }
"[DEBUG] watch start $((Get-Date).ToUniversalTime().ToString('o')) hosts $($seenHosts.Count) windows $($seenWindows.Count)"
$end = (Get-Date).AddSeconds($Seconds)
while ((Get-Date) -lt $end) {
  Get-CimInstance Win32_Process | Where-Object { $_.Name -match '^(WindowsTerminal|OpenConsole)\.exe$' -and -not $seenHosts[$_.ProcessId] } | ForEach-Object { $seenHosts[$_.ProcessId] = 1; "[DEBUG] new host $($_.Name) $($_.ProcessId) at $($_.CreationDate.ToUniversalTime().ToString('HH:mm:ss'))" }
  Get-Process | Where-Object { $_.MainWindowHandle -ne 0 -and -not $seenWindows[$_.Id] } | ForEach-Object { $seenWindows[$_.Id] = 1; "[DEBUG] new window $($_.ProcessName) $($_.Id) [$($_.MainWindowTitle)]" }
  Start-Sleep -Milliseconds 400
}
Get-ScheduledTask | Where-Object { $_.TaskName -match '^semio-fleet' } | ForEach-Object { $i = $_ | Get-ScheduledTaskInfo; '{0} {1} last {2:HH:mm:ss} result {3}' -f $_.TaskName, $_.State, $i.LastRunTime.ToUniversalTime(), $i.LastTaskResult }
"[DEBUG] watch end hosts now $(@(Get-CimInstance Win32_Process | Where-Object { $_.Name -match '^(WindowsTerminal|OpenConsole)\.exe$' }).Count)"
