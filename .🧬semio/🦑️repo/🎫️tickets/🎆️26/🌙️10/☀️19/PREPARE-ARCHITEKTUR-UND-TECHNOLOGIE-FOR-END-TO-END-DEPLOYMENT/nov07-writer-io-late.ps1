& {
param([int]$WriterId,[Parameter(Mandatory=$true)][string]$Output)
$ErrorActionPreference='Stop'
$rows=@()
for($sample=0;$sample -lt 2;$sample++) {
  $row=Get-CimInstance -ClassName Win32_Process -Filter ('ProcessId = '+$WriterId) -OperationTimeoutSec 5
  if($null -eq $row -or [System.IO.Path]::GetFileName($row.ExecutablePath) -ine 'bun.exe') { throw 'Expected Bun writer is absent or changed' }
  if([int]$row.ParentProcessId -ne 6064 -or -not ([string]$row.CommandLine).Contains('35fff2ced85c1fb351c3e6451b210b175ad2e025bd7a2ed0db113b9a8e15aeb0')) { throw 'Expected writer command or parent changed' }
  if($row.CreationDate.ToUniversalTime().ToString('o') -ne '2026-11-07T02:43:01.3718540Z') { throw 'Expected original writer birth changed' }
  $process=Get-Process -Id $WriterId -ErrorAction Stop
  $rows+=,[ordered]@{observedUtc=[DateTime]::UtcNow.ToString('o');pid=[int]$row.ProcessId;parentPid=[int]$row.ParentProcessId;image=$row.ExecutablePath;creationUtc=$row.CreationDate.ToUniversalTime().ToString('o');cpuMs=$process.TotalProcessorTime.TotalMilliseconds;workingSetBytes=$process.WorkingSet64;readBytes=[string]$row.ReadTransferCount;writeBytes=[string]$row.WriteTransferCount;readOperations=[string]$row.ReadOperationCount;writeOperations=[string]$row.WriteOperationCount;otherBytes=[string]$row.OtherTransferCount;otherOperations=[string]$row.OtherOperationCount}
  if($sample -eq 0) { Start-Sleep -Seconds 10 }
}
if($rows[0].creationUtc -ne $rows[1].creationUtc -or $rows[0].image -ne $rows[1].image) { throw 'Writer identity changed during sample' }
$result=[ordered]@{samples=$rows;cpuDeltaMs=($rows[1].cpuMs-$rows[0].cpuMs);elapsedMs=([DateTime]::Parse($rows[1].observedUtc)-[DateTime]::Parse($rows[0].observedUtc)).TotalMilliseconds;limitation='Process counters do not establish dependency publication or stage completion.'}
$result | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath $Output -Encoding UTF8
Write-Output ('[DEBUG] '+($result | ConvertTo-Json -Depth 5 -Compress))

} -WriterId 12572 -Output 'D:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️19\PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT\🗑️generated\nov07-writer-io-late.json'