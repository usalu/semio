import{join}from"node:path";
import{pathToFileURL}from"node:url";
const{runCmd}=await import(pathToFileURL(join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
runCmd("powershell.exe",["-NoProfile","-NonInteractive","-Command","$p=@(Get-CimInstance Win32_Process -OperationTimeoutSec 10 | Where-Object { $_.CommandLine -like '*show project workspace*' -and $_.CommandLine -notlike '*inspect-show-workspace*' }); foreach($x in $p){Write-Output ('[DEBUG] PID='+$x.ProcessId+' Parent='+$x.ParentProcessId+' Born='+$x.CreationDate.ToUniversalTime().ToString('o')+' Command='+$x.CommandLine)}; Write-Output ('[DEBUG] Matching processes='+$p.Count)"]);
