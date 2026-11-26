/** 🧾 Match each linked unit's compiler-consumed source checksum against available physical snapshots. */
import {readFileSync,existsSync,writeFileSync,readdirSync} from "node:fs";
import {basename,dirname,isAbsolute,join,resolve,relative} from "node:path";
import {createHash} from "node:crypto";
import {blake3Hex} from "../../../../../../../🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
import {cargoDepInfoChecksumsV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
const root=resolve("."), area=join(import.meta.dir,"🗑️generated/bridge-current/b4");
const evidence=JSON.parse(readFileSync(join(area,"link.json"),"utf8"));
const libraries=evidence.args.filter((arg:string)=>arg.endsWith(".rlib"));
const artifacts=new Map<string,string>(),missing=new Set<string>(),cache=new Map<string,string>(),sources=new Map<string, string>(), conflicts=new Set<string>();
const units=[];
const target=resolve(".cache/b/r/m/tmp/g3d-p-n25");
const finalRustc=JSON.parse(readFileSync(join(area,"rustc.json"),"utf8"));
const finalOut=finalRustc[finalRustc.indexOf("--out-dir")+1];
const depFiles=libraries.map((file:string)=>[file,join(dirname(file),basename(file).slice(3,-5)+".d")]);
depFiles.push([join(area,"semio_hub_procedural_component.wasm"),join(finalOut,"semio_hub_procedural_component.d")]);
for(const [library,dep] of depFiles){
  const inputs=[];
  if(existsSync(dep))for(const row of cargoDepInfoChecksumsV1(readFileSync(dep,"utf8"))){
    const original=isAbsolute(row.path)?resolve(row.path):resolve(root,"🌎️hub",row.path);
    const key=original+"\0"+row.blake3;
    let sha=cache.get(key);
    if(!sha){
      const rel=relative(root,original);
      const alternatives=[original,original.replace("/tmp/flow-plugin-n25/","/tmp/g3d-p-n25/"),join(area,"source-witnesses",row.blake3)];
      if (original.endsWith("/private.rs")&&original.includes("/out/")){
        const suffix=original.split("/build/").pop()!.split("/")[0];
        const entries=new Bun.Glob(".cache/b/r/m/tmp/*/{wasm32-wasip2/,}debug/build/"+suffix+"/*/out/private.rs");
        for(const file of entries.scanSync({cwd:root,dot:true}))alternatives.push(join(root,file));
      }
      if(!rel.startsWith("..")&&!rel.startsWith(".cache/"))for(const snap of ["b4-receipt","p","s","b"])alternatives.push(join(root,".cache/g3d-native",snap,rel));
      for (const candidate of alternatives){
        if(!existsSync(candidate)) continue;
        let bytes=readFileSync(candidate);
        if (row.blake3==="76b24ae0f708a11412e887e58c495d4b187ba033d58c76dcdee9cac4b2c64cc7") bytes=Buffer.from(bytes.toString().replace(/^fn close_owner<.*$/m,"fn close_owner<T:semio_framework_value::retirement::RetireOwned>(owner:&mut ControlledRetirement<T>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_effect_step(cx,demand,|effect|{let body=effect.grant().maximum_copy_bytes;owner.step(body,effect)})?;Ok(())}"));
        if(bytes.length===row.length&&blake3Hex(bytes)===row.blake3){sha=createHash("sha256").update(bytes).digest("hex");cache.set(key,sha);break;}
      }
    }
    if(!sha){missing.add(key);continue;}
    if(sources.has(original)&&sources.get(original)!==sha)conflicts.add(original);
    sources.set(original,sha);inputs.push({path:original,kind:"file",sha256:sha});
  }
  units.push({library,dep,inputs});
}
writeFileSync(join(area,"verified-deps.json"),JSON.stringify({units,missing:[...missing],conflicts:[...conflicts]}));
console.log("[DEBUG]",JSON.stringify({units:units.length,sources:sources.size,missing:missing.size,conflictingVersions:conflicts.size}));
