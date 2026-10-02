# 🧱️ Starts and stops the throw-away dev stack of the "UI polish" measurements: the site on 6071 and the proctor on 8801,
# with their own data directory and dependency cache under <ticket>/🗑️generated/polish/stack, through the site's own
# stack helper (`bun ./📜️script.ts dev`). The dev server does not watch the sources, so an edit by anybody never reloads
# a page under measurement; restart the stack to measure a change.
#   pwsh ui_polish_stack.ps1 start    # returns once the site answers
#   pwsh ui_polish_stack.ps1 stop     # ends every process of the stack: whoever listens on the two ports, with its parents up to the stack's own supervisor
param([Parameter(Mandatory = $true)][ValidateSet("start", "stop")][string]$Verb)

$ErrorActionPreference = "Stop"
$ticket = $PSScriptRoot
$repo = (Resolve-Path (Join-Path $ticket "../../../../../../..")).Path
$work = Join-Path $ticket "🗑️generated/polish/stack"
$ports = 6071, 8801

function Stop-Stack {
  $owners = Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in $ports } | Select-Object -ExpandProperty OwningProcess -Unique
  $supervisors = @()
  foreach ($owner in $owners) {
    $current = Get-CimInstance Win32_Process -Filter "ProcessId=$owner"
    while ($current -and $current.CommandLine -notmatch "script\.ts dev(\s|$)") { $current = Get-CimInstance Win32_Process -Filter "ProcessId=$($current.ParentProcessId)" }
    if ($current) { $supervisors += $current.ProcessId } else { $supervisors += $owner }
  }
  foreach ($id in ($supervisors | Select-Object -Unique)) { taskkill /PID $id /T /F | Out-Null }
  Start-Sleep -Seconds 1
  $left = (Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in $ports } | Measure-Object).Count
  Write-Output "stack stopped: $(@($supervisors | Select-Object -Unique).Count) process tree(s) ended, $left listener(s) left on $($ports -join ', ')"
}

if ($Verb -eq "stop") { Stop-Stack; return }

Stop-Stack | Out-Null
New-Item -ItemType Directory -Force (Join-Path $work "data") | Out-Null
$env:PROCTOR_DATA = Join-Path $work "data"
$env:PROCTOR_PORT = "8801"
$env:TEACHING_ARCHITECTURE_QUIZ_PORT = "6071"
$env:TEACHING_ARCHITECTURE_QUIZ_CACHE = Join-Path $work "node_modules/.vite"
$env:TEACHING_ARCHITECTURE_QUIZ_WATCH = "off"
$log = Join-Path $work "stack.log"
$first = Get-Command bun -CommandType Application | Select-Object -First 1
$bun = if ($first.Source -like "*.exe") { $first.Source } else { Join-Path (Split-Path $first.Source) "node_modules/bun/bin/bun.exe" }
if (-not (Test-Path $bun)) { $bun = (Get-Command bun.exe | Select-Object -First 1).Source }
Start-Process -FilePath $bun -ArgumentList "./📜️script.ts", "dev" -WorkingDirectory (Join-Path $repo "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript") -RedirectStandardOutput $log -RedirectStandardError "$log.err" -WindowStyle Hidden | Out-Null
$deadline = (Get-Date).AddMinutes(15)
while ((Get-Date) -lt $deadline) {
  if ((Test-Path $log) -and (Select-String -Path $log -Pattern "site ready at" -Quiet)) { Write-Output "stack ready: site http://127.0.0.1:6071, proctor http://127.0.0.1:8801"; return }
  Start-Sleep -Seconds 2
}
throw "the stack did not become ready within 15 minutes; see $log"
