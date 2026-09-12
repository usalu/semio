import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, rmdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),output=join(ticket,"🗑️generated/testing-taxonomy/reference-oracles");
const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",flow="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs";
const replacements=[
 [library+"/🧪️tests/↪️rust-divergence-callback/🦀️.rs",library+"/🔮️oracles/↪️rust-divergence-callback/🦀️.rs"],
 [library+"/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml",library+"/🔮️oracles/🧲️rust-physical-reference-context/⚙️.toml"],
 [library+"/🧪️tests/🧲️rust-physical-reference-context/🦀️.rs",library+"/🔮️oracles/🧲️rust-physical-reference-context/🦀️.rs"],
 [flow+"/🧫️fixtures/🔮️oracle/🔣️.json",flow+"/🧫️fixtures/🧾️semantic-history/🔣️.json"]
];
const consumers=[library+"/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json",library+"/🧫️fixtures/↪️rust-divergence-callback/🔣️.json",library+"/🧬️schema/↪️rust-divergence-callback/🔣️.json",flow+"/🧪️tests/🔬️flow-vcs/🦀️.rs"];
const inputPath=join(import.meta.dir,"📥️input.md"),report=join(ticket,"📓️reference-oracle-classification-2026-09-12.md"),command=process.argv[2];
mkdirSync(output,{recursive:true});
const hash=(p:string)=>createHash("sha256").update(readFileSync(join(root,p))).digest("hex");
if(command==="capture"){
 const input={moves:replacements.map(([oldPath,newPath])=>({oldPath,newPath,sha256:hash(oldPath)})),consumers};
 writeFileSync(inputPath,"# Reference Oracle Relocation Input\n\n```json\n"+JSON.stringify(input,null,2)+"\n```\n");
 writeFileSync(report,(existsSync(report)?readFileSync(report,"utf8")+"\n":"")+"# Reference Oracle Classification — 2026-09-12\n\nThe syn Rust executable and its Cargo manifest are independent comparator code and belong together in the library's canonical oracles. Flow VCS's serialized expected operation history is static test data and receives a descriptive fixture case name.\n\n## Exact Paths and Pre-Move Hashes\n\n```json\n"+JSON.stringify(input,null,2)+"\n```\n");
 console.log("[DEBUG] captured four reference/data moves and four consumers");
}else if(command==="move"){
 const input=JSON.parse(readFileSync(inputPath,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]);
 for(const row of input.moves){assert.equal(hash(row.oldPath),row.sha256);assert(!existsSync(join(root,row.newPath)));mkdirSync(dirname(join(root,row.newPath)),{recursive:true});renameSync(join(root,row.oldPath),join(root,row.newPath));assert.equal(hash(row.newPath),row.sha256);}
 for(const path of consumers){const before=readFileSync(join(root,path),"utf8");let after=before;for(const [from,to]of replacements)after=after.replaceAll(from,to);if(path.startsWith(flow))after=after.replaceAll("🧫️fixtures/🔮️oracle/🔣️.json","🧫️fixtures/🧾️semantic-history/🔣️.json");assert.notEqual(after,before,path);writeFileSync(join(root,path),after);}
 rmdirSync(join(root,library,"🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle"));rmdirSync(join(root,flow,"🧫️fixtures/🔮️oracle"));
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Move Verification\n\nAll four hashes are preserved; old paths and empty alias directories are absent. Four active consumers have exact new references.\n");console.log("[DEBUG] four moves preserve bytes and four consumers updated");
}else if(command==="baseline"||command==="verify"){
 const temp=join(output,command,"temp");mkdirSync(temp,{recursive:true});
 const tests=["🧲️rust-physical-reference-context","↪️rust-divergence-callback"].map(name=>join(root,library,"🧪️tests",name,"🟦️.ts"));
 const args=["bun","test",...tests,"--test-name-pattern","independent syn (parsing|callback)","--timeout","180000"],stdout=join(output,command+".stdout.log"),stderr=join(output,command+".stderr.log");
 writeFileSync(stdout,"");writeFileSync(stderr,"");
 const child=Bun.spawn(args,{cwd:root,env:{...process.env,TMPDIR:temp,TMP:temp,TEMP:temp,CARGO_BUILD_JOBS:"3"},stdout:Bun.file(stdout),stderr:Bun.file(stderr)});
 const cancel=()=>child.kill("SIGTERM"),timer=setTimeout(cancel,300000);process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
 const status=await child.exited;clearTimeout(timer);process.off("SIGINT",cancel);process.off("SIGTERM",cancel);
 const text=readFileSync(stdout,"utf8")+readFileSync(stderr,"utf8"),receipt={args,status,tail:text.split("\n").slice(-35).join("\n")};
 writeFileSync(report,readFileSync(report,"utf8")+"\n## "+command+" Runtime\n\n```json\n"+JSON.stringify(receipt,null,2)+"\n```\n");
 console.log("[DEBUG] "+command+" reference comparators exit="+status);console.log(receipt.tail);process.exitCode=status;
}else throw new Error("Expected capture, baseline, move or verify");
