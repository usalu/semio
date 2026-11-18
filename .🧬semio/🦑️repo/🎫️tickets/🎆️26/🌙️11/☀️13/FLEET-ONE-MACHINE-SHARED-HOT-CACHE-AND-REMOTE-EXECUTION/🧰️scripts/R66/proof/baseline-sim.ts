// R66: the same simulated fleets on the live engine (old API) and on the new engine.
import{createHash}from"node:crypto";
const mode=process.argv[2]!,root=process.argv[3]!;
const OP=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
const pool=await import(OP);const P=pool.OWNER_POOL_POLICY_V1;
const sha=(t:string)=>createHash("sha256").update(t).digest("hex"),pause=(ms:number)=>new Promise<void>(d=>setTimeout(d,ms));
type U={name:string;kind:string;expectedMilliseconds:number;json:{}};
const unitsOf=(n:number):U[]=>Array.from({length:n},(_a,i)=>({name:`owner-${String(i).padStart(4,"0")}`,kind:"k",expectedMilliseconds:60000,json:{}}));
const claimOf=(table:Map<string,string>,c:{asked:number})=>async(unit:U):Promise<any>=>{c.asked++;await pause(1);const h=table.get(unit.name);if(h==="stored")return{state:"stored"};if(h==="held")return{state:"held",by:"x"};table.set(unit.name,"held");return{state:"granted",digest:sha(unit.name),why:()=>"",deterministic:()=>false,publish:async()=>{table.set(unit.name,"stored");return true},abandon:async()=>{table.delete(unit.name)}};};
const fast={...P,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
async function run(pullers:number,count:number,slotsOrCores:number,burn:number){
 const table=new Map<string,string>(),c={asked:0},units=unitsOf(count),began=Date.now();
 const stats=await Promise.all(Array.from({length:pullers},(_a,i)=>mode==="live"
  ?pool.pullOwnerPoolV1({units,policy:fast,cores:slotsOrCores,headroomBytes:()=>64*1024**3,peaks:{k:[10*1048576]},claim:claimOf(table,c),run:async()=>{await pause(burn);return true}})
  :pool.pullOwnerPoolV1({units:()=>units,puller:`m${i}:1`,policy:fast,slots:()=>slotsOrCores,claim:claimOf(table,c),run:async()=>{await pause(burn);return true}})));
 const published=stats.reduce((s:number,r:any)=>s+r.published,0),s=(Date.now()-began)/1000;
 return{pullers,count,slotsOrCores,seconds:+s.toFixed(2),published,attemptsPerSuccess:+(c.asked/Math.max(1,published)).toFixed(2),ownersPerMinute:Math.round(published/s*60)};
}
const out:any[]=[];
for(const slots of mode==="live"?[2,4,8,16]:[1,3,8,15])out.push({case:"one puller, slots/cores "+slots,...await run(1,96,slots,40)});
out.push({case:"ten pullers, 1500 owners",...await run(10,1500,mode==="live"?4:1,1)});
console.log(JSON.stringify(out,null,0).replace(/},{/g,"},\n{"));process.exit(0);
