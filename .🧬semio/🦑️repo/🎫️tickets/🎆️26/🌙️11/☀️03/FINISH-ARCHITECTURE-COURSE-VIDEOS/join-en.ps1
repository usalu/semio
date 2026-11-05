# After the six BIM lessons settle, write the English media manifest and join every fresh master into the course preview.
$ErrorActionPreference = 'Continue'
$cache = 'C:\git\semio\.🧬semio\🎓️teaching\⚡️cache'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:SEMIO_TEACHING_CACHE_ROOT = $cache
$env:UV_PROJECT_ENVIRONMENT = Join-Path $cache 'python'
$lock = Join-Path $cache 'videos\en\.media-lock'
$py = 'C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python'
$media = 'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS\🧰️r3-media'
$bimLog = Join-Path $PSScriptRoot 'generated\render-bim.log'
$log = Join-Path $PSScriptRoot 'generated\join-en.log'
New-Item -ItemType Directory -Force -Path (Split-Path $log) | Out-Null
while ($true) {
  $text = if (Test-Path -LiteralPath $bimLog) { Get-Content -LiteralPath $bimLog -Raw -ErrorAction SilentlyContinue } else { '' }
  if ($text -match 'bim_consistency gave up') { "[DEBUG] BIM did not finish $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append; exit 1 }
  if ($text -match 'bim_consistency ok' -and -not (Test-Path -LiteralPath $lock)) { break }
  "[DEBUG] waiting for BIM and the English lock $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
  Start-Sleep -Seconds 60
}
Set-Location 'C:\git\semio'
"[DEBUG] manifest $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
& repo tools exec uv -- run --project $py --python 3.12 python (Join-Path $media 'manifest.py') en speaker 2>&1 | Tee-Object -FilePath $log -Append
if ($LASTEXITCODE -ne 0) { "[DEBUG] manifest failed $LASTEXITCODE" | Tee-Object -FilePath $log -Append; exit $LASTEXITCODE }
"[DEBUG] course preview $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
& repo tools exec uv -- run --project $py --python 3.12 python (Join-Path $media 'course_preview.py') en 2>&1 | Tee-Object -FilePath $log -Append
"[DEBUG] join exit $LASTEXITCODE $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
exit $LASTEXITCODE
