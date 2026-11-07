import {readFileSync} from "node:fs";
import {win32} from "node:path";
const root=new URL("../../../../../../../",import.meta.url),schema=JSON.parse(readFileSync(new URL("linux-command-schema.json",import.meta.url),"utf8")),{validateJsonSchemaSubset}=await import(new URL("🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts",root));
export function linuxCommandPlanV1(input){
 if(validateJsonSchemaSubset(schema,input,schema).length)throw Error("Closed Linux command input required");
 for(const path of [input.systemRoot,input.localAppData,input.artifactDirectory])if(!/^[a-z]:\\/iu.test(path)||win32.normalize(path)!==path||path.split("\\").includes(".."))throw Error("Normalized local Windows path required");
 const system=win32.join(input.systemRoot,"System32"),directory=win32.join(input.localAppData,"Semio","linux","Semio-Debian12"),artifact=name=>win32.join(input.artifactDirectory,name);
 const plan=input.action==="enable-vmp"?{program:win32.join(system,"dism.exe"),args:["/Online","/Enable-Feature","/FeatureName:VirtualMachinePlatform","/All","/NoRestart"]}:input.action==="install-wsl"?{program:win32.join(system,"msiexec.exe"),args:["/i",artifact("wsl.3.0.3.0.x64.msi"),"/quiet","/norestart"]}:{program:win32.join(system,"wsl.exe"),args:["--import","Semio-Debian12",directory,artifact("debian12.amd64.tar"),"--version","2"]};
 if([plan.program,...plan.args].some(value=>value.length>256))throw Error("Bounded command path required");return plan;
}
