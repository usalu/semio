import { readFileSync, writeFileSync, mkdirSync, renameSync, rmdirSync, existsSync } from "node:fs";
import { join, resolve, dirname, relative } from "node:path";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=resolve(import.meta.dir,"../.."),output=join(ticket,"🗑️generated/testing-taxonomy/animate-energy");
mkdirSync(output,{recursive:true});
const animate="✏️s/🔌️plugins/🎞️animate",energy="✏️s/🔌️plugins/🔋️energy";
const mappings=[
 [animate+"/🪨️tests/🟦️.ts",animate+"/🧫️fixtures/🌐️browser-environment/🟦️.ts"],
 [energy+"/🪨️tests/🧮️p7c1-energy-numerical-laws.json",energy+"/🧫️fixtures/🧮️numerical-laws/🔣️.json"],
 [energy+"/🪨️tests/🔗️p7c2-energy-retained-wire-laws.json",energy+"/🧫️fixtures/🔗️retained-wire-laws/🔣️.json"],
 [energy+"/🪨️tests/🦠️p7c2-energy-retained-wire-mutations.json",energy+"/🧫️fixtures/🦠️retained-wire-mutations/🔣️.json"]
];
const config=animate+"/📦️packages/🟦️typescript/vitest.config.ts",reader=energy+"/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/🦀️.rs";
const report=join(ticket,"📓️animate-energy-testing-taxonomy-2026-09-12.md"),input=join(import.meta.dir,"📥️input.md"),hash=(p:string)=>createHash("sha256").update(readFileSync(join(root,p))).digest("hex"),command=process.argv[2];
if(command==="capture"){
 const moves=mappings.map(([from,to])=>({from,to,sha256:hash(from)}));writeFileSync(input,"# Animate and Energy Fixture Input\n\nThe animation browser stand-ins provide a fake input environment. Energy law documents are immutable neutral data.\n\n```json\n"+JSON.stringify({moves},null,2)+"\n```\n");
 writeFileSync(report,"# Animate and Energy Testing Taxonomy — 2026-09-12\n\nThe browser environment is executable fixture input for jsdom tests. The three energy documents are neutral fixture data.\n\n"+moves.map(m=>"- `"+m.from+"` → `"+m.to+"` — SHA-256 `"+m.sha256+"`").join("\n")+"\n");
}else if(command==="baseline"||command==="verify"){
 const {startVitest}=await import("vitest/node");const result=join(output,command+"-vitest.json");
 const ctx=await startVitest("test",[],{root:join(root,animate,"📦️packages/🟦️typescript"),config:join(root,config),include:["../../🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts"],includeSource:[],watch:false,reporters:["json"],outputFile:result});
 await ctx?.close();
 const value=JSON.parse(readFileSync(result,"utf8"));writeFileSync(report,readFileSync(report,"utf8")+"\n## "+command+" Animation Runtime\n\nActual Vitest selected the canonical presentation index case. The baseline selection overrides the stale legacy include path solely to compare the same assertions.\n\n```json\n"+JSON.stringify({passed:value.numPassedTests,failed:value.numFailedTests,success:value.success,message:value.testResults?.map((r:any)=>r.message)},null,2)+"\n```\n");
 if(!value.success||!value.numPassedTests)process.exitCode=1;
 if(command==="verify"){
 const data=JSON.parse(readFileSync(input,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]);
 for(const m of data.moves)assert.equal(hash(m.to),m.sha256);
 const body=readFileSync(join(root,reader),"utf8"),token=body.match(/let source = include_str!\("([^"]*numerical-laws[^"]*)"\)/)![1],path=resolve(root,dirname(reader),token);
 assert.equal(path,join(root,mappings[1][1]));assert.equal(JSON.parse(readFileSync(path,"utf8")).schema,"semio.energy.numerical-laws/1");
 for(const [,to] of mappings.slice(1))JSON.parse(readFileSync(join(root,to),"utf8"));
 writeFileSync(report,readFileSync(report,"utf8")+"\nAll four final files retain their captured hashes. The actual Rust include path resolves to the canonical numerical-laws fixture; independent Node JSON parsing accepted all three documents.\n");
 }
}else if(command==="move"){
 const data=JSON.parse(readFileSync(input,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]);
 for(const m of data.moves){assert.equal(hash(m.from),m.sha256);assert(!existsSync(join(root,m.to)));mkdirSync(dirname(join(root,m.to)),{recursive:true});renameSync(join(root,m.from),join(root,m.to));assert.equal(hash(m.to),m.sha256);}
 for(const owner of [animate,energy])rmdirSync(join(root,owner,"🪨️tests"));
 let c=readFileSync(join(root,config),"utf8").replace("../../🪨️tests/🟦️.ts","../../🧫️fixtures/🌐️browser-environment/🟦️.ts").replace("../../🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🧪️index.test.ts","../../🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts");writeFileSync(join(root,config),c);
 const before=readFileSync(join(root,reader),"utf8"),token="../../../../../../🪨️tests/🧮️p7c1-energy-numerical-laws.json";assert(before.includes(token));writeFileSync(join(root,reader),before.replace(token,relative(dirname(join(root,reader)),join(root,mappings[1][1])).split("\\").join("/")));
 writeFileSync(report,readFileSync(report,"utf8")+"\n## Consumers\n\nUpdated `"+config+"` and `"+reader+"`. The obsolete test filename is removed from Vitest discovery.\n");
}else throw Error("Expected capture, baseline, move or verify");
console.log(JSON.stringify({command,report}));

