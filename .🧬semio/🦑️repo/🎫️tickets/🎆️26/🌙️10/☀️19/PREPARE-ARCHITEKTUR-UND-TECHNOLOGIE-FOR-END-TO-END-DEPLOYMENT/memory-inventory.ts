import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
runCmd('powershell.exe',['-NoProfile','-NonInteractive','-Command',"$rows=Get-Process | Sort-Object WorkingSet64 -Descending | Select-Object -First 20 Id,ProcessName,WorkingSet64,CPU; $rows | Format-Table -AutoSize; foreach($row in $rows){if($row.ProcessName -match '^(node|bun|repo)$'){ $p=Get-CimInstance Win32_Process -Filter ('ProcessId='+$row.Id) -OperationTimeoutSec 5; if($p){[PSCustomObject]@{Id=$p.ProcessId;Parent=$p.ParentProcessId;Image=$p.ExecutablePath;Command=$p.CommandLine} | Format-List} }}"]);
