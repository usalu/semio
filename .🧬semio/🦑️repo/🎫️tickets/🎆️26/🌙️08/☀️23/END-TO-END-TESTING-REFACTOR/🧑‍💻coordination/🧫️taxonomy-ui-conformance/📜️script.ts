import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { pathToFileURL } from "node:url";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir));
const owner="🧰️framework/🔨️modules/🖱️ui/🧬️contract",oldRoot=owner+"/📚️examples/🧪️conformance",newRoot=owner+"/🧫️fixtures/🧪️conformance";
const output=join(ticket,"🗑️generated/testing-taxonomy/ui-conformance"),inputPath=join(import.meta.dir,"📥️input.md"),report=join(ticket,"📓️ui-conformance-fixture-classification-2026-09-12.md");
mkdirSync(output,{recursive:true});
const files=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?files(join(directory,entry.name)):[join(directory,entry.name)]);
const command=process.argv[2];
if(command==="capture"){
 const consumers=Bun.spawn(["rg","-l","-F","📚️examples/🧪️conformance","🧰️framework","📜️script.ts","-g","!AGENTS.md","-g","!node_modules","-g","!🗑️generated","-g","!target"],{cwd:root,stdout:"pipe",stderr:"inherit"});
 const paths=(await new Response(consumers.stdout).text()).trim().split("\n").filter(Boolean).sort();assert.equal(await consumers.exited,0);
 const ledger=files(join(root,oldRoot)).sort().map(path=>({oldPath:relative(root,path).replaceAll("\\","/"),newPath:newRoot+"/"+relative(join(root,oldRoot),path).replaceAll("\\","/"),sha256:createHash("sha256").update(readFileSync(path)).digest("hex")}));
 assert.equal(ledger.length,147);assert.equal(paths.length,6);
 writeFileSync(inputPath,"# UI Conformance Relocation Input\n\n```json\n"+JSON.stringify({oldRoot,newRoot,ledger,consumers:paths},null,2)+"\n```\n");
 console.log("[DEBUG] captured 147 fixture files and six readers/documentation consumers");
}else if(command==="baseline"||command==="verify"){
 const input=JSON.parse(readFileSync(inputPath,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]);
 if(command==="verify"){assert(!existsSync(join(root,oldRoot)));for(const item of input.ledger)assert.equal(createHash("sha256").update(readFileSync(join(root,item.newPath))).digest("hex"),item.sha256,item.newPath);}
 const {conformanceCorpusSelfTests}=await import(pathToFileURL(join(root,owner,"🧪️tests/🔬️conformance-corpus/🟦️.ts")).href);
 const cases=conformanceCorpusSelfTests();assert.equal(cases,62);
 console.log("[DEBUG] "+command+" UI conformance corpus Ajv oracle passed: "+cases+" cases");
 const section="\n## "+command+" Runtime\n\nThe actual conformanceCorpusSelfTests export passed all 62 catalog cases using strict Ajv, exact directory-role comparison and expected-case identities.\n";
 if(command==="baseline")writeFileSync(report,"# UI Conformance Fixture Classification — 2026-09-12\n\nThe 147-file conformance corpus is used only by test-gated Rust and renderer tests. It belongs in the UI contract owner's fixtures, outside cases and product examples.\n"+section+"\n## Exact Moves\n\n```json\n"+JSON.stringify(input.ledger,null,2)+"\n```\n\n## Exact Authored Consumer Paths\n\n```json\n"+JSON.stringify(input.consumers,null,2)+"\n```\n");
 else writeFileSync(report,readFileSync(report,"utf8")+section+"\nThe old root is absent; all 147 relocated file hashes equal their retained pre-move hashes.\n");
}else if(command==="native"){
 const log=join(output,"native.log"),args=["cargo","test","--locked","--offline","--manifest-path",join(root,owner,"📦️packages/🦀️rust/Cargo.toml"),"--all-features","--lib","--","conformance::","--nocapture"];
 const child=Bun.spawn(args,{cwd:root,env:{...process.env,CARGO_TARGET_DIR:join(output,"target"),CARGO_BUILD_JOBS:"4"},stdout:Bun.file(log+".stdout"),stderr:Bun.file(log)});
 const cancel=()=>child.kill("SIGTERM"),timer=setTimeout(cancel,300_000);process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
 const status=await child.exited;clearTimeout(timer);process.off("SIGINT",cancel);process.off("SIGTERM",cancel);
 const text=readFileSync(log+".stdout","utf8")+readFileSync(log,"utf8"),receipt={status,args,result:text.match(/test result:[^\n]+/g)??[],tail:text.split("\n").slice(-35).join("\n")};
 writeFileSync(join(output,"native.json"),JSON.stringify(receipt,null,2));
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Native Consumer Runtime\n\n```json\n"+JSON.stringify(receipt,null,2)+"\n```\n");
 console.log("[DEBUG] UI conformance native consumer exit="+status+" "+JSON.stringify(receipt.result));if(status!==0)console.log(receipt.tail);process.exitCode=status;
}else throw new Error("Expected capture, baseline, verify or native");
