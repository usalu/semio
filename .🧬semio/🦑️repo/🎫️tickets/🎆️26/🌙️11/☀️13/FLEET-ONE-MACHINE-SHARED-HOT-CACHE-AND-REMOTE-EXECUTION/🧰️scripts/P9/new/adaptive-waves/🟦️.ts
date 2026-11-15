/** 📈️ Contract: a wave that infers nothing (every project is a hit of its stored record) costs a fixed time of the whole graph, so the next wave grows toward a time budget, bounded by a growth factor and a maximum; a wave that infers returns to the base wave, whose publication is the progress the fleet waits on. The cases are language-agnostic data in `./🔣️.json`. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {GENERATION_POLICY_V1,nextOwnerWaveV1} from "../../🟨️.mjs";
import {shapeComputeResultV1} from "../../🔌️load/🟨️.mjs";
import {createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const BASE=GENERATION_POLICY_V1.ownerWave,scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-adaptive-waves-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const waved=${at};\n`);};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the policy declares the budget, the maximum and the growth",()=>{
 expect(GENERATION_POLICY_V1.ownerWaveBudgetMilliseconds).toBe(300000);expect(GENERATION_POLICY_V1.ownerWaveMaximum).toBe(960);expect(GENERATION_POLICY_V1.ownerWaveGrowth).toBe(4);expect(BASE).toBe(48);
});

for(const row of fixture.cases)test(row.name,()=>{
 expect(nextOwnerWaveV1({size:row.size,elapsedMilliseconds:row.elapsedMilliseconds,inferred:(row as {inferred?:number}).inferred,basisPoints:(row as {basisPoints?:number}).basisPoints})).toBe(row.expected);
});

test("an independent computation of the rule agrees on a grid",()=>{
 const base=BASE,maximum=960,growth=4,budget=300000;
 for(const size of [48,96,192,500,960])for(const elapsed of [0,1000,60000,300000,900000,5000000])for(const inferred of [undefined,0,1,size/4,size/4+1]){
  const hit=inferred!==undefined&&inferred*4<=size,scaled=Math.floor(size*budget/Math.max(1,elapsed)),expected=hit?Math.max(base,Math.min(maximum,inferred===0?size*growth:size,scaled)):base;
  expect(nextOwnerWaveV1({size,elapsedMilliseconds:elapsed,inferred})).toBe(expected);
 }
});

const COUNT=BASE*7+5;
test("waves that report no inference grow, and the final generation holds every owner once",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>({...await compute(spec,signal),inferred:0})});
  await world.refresh();
  const before=(await world.store.read())!.pointer.generation;
  await edit(dir,COUNT);
  world.log.length=0;
  const result=await world.refresh();
  expect(result.published).toBe(true);expect(result.computed).toBe(COUNT);
  expect(world.log.map(entry=>entry.full.length)).toEqual([BASE,BASE*4,COUNT-BASE*5]);
  expect((await world.store.read())!.pointer.generation-before).toBe(3);
  expect(new Set(world.log.flatMap(entry=>entry.full)).size).toBe(COUNT);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("waves that report no information keep the base wave",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh();
  await edit(dir,COUNT);
  world.log.length=0;
  await world.refresh();
  expect(world.log.map(entry=>entry.full.length)).toEqual([BASE,BASE,BASE,BASE,BASE,BASE,BASE,5]);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a wave that inferred most of its projects returns to the base wave",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  let wave=0;
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>({...await compute(spec,signal),inferred:++wave===2?spec.owners.length:0})});
  await world.refresh();wave=0;
  await edit(dir,COUNT);
  world.log.length=0;
  await world.refresh();
  expect(world.log.map(entry=>entry.full.length)).toEqual([BASE,BASE*4,BASE,COUNT-BASE*6]);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the elapsed time of a wave decides how far the next one grows",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  let clock=0;
  const world=createWorld(dir,{now:()=>clock,computeWrap:compute=>async(spec,signal)=>{const result=await compute(spec,signal);clock+=100000;return {...result,inferred:0};}});
  await world.refresh();clock=0;
  await edit(dir,COUNT);
  world.log.length=0;
  await world.refresh();
  expect(world.log.map(entry=>entry.full.length)).toEqual([BASE,BASE*3,COUNT-BASE*4]);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("every published wave carries the seconds of its stages, the stages of the part included",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>({...await compute(spec,signal),inferred:0,phases:{setup:7,projects:11,tail:3}})});
  await world.refresh();
  await edit(dir,COUNT);
  const told:{done:number;total:number;phases?:Record<string,number>}[]=[];
  await world.refresh({published:(_generation:number,info:never)=>{told.push(info);}});
  expect(told.length).toBe(3);
  for(const info of told){
   expect(Object.keys(info.phases!).sort()).toEqual(["compute","compute.projects","compute.setup","compute.tail","fragments","pack","patch","plan","prime","publish"]);
   for(const value of Object.values(info.phases!))expect(Number.isFinite(value)&&value>=0).toBe(true);
   expect([info.phases!["compute.setup"],info.phases!["compute.projects"],info.phases!["compute.tail"]]).toEqual([7,11,3]);
  }
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the shaped compute result forwards what the part said about inference and its stages, and nothing else",()=>{
 const part={owners:[],edgeOnly:[],edges:[],rows:[],env:{digest:"x",rows:[]},externalNodes:{},parseUnavailable:[]};
 const shape=(extra:object)=>shapeComputeResultV1({workspaceRoot:"/w",part:{...part,...extra} as never,stubs:{}});
 expect(shape({inferred:3,phases:{tail:5}})).toMatchObject({inferred:3,phases:{tail:5}});
 const bare=shape({}) as Record<string,unknown>;expect("inferred" in bare).toBe(false);expect("phases" in bare).toBe(false);
 expect("inferred" in (shape({inferred:"3"}) as object)).toBe(false);
});
