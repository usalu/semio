param([int]$WriterId=25716,[Parameter(Mandatory=$true)][string]$Output)
$ErrorActionPreference='Stop'
$parent=Get-Process -Id $WriterId -ErrorAction Stop
if($parent.ProcessName -ne 'bun') { throw 'Known writer PID no longer belongs to Bun' }
$children=@(Get-CimInstance -ClassName Win32_Process -Filter ('ParentProcessId = '+$WriterId) -OperationTimeoutSec 5)
$rows=@()
for($sample=0;$sample -lt 2;$sample++) {
  $facts=@()
  foreach($known in $children) {
    $child=Get-CimInstance -ClassName Win32_Process -Filter ('ProcessId = '+$known.ProcessId+' AND ParentProcessId = '+$WriterId) -OperationTimeoutSec 5
    if($null -eq $child) { $facts+=,@{pid=$known.ProcessId;exited=$true}; continue }
    $process=Get-Process -Id $child.ProcessId -ErrorAction Stop
    $command=[string]$child.CommandLine
    $facts+=,[ordered]@{pid=[int]$child.ProcessId;parentPid=[int]$child.ParentProcessId;image=$child.ExecutablePath;creationUtc=$child.CreationDate.ToUniversalTime().ToString('o');frozenLockfile=($command -match '(?:^|\s)--frozen-lockfile(?:\s|=|$)');ignoreScripts=($command -match '(?:^|\s)--ignore-scripts(?:\s|=|$)');copyfile=($command -match '(?:^|\s)--backend(?:=|\s+)copyfile(?:\s|$)');cpuMs=$process.TotalProcessorTime.TotalMilliseconds;workingSetBytes=$process.WorkingSet64;privateBytes=$process.PrivateMemorySize64;readOperations=[string]$child.ReadOperationCount;writeOperations=[string]$child.WriteOperationCount;otherOperations=[string]$child.OtherOperationCount;readBytes=[string]$child.ReadTransferCount;writeBytes=[string]$child.WriteTransferCount;otherBytes=[string]$child.OtherTransferCount}
  }
  $rows+=,[ordered]@{observedUtc=[DateTime]::UtcNow.ToString('o');children=$facts}
  if($sample -eq 0) { Start-Sleep -Seconds 3 }
}
$result=[ordered]@{parentPid=$WriterId;parentStartUtc=$parent.StartTime.ToUniversalTime().ToString('o');samples=$rows;limitation='Only immediate children of the confirmed writer are sampled; command lines are reduced to three recipe booleans. No inference about dependency publication.'}
$result | ConvertTo-Json -Depth 7 | Set-Content -LiteralPath $Output -Encoding UTF8
Write-Output ('[DEBUG] '+($result | ConvertTo-Json -Depth 7 -Compress))
