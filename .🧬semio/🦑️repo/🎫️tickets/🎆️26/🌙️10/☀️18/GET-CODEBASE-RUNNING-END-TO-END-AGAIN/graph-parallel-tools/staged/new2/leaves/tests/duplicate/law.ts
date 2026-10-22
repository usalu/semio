import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{createFarmV1}from"../../../../../🏊️farm/🟨️.mjs";
import{createBuildGraphOptionalQuotaV1}from"../../../🟨️.mjs";
import{createOptionalAccountingV1}from"../../🟨️.mjs";

const entry=fileURLToPath(new URL("../../../../../🏊️farm/👷️worker/🟨️.mjs",import.meta.url)),service=new URL("./🧰️service/🟨️.mjs",import.meta.url).href,ROUNDS=12;

test("two workers that write the same content addressed block at the same instant leave exactly one intact leaf, no temporary file, and one account of it",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"duplicate-law-")),inference=join(directory,"inference");
 mkdirSync(join(inference,"np"),{recursive:true});
 const farm=createFarmV1({entry,services:{duplicate:service},workers:2});
 farm.open("duplicate",{roots:{inference}},{accept:(_kind:string,identity:unknown)=>identity==="duplicate-law"});
 try{
  farm.prestart();await new Promise(done=>setTimeout(done,1500));
  const quota=createBuildGraphOptionalQuotaV1({maximumBytes:2**30,usedBytes:0,availableBytes:2**40,minimumAvailableBytes:1}),removed:string[]=[],accounting=createOptionalAccountingV1({ledger:{publish:(_plan:any,write:()=>void)=>{write();return true;}},quota});
  let duplicated=0,expected=0;
  for(let round=0;round<ROUNDS;round++){
   const text="block "+round+" ".repeat(round*7),digest=createHash("sha256").update(text).digest("hex"),startAt=Date.now()+300,job={startAt,blocks:[{digest,text}]};
   const replies=await Promise.all([farm.infer("duplicate",job),farm.infer("duplicate",job)]);
   for(const outcome of replies)expect(outcome&&"reply"in outcome,JSON.stringify(outcome)).toBe(true);
   const leaves=replies.flatMap(outcome=>(outcome as {reply:{leaves:{name:string;bytes:number}[]}}).reply.leaves);
   expect(leaves.length).toBeGreaterThanOrEqual(1);expect(leaves.length).toBeLessThanOrEqual(2);
   if(leaves.length===2)duplicated++;
   for(const leaf of leaves)expect(leaf.name).toBe("np/ob/"+digest+".br");
   const target=join(inference,"np","ob",digest+".br");
   expect(readFileSync(target,"utf8")).toBe(text);
   expect(readdirSync(join(inference,"np","ob")).filter(name=>name.endsWith(".tmp"))).toEqual([]);
   const outcome=accounting.adopt(leaves,name=>{removed.push(name);rmSync(join(inference,name),{force:true});});
   expect(outcome.accepted.length).toBe(1);expect(removed).toEqual([]);expect(existsSync(target)).toBe(true);
   expected+=statSync(target).size;
  }
  expect(quota.usedBytes).toBe(expected);
  expect(accounting.facts().duplicates).toBe(duplicated);
 }finally{await farm.close();rmSync(directory,{recursive:true,force:true});}
},120000);

test("a block the quota refuses is removed once, never again for a second report of it, and an accepted block is never removed because of one",()=>{
 const quota=createBuildGraphOptionalQuotaV1({maximumBytes:100,usedBytes:0,availableBytes:2**40,minimumAvailableBytes:1}),removed:string[]=[],accounting=createOptionalAccountingV1({ledger:{publish:(_plan:any,write:()=>void)=>{write();return true;}},quota});
 const small="np/ob/"+"1".repeat(64)+".br",big="np/ob/"+"2".repeat(64)+".br";
 expect(accounting.adopt([{name:small,bytes:60},{name:big,bytes:60}],name=>removed.push(name))).toEqual({accepted:[small],refused:[big]});
 expect(accounting.adopt([{name:big,bytes:60},{name:small,bytes:60}],name=>removed.push(name))).toEqual({accepted:[],refused:[]});
 expect(removed).toEqual([big]);expect(quota.usedBytes).toBe(60);expect(accounting.facts().duplicates).toBe(2);
});
