# 🛑️ Windows-host check of the dev stack's stop behaviour: starts `Command` in a console of its own (as a launch.json
# terminal does, with Ctrl+C enabled), waits until every port in `Ports` listens, then stops it the way `Stop` names —
# `ctrl-c` (one Ctrl+C to the console), `ctrl-break`, `close` (the console is closed) or `tree` (the top process tree is
# killed, as nx and editors do) — and reports what still listens and which processes of the command are left (the
# wrapper shell that gives the command its console is not the command).
# pwsh dev_stack_ctrl_c.ps1 -Cwd <dir> -Command "bun ./📜️script.ts dev" -Ports 6171,8871 -Log <file> [-Stop ctrl-c]
param(
  [Parameter(Mandatory = $true)][string]$Cwd,
  [Parameter(Mandatory = $true)][string]$Command,
  [Parameter(Mandatory = $true)][int[]]$Ports,
  [Parameter(Mandatory = $true)][string]$Log,
  [ValidateSet("ctrl-c", "ctrl-break", "close", "tree")][string]$Stop = "ctrl-c",
  [int]$ReadySeconds = 600,
  [int]$StopSeconds = 30
)

$native = 'Add-Type -Namespace Native -Name Console -MemberDefinition ''[DllImport("kernel32.dll")] public static extern bool AttachConsole(uint id); [DllImport("kernel32.dll")] public static extern bool FreeConsole(); [DllImport("kernel32.dll")] public static extern bool SetConsoleCtrlHandler(IntPtr handler, bool add); [DllImport("kernel32.dll")] public static extern bool GenerateConsoleCtrlEvent(uint kind, uint group);'''

function Listening([int[]]$wanted) {
  @(Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in $wanted })
}

function Descendants([int]$root) {
  $all = @(Get-CimInstance Win32_Process)
  $found = @($root)
  for ($grew = $true; $grew; ) {
    $grew = $false
    foreach ($process in $all) {
      if (($found -contains [int]$process.ParentProcessId) -and -not ($found -contains [int]$process.ProcessId)) { $found += [int]$process.ProcessId; $grew = $true }
    }
  }
  @($all | Where-Object { $found -contains [int]$_.ProcessId })
}

function Encoded([string]$text) {
  [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($text))
}

function Alive([int[]]$processIds) {
  @($processIds | Where-Object { $_ -gt 4 } | ForEach-Object { Get-Process -Id $_ -ErrorAction SilentlyContinue })
}

if ((Listening $Ports).Count -gt 0) { throw "ports $($Ports -join ',') are already in use" }
Remove-Item -LiteralPath $Log -ErrorAction SilentlyContinue
$env:STACK_COMMAND = $Command
$env:STACK_LOG = $Log
$wrapper = "`$ErrorActionPreference = 'Stop'; $native; [Native.Console]::SetConsoleCtrlHandler([IntPtr]::Zero, `$false) | Out-Null; cmd.exe /d /c `"chcp 65001 >nul & %STACK_COMMAND% > `"`"%STACK_LOG%`"`" 2>&1`""
$top = Start-Process -FilePath "pwsh" -ArgumentList "-NoProfile", "-NonInteractive", "-EncodedCommand", (Encoded $wrapper) -WorkingDirectory $Cwd -PassThru -WindowStyle Hidden
"[DEBUG] started pwsh pid $($top.Id) in its own console; stop = $Stop"
$deadline = (Get-Date).AddSeconds($ReadySeconds)
while ((Listening $Ports).Count -lt $Ports.Count) {
  if ($top.HasExited) { Get-Content -LiteralPath $Log -Tail 30 -ErrorAction SilentlyContinue; throw "the command exited with $($top.ExitCode) before every port listened" }
  if ((Get-Date) -gt $deadline) { taskkill /PID $top.Id /T /F | Out-Null; throw "not ready within $ReadySeconds s" }
  Start-Sleep -Milliseconds 500
}
Start-Sleep -Seconds 3
$family = Descendants $top.Id
"[DEBUG] ready; process tree:"
$family | ForEach-Object { "[DEBUG]   pid $($_.ProcessId) parent $($_.ParentProcessId) $($_.Name)" }
$familyIds = @($family | Where-Object { $_.Name -notin "pwsh.exe", "cmd.exe", "conhost.exe" } | ForEach-Object { [int]$_.ProcessId })
if ($familyIds.Count -lt 3) { taskkill /PID $top.Id /T /F | Out-Null; throw "the process tree was not captured" }

$stopped = Get-Date
if ($Stop -eq "tree") {
  taskkill /PID $top.Id /T /F | Out-Null
} elseif ($Stop -eq "close") {
  $window = $family | Where-Object { $_.Name -eq "conhost.exe" } | Select-Object -First 1
  if (-not $window) { taskkill /PID $top.Id /T /F | Out-Null; throw "the console host of pid $($top.Id) was not found" }
  "[DEBUG] closing console host pid $($window.ProcessId)"
  Stop-Process -Id $window.ProcessId -Force
} else {
  $kind = if ($Stop -eq "ctrl-break") { 1 } else { 0 }
  $sender = "`$ErrorActionPreference = 'Stop'; $native; [Native.Console]::FreeConsole() | Out-Null; if (-not [Native.Console]::AttachConsole($($top.Id))) { exit 3 }; [Native.Console]::SetConsoleCtrlHandler([IntPtr]::Zero, `$true) | Out-Null; if (-not [Native.Console]::GenerateConsoleCtrlEvent($kind, 0)) { exit 4 }; Start-Sleep -Seconds 2; exit 0"
  $sent = Start-Process -FilePath "pwsh" -ArgumentList "-NoProfile", "-NonInteractive", "-EncodedCommand", (Encoded $sender) -PassThru -Wait -WindowStyle Hidden
  "[DEBUG] $Stop sender exited $($sent.ExitCode)"
}

$deadline = (Get-Date).AddSeconds($StopSeconds)
while ((Get-Date) -lt $deadline) {
  if ((Alive $familyIds).Count -eq 0 -and (Listening $Ports).Count -eq 0) { break }
  Start-Sleep -Milliseconds 250
}
"[DEBUG] settled after $([int]((Get-Date) - $stopped).TotalMilliseconds) ms"
$left = Alive $familyIds
$open = Listening $Ports
"[DEBUG] processes of the tree still alive: $($left.Count) $(($left | ForEach-Object { "$($_.Id):$($_.ProcessName)" }) -join ' ')"
"[DEBUG] ports still listening: $($open.Count) $(($open | ForEach-Object { $_.LocalPort }) -join ' ')"
"[DEBUG] log tail:"
Get-Content -LiteralPath $Log -Tail 12 -Encoding utf8 -ErrorAction SilentlyContinue | ForEach-Object { "[DEBUG]   $_" }
foreach ($process in $left) { taskkill /PID $process.Id /T /F | Out-Null }
if (-not $top.HasExited) { taskkill /PID $top.Id /T /F 2>&1 | Out-Null }
if ($left.Count -gt 0 -or $open.Count -gt 0) { "[DEBUG] FAIL: something was left"; exit 1 }
"[DEBUG] PASS: nothing left"
