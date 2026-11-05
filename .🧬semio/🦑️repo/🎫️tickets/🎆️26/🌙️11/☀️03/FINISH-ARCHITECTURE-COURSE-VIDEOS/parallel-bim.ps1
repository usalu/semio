# Fill idle cores with the six English BIM lessons while anatomy still holds videos\en\.media-lock.
# This driver does not take that lock and does not write render-bim.log, so the waiting BIM pass and join stay intact.
$ErrorActionPreference = 'Continue'
$cache = 'C:\git\semio\.🧬semio\🎓️teaching\⚡️cache'
$env:PYTHONUTF8 = '1'
$env:PYTHONIOENCODING = 'utf-8'
$env:PYTHONDONTWRITEBYTECODE = '1'
$env:SEMIO_TEACHING_CACHE_ROOT = $cache
$env:SEMIO_TEACHING_OUTPUT_ROOT = Join-Path $cache 'videos'
$env:UV_PROJECT_ENVIRONMENT = Join-Path $cache 'python'
$script = 'C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📜️script.ts'
$serialLog = Join-Path $PSScriptRoot 'generated\render-bim.log'
$out = Join-Path $PSScriptRoot 'generated'
New-Item -ItemType Directory -Force -Path $out | Out-Null
$queue = [System.Collections.Queue]::new()
foreach ($video in @(
  'bim_information','bim_objects','bim_representations','bim_models','bim_exchange','bim_consistency'
)) { $queue.Enqueue($video) }
$running = @{}
$minFreeGb = 8
while ($queue.Count -gt 0 -or $running.Count -gt 0) {
  foreach ($id in @($running.Keys)) {
    if ($running[$id].HasExited) {
      "[DEBUG] $id exit=$($running[$id].ExitCode) $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-bim.log') -Append
      $running.Remove($id)
    }
  }
  $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  $serial = if (Test-Path -LiteralPath $serialLog) { Get-Content -LiteralPath $serialLog -Raw -ErrorAction SilentlyContinue } else { '' }
  while ($queue.Count -gt 0 -and $freeGb -gt $minFreeGb -and $running.Count -lt 4) {
    $video = $queue.Peek()
    if ($serial -match [regex]::Escape("[DEBUG] $video attempt")) { $queue.Dequeue(); continue }
    $queue.Dequeue()
    $lessonLog = Join-Path $out "parallel-$video.log"
    "[DEBUG] start $video freeGb=$freeGb $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-bim.log') -Append
    $proc = Start-Process -FilePath 'repo' -WorkingDirectory 'C:\git\semio' -PassThru -WindowStyle Hidden `
      -RedirectStandardOutput $lessonLog -RedirectStandardError (Join-Path $out "parallel-$video.err.log") `
      -ArgumentList @('tools','exec','bun','--',$script,'render','--video',$video,'--language','en','--audience','speaker','--quality','l','--fps','15','--no-play','--log-output','--jobs','1')
    $running[$video] = $proc
    Start-Sleep -Seconds 20
    $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  }
  if ($running.Count -gt 0) { Start-Sleep -Seconds 20 }
}
"[DEBUG] parallel bim queue done $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-bim.log') -Append
