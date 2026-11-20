# 🧹️ Removes cargo build-dir units of the shared native lane whose newest file is older than the threshold; cargo rebuilds any it needs again.
param([string]$Root = 'C:\semio\.cache\b\r\m\u\l\d\b\debug\build', [int]$Hours = 6, [switch]$DryRun)
$cut = (Get-Date).AddHours(-$Hours); $freed = 0; $count = 0
foreach ($unit in Get-ChildItem $Root -Directory -Force | Get-ChildItem -Directory -Force) {
  $files = Get-ChildItem $unit.FullName -Recurse -File -Force -ErrorAction SilentlyContinue
  $newest = ($files | Measure-Object LastWriteTime -Maximum).Maximum
  if ($newest -and $newest -ge $cut) { continue }
  $size = ($files | Measure-Object Length -Sum).Sum
  if (-not $DryRun) { Remove-Item -LiteralPath $unit.FullName -Recurse -Force -ErrorAction SilentlyContinue; if (Test-Path -LiteralPath $unit.FullName) { continue } }
  $freed += $size; $count++
}
'{0} units, {1:N1} GB {2}' -f $count, ($freed / 1GB), ($(if ($DryRun) { 'would be freed' } else { 'freed' }))
