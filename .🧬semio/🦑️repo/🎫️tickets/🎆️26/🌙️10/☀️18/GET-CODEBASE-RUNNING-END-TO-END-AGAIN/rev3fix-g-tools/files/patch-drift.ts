/** 🌊️ Contract: a long run of single-owner patches never lets the class lists of the fact index hold more row ids than the declared multiple of its rows. The re-partition trigger looks at the index it has just patched (the duplication of its class lists), not at the previous generation, so slow drift compacts as surely as one big patch; the consumers of every owner equal a naive owner-to-rows table before and after. The independent oracle is that table and the sum of the class lists of the coarsest partition (one class per row, so exactly the number of rows). */
import {expect,test} from "bun:test";
import {GENERATION_POLICY_V1,canonicalJsonV1,createFactIndexV1,patchFactIndexV1,refineFactIndexV1,rowKeyV1} from "../../🟨️.mjs";

type Row={kind:string;path:string;fact:{sha256:string}};
const row=(path:string,version:string):Row=>({kind:"file",path,fact:{sha256:version.padStart(64,"0")}});
const seeded=(seed:number)=>()=>(seed=(Math.imul(seed,1664525)+1013904223)>>>0)/4294967296;
const ids=(index:{classes:{r:number[]}[]})=>index.classes.reduce((sum,cls)=>sum+cls.r.length,0);
const naive=(table:Map<string,Row[]>)=>new Map([...table].map(([owner,rows])=>[owner,new Set(rows.map(item=>canonicalJsonV1([item.kind,item.path,item.fact])))]));
const consumers=(index:ReturnType<typeof createFactIndexV1>)=>{
 const out=new Map<string,Set<string>>();
 for(const cls of index.classes)for(const id of cls.o){const set=out.get(index.owners[id]!)??new Set<string>();for(const r of cls.r)set.add(rowKeyV1(index.rows[r]!));out.set(index.owners[id]!,set);}
 return out;
};

test("the duplication bound is declared and above one, so a coarsest partition never trips it",()=>{
 expect(GENERATION_POLICY_V1.compactionDuplication).toBeGreaterThan(1);
});

test("200 generations that each upsert one owner keep the row ids of the class lists within the declared multiple of the rows",()=>{
 const next=seeded(7),OWNERS=150,BLOCKS=600,PER_BLOCK=5,PRIVATE=20,GENERATIONS=200;
 const name=(owner:number)=>"o"+String(owner).padStart(3,"0");
 const table=new Map<string,Row[]>(Array.from({length:OWNERS},(_,owner)=>[name(owner),[] as Row[]]));
 for(let block=0;block<BLOCKS;block++){
  const shared=Array.from({length:PER_BLOCK},(_,at)=>row("/w/b"+block+"/"+at,"b"+block)),holders=new Set<number>();
  while(holders.size<3)holders.add(Math.floor(next()*OWNERS));
  for(const owner of holders)table.get(name(owner))!.push(...shared);
 }
 const own=(owner:number,version:string)=>Array.from({length:PRIVATE},(_,at)=>row("/w/o"+owner+"/"+at,version+"_"+at));
 for(let owner=0;owner<OWNERS;owner++)table.get(name(owner))!.push(...own(owner,"0"));
 let index=createFactIndexV1(table as never),compactions=0;
 expect(ids(index)).toBe(index.rows.length);
 const limit=(patched:{rows:unknown[]})=>Math.max(GENERATION_POLICY_V1.compactionFloor,GENERATION_POLICY_V1.compactionDuplication*patched.rows.length);
 for(let generation=1;generation<=GENERATIONS;generation++){
  const owner=Math.floor(next()*OWNERS),shared=table.get(name(owner))!.filter(item=>!item.path.startsWith("/w/o")),rows=[...shared,...own(owner,String(generation))];
  table.set(name(owner),rows);
  index=patchFactIndexV1(index,{upserts:new Map([[name(owner),rows]])}) as never;
  if(ids(index)===index.rows.length)compactions++;
  expect(ids(index),"generation "+generation).toBeLessThanOrEqual(limit(index));
 }
 expect(compactions).toBeGreaterThan(0);
 expect(consumers(index)).toEqual(naive(table));
 const coarsest=refineFactIndexV1(index);
 expect(ids(coarsest)).toBe(coarsest.rows.length);
 expect(consumers(coarsest)).toEqual(naive(table));
},120000);
