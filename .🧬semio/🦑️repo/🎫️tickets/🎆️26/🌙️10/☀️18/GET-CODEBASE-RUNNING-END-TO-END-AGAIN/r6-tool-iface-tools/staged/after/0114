
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { collectionManifestsFromInventory } from "../../../🚀️bootstrap/📥️admission/🟦️.ts";
import policy from "../../../🔣️policy.json" with {type:"json"};
import ownership from "../../../../🗂️workspaces/🔣️policy.json" with {type:"json"};
import protocol from "./🧬️schema/🔣️.json" with {type:"json"};
import { startToolV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
const inventory=protocol.definitions.Protocol.const;

/** 📥️ Reads exact current source membership from Git and admitted ignored structural manifests with bounded original cancellation. */
export async function nxGraphSourceMembershipV2(root:string,signal?:AbortSignal):Promise<{files:string[];digest:string}> {
 const started=performance.now();signal?.throwIfAborted();
 let ignored="";try{ignored=await readFile(join(root,".nxignore"),"utf8");}catch{}
 const roots:string[]=[],names=new Set<string>(policy.generatedDirectories);for(const line of ignored.split(/\r?\n/u)){const anchored=line.trim().match(/^\/([^*?[\]!#]+)\/\*\*$/u),name=line.trim().match(/^\*\*\/([^/*?[\]!#]+)\/\*\*$/u);if(anchored)roots.push(anchored[1]!);else if(name)names.add(name[1]!);}
 const excluded=[...roots.map(path=>`--exclude=/${path}/`),...names].map(value=>value.startsWith("--exclude=")?value:`--exclude=**/${value}/`);
 const read=(args:readonly string[],input?:string,allowedCodes:readonly number[]=[0])=>new Promise<string>((accept,reject)=>{
  const readStarted=performance.now();
  const child=startToolV1("git",["-c","core.quotepath=false","-c","core.fsmonitor=false",...args],{cwd:root,windowsHide:true,stdio:[input===undefined?"ignore":"pipe","pipe","ignore"]}),chunks:Buffer[]=[];let bytes=0,settled=false;
  const finish=(error?:unknown)=>{if(settled)return;settled=true;if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1")console.log(`[nx] Git membership ${args.includes("check-ignore")?"ignore-filter":args.includes("--deleted")?"deleted":args.includes("--others")?"untracked":"tracked"} elapsedMs=${Math.round(performance.now()-readStarted)} status=${error?"refused":"complete"}`);clearTimeout(deadline);signal?.removeEventListener("abort",abort);if(error)reject(error);else accept(Buffer.concat(chunks).toString("utf8"));},abort=()=>{child.kill();finish(signal?.reason??Error("Graph membership cancelled"));},deadline=setTimeout(()=>{child.kill();finish(Error(`Graph membership inventory exceeded ${Math.round(inventory.readDeadlineMilliseconds/1000)} seconds`));},inventory.readDeadlineMilliseconds);
  signal?.addEventListener("abort",abort,{once:true});child.stdout!.on("data",chunk=>{bytes+=chunk.length;if(bytes>inventory.maximumInventoryBytes){child.kill();finish(Error("Graph membership inventory exceeds "+inventory.maximumInventoryBytes+" bytes"));}else chunks.push(chunk);});child.once("error",finish);child.once("close",code=>finish(code!==null&&allowedCodes.includes(code)?undefined:Error(`Git source membership refused (${code})`)));if(input!==undefined){child.stdin?.once("error",finish);child.stdin?.end(input);}
 });
 const [tracked,untracked,removed]=await Promise.all([read(["ls-files","-z","--cached"]),read(["ls-files","-z","--others",...excluded]),read(["ls-files","-z","--deleted"])]);
 const ignoredFiles=untracked?new Set((await read(["check-ignore","--stdin","-z","--no-index"],untracked,[0,1])).split("\0").filter(Boolean)):new Set<string>();
 const output=tracked+untracked.split("\0").filter(path=>path&&!ignoredFiles.has(path)).map(path=>path+"\0").join(""),deleted=new Set(removed.split("\0").filter(Boolean));
 const inventoryFinished=performance.now();
 const admitted=await collectionManifestsFromInventory(root,[output],policy,ownership.collections,signal);
 const admissionFinished=performance.now();
 const files=[...new Set([...output.split("\0").filter(Boolean),...admitted])].filter(path=>!deleted.has(path)&&!roots.some(root=>path===root||path.startsWith(root+"/"))&&!path.split("/").some(part=>part===".git"||part==="node_modules"||names.has(part))).sort();
 const digest=createHash("sha256");for(const file of files)digest.update(file+"\0");const identity=digest.digest("hex");if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1")console.log(`[nx] Graph membership files=${files.length} inventoryMs=${Math.round(inventoryFinished-started)} admissionMs=${Math.round(admissionFinished-inventoryFinished)} digestMs=${Math.round(performance.now()-admissionFinished)}`);return {files,digest:identity};
}
