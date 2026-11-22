/** 🧪️ Verifies page-size retirement using the exact staged production paged list. */
import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
const ticket=import.meta.dir;
const output=join(ticket,"🗑️generated","paged-octet-close");
await mkdir(output,{recursive:true});
const staged: [string,string,string][]=await Bun.file(join(ticket,"🗑️generated","paged-octet-retirement-overlay.json")).json();
const [leaf,preimage,code]=staged[0]!;
assert.equal(new Bun.CryptoHasher("sha256").update(await Bun.file(leaf).bytes()).digest("hex"),preimage);
const discard=join(output,"discard.rs");
await writeFile(discard,code);
const root="🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs";
const current=await Bun.file(root).text();
const list=join(output,"list.rs");
await writeFile(list,current.replace(/(#\[path\s*=\s*)"([^"]+)"/gu,(_whole,lead,path)=>
 lead+JSON.stringify(path==="📤️discard/🦀️.rs"?discard:resolve(dirname(root),path))));
const source=join(output,"law.rs");
const law=await Bun.file(join(ticket,"📜️paged-octet-retirement-law.rs")).text();
await writeFile(source,"pub use semio_framework_value_control_contract::*;\n#[path = "+JSON.stringify(list)+"] mod list;\n"+law);
const cache=resolve(process.argv[2]??"");
const target=join(output,process.platform==="win32"?"law.exe":"law");
const args=["--edition","2024","-Copt-level=2","-Awarnings","--crate-name","paged_octet_close_law",source,"-o",target];
for(const project of ["semio-framework-value-control-contract","semio-framework-value-retirement-contract", "semio-framework-value-allocation-return"]){
const symbol=project.replaceAll("-","_");
const libs=await Array.fromAsync(new Bun.Glob("*/out/lib"+symbol+"-*.rlib").scan({cwd:join(cache,project),onlyFiles:true}));
assert.equal(libs.length,1,project);
args.push("--extern",symbol+"="+join(cache,project,libs[0]!.replace(/\.rlib$/u,".rmeta")),
  "--extern",symbol+"="+join(cache,project,libs[0]!));
}
for await(const directory of new Bun.Glob("*/*/out").scan({cwd:cache,onlyFiles:false}))args.push("-Ldependency="+join(cache,directory));
const response=join(output,"law.args");
await writeFile(response,args.join("\n")+"\n");
const build=Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+response],{stderr:"inherit",stdout:"inherit"});
assert.equal(await build.exited,0);
const check=Bun.spawn([target,join(ticket,"🗑️generated","pdf-segmented-source.pdf")],{stderr:"inherit",stdout:"inherit"});
assert.equal(await check.exited,0);
