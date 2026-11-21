/** 🧾 Keep the final paid initializer step in the worker's close receipt. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,renameSync,statSync} from "node:fs";
import {join,resolve} from "node:path";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay";
const owner=join(root,"🦀️.rs");
const test=join(root,"🧪️tests","🔬️unit","🦀️.rs");
const sha=(s:string)=>createHash("sha256").update(s).digest("hex");
const orig=readFileSync(owner,"utf8");
const law=readFileSync(test,"utf8");
function exact(s:string,a:string,b:string){assert.equal(s.split(a).length,2,a);return s.replace(a,b);}
const a=`        if !native_retired {
            let after = cx.retained_progress();
`;
const b=`        let after = cx.retained_progress();
        if !native_retired || after != before {
`;
const updated=exact(orig,a,b);
const old=`        let step = original.turn(WIDE,|cx|ArtifactStoreInitializationAuthority::close_step(init,cx)).expect("the initializer closes under its original wide loan");
        assert!(step.progress().fits(WIDE));`;
const next=`        let step = original.turn(WIDE, |cx| {
            let before = cx.retained_progress();
            let step = ArtifactStoreInitializationAuthority::close_step(init, cx)?;
            assert_eq!(before.checked_add(step.progress()).unwrap(), cx.retained_progress(), "the reply includes the final paid native transition");
            Ok(step)
        }).expect("the initializer closes under its original wide loan");
        assert!(step.progress().fits(WIDE));`;
const newLaw=exact(law,old,next);
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const manifest=resolve(root,"📦️packages","🦀️rust");
const metadata=Bun.TOML.parse(readFileSync(join(manifest,"Cargo.toml"),"utf8"));
const overlay=new Map([[resolve(owner),updated],[resolve(test),newLaw]]);
const checks=[];
for(const os of ["macos","linux","windows","wasi"]){
 const cfg=[`target_os="${os}"`,os==="wasi"?'target_arch="wasm32"':os==="macos"?'target_arch="aarch64"':'target_arch="x86_64"',os==="wasi"?"wasm":os==="windows"?"windows":"unix"];
 for(const test of [false,true]){
    const {files,lines,bytes}=await measureRustCrate(process.cwd(),resolve(manifest,metadata.lib.path),test,overlay,manifest,cfg);
    assert.ok(files<=32&&lines<=4000&&bytes<=262144,JSON.stringify({os,test,files,lines,bytes}));
    checks.push({os,test,files,lines,bytes});
 }
}
const generated=join(import.meta.dir,"🗑️generated");
const data=[[owner,sha(orig),updated],[test,sha(law),newLaw]];
for(const [i,[file,before,body]]of data.entries()) {
 assert.equal(sha(readFileSync(file,"utf8")),before);
 if(!process.argv.includes("--apply"))continue;
 const path=join(generated,`replay-receipt-${process.pid}-${i}.rs`);
 writeFileSync(path,body,{flag:"wx",mode:statSync(file).mode&0o777});
 assert.equal(sha(readFileSync(file,"utf8")),before);
 renameSync(path,file);
 assert.equal(readFileSync(file,"utf8"),body);
}
writeFileSync(join(generated,"replay-close-receipt-stage.json"),JSON.stringify({data,checks},null,2)+"\n");
console.log("[DEBUG] Verified native replay's final receipt under",checks.length,"source slices");
