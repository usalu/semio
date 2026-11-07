import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
runCmd('powershell.exe',['-NoProfile','-NonInteractive','-Command',"Get-CimInstance Win32_Process -Filter \"Name='bun.exe' OR Name='repo.exe'\" -OperationTimeoutSec 5 | Where-Object { $_.CommandLine -like '*stop-owned-preflight.ts*' -and $_.CommandLine -like '*capacity*' -and $_.CommandLine -like '*512*' } | Select-Object ProcessId,ParentProcessId,CommandLine | Format-List"]);
