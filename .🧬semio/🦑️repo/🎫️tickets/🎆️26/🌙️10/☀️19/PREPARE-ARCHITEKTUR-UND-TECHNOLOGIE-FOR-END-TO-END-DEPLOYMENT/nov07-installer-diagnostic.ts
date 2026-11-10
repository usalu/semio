import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
await runCmd('powershell.exe',['-NoProfile','-NonInteractive','-Command',`$ErrorActionPreference='Stop'
$row=Get-CimInstance Win32_Process -Filter 'ProcessId = 29576' -OperationTimeoutSec 10
if($null -eq $row -or [int]$row.ParentProcessId -ne 12572 -or [Math]::Abs(($row.CreationDate.ToUniversalTime()-[DateTime]::Parse('2026-11-07T03:39:36.0646050Z').ToUniversalTime()).TotalMilliseconds) -gt 1 -or [IO.Path]::GetFileName($row.ExecutablePath) -ine 'bun.exe'){throw 'Installer identity mismatch or process settled'}
$line=[string]$row.CommandLine
$flags=[ordered]@{}
foreach($flag in @('install','--frozen-lockfile','--ignore-scripts','--offline','--no-progress','--silent','--verbose')){$flags[$flag]=$line.Contains($flag)}
$proc=Get-Process -Id 29576 -ErrorAction Stop
$threads=@($proc.Threads | ForEach-Object {if($_.ThreadState -eq 'Wait'){'Wait:'+ $_.WaitReason}else{[string]$_.ThreadState}} | Group-Object | ForEach-Object {[ordered]@{state=$_.Name;count=$_.Count}})
$connections=@(Get-NetTCPConnection -OwningProcess 29576 -ErrorAction SilentlyContinue | Group-Object State | ForEach-Object {[ordered]@{state=[string]$_.Name;count=$_.Count}})
[ordered]@{observedUtc=[DateTime]::UtcNow.ToString('o');pid=29576;parent=12572;birth=$row.CreationDate.ToUniversalTime().ToString('o');arguments=$flags;threadStates=$threads;tcpStates=$connections;limitation='Connections are a snapshot; no result or exact await follows.'}|ConvertTo-Json -Depth 6
`]);
