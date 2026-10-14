import {readFileSync,readdirSync,lstatSync,realpathSync,unlinkSync,rmdirSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import{mkdir}from"node:fs/promises";
import {join,resolve,basename} from "node:path";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {captureOwnedProcess} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {withResourceLeases,checkResourceLeaseControl} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {nxAuthorizerReceiptHoldsV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts";
import {planNxToolingRetentionV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🟦️.ts";
const command=process.argv[2],root=resolve(import.meta.dir,"../../../../../../../.."),store=join(root,".🧬semio/🦑️repo/⚡️cache/tools/nx"),leases=join(root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases");if(process.argv.length!==3||!["preview","prune"].includes(command!))throw Error("Expected exact tooling maintenance command");
await receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"nx:toolkit-maintenance",0),current=basename(realpathSync(join(root,".nx/installation"))),images=readdirSync(store).filter(name=>/^[a-f0-9]{64}$/u.test(name)).map(identity=>{const path=join(store,identity),stat=lstatSync(path),marker=JSON.parse(readFileSync(join(path,".semio-nx-tooling.json"),"utf8"));if(!stat.isDirectory()||stat.isSymbolicLink()||marker.version!==1||marker.digest!==identity)throw Error("Owned immutable toolkit marker required");return {identity,createdAt:stat.birthtimeMs||stat.mtimeMs};}),eligible=planNxToolingRetentionV1(images,current,4),rows:{identity:string;outcome:string;entries?:number;references?:number[]}[]=[];
 console.log("[DEBUG] exact toolkit retention current="+current+" images="+images.length+" keepNewest=4 eligible="+eligible.length);
 if(command==="preview"){console.log(JSON.stringify({current,eligible,images}));return;}
 if(process.platform!=="darwin")throw Error("This one-off physical foreign reference probe is authored for the current macOS host");
 const artifacts=join(import.meta.dir,"../🗑️generated/tools-maintenance");await mkdir(artifacts,{recursive:true});let probes=0;
 const inventory=async(command:string,args:string[])=>{checkResourceLeaseControl(control);const name=command+"-"+process.pid+"-"+(++probes),result=await captureOwnedProcess(command,args,{invocation:original,cwd:root,env:scriptInvocationEnvironment(original,process.env),budgetMs:600000,maxOutputBytes:16777216,stdoutPath:join(artifacts,name+".txt"),stderrPath:join(artifacts,name+"-error.txt")});if(result.status!==0||result.signal!==null||result.reason!=="exit"||result.stderr.trim())throw Error("Complete physical toolkit reference inventory required");return result.stdout;};
 let operations=0;const advance=async()=>{checkResourceLeaseControl(control);if(++operations%64===0){await control.publish({resource:"nx:toolkit-maintenance:"+operations,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);}};
 const remove=async(path:string)=>{if(!existsSync(path))return 0;const stat=lstatSync(path);if(!stat.isDirectory()||stat.isSymbolicLink()||realpathSync(path)!==path)throw Error("Physical original cache directory required");const entries:{path:string;directory:boolean}[]=[],walk=async(directory:string)=>{for(const name of readdirSync(directory)){await advance();const child=join(directory,name),status=lstatSync(child),isDirectory=status.isDirectory()&&!status.isSymbolicLink();if(isDirectory)await walk(child);entries.push({path:child,directory:isDirectory});}};await walk(path);checkResourceLeaseControl(control);for(const entry of entries){await advance();if(entry.directory)rmdirSync(entry.path);else unlinkSync(entry.path);}await advance();rmdirSync(path);return entries.length;};
 for(const identity of eligible){
  const path=join(store,identity),namespace=createHash("sha256").update(JSON.stringify([root.replaceAll("\\","/"),identity])).digest("hex").slice(0,24),data=join(root,".nx/workspace-data/tooling",namespace),socket="/tmp/semio-nx-"+namespace;await withResourceLeases({directory:leases,resources:[{resource:"nx-tooling-activation",mode:"shared"}],signal:control.signal,control},async()=>{
   if(realpathSync(join(root,".nx/installation"))===path){rows.push({identity,outcome:"current"});return;}
   try{await withResourceLeases({directory:leases,resources:[{resource:"nx-tooling:"+identity,mode:"exclusive"}],signal:control.signal,control,timeoutMs:0},async()=>{
    const ps=await inventory("ps",["-axo","pid=,command="]),opened=await inventory("lsof",["-nP","-Fpn"]),references=new Set<number>(),prefixes=[path,data,socket];
    for(const line of ps.split("\n")){const match=line.trim().match(/^(\d+)\s+(.*)$/u);if(match&&prefixes.some(prefix=>match[2]!.includes(prefix)))references.add(Number(match[1]));}let pid=0;for(const line of opened.split("\n")){if(/^p\d+$/u.test(line))pid=Number(line.slice(1));else if(line.startsWith("n")&&prefixes.some(prefix=>line.slice(1)===prefix||line.slice(1).startsWith(prefix+"/")))references.add(pid);}
    if(references.size){rows.push({identity,outcome:"physical-reference",references:[...references].sort((a,b)=>a-b)});return;}
    const marker=JSON.parse(readFileSync(join(path,".semio-nx-tooling.json"),"utf8"));if(marker.version!==1||marker.digest!==identity||realpathSync(path)!==path||lstatSync(path).isSymbolicLink()||!nxAuthorizerReceiptHoldsV1(path,marker.authorizers))throw Error("Original inactive toolkit receipt changed");
    const entries=await remove(data)+await remove(path);rows.push({identity,outcome:"retired",entries});console.log("[DEBUG] exact inactive toolkit retired "+identity+" namespace="+namespace+" entries="+entries);
   });}catch(error){if(error instanceof Error&&error.message==="Resource lease timed out: nx-tooling:"+identity)rows.push({identity,outcome:"live-shared-owner"});else throw error;}
  });
 }
 console.log(JSON.stringify({current,images:images.length,rows}));
});
