# Watch the two Excel renders started beside anatomy. Stop one if the serial
# driver logs an attempt for it, so that lesson has a single writer.
$ErrorActionPreference = 'Continue'
$serialLog = Join-Path $PSScriptRoot 'generated\render-en.log'
$out = Join-Path $PSScriptRoot 'generated\parallel-rest.log'
$videos = @('excel_energy','excel_program')

function Stop-Tree([int]$ProcessId) {
  if ($ProcessId -in 77580, 79444, 66820, 48908, 0) { return }
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

"[DEBUG] yield watch $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $out -Append
$alive = @{}
foreach ($video in $videos) { $alive[$video] = $true }
while ($alive.Values -contains $true) {
  $serial = if (Test-Path -LiteralPath $serialLog) { Get-Content -LiteralPath $serialLog -Raw -ErrorAction SilentlyContinue } else { '' }
  $procs = @(Get-CimInstance Win32_Process -Filter "Name='python.exe'" -ErrorAction SilentlyContinue)
  foreach ($video in @($alive.Keys)) {
    if (-not $alive[$video]) { continue }
    $holders = @($procs | Where-Object { $_.CommandLine -and $_.CommandLine -like "*--video $video *" })
    if ($serial -match [regex]::Escape("[DEBUG] $video attempt")) {
      "[DEBUG] yield $video to serial $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $out -Append
      $roots = @{}
      foreach ($holder in $holders) {
        $root = Repo-Root ([int]$holder.ProcessId)
        if ($root -gt 0) { $roots[$root] = $true }
      }
      foreach ($root in $roots.Keys) { Stop-Tree ([int]$root) }
      $alive[$video] = $false
      continue
    }
    if ($holders.Count -eq 0) {
      "[DEBUG] $video left the process list $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $out -Append
      $alive[$video] = $false
    }
  }
  if ($alive.Values -contains $true) { Start-Sleep -Seconds 5 }
}
"[DEBUG] yield watch done $(Get-Date -Format 'HH:mm:ss')" | Tee-Object -FilePath $out -Append
