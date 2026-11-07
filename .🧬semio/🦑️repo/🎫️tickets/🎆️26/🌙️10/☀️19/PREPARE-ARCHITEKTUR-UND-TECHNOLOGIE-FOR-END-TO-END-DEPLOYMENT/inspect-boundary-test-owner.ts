import{join}from"node:path";
import{pathToFileURL}from"node:url";
const{runCmd}=await import(pathToFileURL(join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
runCmd("powershell.exe",["-NoProfile","-NonInteractive","-Command","$ErrorActionPreference='Stop'; $rows=Get-CimInstance Win32_Process -Filter \"Name = 'bun.exe'\" -OperationTimeoutSec 5 | Where-Object { $_.CommandLine -like '*native-toolchain*' -and $_.CommandLine -like '*test*' }; foreach($p in $rows){Write-Output ('[DEBUG] PID='+$p.ProcessId+' Parent='+$p.ParentProcessId+' Born='+$p.CreationDate.ToUniversalTime().ToString('o')+' Command='+$p.CommandLine)}; Write-Output ('[DEBUG] Matching processes='+@($rows).Count); exit 0"]);
