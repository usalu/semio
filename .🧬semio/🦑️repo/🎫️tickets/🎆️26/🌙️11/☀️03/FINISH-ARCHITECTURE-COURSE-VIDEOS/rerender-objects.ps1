$ErrorActionPreference = "Stop"
$ticket = $PSScriptRoot
$log = Join-Path $ticket "🗑️generated\bim-objects-rerender.log"
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $log) | Out-Null
function Say([string]$message) { Add-Content -LiteralPath $log -Value ("{0:o} {1}" -f (Get-Date), $message) }
$start = Get-Date "2026-11-10T12:25:49"
Say "waiting for bim rerender repo 71516"
while ($true) {
    $proc = Get-Process -Id 71516 -ErrorAction SilentlyContinue
    if (-not $proc) { break }
    if ($proc.StartTime -gt $start.AddMinutes(2)) { break }
    Start-Sleep -Seconds 30
}
Say "rerender exited; starting objects"
$env:SEMIO_BIM_SLAB_CACHE = Join-Path $ticket "🗑️generated\full-slab-render"
Remove-Item Env:SEMIO_TEACHING_OUTPUT_ROOT -ErrorAction SilentlyContinue
Set-Location "C:\git\semio"
& repo capacity run --operation=command --memory=16384 --cpu=1 -- repo tools exec bun -- "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts" render --language en --audience speaker --quality l --fps 15 --jobs 1 --no-play --log-output --video bim_objects
Say ("objects exit {0}" -f $LASTEXITCODE)
exit $LASTEXITCODE
