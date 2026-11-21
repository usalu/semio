/** 📐 Measures both targets of the actual owners with the full staged archive overlay. */
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const {CRATE_HARD_BUDGET}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts"));
const fits=(row:{files:number,lines:number,bytes:number})=>row.files<=CRATE_HARD_BUDGET.files&&row.lines<=CRATE_HARD_BUDGET.lines&&row.bytes<=CRATE_HARD_BUDGET.bytes;
const root=process.cwd(),[pdf,out,inspect]=process.argv.slice(2);
const receipt=JSON.parse(readFileSync(join(out,"archive-install","receipt.json"),"utf8"));
const revision="🏅️standards/7️⃣1.7/🪆️subsets", base=join(pdf,revision,"🧱️base");
const overlay=new Map<string,string>();
for(const [file,row] of Object.entries<{stage:string}>(receipt.paths)){
  overlay.set(resolve(file),readFileSync(join(out,"archive-install",row.stage),"utf8"));
}
const owners={
  pack: "🧰️framework/🔨️modules/🎒️pack/🔤️json",
  editor: join(base,"🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value"),
  viewer: join(pdf,revision,"🧾️vt/👁️viewer"),
  "base-editor": join(pdf,revision,"🧾️vt/🧬️schema"),
  binary: join(base,"🔨️modules/🖋️content/🧪️tests/🔬️unit"),
  testkit: join(base,"🧬️schema/🧪️testkit"),
};
let outside=false;
for(const os of ["linux","macos","windows","wasi"]){
  const cfg=[`target_os="${os}"`,os==="wasi"?'target_arch="wasm32"':os==="macos"?'target_arch="aarch64"':'target_arch="x86_64"',os==="wasi"?'target_family="wasm"':os==="windows"?"windows":"unix"];
  for(const [owner,path] of Object.entries(owners)){
    const dir=resolve(path,"📦️packages","🦀️rust");
    const manifest=overlay.get(join(dir,"Cargo.toml")) ?? readFileSync(join(dir,"Cargo.toml"),"utf8");
    const library=/^\[lib\]\s*\n(?:[^\n]+\n)*?path\s*=\s*"([^"]+)"/m.exec(manifest)?.[1] ?? "src/lib.rs";
    for(const test of [false,true]){
      const row=await measureRustCrate(root,join(dir,library),test,overlay,dir,cfg);
      const admitted=fits(row);
      outside ||= !admitted;
      console.log("[DEBUG]",os,owner,test?"test":"lib",row.files,row.lines,row.bytes,admitted?"admitted":"exceeds");
      if(inspect===owner&&os==="macos"&&test){
        const current=await measureRustCrate(root,join(dir,library),test,undefined,dir,cfg);
        console.log("[DEBUG]",owner,"current",current.files,current.lines,current.bytes);
        const withoutTests=await measureRustCrate(root,join(dir,library),false,overlay,dir,cfg);
        for(const path of row.paths.filter(path=>!withoutTests.paths.includes(path)))console.log("[DEBUG]",row.entries.get(path)?.bytes,path);
      }
    }
    if(owner==="testkit"||owner==="pack"){
      const name=owner==="testkit"?"original_mutation_archive":"original-json-owner";
      const target=new RegExp(`\\[\\[test\\]\\]\\s*\\nname = "${name}"\\s*\\npath = "([^"]+)"`).exec(manifest)?.[1];
      if(!target)throw new Error(`Original test target ${name} is not registered`);
      const row=await measureRustCrate(root,join(dir,target),true,overlay,dir,cfg);
      const admitted=fits(row);
      outside ||= !admitted;
      console.log("[DEBUG]",os,name,"test",row.files,row.lines,row.bytes,admitted?"admitted":"exceeds");
    }
  }
}
if(outside)process.exitCode=1;
