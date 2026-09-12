import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { startVitest } from "vitest/node";
import { resolve } from "node:path";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),output=join(ticket,"🗑️generated/testing-taxonomy/core"),command=process.argv[2];
mkdirSync(output,{recursive:true});
const report=join(ticket,"📓️core-testing-taxonomy-2026-09-12.md");
if(command==="native-readers"){
 const packets=[...readFileSync(join(import.meta.dir,"📥️input.md"),"utf8").matchAll(/```json\n([\s\S]*?)\n```/g)].map(match=>JSON.parse(match[1]!)),packet=packets.find(row=>row.reads),fixture=readFileSync(join(root,packet.reads[0].fixture),"utf8");
 const paths=packet.reads.map((row:any)=>{const source=readFileSync(join(root,row.path),"utf8"),matches=[...source.matchAll(/include_str!\("([^"]*boxed-fixed-slots[^"]*)"\)/g)];assert.equal(matches.length,1);const path=resolve(dirname(join(root,row.path)),matches[0]![1]!);assert.equal(readFileSync(path,"utf8"),fixture);return path;});
 const input=join(import.meta.dir,"🦀️.rs"),binary=join(output,process.platform==="win32"?"reader.exe":"reader.bin");
 writeFileSync(input,"fn main() { let sources = ["+paths.map((path:string)=>"include_str!("+JSON.stringify(path)+")").join(",")+"]; for source in sources { assert_eq!(source, sources[0]); } print!(\"{}\", sources[0]); }\n");
 const compiled=Bun.spawnSync(["rustc","--edition=2021","--crate-name","taxonomy_fixture_readers",input,"-o",binary],{cwd:root,stdout:"pipe",stderr:"pipe",timeout:30000});assert.equal(compiled.exitCode,0,compiled.stderr.toString());
 const ran=Bun.spawnSync([binary],{cwd:root,stdout:"pipe",stderr:"pipe",timeout:10000});assert.equal(ran.exitCode,0,ran.stderr.toString());assert.deepEqual(JSON.parse(ran.stdout.toString()),JSON.parse(fixture));
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Native Fixture Reader Runtime\n\nRustc compiled all six exact current include_str fixture references. The native program confirmed byte equality across them and printed the embedded JSON, which independently matched Node JSON parsing of the committed fixture. Compiler and native execution both exited zero.\n");console.log("[DEBUG] six exact native fixture readers compiled and matched committed JSON");process.exit(0);
}
const specs=command==="actor"?[["actor","🧰️framework/🔨️modules/🎭️actor","send-message effects at the wire"]]as const:[["async","🧰️framework/🔨️modules/⏳️async",undefined],["kernel","🧰️framework/🔨️modules/🎠️kernel","exampleArtifactSources|scopeContributionsJson"]]as const;
for(const [name,path,pattern]of specs){
 const context=await startVitest("test",[],{config:join(root,path,"📦️packages/🟦️typescript/vitest.config.ts"),run:true,maxWorkers:1,fileParallelism:false,testNamePattern:pattern},{cacheDir:join(output,name+"-cache")});
 try{
  const tasks=(nodes:any[]):any[]=>nodes.flatMap(node=>node.type==="test"?[node]:tasks(node.tasks??[])),tests=tasks(context.state.getFiles());
  const receipt={command,name,passed:tests.filter(t=>t.result?.state==="pass").length,failed:tests.filter(t=>t.result?.state==="fail").length,files:context.state.getFiles().map((f:any)=>({path:f.filepath,state:f.result?.state,errors:f.result?.errors?.map((e:any)=>e.message)})),errors:context.state.getUnhandledErrors().map(String)};
  let text="";try{text=readFileSync(report,"utf8")}catch{}writeFileSync(report,text+"\n## "+command+" "+name+" Runtime\n\n```json\n"+JSON.stringify(receipt,null,2)+"\n```\n");
  assert.equal(receipt.failed,0);assert.equal(receipt.errors.length,0);assert(receipt.passed>0);assert(receipt.files.every(f=>f.state!=="fail"));console.log("[DEBUG] "+name+" "+command+" passed="+receipt.passed);
 }finally{await context.close();}
}
