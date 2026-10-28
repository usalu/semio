/** 🎯️ Law: every project that applies the input precision rules records the facts of the rules (the engine, its schema and the rules file), whichever project of the same process asked first, so a record inferred by a farm worker in a fresh process holds the same rows as one inferred in the parent. Independent oracle: the sha256 Python computes for each of the three files. */
import{expect,test}from"bun:test";
import{dirname,join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{observeDetachedV1}from"../../../../👁️observation/🟨️.mjs";
import{libraryBootstrap}from"../../🟨️.mjs";
import{readProjectInputPrecisionV1}from"../🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),library=resolve(here,"../../../../.."),python=testPythonV1();
const files=["🏗️build/📥️inputs/🎯️precision/🟦️.ts","🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json","🏗️build/📥️inputs/🎯️precision/🔣️.json"].map(path=>join(library,path));

test("each of two projects in one process records the facts of the precision rules it applies, equal to the independent digests",async()=>{
 await libraryBootstrap;
 const expected=JSON.parse(Bun.spawnSync([python!,"-I","-c","import hashlib,json,sys;print(json.dumps([hashlib.sha256(open(p,'rb').read()).hexdigest() for p in sys.argv[1:]]))",...files]).stdout.toString());
 const project=async()=>{const{value,rows,layers}=await observeDetachedV1(async()=>readProjectInputPrecisionV1());const all=[...rows,...layers.flat()] as{kind:string;path:string;fact:{sha256?:string}}[];return{value,digests:files.map(file=>all.find(row=>row.kind==="file"&&row.path===file)?.fact?.sha256)};};
 const first=await project(),second=await project();
 expect(first.value).toBeDefined();expect(second.value).toBe(first.value);
 expect(first.digests).toEqual(expected);expect(second.digests).toEqual(expected);
},60000);
