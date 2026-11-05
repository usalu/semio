/** 🧪️ Lists the actual Forms workspace library test owners and their bounded physical Source. */
import {readFileSync,statSync,writeFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {readRustSourceClosureV1,selectRustSourceV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const [platform,output]=process.argv.slice(2);
if(!["macos","linux","windows"].includes(platform??""))throw Error("Explicit diagnostic platform required");
const root=process.cwd(),ticket=resolve(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FORMS-PLUGIN-END-TO-END"),destination=resolve(root,output??"");
if(!destination.startsWith(resolve(ticket,"🗑️generated")+"/")&&!destination.startsWith(resolve(ticket,"🗑️generated")+"\\"))throw Error("Ticket generated output required");
const workspace=resolve(root,"✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms"),declaration=Bun.TOML.parse(readFileSync(resolve(workspace,"Cargo.toml"),"utf8")) as any,members=declaration.workspace.members as string[];
if(members.length>64)throw Error("Bounded Forms membership required");
const planText=readFileSync(resolve(ticket,"native-validation-plan.md"),"utf8"),allSelected=planText.split("## Generic UI")[0].includes("--workspace");
const selected=new Set([...planText.matchAll(/-p ([a-z0-9-]+)/g)].map(match=>match[1])),policy=JSON.parse(readFileSync(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"),"utf8")).storage.sourceBoundary;
const read=async(path:string)=>{try{if(statSync(path).size>1048576)throw Error("Finite diagnostic input required");return new Uint8Array(readFileSync(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}};
const rows=[];
for(const member of members){
 const manifest=resolve(workspace,member,"Cargo.toml"),cargo=Bun.TOML.parse(readFileSync(manifest,"utf8")) as any;if(!cargo.lib)continue;
 const arch=platform==="macos"?"aarch64":"x86_64",family=platform==="windows"?"windows":"unix",environment=platform==="macos"?"":platform==="windows"?"msvc":"gnu",vendor=platform==="macos"?"apple":platform==="windows"?"pc":"unknown";
 const cfg=["test","debug_assertions",family,'target_arch="'+arch+'"','target_os="'+platform+'"','target_family="'+family+'"','target_env="'+environment+'"','target_vendor="'+vendor+'"','target_pointer_width="64"','target_endian="little"','panic="unwind"'];
 const inputs=await readRustSourceClosureV1([resolve(dirname(manifest),cargo.lib.path??"src/lib.rs")],root,cfg,read,{manifestDirectory:dirname(manifest),generated:()=>{throw Error("Actual compiler ownership required for generated diagnostic input");}});
 const names=inputs.filter(input=>input.kind==="source").flatMap(input=>[...selectRustSourceV1(readFileSync(resolve(root,input.path),"utf8"),cfg).matchAll(/#\[(?:\w+::)*\w*test(?:\([^)]*\))?\]\s*(?:#\[[^\]]*\]\s*)*(?:async\s+)?fn\s+(\w+)/g)].map(match=>match[1]));
 const bytes=inputs.reduce((sum,input)=>sum+input.bytes,0),activeLines=inputs.reduce((sum,input)=>sum+input.activeLines,0);
 rows.push({name:cargo.package.name,manifest,selected:allSelected||selected.has(cargo.package.name),testEnabled:cargo.lib.test!==false,explicitTests:names,files:inputs.length,bytes,activeLines,withinLimits:inputs.length<=policy.maxSourceFiles&&bytes<=policy.maxSourceBytes&&activeLines<=policy.maxActiveLines,features:cargo.features??{}});
}
writeFileSync(destination,JSON.stringify({platform,policy,rows},null,2)+"\n");
console.log(JSON.stringify({members:rows.length,omitted:rows.filter(row=>!row.selected).map(row=>({name:row.name,tests:row.explicitTests.length,files:row.files,bytes:row.bytes,activeLines:row.activeLines,withinLimits:row.withinLimits})),violations:rows.filter(row=>!row.withinLimits).map(row=>row.name)},null,2));
