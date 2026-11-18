<#
.SYNOPSIS
🔎 Read-only: for every OpenConsole/WindowsTerminal process lists the processes created within two seconds of it (the likely console clients) with their ancestry, plus the visible windows.
#>
$ErrorActionPreference = 'SilentlyContinue'
$all = Get-CimInstance Win32_Process
$byId = @{}; $all | ForEach-Object { $byId[[int]$_.ProcessId] = $_ }
function Chain($p) { $out = @(); $c = $p; for ($i = 0; $i -lt 7 -and $c; $i++) { $out += "$($c.Name)($($c.ProcessId))"; $c = $byId[[int]$c.ParentProcessId] }; $out -join ' < ' }
$hosts = $all | Where-Object { $_.Name -eq 'OpenConsole.exe' } | Sort-Object CreationDate
"hosts: $(@($hosts).Count), WindowsTerminal: $(@($all | Where-Object Name -eq 'WindowsTerminal.exe').Count)"
$groups = $hosts | Group-Object { $_.CreationDate.ToUniversalTime().ToString('HH:mm:ss') }
foreach ($g in $groups) {
  $t = $g.Group[0].CreationDate
  "== $($g.Name) x$($g.Count)"
  $all | Where-Object { $_.Name -notmatch 'OpenConsole|svchost|RuntimeBroker' -and [math]::Abs(($_.CreationDate - $t).TotalSeconds) -le 2 } | Select-Object -First 12 | ForEach-Object {
    $c = '' + $_.CommandLine
    "   $(Chain $_) :: $($c.Substring(0, [math]::Min(140, $c.Length)).Replace("`n", ' '))"
  }
}
'--- windows'
Get-Process | Where-Object { $_.MainWindowHandle -ne 0 } | ForEach-Object { "$($_.ProcessName) $($_.Id) [$($_.MainWindowTitle)]" }
