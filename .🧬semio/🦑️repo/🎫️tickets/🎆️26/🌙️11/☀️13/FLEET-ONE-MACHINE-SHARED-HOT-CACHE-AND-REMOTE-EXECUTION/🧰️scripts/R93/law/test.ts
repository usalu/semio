import{afterAll,expect,test}from"bun:test";
import{spawn}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";

/** 🌊️ A cold machine takes an owner whose record names many large closure entries one entry at a time: it holds a bounded window of them, not all of them unpacked and localised at once (a 16 GB machine was cut at 10 GB while it seeded one owner), and an entry that two owners share by digest is downloaded and stored once. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`seed-stream-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),BUNDLES=24,ROWS=14000,LIMIT_MEGABYTES=400;

const recipes=Array.from({length:BUNDLES},(_,index)=>({fn:"native-command",n:index}));
const image=JSON.stringify({name:"big",closureRecipes:Object.fromEntries(recipes.map(recipe=>[closureRecipeKeyV1(recipe),recipe]))});
const populate=async(root:string)=>{
 mkdirSync(join(root,"ce"),{recursive:true});
 const store=createClosureStoreV1({directory:join(root,"ce")});
 for(const [index,recipe] of recipes.entries()){
  const rows=Array.from({length:ROWS},(_,at)=>({kind:"file",path:`${root}/g/${index}/${at}.ts`,fact:{sha256:sha(index+"."+at),size:at,mtimeMs:1,ctimeMs:1,ino:at}}));
  await store.save(`${root}\0${closureRecipeKeyV1(recipe)}`,{recipe,value:[],rows,layers:[rows.slice(0,ROWS/2)],sidecar:{facts:[null],members:[null]}});
 }
 bindClosureStoreV1(root,join(root,"ce"),true);
};
const infoOf=(root:string,name:string):ProjectRecordInfoV1=>({cachePath:join(root,"np",name+".json"),fleet:"libs/"+name,keyText:JSON.stringify(["rev",null,name]),exportRecord:()=>({digest:sha(name),authority:[],observations:[{kind:"file",path:`${root}/libs/${name}/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image}),async import(){return true;},closures:closureTravelV1});

test("a cold machine seeds an owner with many large closure entries one entry at a time within a bounded resident set, and a second owner that shares them downloads none of them again",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 try{
  const rootA=temp("a"),rootB=temp("b");await populate(rootA);
  const producer=createProjectFleetPortV1({root:rootA,environment,owner:"producer:1"})!;
  for(const name of["alpha","beta"]){const gate=await producer.gate(infoOf(rootA,name));expect(gate.seeded).toBe(false);gate.publish();}
  await producer.flush();expect(producer.stats().published,"both owners were published").toBe(2);
  const run=await new Promise<{stdout:string;stderr:string}>(done=>{const child=spawn(process.execPath,[join(dirname(fileURLToPath(import.meta.url)),"👶️child.ts"),JSON.stringify({root:rootB,owners:["alpha","beta"]})],{env:{...process.env,...environment}});let stdout="",stderr="";child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});child.on("close",()=>done({stdout,stderr}));});
  const line=run.stdout.split("\n").find(each=>each.startsWith("RESULT "));
  expect(line,run.stderr.slice(-2000)).toBeDefined();
  const result=JSON.parse(line!.slice(7)) as {out:{name:string;seeded:boolean;closures:number;blobs:number}[];peakMegabytes:number};
  expect(result.out.map(each=>[each.name,each.seeded])).toEqual([["alpha",true],["beta",true]]);
  expect(result.out[0]!.closures,"every entry of the first owner was stored").toBe(BUNDLES);
  expect(result.out[1]!.closures,"the entries the second owner shares were stored already").toBe(0);
  expect(result.out[1]!.blobs,"only the record of the second owner was downloaded").toBeLessThanOrEqual(1);
  expect(result.peakMegabytes,"the resident set of a cold seeding stays within a window of entries").toBeLessThan(LIMIT_MEGABYTES);
  const store=createClosureStoreV1({directory:join(rootB,"ce")}),entry=await store.bundle(`${rootB}\0${closureRecipeKeyV1(recipes[3]!)}`);
  expect(entry!.rows.length).toBe(ROWS);expect((entry!.rows[7] as {path:string}).path).toBe(`${rootB}/g/3/7.ts`);expect(JSON.stringify(entry!.rows[7])).not.toContain(rootA);
 }finally{await server.stop();}
},300000);
