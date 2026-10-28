/** 🧱️ Contract: the fact index of a cold build is built without expanding the (owner, row) pairs and still equals the expanding builder byte for byte (rows, classes, order) on seeded corpora with shared blocks, private rows, duplicate rows, equal rows of different objects and empty owners, and the Python partition model; thousands of owners that share a block cost memory by the block, not by the pairs; a bulk patch (many upserted owners) names the same consumers for every row as the per-owner patch. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{compareCodePointsV1,createFactIndexV1,patchFactIndexV1,rowKeyV1}from"../../🟨️.mjs";
import{legacyCreateFactIndexV1}from"./legacy.ts";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

type Row={kind:string;path:string;fact:unknown};
const random=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const pool=(next:()=>number,size:number):Row[]=>Array.from({length:size},(_,i)=>({kind:["exists","file","readdir","resolve"][i%4]!,path:"/w/p"+(i%977)+"/"+"x".repeat(i%7),fact:i%5===0?{size:i,mtimeMs:1,ctimeMs:1,ino:1,sha256:"h"+(i%13)}:"v"+(i%11)}));
function corpus(seed:number):Map<string,Row[]>{
 const next=random(seed),rows=pool(next,200+Math.floor(next()*3000)),blocks=Array.from({length:1+Math.floor(next()*5)},()=>rows.slice(Math.floor(next()*rows.length*0.5)).slice(0,20+Math.floor(next()*600))),owners=new Map<string,Row[]>(),count=1+Math.floor(next()*120);
 for(let owner=0;owner<count;owner++){
  const list:Row[]=[];
  if(next()<0.9)for(const block of blocks)if(next()<0.7)for(const row of block)list.push(next()<0.02?{...row}:row);
  for(let extra=Math.floor(next()*40);extra>0;extra--)list.push(rows[Math.floor(next()*rows.length)]!);
  if(next()<0.1)list.length=0;
  if(next()<0.2)list.push(...list.slice(0,10));
  owners.set("o"+String(Math.floor(next()*1e6)).padStart(6,"0")+"-"+owner,list);
 }
 return owners;
}
const consumers=(index:ReturnType<typeof createFactIndexV1>)=>{
 const out=new Map<string,Set<string>>();
 for(const cls of index.classes)for(const id of cls.r){const key=rowKeyV1(index.rows[id] as never);let set=out.get(key);if(!set)out.set(key,set=new Set());for(const owner of cls.o)set.add(index.owners[owner]!);}
 return new Map([...out].map(([key,set])=>[key,[...set].sort().join("\u0001")]));
};

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the chunked builder equals the expanding builder byte for byte on seeded corpora",()=>{
 for(let seed=1;seed<=fixture.equality.seeds;seed++){
  const input=corpus(seed),actual=createFactIndexV1(input as never),expected=legacyCreateFactIndexV1(input,{rowKeyV1:rowKeyV1 as never,compareCodePointsV1});
  expect(actual.owners).toEqual(expected.owners);
  expect(actual.rows.length).toBe(expected.rows.length);
  expect(actual.rows.every((row,at)=>row===expected.rows[at])).toBe(true);
  expect(actual.classes).toEqual(expected.classes);
 }
});

/** 🔢️ Counts the bytes of the typed arrays a call allocates: the pairs of (owner, row) of an expanding builder live in typed arrays, so the sum measures them without depending on collection timing, the runtime or the platform. */
function typedBytes<T>(run:()=>T):{value:T;bytes:number}{
 const Original=globalThis.Uint32Array;let bytes=0;
 class Counting extends Original{constructor(...args:unknown[]){super(...(args as [number]));if(typeof args[0]==="number")bytes+=args[0]*4;else if(args[0]&&typeof (args[0] as {length?:number}).length==="number")bytes+=(args[0] as {length:number}).length*4;}}
 (globalThis as {Uint32Array:unknown}).Uint32Array=Counting;
 try{return{value:run(),bytes};}finally{(globalThis as {Uint32Array:unknown}).Uint32Array=Original;}
}

test("owners that share a block cost memory by the block, not by the pairs",()=>{
 const next=random(5),shared=pool(next,fixture.memory.sharedRows),priv=pool(next,fixture.memory.owners*fixture.memory.privateRows).map((row,at)=>({...row,path:"/w/private/"+at})),owners=new Map<string,Row[]>();
 for(let owner=0;owner<fixture.memory.owners;owner++)owners.set("o"+String(owner).padStart(6,"0"),[...shared,...priv.slice(owner*fixture.memory.privateRows,(owner+1)*fixture.memory.privateRows)]);
 const chunked=typedBytes(()=>createFactIndexV1(owners as never)),expanding=typedBytes(()=>legacyCreateFactIndexV1(owners,{rowKeyV1:rowKeyV1 as never,compareCodePointsV1})),pairs=fixture.memory.owners*(fixture.memory.sharedRows+fixture.memory.privateRows);
 expect(chunked.value.classes.length).toBeLessThan(fixture.memory.owners+10);
 expect(expanding.bytes,"the expanding builder holds its pairs in typed arrays").toBeGreaterThan(pairs*4);
 expect(chunked.bytes,`${pairs} pairs held in ${Math.round(chunked.bytes/1024)} KiB of typed arrays (expanding: ${Math.round(expanding.bytes/1048576)} MiB)`).toBeLessThan(fixture.memory.budgetMegabytes*1048576);
},120000);

test("a bulk patch names the same consumers for every row as the per-owner patch",()=>{
 for(let seed=1;seed<=fixture.bulkPatch.seeds;seed++){
  const next=random(seed*31),rows=pool(next,300),names=Array.from({length:fixture.bulkPatch.owners},(_,at)=>"o"+String(at).padStart(4,"0")),table=new Map<string,Row[]>(names.map(name=>[name,Array.from({length:Math.floor(next()*60)},()=>rows[Math.floor(next()*rows.length)]!)]));
  let single=createFactIndexV1(table as never),bulk=single;
  for(let step=0;step<fixture.bulkPatch.steps;step++){
   const upserts=new Map<string,Row[]>(),kind=next();
   for(const name of names)if(next()<0.35)upserts.set(name,kind<0.2?[]:Array.from({length:Math.floor(next()*80)},()=>rows[Math.floor(next()*rows.length)]!));
   if(next()<0.2)upserts.set("n"+step,Array.from({length:10},()=>rows[Math.floor(next()*rows.length)]!));
   single=patchFactIndexV1(single,{upserts: upserts as never},{bulkRows:Number.MAX_SAFE_INTEGER});
   bulk=patchFactIndexV1(bulk,{upserts: upserts as never},{bulkRows:fixture.bulkPatch.bulkRows});
   expect(consumers(bulk)).toEqual(consumers(single));
   expect(new Set(bulk.owners)).toEqual(new Set(single.owners));
  }
 }
});
