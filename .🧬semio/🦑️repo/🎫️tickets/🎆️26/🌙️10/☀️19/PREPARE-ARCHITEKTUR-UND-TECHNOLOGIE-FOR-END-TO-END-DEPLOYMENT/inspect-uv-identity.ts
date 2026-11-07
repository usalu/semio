import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
runCmd('powershell.exe',['-NoProfile','-NonInteractive','-Command',"$p=Get-CimInstance Win32_Process -Filter 'ProcessId=31828' -OperationTimeoutSec 5;if(!$p){Write-Output 'absent 31828'}else{[PSCustomObject]@{pid=$p.ProcessId;parent=$p.ParentProcessId;birth=$p.CreationDate.ToUniversalTime().ToString('o');image=$p.ExecutablePath;tail=$p.CommandLine.Substring([Math]::Max(0,$p.CommandLine.Length-300))}|Format-List}"]);
