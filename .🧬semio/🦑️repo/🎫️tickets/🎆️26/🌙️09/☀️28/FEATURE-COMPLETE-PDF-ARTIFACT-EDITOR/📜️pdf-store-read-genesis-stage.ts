/** 🪪 Publish cold Store genesis to its exact snapshot lease authority before exposing Store. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,renameSync,statSync} from "node:fs";
import {join,resolve} from "node:path";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🌱️runtime-seed-construction-and-owned-close/🦀️.rs";
const before=readFileSync(file,"utf8");
const digest=(s:string)=>createHash("sha256").update(s).digest("hex");
function replace(body:string,a:string,b:string){
    assert.equal(body.split(a).length,2,a);
    return body.replace(a,b);
}
let candidate=replace(before,"        let local_actor_id = actor;\n",`        let local_actor_id = actor;
        let snapshot_read_leases = SnapshotReadLeaseRegistryOwner::new();
        assert!(snapshot_read_leases.publish_authority(0, content_revision), "a fresh Store must publish its exact genesis read authority");
`);
candidate=replace(candidate,"std::mem::ManuallyDrop::new(SnapshotReadLeaseRegistryOwner::new())","std::mem::ManuallyDrop::new(snapshot_read_leases)");
const repo=process.cwd();
const root=resolve(file,"../../../..");
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const manifest=join(root,"📦️packages","🦀️rust");
const overlay=new Map([[resolve(file),candidate]]);
const evidence=[];
for(const os of ["linux","macos","windows","wasi"]){
  const arch=os==="macos"?"aarch64":os==="wasi"?"wasm32":"x86_64";
  const cfg=[`target_os="${os}"`,`target_arch="${arch}"`,os==="wasi"?"wasm":os==="windows"?"windows":"unix"];
  const {files,lines,bytes,sourcePaths}=await measureRustCrate(repo,join(root,"🦀️.rs"),false,overlay,manifest,cfg);
  assert.ok(sourcePaths.includes(file),JSON.stringify({os,files,lines,bytes,root,manifest}));
  assert.ok(files<=32&&lines<=4000&&bytes<=262144);
  evidence.push({os,files,lines,bytes});
}
const out=join(import.meta.dir,"🗑️generated");
const temporary=join(out,`store-read-genesis-${process.pid}.rs`);
if(process.argv.includes("--apply")){
  writeFileSync(temporary,candidate,{flag:"wx",mode:statSync(file).mode&0o777});
  assert.equal(readFileSync(file,"utf8"),before);
  renameSync(temporary,file);
  assert.equal(digest(readFileSync(file,"utf8")),digest(candidate));
}
writeFileSync(join(out,"store-read-genesis-evidence.json"),JSON.stringify({file,before:digest(before),after:digest(candidate),evidence,installed:process.argv.includes("--apply")},null,2)+"\n");
console.log("[DEBUG] Cold Store generation 0 registered before its strict snapshot read",evidence);
