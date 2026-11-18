<#
.SYNOPSIS
🧪 Runs hide-probe.mjs with the workspace Bun and reports, per variant, whether the leaf got a terminal host (OpenConsole) or only a hidden conhost.
#>
$ErrorActionPreference = 'SilentlyContinue'
$dir = $PSScriptRoot
$bun = (Get-ChildItem "$env:LOCALAPPDATA\semio\runtimes\bun" -Filter bun.exe -Recurse | Sort-Object LastWriteTime -Descending | Select-Object -First 1).FullName
$log = Join-Path $dir 'hide-probe.log'
Remove-Item $log
& $bun --version
foreach ($variant in 'plain', 'hide', 'ipc-hide', 'bun-plain', 'bun-hide') {
  $before = @(Get-CimInstance Win32_Process -Filter "Name='OpenConsole.exe'").ProcessId
  & $bun (Join-Path $dir 'hide-probe.mjs') start $variant $log
  Start-Sleep -Milliseconds 2500
  $all = Get-CimInstance Win32_Process
  $id = [int]((Get-Content $log | Select-Object -Last 1) -split ' ')[2]
  $leaf = $all | Where-Object ProcessId -eq $id
  $con = @($all | Where-Object { $_.ParentProcessId -eq $id -and $_.Name -eq 'conhost.exe' }).Count
  $open = @($all | Where-Object { $_.Name -eq 'OpenConsole.exe' -and $before -notcontains $_.ProcessId -and [math]::Abs(($_.CreationDate - $leaf.CreationDate).TotalSeconds) -le 1 }).Count
  '{0,-10} leaf {1} alive {2} own conhost {3} new OpenConsole at its start {4}' -f $variant, $id, [bool]$leaf, $con, $open
}
Remove-Item $log
