# After the current English media lock is released, retry energy balance, then cooling,
# then the six English BIM lessons.
$ErrorActionPreference = 'Continue'
$cache = 'C:\git\semio\.🧬semio\🎓️teaching\⚡️cache'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:SEMIO_TEACHING_CACHE_ROOT = $cache
$env:UV_PROJECT_ENVIRONMENT = Join-Path $cache 'python'
$lock = Join-Path $cache 'videos\en\.media-lock'
$script = 'C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📜️script.ts'
$log = Join-Path $PSScriptRoot 'generated\render-bim.log'
New-Item -ItemType Directory -Force -Path (Split-Path $log) | Out-Null
while (Test-Path -LiteralPath $lock) {
  "[DEBUG] waiting for $lock $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
  Start-Sleep -Seconds 60
}
New-Item -ItemType Directory -Path $lock | Out-Null
$videos = @('energy_balance','cooling','bim_information','bim_objects','bim_representations','bim_models','bim_exchange','bim_consistency')
try {
  Set-Location 'C:\git\semio'
  foreach ($video in $videos) {
    $ok = $false
    foreach ($attempt in 1..2) {
      "[DEBUG] $video attempt=$attempt $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
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
