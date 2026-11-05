# Fill idle cores with the eight tower lessons. Ordinals 121-128 only.
# This driver does not take videos\en\.media-lock and does not write render-en.log.
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
"[DEBUG] boot $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-rest.log') -Append
$queue = [System.Collections.Queue]::new()
foreach ($video in @(
  'excel_energy','excel_program','excel_references','excel_anatomy',
  'grasshopper_trees','grasshopper_casting','grasshopper_tower','grasshopper_capsule'
)) { $queue.Enqueue($video) }
$running = @{}
$minFreeGb = 12
$lastStart = [datetime]::MinValue

function Stop-Tree([int]$ProcessId) {
  if ($ProcessId -in 77580, 79444, 66820, 0) { return }
  $children = @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$ProcessId" -ErrorAction SilentlyContinue)
  foreach ($child in $children) { Stop-Tree ([int]$child.ProcessId) }
  Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
}

while ($queue.Count -gt 0 -or $running.Count -gt 0) {
  $serial = if (Test-Path -LiteralPath $serialLog) { Get-Content -LiteralPath $serialLog -Raw -ErrorAction SilentlyContinue } else { '' }
  foreach ($id in @($running.Keys)) {
    if ($serial -match [regex]::Escape("[DEBUG] $id attempt")) {
      "[DEBUG] yield $id to serial $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-rest.log') -Append
      Stop-Tree $running[$id].Id
      $running.Remove($id)
      continue
    }
    if ($running[$id].HasExited) {
      "[DEBUG] $id exit=$($running[$id].ExitCode) $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-rest.log') -Append
      $running.Remove($id)
    }
  }
  $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  $ready = ((Get-Date) - $lastStart).TotalSeconds -ge 20
  while ($queue.Count -gt 0 -and $freeGb -gt $minFreeGb -and $running.Count -lt 8 -and $ready) {
    $video = $queue.Peek()
    if ($serial -match [regex]::Escape("[DEBUG] $video attempt")) { $queue.Dequeue(); continue }
    $queue.Dequeue()
    $lessonLog = Join-Path $out "parallel-$video.log"
    "[DEBUG] start $video freeGb=$freeGb $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-rest.log') -Append
    $proc = Start-Process -FilePath 'repo' -WorkingDirectory 'C:\git\semio' -PassThru -WindowStyle Hidden `
      -RedirectStandardOutput $lessonLog -RedirectStandardError (Join-Path $out "parallel-$video.err.log") `
      -ArgumentList @('tools','exec','bun','--',$script,'render','--video',$video,'--language','en','--audience','speaker','--quality','l','--fps','15','--no-play','--log-output','--jobs','1')
    $running[$video] = $proc
    $lastStart = Get-Date
    $ready = $false
    $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  }
  if ($running.Count -gt 0 -or $queue.Count -gt 0) { Start-Sleep -Seconds 5 }
}
"[DEBUG] parallel rest done $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath (Join-Path $out 'parallel-rest.log') -Append
