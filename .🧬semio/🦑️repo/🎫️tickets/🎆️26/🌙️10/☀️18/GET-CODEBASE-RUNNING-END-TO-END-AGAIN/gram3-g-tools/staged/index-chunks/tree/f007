/** 🧱️ Contract: the chunked construction declares its constants in its own schema and the module spells the same numbers; the runs it interns concatenate to each owner's row numbers in order, owners that share a long block hold the same runs, and the classes it returns are the coarsest exact partition of an independent model (rows grouped by their set of owners). */
import{expect,test}from"bun:test";
import schema from"../🧬️schema/🔣️.json";
import{INDEX_CHUNK_POLICY_V1,addOwnerRunsV1,classesOfChunksV1,createChunkInternerV1}from"../🟨️.mjs";

test("the policy of the module equals the schema constant",()=>{
 expect(INDEX_CHUNK_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof INDEX_CHUNK_POLICY_V1);
 expect(Object.isFrozen(INDEX_CHUNK_POLICY_V1)).toBe(true);
});

const draw=(seed:number)=>{let state=seed;return(limit:number)=>{state=(state*1103515245+12345)%2147483648;return(state>>8)%limit;};};

test("runs concatenate to each owner's list, shared blocks are interned once and classes equal the model partition",()=>{
 const random=draw(17),rowCount=3000,shared=Array.from({length:1500},(_,index)=>index),owners:number[][]=[Array.from({length:rowCount},(_,index)=>index)];
 for(let owner=1;owner<40;owner++){const own=new Set<number>(shared);for(let extra=0;extra<random(40);extra++)own.add(1500+random(1500));owners.push([...own].sort((a,b)=>a-b));}
 const interner=createChunkInternerV1();
 owners.forEach((list,owner)=>addOwnerRunsV1(interner,Uint32Array.from(list),list.length,owner));
 owners.forEach((list,owner)=>{
  const mine=interner.chunks.map((ids,chunk)=>[chunk,ids] as const).filter(([chunk])=>interner.holders[chunk]!.includes(owner));
  expect(mine.flatMap(([,ids])=>[...ids]).sort((a,b)=>a-b)).toEqual(list);
 });
 const held=interner.holders.reduce((sum,list)=>sum+list.length,0);
 expect(held).toBeLessThan(owners.reduce((sum,list)=>sum+list.length,0)/10);
 const classes=classesOfChunksV1(rowCount,interner),consumers=new Map<number,number[]>();
 owners.forEach((list,owner)=>list.forEach(row=>{(consumers.get(row)??consumers.set(row,[]).get(row)!).push(owner);}));
 const model=new Map<string,number[]>();
 for(const[row,list]of[...consumers].sort((a,b)=>a[0]-b[0])){const key=list.join(",");(model.get(key)??model.set(key,[]).get(key)!).push(row);}
 expect(classes.map(cls=>[cls.o.join(","),cls.r])).toEqual([...model].map(([key,rows])=>[key,rows]).sort((a,b)=>(a[1] as number[])[0]!-(b[1] as number[])[0]!));
});
