<#
.SYNOPSIS
🧪 Probe with two temporary scheduled tasks of the current user (removed at the end): does an interactive task action open a terminal host when it runs `cmd.exe` directly, and when it runs under `conhost.exe --headless`?
#>
$ErrorActionPreference = 'SilentlyContinue'
function Probe([string]$Name, [string]$Command, [string]$Arguments) {
  $action = New-ScheduledTaskAction -Execute $Command -Argument $Arguments
  Register-ScheduledTask -TaskName $Name -Action $action -Force | Out-Null
  if (-not (Get-ScheduledTask -TaskName $Name)) { "$Name could not be registered"; return }
  $before = @(Get-CimInstance Win32_Process -Filter "Name='OpenConsole.exe' OR Name='WindowsTerminal.exe'").ProcessId
  Start-ScheduledTask -TaskName $Name
  Start-Sleep 3
  $all = Get-CimInstance Win32_Process
  $new = @($all | Where-Object { $_.Name -match '^(OpenConsole|WindowsTerminal)\.exe$' -and $before -notcontains $_.ProcessId }).Count
  $ping = @($all | Where-Object { $_.Name -eq 'PING.EXE' }).Count
  Start-Sleep 6
  $info = Get-ScheduledTaskInfo -TaskName $Name
  '{0}: new terminal hosts {1}, command running {2}, result {3}' -f $Name, $new, ($ping -gt 0), $info.LastTaskResult
  Unregister-ScheduledTask -TaskName $Name -Confirm:$false
}
Probe 'semio-probe-plain' 'cmd.exe' '/d /c ping -n 6 127.0.0.1'
Probe 'semio-probe-headless' 'conhost.exe' '--headless cmd.exe /d /c ping -n 6 127.0.0.1'
"left: $(@(Get-ScheduledTask | Where-Object TaskName -match 'semio-probe').Count) probe tasks"
