/** 🎟️ Captures one original registered GUI caller through its defining configured initial factory. */
import {createBuildBootstrapCapabilitiesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts";
import {readFileSync,writeFileSync,mkdirSync,openSync,closeSync} from "node:fs";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";

for(const key of["SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_SCRIPT_CAPABILITIES","SEMIO_SCRIPT_PRIVATE_CUSTODY"])if(process.env[key]!==undefined)throw Error("Preexisting initial authority refused: "+key);
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),directory=join(ticket,"🗑️generated/process-compiler-capacity-current"),launchPath=join(root,".vscode/launch.json"),launchSource=readFileSync(launchPath,"utf8"),rows=JSON.parse(launchSource).configurations.filter((row:{command?:string})=>row.command==="bun repo run @semio-tech/framework-process:test-compiler-capacity");
if(rows.length!==1)throw Error("One actual registered Process compiler-capacity caller required");
const row=rows[0],policy=JSON.parse(row.env.SEMIO_SCRIPT_POLICY),argv:string[]=row.command.split(" ");if(policy.version!==1||policy.owner!=="process:compiler-capacity:caller"||policy.maximumElapsedMilliseconds!==300000||row.env.SEMIO_SCRIPT_CAPABILITIES!==undefined)throw Error("Exact finite initial GUI policy required");
mkdirSync(directory,{recursive:true});writeFileSync(join(ticket,"process-compiler-capacity-gui-caller1.json"),JSON.stringify({row,launchSha256:createHash("sha256").update(launchSource).digest("hex"),outputRedirectOnly:directory},null,2)+"\n");
const environment={...process.env,...Object.fromEntries(Object.entries(row.env as Record<string,string>).map(([key,value])=>[key,value.replaceAll("${workspaceFolder}",root)])),SEMIO_TEST_ARTIFACT_DIR:directory},capabilities=await createBuildBootstrapCapabilitiesV1(root,argv.slice(2),environment),encoded=JSON.stringify(capabilities);
writeFileSync(join(directory,"factory.json"),encoded+"\n");
const descriptor=openSync(join(directory,"current.log"),"w"),started=Date.now();console.log("[DEBUG] Actual registered Process compiler-capacity capture started "+JSON.stringify({argv,initialPolicy:policy,factory:join(directory,"factory.json"),log:join(directory,"current.log")}));
let status:number;try{const child=Bun.spawn(argv,{cwd:root,env:{...environment,SEMIO_SCRIPT_CAPABILITIES:encoded},stdout:descriptor,stderr:descriptor});writeFileSync(join(directory,"started.json"),JSON.stringify({pid:child.pid,startedAt:new Date(started).toISOString(),argv,initialPolicy:policy},null,2)+"\n");status=await child.exited;}finally{closeSync(descriptor);}
const result={exitCode:status,elapsedMilliseconds:Date.now()-started,initialPolicy:policy,factorySha256:createHash("sha256").update(encoded).digest("hex"),registeredCommand:row.command};writeFileSync(join(directory,"result.json"),JSON.stringify(result,null,2)+"\n");console.log("[DEBUG] Actual registered Process compiler-capacity capture terminal "+JSON.stringify(result));process.exitCode=status;
