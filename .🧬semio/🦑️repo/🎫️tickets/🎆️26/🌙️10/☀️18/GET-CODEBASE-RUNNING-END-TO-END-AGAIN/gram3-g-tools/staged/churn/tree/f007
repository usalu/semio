/** 🦀️ Contract: the mounted Rust source files of a crate are the same whether the project asks once or for every one of its targets: repeated asks with one fact table answer the same files and replay the same observation rows as a fresh ask, each asker owns its list, and an edit of the crate (a new module declaration) is seen by the next ask. The files equal an independent glob of the crate's sources. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fastGlob from"fast-glob";

const inference=await import(new URL("../../../🟨️.mjs",import.meta.url).href) as any;
await inference.libraryBootstrap;
const observation=await import(new URL("../../../../../👁️observation/🟨️.mjs",import.meta.url).href) as any;
const{cargoSourceInputs}=inference.cacheInternals as{cargoSourceInputs:(root:string,workspace:string,facts:Map<unknown,unknown>,includeTests?:boolean,compiled?:boolean)=>string[]|undefined};
const crate=(workspace:string)=>{
 mkdirSync(join(workspace,"c1","src"),{recursive:true});mkdirSync(join(workspace,"c1","tests"),{recursive:true});
 writeFileSync(join(workspace,"c1","Cargo.toml"),'[package]\nname = "c1"\nversion = "0.1.0"\nedition = "2021"\n[lib]\npath = "src/lib.rs"\n');
 writeFileSync(join(workspace,"c1","src","lib.rs"),"mod a;\nmod b;\n");
 writeFileSync(join(workspace,"c1","src","a.rs"),"pub fn a() {}\n");
 writeFileSync(join(workspace,"c1","src","b.rs"),"pub fn b() {}\n");
 writeFileSync(join(workspace,"c1","tests","t.rs"),"#[test] fn t() {}\n");
};
const capture=(compute:()=>unknown)=>observation.captureObservedSourceInputsV1(async()=>compute(),undefined,false) as Promise<{value:string[]|undefined;observations:{kind:string;path:string;fact:unknown}[]}>;
const sorted=(rows:{kind:string;path:string;fact:unknown}[],workspace:string)=>rows.filter(row=>row.path.startsWith(workspace)).map(row=>JSON.stringify([row.kind,row.path,row.fact])).sort();
const glob=async(workspace:string,patterns:string[])=>(await fastGlob(patterns,{cwd:workspace})).sort().map(path=>"{workspaceRoot}/"+path);
const settle=()=>Bun.sleepSync(2300);

test("repeated asks with one fact table equal a fresh ask, replay its rows and hand out independent lists",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"source-inputs-memo-"));crate(workspace);settle();
 try{
  const expectedSources=await glob(workspace,["c1/src/*.rs"]),expectedAll=await glob(workspace,["c1/src/*.rs","c1/tests/*.rs"]);
  await capture(()=>cargoSourceInputs("c1",workspace,new Map(),true));
  const fresh=await capture(()=>cargoSourceInputs("c1",workspace,new Map(),true));
  expect(fresh.value).toEqual(expectedAll);
  expect(sorted(fresh.observations,workspace).length).toBeGreaterThan(5);
  expect((await capture(()=>cargoSourceInputs("c1",workspace,new Map(),false))).value).toEqual(expectedSources);
  const facts=new Map(),first=await capture(()=>cargoSourceInputs("c1",workspace,facts,true)),second=await capture(()=>cargoSourceInputs("c1",workspace,facts,true)),third=await capture(()=>cargoSourceInputs("c1",workspace,facts,true));
  for(const round of[first,second,third]){expect(round.value).toEqual(expectedAll);expect(sorted(round.observations,workspace)).toEqual(sorted(fresh.observations,workspace));}
  expect(second.value).not.toBe(first.value);
  second.value!.push("{workspaceRoot}/c1/src/mutated.rs");
  expect((await capture(()=>cargoSourceInputs("c1",workspace,facts,true))).value).toEqual(expectedAll);
  expect((await capture(()=>cargoSourceInputs("c1",workspace,facts,false))).value).toEqual(expectedSources);
  expect((await capture(()=>cargoSourceInputs("missing",workspace,facts,true))).value).toBeUndefined();
  expect((await capture(()=>cargoSourceInputs("missing",workspace,facts,true))).value).toBeUndefined();
 }finally{rmSync(workspace,{recursive:true,force:true});}
},60_000);

test("an edit of the crate is seen by the next ask with the same fact table",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"source-inputs-edit-"));crate(workspace);settle();
 try{
  const facts=new Map();
  expect((await capture(()=>cargoSourceInputs("c1",workspace,facts,false))).value).toEqual(await glob(workspace,["c1/src/*.rs"]));
  writeFileSync(join(workspace,"c1","src","lib.rs"),"mod a;\nmod b;\nmod c;\n");writeFileSync(join(workspace,"c1","src","c.rs"),"pub fn c() {}\n");settle();
  const after=await capture(()=>cargoSourceInputs("c1",workspace,facts,false));
  expect(after.value).toEqual(await glob(workspace,["c1/src/*.rs"]));
  expect(after.value).toContain("{workspaceRoot}/c1/src/c.rs");
 }finally{rmSync(workspace,{recursive:true,force:true});}
},60_000);
