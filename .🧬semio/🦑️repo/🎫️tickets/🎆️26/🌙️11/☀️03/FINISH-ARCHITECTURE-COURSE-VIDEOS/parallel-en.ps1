# Fill idle cores with lessons whose segments do not overlap the anatomy render.
# This driver does not take videos\en\.media-lock, so the serial anatomy run keeps that lock.
$ErrorActionPreference = 'Continue'
$cache = 'C:\git\semio\.🧬semio\🎓️teaching\⚡️cache'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:SEMIO_TEACHING_CACHE_ROOT = $cache
$env:SEMIO_TEACHING_OUTPUT_ROOT = Join-Path $cache 'videos'
$env:UV_PROJECT_ENVIRONMENT = Join-Path $cache 'python'
$script = 'C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📜️script.ts'
$serialLog = Join-Path $PSScriptRoot 'generated\render-en.log'
$out = Join-Path $PSScriptRoot 'generated'
New-Item -ItemType Directory -Force -Path $out | Out-Null
$queue = [System.Collections.Queue]::new()
foreach ($video in @(
  'optimization_extrema','optimization_sampling','optimization_experiments','optimization_index','optimization_pareto','optimization_algorithms'
)) { $queue.Enqueue($video) }
$running = @{}
$minFreeGb = 8
while ($queue.Count -gt 0 -or $running.Count -gt 0) {
  foreach ($id in @($running.Keys)) {
    if ($running[$id].HasExited) {
      "[DEBUG] $id exit=$($running[$id].ExitCode) $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-en.log') -Append
      $running.Remove($id)
    }
  }
  $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  $serial = if (Test-Path -LiteralPath $serialLog) { Get-Content -LiteralPath $serialLog -Raw -ErrorAction SilentlyContinue } else { '' }
  while ($queue.Count -gt 0 -and $freeGb -gt $minFreeGb -and $running.Count -lt 6) {
    $video = $queue.Peek()
    if ($serial -match [regex]::Escape("[DEBUG] $video attempt")) { $queue.Dequeue(); continue }
    $queue.Dequeue()
    $lessonLog = Join-Path $out "parallel-$video.log"
    "[DEBUG] start $video freeGb=$freeGb $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-en.log') -Append
    $proc = Start-Process -FilePath 'repo' -WorkingDirectory 'C:\git\semio' -PassThru -WindowStyle Hidden `
      -RedirectStandardOutput $lessonLog -RedirectStandardError (Join-Path $out "parallel-$video.err.log") `
      -ArgumentList @('tools','exec','bun','--',$script,'render','--video',$video,'--language','en','--audience','speaker','--quality','l','--fps','15','--no-play','--log-output','--jobs','1')
    $running[$video] = $proc
    $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  }
  if ($running.Count -gt 0) { Start-Sleep -Seconds 20 }
}
"[DEBUG] parallel queue done $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-en.log') -Append
