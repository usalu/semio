$ErrorActionPreference='Stop'
$owner=Get-CimInstance Win32_Process -Filter 'ProcessId = 12572' -OperationTimeoutSec 5
if($null -eq $owner -or $owner.CreationDate.ToUniversalTime().ToString('o') -ne '2026-11-07T02:43:01.3718540Z' -or [int]$owner.ParentProcessId -ne 6064 -or -not ([string]$owner.CommandLine).Contains('35fff2ced85c1fb351c3e6451b210b175ad2e025bd7a2ed0db113b9a8e15aeb0')){throw 'Original dependency owner changed'}
$children=@(Get-CimInstance Win32_Process -Filter 'ParentProcessId = 12572' -OperationTimeoutSec 5|Where-Object{[System.IO.Path]::GetFileName([string]$_.ExecutablePath) -ieq 'bun.exe'})
if($children.Count -ne 1){throw ('Expected one current owned Bun child, observed '+$children.Count)}
$child=$children[0];$childId=[int]$child.ProcessId;$childBirth=$child.CreationDate.ToUniversalTime().ToString('o');$childImage=[string]$child.ExecutablePath
$classification=[ordered]@{install=([string]$child.CommandLine).Contains('install');frozenLockfile=([string]$child.CommandLine).Contains('--frozen-lockfile');ignoreScripts=([string]$child.CommandLine).Contains('--ignore-scripts')}
$activityRows=@()
for($sample=0;$sample -lt 2;$sample++){
 $row=Get-CimInstance Win32_Process -Filter ('ProcessId = '+$childId) -OperationTimeoutSec 5
 if($null -eq $row -or [int]$row.ParentProcessId -ne 12572 -or [string]$row.ExecutablePath -cne $childImage -or $row.CreationDate.ToUniversalTime().ToString('o') -ne $childBirth){throw 'Owned Bun child ended or changed during sample'}
 $process=Get-Process -Id $childId -ErrorAction Stop
 $activityRows+=,[ordered]@{observedUtc=[DateTime]::UtcNow.ToString('o');pid=$childId;parent=12572;birth=$childBirth;image=$childImage;cpuMs=$process.TotalProcessorTime.TotalMilliseconds;workingSetBytes=$process.WorkingSet64;readBytes=[string]$row.ReadTransferCount;writeBytes=[string]$row.WriteTransferCount;readOperations=[string]$row.ReadOperationCount;writeOperations=[string]$row.WriteOperationCount;otherBytes=[string]$row.OtherTransferCount;otherOperations=[string]$row.OtherOperationCount}
 if($sample -eq 0){Start-Sleep -Seconds 10}
}
$ownerAfter=Get-CimInstance Win32_Process -Filter 'ProcessId = 12572' -OperationTimeoutSec 5
if($null -eq $ownerAfter -or $ownerAfter.CreationDate.ToUniversalTime().ToString('o') -ne '2026-11-07T02:43:01.3718540Z'){throw 'Original owner changed during sample'}
$activityResult=[ordered]@{classification=$classification;samples=$activityRows;elapsedMs=([DateTime]::Parse($activityRows[1].observedUtc)-[DateTime]::Parse($activityRows[0].observedUtc)).TotalMilliseconds;cpuDeltaMs=($activityRows[1].cpuMs-$activityRows[0].cpuMs);limitation='Process activity does not establish installer outcome or all-current readiness.'}
$activityResult|ConvertTo-Json -Depth 5|Set-Content -LiteralPath 'D:/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️19/PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT/🗑️generated/nov07-current-bun-activity.json' -Encoding UTF8
Write-Output ('[DEBUG] '+($activityResult|ConvertTo-Json -Depth 5 -Compress))