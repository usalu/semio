import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{expandRootV1,neutralizeRootV1}from"../../../🛰️exchange/🟦️.ts";
import{observationRowsHoldV1,withObservedWorldV1}from"../../../../👁️observation/🟨️.mjs";
import{createProjectFleetPortV1,projectRecordStateV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{dependencyGenerationV1,localRowsV1,portableRowsV1,type ObservationRowV1}from"../../🧳️portable/🟦️.ts";

/** 🌍️ Contract: a project record produced on one platform and checkout is taken by a machine of the other POSIX platform at another checkout, with that machine's paths and its own dependency generation and without the facts that name the producer's machine; a project that is bound to its host, and a machine of another class, never share a record. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),`records-two-${label}-`)));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLAIN=JSON.stringify(["rev",null,[["authority"],"libs/alpha",{name:"alpha",targets:{build:{}}}]]),BOUND=JSON.stringify(["rev",null,[["authority"],"libs/emitter",{name:"emitter",targets:{build:{metadata:{semio:{hostPlatformOutputsV1:{version:1}}}}}}]]);
const GENERATION_A="86415d695aa883fffc22",GENERATION_B="8714ffcdb7fdf62c3f57";

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
function checkout(label:string,generation?:string){
 const root=temp(label);
 if(generation!==undefined){const modules=join(root,".cache","b","d","m","j",generation,"node_modules");mkdirSync(modules,{recursive:true});symlinkSync(modules,join(root,"node_modules"));}
 return root;
}
const above=(root:string)=>{const rows:{kind:string;path:string;fact:string}[]=[];for(let at=dirname(root);;at=dirname(at)){rows.push({kind:"lstat",path:at,fact:"directory"});if(dirname(at)===at)break;}return rows;};
const recordOf=(root:string,generation:string)=>({digest:"d".repeat(64),authority:[],observations:[...above(root),{kind:"lstat",path:root,fact:"directory"},{kind:"file",path:`${root}/libs/alpha/src/a.ts`,fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}},{kind:"lstat",path:`${root}/.cache/b/d/m/j/${generation}/node_modules/typescript/lib/typescript.js`,fact:"file"},{kind:"field",path:`${root}/.cache/b/d/m/j/${generation}/node_modules/typescript/lib/typescript.js`,fact:{digest:"f".repeat(64),format:"package-identity",pointer:""}}],image:JSON.stringify({name:"alpha",root:"libs/alpha"})});
const infoOf=(root:string,keyText:string,store:{record?:unknown;imported?:unknown[]}):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText,exportRecord:()=>store.record,async import(portable){(store.imported??=[]).push(portable);return true;}});
type Row={kind:string;path:string;fact:unknown};

test("the state is the same for darwin-arm64 and linux-x64, differs for Windows and for a host-bound project, and a host-bound project is keyed by the exact platform",()=>{
 const state=(keyText:string,platform:string)=>projectRecordStateV1({root:"/r",fleet:"libs/alpha",keyText,platform});
 expect(state(PLAIN,"darwin-arm64").digest).toBe(state(PLAIN,"linux-x64").digest);
 expect(state(PLAIN,"darwin-arm64").frame.platform).toBe("posix-shared");
 expect(state(PLAIN,"win32-x64").digest).not.toBe(state(PLAIN,"linux-x64").digest);
 expect(state(BOUND,"darwin-arm64").digest).not.toBe(state(BOUND,"linux-x64").digest);expect(state(BOUND,"linux-x64").frame.platform).toBe("linux-x64");
});

test("a macOS record is taken by a Linux machine at another checkout: its paths, its own dependency generation, none of the producer's location rows; the original stays whole",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=checkout("mac",GENERATION_A),rootB=checkout("linux",GENERATION_B),store:{record?:unknown;imported?:unknown[]}={record:recordOf(rootA,GENERATION_A)},other:{record?:unknown;imported?:unknown[]}={};
  const mac=createProjectFleetPortV1({root:rootA,environment,owner:"mac:1",platform:"darwin-arm64"})!,linux=createProjectFleetPortV1({root:rootB,environment,owner:"linux:1",platform:"linux-x64"})!;
  const gate=await mac.gate(infoOf(rootA,PLAIN,store));expect(gate.seeded).toBe(false);gate.publish();await mac.flush();
  const taken=await linux.gate(infoOf(rootB,PLAIN,other));expect(taken.seeded).toBe(true);expect(linux.stats().seeded).toBe(1);
  const record=other.imported![0] as {observations:Row[];image:string};
  const rows=record.observations,text=JSON.stringify(rows);
  expect(text).not.toContain(rootA);expect(text).not.toContain(GENERATION_A);
  expect(rows.some(row=>row.kind==="lstat"&&above(rootA).some(ancestor=>ancestor.path===row.path))).toBe(false);
  expect(rows).toContainEqual({kind:"lstat",path:rootB,fact:"directory"});
  expect(rows).toContainEqual({kind:"file",path:`${rootB}/libs/alpha/src/a.ts`,fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}});
  expect(rows).toContainEqual({kind:"lstat",path:`${rootB}/.cache/b/d/m/j/${GENERATION_B}/node_modules/typescript/lib/typescript.js`,fact:"file"});
  expect(rows.filter(row=>row.kind==="field").map(row=>row.path)).toEqual([`${rootB}/.cache/b/d/m/j/${GENERATION_B}/node_modules/typescript/lib/typescript.js`]);
  expect(record.image).toBe(JSON.stringify({name:"alpha",root:"libs/alpha"}));
  expect(JSON.stringify(store.record)).toContain(GENERATION_A);expect((store.record as {observations:Row[]}).observations.some(row=>row.path==="/")).toBe(true);
 }finally{await server.stop();}
},60000);

test("a machine without a dependency generation takes the record with rows that cannot hold, so it infers the project itself",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=checkout("mac2",GENERATION_A),rootB=checkout("bare"),store={record:recordOf(rootA,GENERATION_A)},other:{imported?:unknown[]}={};
  const mac=createProjectFleetPortV1({root:rootA,environment,owner:"mac:1",platform:"darwin-arm64"})!,linux=createProjectFleetPortV1({root:rootB,environment,owner:"linux:1",platform:"linux-x64"})!;
  (await mac.gate(infoOf(rootA,PLAIN,store))).publish();await mac.flush();
  expect((await linux.gate(infoOf(rootB,PLAIN,other))).seeded).toBe(true);
  expect(JSON.stringify(other.imported)).toContain("generation-unavailable");
 }finally{await server.stop();}
},60000);

test("a host-bound project and a Windows machine never take a record of another class",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=checkout("mac3",GENERATION_A),rootB=checkout("linux3",GENERATION_B),rootW=checkout("win3",GENERATION_B);
  const mac=createProjectFleetPortV1({root:rootA,environment,owner:"mac:1",platform:"darwin-arm64"})!,linux=createProjectFleetPortV1({root:rootB,environment,owner:"linux:1",platform:"linux-x64"})!,windows=createProjectFleetPortV1({root:rootW,environment,owner:"win:1",platform:"win32-x64"})!;
  const store={record:recordOf(rootA,GENERATION_A)};
  (await mac.gate(infoOf(rootA,BOUND,store))).publish();(await mac.gate(infoOf(rootA,PLAIN,store))).publish();await mac.flush();
  const bound=await linux.gate(infoOf(rootB,BOUND,{}));expect(bound.seeded).toBe(false);await bound.abandon();
  const win=await windows.gate(infoOf(rootW,PLAIN,{}));expect(win.seeded).toBe(false);await win.abandon();
  expect((await linux.gate(infoOf(rootB,PLAIN,{}))).seeded).toBe(true);
 }finally{await server.stop();}
},60000);

test("the real replay of the observations holds in the tree of the taker with the portable rows and fails without the rewrite: the rows of a machine whose checkout lies elsewhere",()=>{
 const rootB=checkout("hold",GENERATION_B),PRODUCER="/zz-p14-producer/Users/Shared/semio",G=GENERATION_A,sha=(text:string)=>createHash("sha256").update(text).digest("hex");
 mkdirSync(join(rootB,"libs","alpha","src"),{recursive:true});writeFileSync(join(rootB,"libs","alpha","src","a.ts"),"abc");
 const lib=join(rootB,".cache","b","d","m","j",GENERATION_B,"node_modules","typescript","lib");mkdirSync(lib,{recursive:true});writeFileSync(join(lib,"typescript.js"),"x");
 const rows:ObservationRowV1[]=[{kind:"lstat",path:"/",fact:"directory"},{kind:"lstat",path:"/zz-p14-producer",fact:"directory"},{kind:"lstat",path:"/zz-p14-producer/Users",fact:"directory"},{kind:"lstat",path:"/zz-p14-producer/Users/Shared",fact:"directory"},{kind:"lstat",path:PRODUCER,fact:"directory"},
  {kind:"file",path:`${PRODUCER}/libs/alpha/src/a.ts`,fact:{size:3,mtimeMs:1,ctimeMs:1,ino:1,sha256:sha("abc")}},{kind:"lstat",path:`${PRODUCER}/.cache/b/d/m/j/${G}/node_modules/typescript/lib/typescript.js`,fact:"file"},{kind:"fileExists",path:`${PRODUCER}/.cache/b/d/m/j/${G}/node_modules/typescript/lib/typescript.js`,fact:true}];
 const moved=(list:readonly ObservationRowV1[])=>JSON.parse(Buffer.from(expandRootV1(neutralizeRootV1(Buffer.from(JSON.stringify(list)),PRODUCER),rootB)).toString("utf8")) as ObservationRowV1[];
 const holds=(list:readonly ObservationRowV1[])=>withObservedWorldV1(undefined as never,()=>observationRowsHoldV1(list as never));
 expect(holds(moved(rows))).toBe(false);
 const shared=localRowsV1(moved(portableRowsV1(rows,PRODUCER)),dependencyGenerationV1(rootB));
 expect(holds(shared)).toBe(true);
 expect(holds(localRowsV1(moved(portableRowsV1(rows,PRODUCER)),undefined))).toBe(false);
});
