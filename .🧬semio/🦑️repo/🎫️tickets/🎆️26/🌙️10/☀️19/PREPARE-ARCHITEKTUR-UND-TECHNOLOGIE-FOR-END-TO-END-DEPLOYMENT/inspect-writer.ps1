param([int]$WriterId=25716,[Parameter(Mandatory=$true)][string]$Output)
$ErrorActionPreference='Stop'
$rows=@()
for($sample=0;$sample -lt 2;$sample++) {
  $writer=Get-Process -Id $WriterId -ErrorAction Stop
  if($writer.ProcessName -ne 'bun') { throw 'Known writer PID no longer belongs to Bun; refuse process substitution' }
  $writer.Refresh()
  $threads=@($writer.Threads | Group-Object ThreadState | ForEach-Object { @{state=$_.Name;count=$_.Count} })
  $row=[ordered]@{observedUtc=[DateTime]::UtcNow.ToString('o');pid=$writer.Id;name=$writer.ProcessName;startUtc=$writer.StartTime.ToUniversalTime().ToString('o');cpuMs=$writer.TotalProcessorTime.TotalMilliseconds;userCpuMs=$writer.UserProcessorTime.TotalMilliseconds;kernelCpuMs=$writer.PrivilegedProcessorTime.TotalMilliseconds;workingSetBytes=$writer.WorkingSet64;privateBytes=$writer.PrivateMemorySize64;handles=$writer.HandleCount;threadStates=$threads}
  $rows+=,$row
  if($sample -eq 0) { Start-Sleep -Seconds 3 }
}
$result=[ordered]@{samples=$rows;cpuDeltaMs=($rows[1].cpuMs-$rows[0].cpuMs);elapsedMs=([DateTime]::Parse($rows[1].observedUtc)-[DateTime]::Parse($rows[0].observedUtc)).TotalMilliseconds;limitation='Basic native process counters only; no inference about Store traversal completion or dependency readiness.'}
$result | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $Output -Encoding UTF8
Write-Output ('[DEBUG] '+($result | ConvertTo-Json -Depth 6 -Compress))
