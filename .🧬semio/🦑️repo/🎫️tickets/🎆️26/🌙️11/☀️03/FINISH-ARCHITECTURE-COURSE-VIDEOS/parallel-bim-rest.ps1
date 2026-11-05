# Render the two BIM lessons still queued after the first four were already running.
$ErrorActionPreference = 'Continue'
$cache = 'C:\git\semio\.🧬semio\🎓️teaching\⚡️cache'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:SEMIO_TEACHING_CACHE_ROOT = $cache
$env:SEMIO_TEACHING_OUTPUT_ROOT = Join-Path $cache 'videos'
$env:UV_PROJECT_ENVIRONMENT = Join-Path $cache 'python'
$script = 'C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📜️script.ts'
$out = Join-Path $PSScriptRoot 'generated'
New-Item -ItemType Directory -Force -Path $out | Out-Null
foreach ($video in @('bim_exchange','bim_consistency')) {
  $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  if ($freeGb -le 8) { "[DEBUG] skip $video freeGb=$freeGb $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-bim.log') -Append; continue }
  "[DEBUG] start $video freeGb=$freeGb $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-bim.log') -Append
  Start-Process -FilePath 'repo' -WorkingDirectory 'C:\git\semio' -WindowStyle Hidden `
    -RedirectStandardOutput (Join-Path $out "parallel-$video.log") -RedirectStandardError (Join-Path $out "parallel-$video.err.log") `
    -ArgumentList @('tools','exec','bun','--',$script,'render','--video',$video,'--language','en','--audience','speaker','--quality','l','--fps','15','--no-play','--log-output','--jobs','1')
  Start-Sleep -Seconds 15
}
"[DEBUG] bim rest launched $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-bim.log') -Append
