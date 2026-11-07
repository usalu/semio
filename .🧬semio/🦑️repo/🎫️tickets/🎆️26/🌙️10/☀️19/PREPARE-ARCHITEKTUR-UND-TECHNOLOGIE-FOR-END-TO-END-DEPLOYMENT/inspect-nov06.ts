import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
runCmd('powershell.exe',['-NoProfile','-NonInteractive','-Command',"$ids=29280,32524,27172,32668,13024; foreach($id in $ids) { $p=Get-CimInstance Win32_Process -Filter ('ProcessId = '+$id) -OperationTimeoutSec 5; if($p) { $g=Get-Process -Id $id -ErrorAction SilentlyContinue; [PSCustomObject]@{pid=$id;parent=$p.ParentProcessId;image=$p.ExecutablePath;birth=$p.CreationDate.ToUniversalTime().ToString('o');command=$p.CommandLine;cpu=$g.CPU} | ConvertTo-Json -Compress } else { Write-Output ('absent '+$id) } }"]);
