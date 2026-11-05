# Fill cores with the lessons still ahead of the serial walker.
# No media lock, no render-en.log. Light scenes share up to six slots.
# Tower and the two large Excel lessons need 18 GB free and a slot of their own.
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
$log = Join-Path $out 'parallel-ahead.log'
"[DEBUG] boot wider $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
$order = @(
  'grasshopper_capsule','grasshopper_casting','grasshopper_trees',
  'excel_anatomy','excel_references','grasshopper_tower',
  'excel_program','excel_energy'
)
$heavy = @('excel_program','excel_energy')
$queue = [System.Collections.Queue]::new()
foreach ($video in $order) { $queue.Enqueue($video) }
$running = @{}
$lastStart = [datetime]::MinValue

function Stop-Tree([int]$ProcessId) {
  if ($ProcessId -in 77580, 79444, 66820, 101176, 34920, 55304, 54492, 78648, 22504, 103636, 89820, 0) { return }
  $children = @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$ProcessId" -ErrorAction SilentlyContinue)
  foreach ($child in $children) { Stop-Tree ([int]$child.ProcessId) }
  Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
}

function Repo-Root([int]$ProcessId) {
  $cur = Get-CimInstance Win32_Process -Filter "ProcessId=$ProcessId" -ErrorAction SilentlyContinue
  while ($cur -and $cur.Name -ne 'repo.exe' -and $cur.ParentProcessId -gt 4) {
    $parent = Get-CimInstance Win32_Process -Filter "ProcessId=$($cur.ParentProcessId)" -ErrorAction SilentlyContinue
    if (-not $parent) { break }
    $cur = $parent
  }
  if ($cur -and $cur.Name -eq 'repo.exe') { return [int]$cur.ProcessId }
  return 0
}

$py = @(Get-CimInstance Win32_Process -Filter "Name='python.exe'" -ErrorAction SilentlyContinue)
foreach ($video in $order) {
  $holders = @($py | Where-Object { $_.CommandLine -and $_.CommandLine -like "*--video $video *" })
  foreach ($holder in $holders) {
    $root = Repo-Root ([int]$holder.ProcessId)
    if ($root -gt 0 -and -not $running.ContainsKey($video)) {
      $running[$video] = Get-Process -Id $root -ErrorAction SilentlyContinue
      "[DEBUG] adopt $video repo=$root $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
    }
  }
}

while ($queue.Count -gt 0 -or $running.Count -gt 0) {
  $serial = if (Test-Path -LiteralPath $serialLog) { Get-Content -LiteralPath $serialLog -Raw -ErrorAction SilentlyContinue } else { '' }
  foreach ($id in @($running.Keys)) {
    if ($serial -match [regex]::Escape("[DEBUG] $id attempt")) {
      "[DEBUG] yield $id to serial $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
      if ($running[$id]) { Stop-Tree $running[$id].Id }
      $running.Remove($id)
      continue
    }
    if (-not $running[$id] -or $running[$id].HasExited) {
      $code = if ($running[$id]) { $running[$id].ExitCode } else { 'missing' }
      "[DEBUG] $id exit=$code $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
      $running.Remove($id)
    }
  }
  $freeGb = [math]::Round((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1MB, 1)
  $ready = ((Get-Date) - $lastStart).TotalSeconds -ge 40
  if ($queue.Count -gt 0 -and $ready) {
    $video = $queue.Peek()
    if ($running.ContainsKey($video) -or ($serial -match [regex]::Escape("[DEBUG] $video attempt"))) {
      $queue.Dequeue()
      continue
    }
    $isHeavy = $heavy -contains $video
    $heavyRunning = @($running.Keys | Where-Object { $heavy -contains $_ }).Count
    $admit = $freeGb -gt 10 -and $running.Count -lt 6
    if ($isHeavy) { $admit = $freeGb -gt 18 -and $heavyRunning -lt 1 -and $running.Count -lt 6 }
    if ($admit) {
      $queue.Dequeue()
      $lessonLog = Join-Path $out "parallel-$video.log"
      "[DEBUG] start $video freeGb=$freeGb $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
      $proc = Start-Process -FilePath 'repo' -WorkingDirectory 'C:\git\semio' -PassThru -WindowStyle Hidden `
        -RedirectStandardOutput $lessonLog -RedirectStandardError (Join-Path $out "parallel-$video.err.log") `
        -ArgumentList @('tools','exec','bun','--',$script,'render','--video',$video,'--language','en','--audience','speaker','--quality','l','--fps','15','--no-play','--log-output','--jobs','1')
      $running[$video] = $proc
      $lastStart = Get-Date
    }
  }
  if ($running.Count -gt 0 -or $queue.Count -gt 0) { Start-Sleep -Seconds 5 }
}
"[DEBUG] parallel ahead done $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $log -Append
