/** 📏️ Inventories selected Forms test owners without issuing compiler or Source authority. */
import {readFileSync,statSync,writeFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const [platform]=process.argv.slice(2);
if(!["macos","linux","windows"].includes(platform??""))throw Error("Explicit platform required");
const root=process.cwd(),ticket=resolve(import.meta.dir),base="✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms";
const names=["config-tests","tests","tests-2","tests-3","schema","questions","viewer","forms","patch-tests",""].map(s=>"semio-s-artifact-forms-forms"+(s?"-"+s:""));
const policy=JSON.parse(readFileSync(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"),"utf8"));
const policies=Object.values(policy).filter((v:any)=>v?.sourceBoundary).map((v:any)=>v.sourceBoundary);
if(policies.length!==1)throw Error("One actual Cargo Source policy required");
const limits=policies[0],family=platform==="windows"?"windows":"unix",arch=platform==="macos"?"aarch64":"x86_64",environment=platform==="macos"?"":platform==="windows"?"msvc":"gnu",vendor=platform==="macos"?"apple":platform==="windows"?"pc":"unknown";
const cfg=["test","debug_assertions",family,'target_arch="'+arch+'"','target_os="'+platform+'"','target_family="'+family+'"','target_env="'+environment+'"','target_vendor="'+vendor+'"','target_pointer_width="64"','target_endian="little"','panic="unwind"'];
const scan=Bun.spawnSync(["rg","--files","--hidden","--no-ignore","--glob","Cargo.toml",base]);
if(scan.exitCode!==0)throw Error(scan.stderr.toString());
const manifests=scan.stdout.toString().trim().split("\n").map(path=>({path,text:readFileSync(path,"utf8")}));
const rows=[];
for(const name of names){
 const candidates=manifests.filter(row=>new RegExp('^name\\s*=\\s*"'+name+'"$',"m").test(row.text));
 if(candidates.length!==1)throw Error("Unique owner manifest required: "+name);
 const manifest=candidates[0]!,library=/^\[lib\]\s*\n(?:[^\n]*\n)*?path\s*=\s*"([^"]+)"\s*$/m.exec(manifest.text);
 if(!library)throw Error("Explicit lib path required: "+name);
 const inputs=await readRustSourceClosureV1([resolve(dirname(manifest.path),library[1]!)],root,cfg,async path=>{try{if(statSync(path).size>1048576)throw Error("Finite diagnostic input required");return new Uint8Array(readFileSync(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},{manifestDirectory:resolve(dirname(manifest.path)),generated:()=>undefined});
 const bytes=inputs.reduce((sum,input)=>sum+input.bytes,0),activeLines=inputs.reduce((sum,input)=>sum+input.activeLines,0),files=inputs.length;
 rows.push({name,manifest:manifest.path,files,bytes,activeLines,fits:files<=limits.maxSourceFiles&&bytes<=limits.maxSourceBytes&&activeLines<=limits.maxActiveLines,inputs});
}
const report={platform,cfg,limits,rows};
writeFileSync(resolve(ticket,"🗑️generated/forms-source-"+platform+".json"),JSON.stringify(report,null,2)+"\n");
console.log(JSON.stringify({platform,limits,rows:rows.map(({inputs,...row})=>row)}));
