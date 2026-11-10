$ErrorActionPreference='Stop'
$auditRows=@(Get-CimInstance Win32_Process -OperationTimeoutSec 5)
if($auditRows.Count -eq 0){throw 'Complete process table required'}
$auditOwn=[System.Collections.Generic.HashSet[int]]::new()
$auditAt=[int]$PID
for($auditDepth=0;$auditDepth -lt 16 -and $auditAt -gt 0;$auditDepth++){[void]$auditOwn.Add($auditAt);$auditParent=$auditRows|Where-Object{[int]$_.ProcessId -eq $auditAt}|Select-Object -First 1;if($null -eq $auditParent){break};$auditAt=[int]$auditParent.ParentProcessId}
$auditFound=@()
foreach($auditRow in $auditRows){if(-not $auditOwn.Contains([int]$auditRow.ProcessId) -and ([string]$auditRow.CommandLine).Contains('PagedListEditCursor|pop_source_step|swap_source_step')){$auditProcess=Get-Process -Id $auditRow.ProcessId -ErrorAction Stop;$auditFound+=[ordered]@{pid=[int]$auditRow.ProcessId;parent=[int]$auditRow.ParentProcessId;birth=$auditRow.CreationDate.ToUniversalTime().ToString('o');image=$auditRow.ExecutablePath;cpuMs=$auditProcess.TotalProcessorTime.TotalMilliseconds;workingSetBytes=$auditProcess.WorkingSet64;threads=$auditProcess.Threads.Count;isGit=([System.IO.Path]::GetFileName([string]$auditRow.ExecutablePath) -ieq 'git.exe');hasUntracked=([string]$auditRow.CommandLine).Contains('--untracked');hasExpectedLog=([string]$auditRow.CommandLine).Contains(('nov07-'+'paid-callers-git.log'))}}}
$auditResult=[ordered]@{observedUtc=[DateTime]::UtcNow.ToString('o');processes=$auditRows.Count;candidates=$auditFound}
$auditResult|ConvertTo-Json -Depth 5|Set-Content -LiteralPath 'D:/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️19/PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT/🗑️generated/nov07-git-audit-identity.json' -Encoding UTF8
Write-Output ('[DEBUG] '+($auditResult|ConvertTo-Json -Depth 5 -Compress))