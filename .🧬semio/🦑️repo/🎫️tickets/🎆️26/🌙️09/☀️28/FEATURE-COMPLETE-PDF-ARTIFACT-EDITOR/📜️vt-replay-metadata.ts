/** 🪪️ Restores exact target metadata from already-published repository CAS receipts for a narrow probe. */
import {casObjectPathV1,casRootV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
const [name,expected]=process.argv.slice(2);
if(!/^[a-zA-Z0-9_\-]+\.rmeta$/u.test(name??"")||!/^[0-9a-f]{32}$/u.test(expected??""))throw Error("Expected a compiler metadata basename and exact Rust hash");
const done=join(import.meta.dirname,"🗑️generated","vt-replay","deps"),path=join(done,name!),scratch=join(done,"candidate.rmeta");
mkdirSync(done,{recursive:true});
const listed=Bun.spawnSync({cmd:["zsh","-c",'rg -l --hidden --no-ignore -F "$1" .cache/b/r/m/u/e',"--",name!]});
for(const row of listed.stdout.toString().trim().split("\n").filter(Boolean)){
  const output=JSON.parse(readFileSync(row,"utf8")).outputs.find((value:any)=>value.name===name);
  if(!output)continue;
  const payload=readFileSync(casObjectPathV1(casRootV1(),output.sha256));
  if(createHash("sha256").update(payload).digest("hex")!==output.sha256)throw Error("Wrong repository CAS identity");
  writeFileSync(scratch,payload);
  const rust=Bun.spawnSync({cmd:["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","-Zls=root",scratch],env:process.env});
  if(rust.status!==0||!rust.stdout.toString().includes("hash "+expected+" stable_crate_id"))continue;
  writeFileSync(path,payload);
  console.log("[DEBUG]",name,"restored",output.sha256);
  process.exit(0);
}
throw Error("Original target metadata is absent");
