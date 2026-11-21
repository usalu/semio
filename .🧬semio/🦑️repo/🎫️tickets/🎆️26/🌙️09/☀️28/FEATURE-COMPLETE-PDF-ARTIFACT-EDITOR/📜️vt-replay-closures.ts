/** 📏️ Measures staged VT replay and binary wire candidates with the repository closure reader. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const root=process.cwd(),[pdf,out]=process.argv.slice(2), manifest=(path:string)=>JSON.parse(readFileSync(path,"utf8"));
const vt=manifest(join(out,"vt-replay","receipt.json")), wire=manifest(join(out,"vt-archive","receipt.json"));
const revision="🏅️standards/7️⃣1.7/🪆️subsets", base=join(pdf,revision,"🧱️base");
const overlay=new Map([[resolve(pdf,vt.relative),readFileSync(join(out,"vt-replay","vt-editor.rs"),"utf8")]]);
for(const [file,row] of Object.entries(wire)) overlay.set(resolve(base,file),readFileSync(join(out,"vt-archive",(row as {overlay:string}).overlay),"utf8"));
const owners={
  "vt-editor":join(base,"🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value"),
  "vt-viewer":join(pdf,revision,"🧾️vt/👁️viewer"),
  "binary":join(base,"🔨️modules/🖋️content/🧪️tests/🔬️unit"),
  "binary-testkit":join(base,"🧬️schema/🧪️testkit"),
};
for(const os of ["linux","macos","windows","wasi"]){
  const cfg=[`target_os="${os}"`,os==="wasi"?'target_arch="wasm32"':os==="macos"?'target_arch="aarch64"':'target_arch="x86_64"',os==="wasi"?'target_family="wasm"':os==="windows"?"windows":"unix"];
  for(const [owner,path] of Object.entries(owners)){
    const dir=resolve(path,"📦️packages","🦀️rust");
    for(const test of [false,true]){
      const row=await measureRustCrate(root,join(dir,"🦀️.rs"),test,overlay,dir,cfg);
      assert(row.files<=32&&row.lines<=4000,[os,owner,test,row.files,row.lines].join(" "));
      console.log("[DEBUG]",os,owner,test?"test":"lib",row.files,row.lines,row.bytes);
    }
  }
}
