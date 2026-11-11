# Wait until the 07:36 BIM rest process exits, then render the two lessons that loaded the old section code.
$ErrorActionPreference = 'Stop'
$ticket = $PSScriptRoot
$log = Join-Path $ticket '🗑️generated\bim-rerender-480p15.log'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $log) | Out-Null
function Say([string]$message) {
    Add-Content -LiteralPath $log -Value ('{0:o} {1}' -f (Get-Date), $message)
}
$others = Get-CimInstance Win32_Process | Where-Object {
    $_.ProcessId -ne $PID -and $_.CommandLine -like '*rerender-failed-bim.ps1*'
}
if ($others) {
    Say 'waiter already running'
    exit 0
}
function StillTheSame([int]$id, [datetime]$start) {
    $proc = Get-Process -Id $id -ErrorAction SilentlyContinue
    if (-not $proc) { return $false }
    return $proc.StartTime -le $start.AddMinutes(2)
}
$shellStart = Get-Date '2026-11-08T07:36:36'
$orchestratorStart = Get-Date '2026-11-08T07:36:38'
Say 'waiting for bim rest shell 63148 and orchestrator 15008'
while ((StillTheSame 63148 $shellStart) -or (StillTheSame 15008 $orchestratorStart)) {
    Start-Sleep -Seconds 30
}
Say 'bim rest exited; starting objects, representations, models, exchange and consistency'
$env:SEMIO_BIM_SLAB_CACHE = Join-Path $ticket '🗑️generated\full-slab-render'
Remove-Item Env:SEMIO_TEACHING_OUTPUT_ROOT -ErrorAction SilentlyContinue
Set-Location 'C:\git\semio'
& repo capacity run --operation=command --memory=16384 --cpu=1 -- repo tools exec bun -- '🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts' render --language en --audience speaker --quality l --fps 15 --jobs 1 --no-play --log-output --video bim_objects --video bim_representations --video bim_models --video bim_exchange --video bim_consistency
Say ("rerender exit {0}" -f $LASTEXITCODE)
exit $LASTEXITCODE

