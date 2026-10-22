import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {expect,test} from "bun:test";
import {planCacheCollectionV1,type CacheTargetV1} from "../../🟦️.ts";
import fixtures from "../🧫️fixtures/🔣️.json" with {type:"json"};

const seeds=[1,2,3,5,8,13,21,34,55,89,144,233],casesPerSeed=20;
function generator(seed:number):()=>number{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};}

function corpus():CacheTargetV1[]{
 const now=fixtures.now,out:CacheTargetV1[]=[];
 for(const seed of seeds){
  const random=generator(seed);
  for(let index=0;index<casesPerSeed;index++){
   const count=Math.floor(random()*14),kind=(["area","retired","undeclared"] as const)[Math.floor(random()*3)]!,units=Array.from({length:count},(_,position)=>({name:"u"+String(position).padStart(2,"0")+(random()<0.2?"/deep":""),bytes:Math.floor(random()*1000),recencyMs:random()<0.15?now-Math.floor(random()*7200000):now-Math.floor(random()*40*86400000),...(random()<0.12?{held:"pid:"+(position+1)}:{})}));
   out.push({id:"s"+seed+"-"+index,kind,budgetBytes:random()<0.4?null:Math.floor(random()*4000),unusedAgeMs:kind==="retired"?0:random()<0.4?null:Math.floor(random()*30)*86400000,quietMs:Math.floor(random()*3)*3600000,...(random()<0.08?{heldBy:"lease"}:{}),units});
  }
 }
 return out;
}

function oracle(target:CacheTargetV1,now:number):{removals:string[][];kept:string[][];retained:number}{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE unit(name TEXT PRIMARY KEY, bytes INTEGER, recency INTEGER, held INTEGER)");
  const insert=database.prepare("INSERT INTO unit VALUES(?,?,?,?)");
  for(const unit of target.units)insert.run(unit.name,unit.bytes,unit.recencyMs,unit.held!==undefined||target.heldBy!==undefined?1:0);
  const base=target.kind==="retired"?"retired":target.kind==="undeclared"?"undeclared":"age";
  database.exec("CREATE TABLE blocked AS SELECT name, CASE WHEN held=1 THEN 'held' WHEN "+now+"-recency<"+target.quietMs+" THEN 'quiet' END AS reason FROM unit");
  const ageSql=target.unusedAgeMs===null?"SELECT name FROM unit WHERE 0":"SELECT unit.name FROM unit JOIN blocked ON blocked.name=unit.name WHERE "+now+"-recency>="+target.unusedAgeMs+" AND blocked.reason IS NULL";
  database.exec("CREATE TABLE aged AS "+ageSql);
  const removals:string[][]=[];
  for(const row of database.query("SELECT name FROM aged ORDER BY name").all() as {name:string}[])removals.push([row.name,base]);
  const total=(database.query("SELECT COALESCE(SUM(bytes),0) AS total FROM unit").get() as {total:number}).total,agedBytes=(database.query("SELECT COALESCE(SUM(bytes),0) AS total FROM unit WHERE name IN (SELECT name FROM aged)").get() as {total:number}).total;
  let remaining=total-agedBytes;
  if(target.budgetBytes!==null&&remaining>target.budgetBytes){
   const evicted=database.query(
    "WITH rest AS (SELECT unit.name, bytes, recency, blocked.reason AS reason FROM unit JOIN blocked ON blocked.name=unit.name WHERE unit.name NOT IN (SELECT name FROM aged)), "+
    "run AS (SELECT name, bytes, recency, reason, COALESCE(SUM(CASE WHEN reason IS NULL THEN bytes ELSE 0 END) OVER (ORDER BY recency, name ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING),0) AS before FROM rest) "+
    "SELECT name, bytes FROM run WHERE reason IS NULL AND "+remaining+"-before>"+target.budgetBytes+" ORDER BY recency, name"
   ).all() as {name:string;bytes:number}[];
   for(const row of evicted){removals.push([row.name,"budget"]);remaining-=row.bytes;}
  }
  const removed=new Set(removals.map(row=>row[0]!)),kept:string[][]=[];
  const wanted=new Set<string>();
  if(target.unusedAgeMs!==null)for(const row of database.query("SELECT name FROM unit WHERE "+now+"-recency>="+target.unusedAgeMs).all() as {name:string}[])wanted.add(row.name);
  if(target.budgetBytes!==null){
   const beforeBudget=total-agedBytes;
   if(beforeBudget>target.budgetBytes){
    let left=beforeBudget;
    for(const row of database.query("SELECT unit.name AS name, bytes, blocked.reason AS reason FROM unit JOIN blocked ON blocked.name=unit.name WHERE unit.name NOT IN (SELECT name FROM aged) ORDER BY recency, unit.name").all() as {name:string;bytes:number;reason:string|null}[]){
     if(left<=target.budgetBytes)break;
     if(row.reason)wanted.add(row.name);else left-=row.bytes;
    }
   }
  }
  for(const row of database.query("SELECT name, reason FROM blocked WHERE reason IS NOT NULL ORDER BY name").all() as {name:string;reason:string}[])if(wanted.has(row.name)&&!removed.has(row.name))kept.push([row.name,row.reason]);
  return{removals,kept,retained:remaining};
 }finally{database.close();}
}

test("a seeded corpus of plans equals an independent SQLite implementation of the same rules",()=>{
 const targets=corpus(),now=fixtures.now;
 expect(targets.length).toBe(seeds.length*casesPerSeed);
 for(const target of targets){
  const[plan]=planCacheCollectionV1([target],now),expected=oracle(target,now);
  const names=(rows:readonly {name:string}[])=>rows.map(row=>row.name);
  const actualRemovals=plan!.removals.map(row=>[row.name,row.reason]);
  const orderOf=(rows:string[][])=>[...rows.filter(row=>row[1]!=="budget").sort((left,right)=>left[0]!<right[0]!?-1:1),...rows.filter(row=>row[1]==="budget")];
  expect(orderOf(actualRemovals),target.id).toEqual(orderOf(expected.removals));
  expect(plan!.retainedBytes,target.id).toBe(expected.retained);
  expect([...plan!.kept.map(row=>[row.name,row.reason])].sort(),target.id).toEqual([...expected.kept].sort());
  for(const row of plan!.removals){const unit=target.units.find(candidate=>candidate.name===row.name)!;expect(unit.held,target.id).toBeUndefined();expect(target.heldBy,target.id).toBeUndefined();expect(now-unit.recencyMs>=target.quietMs,target.id).toBe(true);}
  expect(names(plan!.removals).length).toBe(new Set(names(plan!.removals)).size);
  expect(plan!.removals.reduce((sum,row)=>sum+row.bytes,0)+plan!.retainedBytes).toBe(plan!.totalBytes);
 }
});

test("planning twice over what remains finds nothing more to remove",()=>{
 const now=fixtures.now;
 for(const target of corpus()){
  const[first]=planCacheCollectionV1([target],now),gone=new Set(first!.removals.map(row=>row.name)),[second]=planCacheCollectionV1([{...target,units:target.units.filter(unit=>!gone.has(unit.name))}],now);
  expect(second!.removals.filter(row=>row.reason!=="budget").map(row=>row.name),target.id).toEqual([]);
 }
});

test("the corpus is reproducible and pinned by its digest",()=>{
 const digest=createHash("sha256").update(JSON.stringify(corpus())).digest("hex");
 expect(digest).toBe((fixtures as unknown as {corpusSha256:string}).corpusSha256);
});
