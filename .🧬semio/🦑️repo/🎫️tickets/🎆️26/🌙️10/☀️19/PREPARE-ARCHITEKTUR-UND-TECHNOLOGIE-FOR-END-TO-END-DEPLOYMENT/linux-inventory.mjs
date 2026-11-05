import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {mkdirSync,writeFileSync} from "node:fs";

const root=new URL("../../../../../../../",import.meta.url),ticket=fileURLToPath(new URL(".",import.meta.url));
const{captureOwnedCommandV1}=await import(new URL("🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts",root)),{defaultNativeToolchainPortsV1}=await import(new URL("🧰️framework/🔨️modules/⚙️runtime/🧲️native/🧰️toolchain/🟦️.ts",root));
const executable=join(process.env.SystemRoot,"System32","wsl.exe"),ports=defaultNativeToolchainPortsV1(),controller=new AbortController();
if(process.platform!=="win32"||!ports.isExecutable(executable))throw Error("Existing native Windows inventory port required");
const abort=()=>controller.abort(Error("Inventory cancelled")),timer=setTimeout(()=>controller.abort(Error("Inventory twenty-second deadline")),20000);
process.once("SIGINT",abort);process.once("SIGTERM",abort);
try{
 const queries=[];
 for(const[name,args]of[["distributions",["--list","--quiet"]],["status",["--status"]]]){
  let status=0,text="";try{text=await captureOwnedCommandV1(executable,args,fileURLToPath(root),{signal:controller.signal,env:{...process.env,WSL_UTF8:"1"},output:{maximumBytes:32768,maximumLines:128}});}catch(error){controller.signal.throwIfAborted();const message=String(error.message),terminal=/exited with status (-?\d+) signal null\n/u.exec(message);if(!terminal)throw error;status=Number(terminal[1]);text=message.slice(terminal.index+terminal[0].length);}
  queries.push({name,status,text});
 }
 const result={version:1,platform:"win32",queries};
 if(result.version!==1||result.platform!=="win32"||!Array.isArray(result.queries)||result.queries.length!==2||result.queries.some(row=>typeof row.name!=="string"||!Number.isInteger(row.status)||typeof row.text!=="string"))throw Error("Closed native inventory facts required");
 const generated=join(ticket,"🗑️generated");mkdirSync(generated,{recursive:true});writeFileSync(join(generated,"linux-environment.json"),JSON.stringify(result,null,2)+"\n");
 for(const row of result.queries)console.log("[DEBUG]",row.name,"status",row.status,"text",row.text.replaceAll("\0",""));
}finally{clearTimeout(timer);process.removeListener("SIGINT",abort);process.removeListener("SIGTERM",abort);}
