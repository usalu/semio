/** 🎟️ Stage the Original constructor receipt continuation exposed by registered PDF Open. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const path = "🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs";
const d = join(import.meta.dir, "🗑️generated");
const before = readFileSync(path, "utf8");
const old = ' if admission.source.is_some(){let ready=output.is_some();let bytes=std::mem::size_of_val(&admission.source)+std::mem::size_of_val(&admission.child)+std::mem::size_of_val(&admission.authority)+std::mem::size_of::<bool>();if !construction_metadata(context,bytes,3)?{return Ok(OriginalConstructionStatus::Pending)}admission.source=None;admission.child=None;admission.release_authority();admission.complete=ready;return Ok(if ready{OriginalConstructionStatus::Ready}else{OriginalConstructionStatus::Pending})}';
const next = ` let recovering=admission.frame.phase()!=semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty;
 if admission.source.is_some(){
    if recovering {
        if construction_metadata(context,std::mem::size_of_val(&admission.source),1)?{admission.source=None;}
        return Ok(OriginalConstructionStatus::Pending);
    }
    let ready=output.is_some()&&admission.child.is_some();
    let bytes=std::mem::size_of_val(&admission.source)+std::mem::size_of_val(&admission.child)+std::mem::size_of_val(&admission.authority)+std::mem::size_of::<bool>();
    if !construction_metadata(context,bytes,3)?{return Ok(OriginalConstructionStatus::Pending)}
    admission.source=None;admission.child=None;admission.release_authority();admission.complete=ready;
    return Ok(if ready{OriginalConstructionStatus::Ready}else{OriginalConstructionStatus::Pending})
 }`;
const input = ' if output.is_some(){return Err(ValueError::invariant("construction requires its original empty candidate").into())}';
const construct = '    let construct=construct.take().ok_or_else(||ValueError::invariant("construction call already consumed"))?;';
for(const needle of [old,input,construct]) assert.equal(before.split(needle).length,2,needle);
const after=before.replace(old,next)
    .replace(input,input.replace("output.is_some()","output.is_some()&&!recovering"))
    .replace(construct,'    if output.is_some(){return Err(ValueError::invariant("construction cannot overwrite its retained candidate"))}\n'+construct);
assert.equal(readFileSync(path,"utf8"),before);
const hash=(source:string)=>createHash("sha256").update(source).digest("hex");
const testPath=join(path,"..","🧪️tests","🦀️.rs");
const lawPath=join(path,"..","🧪️tests","🔣️receipt-recovery.json");
const testBefore=readFileSync(testPath,"utf8");
const law=readFileSync(join(import.meta.dir,"📜️pdf-open-construction-resume-law.rs"),"utf8");
const fixture=readFileSync(join(import.meta.dir,"🔣️pdf-open-construction-recovery.json"),"utf8");
assert.ok(!testBefore.includes("ready_never_escapes_an_unfinished_native_construction_receipt")&&!existsSync(lawPath));
assert.ok(resolve(lawPath).normalize("NFD").length<=256);
const testAfter=testBefore+"\n"+law;
const data=[[path,hash(before),after],[testPath,hash(testBefore),testAfter],[lawPath,null,fixture]];
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const pkg=resolve("🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust");
const candidates=new Map(data.map(([file,_,source])=>[resolve(file!),source!]));
const checks=[];
for(const [os,arch,family] of [["linux","x86_64","unix"],["macos","aarch64","unix"],["windows","x86_64","windows"],["wasi","wasm32","wasm"]]){
    const cfg=[`target_os="${os}"`,`target_arch="${arch}"`,`target_family="${family}"`];
    for(const test of [false,true]){
        const {files,lines,bytes}=await measureRustCrate(process.cwd(),resolve(pkg,"🦀️.rs"),test,candidates,pkg,cfg);
        checks.push({os,test,files,lines,bytes,admitted:files<=32&&lines<=4000&&bytes<=262144});
    }
}
assert.ok(checks.every(row=>row.admitted),JSON.stringify(checks));
const receipt={data,after:hash(after),checks};
writeFileSync(join(d,"pdf-open-construction-stage.json"),JSON.stringify(receipt,null,2)+"\n");
console.log("[DEBUG] Staged Original constructor recovery",receipt.after,checks);
