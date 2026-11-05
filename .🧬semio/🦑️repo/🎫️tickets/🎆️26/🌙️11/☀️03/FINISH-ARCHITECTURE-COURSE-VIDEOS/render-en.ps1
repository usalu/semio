# Render every English speaker lesson except the six BIM lessons.
# BIM waits on the roof-gravel finish reserve. One lesson at a time, three attempts.
$ErrorActionPreference = 'Continue'
$cache = 'C:\git\semio\.🧬semio\🎓️teaching\⚡️cache'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:SEMIO_TEACHING_CACHE_ROOT = $cache
$env:UV_PROJECT_ENVIRONMENT = Join-Path $cache 'python'
$lock = Join-Path $cache 'videos\en\.media-lock'
$script = 'C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📜️script.ts'
$log = Join-Path $PSScriptRoot 'generated\render-en.log'
New-Item -ItemType Directory -Force -Path (Split-Path $log) | Out-Null
if (Test-Path -LiteralPath $lock) { throw "another English render holds $lock" }
New-Item -ItemType Directory -Path $lock | Out-Null
$videos = @(
  'physical_fundamentals','ventilation','heating','cooling','energy_balance',
  'grasshopper_anatomy','grasshopper_capsule','grasshopper_tower','grasshopper_casting','grasshopper_trees',
  'excel_anatomy','excel_references','excel_program','excel_energy',
  'optimization_extrema','optimization_sampling','optimization_experiments','optimization_index','optimization_pareto','optimization_algorithms'
)
try {
  Set-Location 'C:\git\semio'
  foreach ($video in $videos) {
    $ok = $false
    foreach ($attempt in 1..3) {
      $stamp = Get-Date -Format 'HH:mm:ss'
      "[DEBUG] $video attempt=$attempt $stamp" | Tee-Object -FilePath $log -Append
      & repo tools exec bun -- $script render --video $video --language en --audience speaker --quality l --fps 15 --no-play --log-output --jobs 1 2>&1 |
        Tee-Object -FilePath $log -Append
      if ($LASTEXITCODE -eq 0) { $ok = $true; "[DEBUG] $video ok attempt=$attempt" | Tee-Object -FilePath $log -Append; break }
      "[DEBUG] $video failed attempt=$attempt exit=$LASTEXITCODE" | Tee-Object -FilePath $log -Append
    }
    if (-not $ok) { "[DEBUG] $video gave up" | Tee-Object -FilePath $log -Append }
  }
} finally {
  Remove-Item -LiteralPath $lock -Force -Recurse -ErrorAction SilentlyContinue
}
