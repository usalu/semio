import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {writeFileSync} from "node:fs";
const root=new URL("../../../../../../../",import.meta.url),ticket=fileURLToPath(new URL(".",import.meta.url));
const {captureOwnedCommandV1}=await import(new URL("🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts",root));
const system=join(process.env.SystemRoot,"System32"),controller=new AbortController(),timer=setTimeout(()=>controller.abort(Error("Inventory deadline")),120000),abort=()=>controller.abort(Error("Inventory cancelled"));
process.once("SIGINT",abort);process.once("SIGTERM",abort);
async function query(program,args){try{return {status:0,text:await captureOwnedCommandV1(join(system,program),args,fileURLToPath(root),{signal:controller.signal,output:{maximumBytes:65536,maximumLines:256}})};}catch(error){controller.signal.throwIfAborted();const message=String(error.message),match=/exited with status (-?\d+) signal null\n/u.exec(message);if(!match)throw error;return {status:Number(match[1]),text:message.slice(match.index+match[0].length)};}}
try{
 const features={};for(const name of ["Microsoft-Windows-Subsystem-Linux","VirtualMachinePlatform"]){const row=await query("dism.exe",["/Online","/Get-FeatureInfo","/FeatureName:"+name,"/English"]);const state=/^State\s*:\s*(.+)$/mu.exec(row.text)?.[1]?.trim()??null;features[name]={status:row.status,state};}
 const identity=await query("whoami.exe",["/groups","/fo","csv"]),elevation=identity.status===0?(/S-1-16-(?:12288|16384)/u.test(identity.text)?"elevated":/S-1-16-8192/u.test(identity.text)?"not-elevated":"unknown"):"unknown";
 const reboot={};for(const [name,key] of [["servicing","HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Component Based Servicing\\RebootPending"],["update","HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\WindowsUpdate\\Auto Update\\RebootRequired"]]){const row=await query("reg.exe",["query",key]);reboot[name]=row.status===0?"present":row.status===1?"not-found-or-inaccessible":"unknown";}
 const facts={version:1,features,elevation,reboot};writeFileSync(join(ticket,"🗑️generated","linux-prerequisite-facts.json"),JSON.stringify(facts,null,2)+"\n");console.log("[DEBUG] Windows Linux prerequisites",JSON.stringify(facts));
}finally{clearTimeout(timer);process.removeListener("SIGINT",abort);process.removeListener("SIGTERM",abort);}
