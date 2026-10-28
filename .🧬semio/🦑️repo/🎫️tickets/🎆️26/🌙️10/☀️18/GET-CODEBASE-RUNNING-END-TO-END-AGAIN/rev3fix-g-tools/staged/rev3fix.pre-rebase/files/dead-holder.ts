/** 🪦️ Contract: a holder killed between its body and its own retention (SIGKILL, out of memory, watchdog) leaves its receipts in the lane, and the pool trimmer that removes that idle lane publishes them into the shared ledger the holder recorded first, so a finished build never loses its provenance to the trimmer. A lane without a record retains nothing, a conflicting name is reported (never replaced) and nothing blocks the removal. The oracle is the shared ledger on disk after a real process was killed. */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {acquirePooledLaneV1,readPoolV1,trimPoolV1} from "../../../🏊️pool/🟦️.ts";
import {retainLaneOutputsV1,withLaneRetentionV1} from "../../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.retention,layout=UNIT_CACHE_POLICY_V1.layout,here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"📥️worker/🟦️.ts"),R="cargo-unit-provenance-";
const scratch=()=>realpathSync(mkdtempSync(join(tmpdir(),"dead-holder-"))),everything={maximumLanes:0,maximumLiveLanes:32,quotaBytes:0};
async function killedHolder(root:string,shared:string):Promise<string>{
 const child=Bun.spawn([process.execPath,worker,root,shared],{stdout:"pipe",stderr:"inherit"}),id=(await new Response(child.stdout).text()).trim();
 expect(await child.exited,"the holder dies of SIGKILL").not.toBe(0);expect(id).toMatch(/^[a-z0-9]{1,12}$/u);
 const claim=join(root,"u",layout.locks,"lane-"+id+".lock"),past=new Date(Date.now()-UNIT_CACHE_POLICY_V1.lock.staleMilliseconds*3);utimesSync(claim,past,past);
 return id;
}

test("the receipts a killed holder left in its lane are published by the trimmer that removes the lane",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"shared");
 try{
  const id=await killedHolder(root,shared);
  expect(existsSync(join(shared,rules.ledger)),"the holder never reached its retention").toBe(false);
  expect(readPoolV1(cache,Date.now()).map(lane=>[lane.id,lane.claimed])).toEqual([[id,false]]);
  const trimmed=trimPoolV1(cache,everything,Date.now());
  expect(trimmed.removed).toBe(1);expect(readPoolV1(cache,Date.now())).toEqual([]);
  expect(readFileSync(join(shared,rules.ledger,R+"killed.json"),"utf8")).toBe('{"killed":true}');
  expect(readdirSync(join(shared,rules.ledger)).filter(name=>name.startsWith(".")||name.endsWith(".tmp"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("a name that the shared ledger holds with other bytes is reported by the trimmer and never replaced, and the lane still goes",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"shared");
 try{
  mkdirSync(join(shared,rules.ledger),{recursive:true});writeFileSync(join(shared,rules.ledger,R+"killed.json"),"first writer");
  const id=await killedHolder(root,shared),reports:[string,Record<string,string|number>][]=[];
  expect(trimPoolV1(cache,everything,Date.now(),(kind,parameters)=>reports.push([kind,parameters])).removed).toBe(1);
  expect(readFileSync(join(shared,rules.ledger,R+"killed.json"),"utf8")).toBe("first writer");
  expect(reports).toEqual([["retentionConflict",{lane:id,count:1,names:R+"killed.json"}]]);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("a lane no holder ever recorded a ledger for retains nothing and is removed",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"shared");
 try{
  const lane=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  mkdirSync(join(lane.paths.build,rules.ledger),{recursive:true});writeFileSync(join(lane.paths.build,rules.ledger,R+"x.json"),"x");lane.release();
  expect(trimPoolV1(cache,everything,Date.now()).removed).toBe(1);
  expect(existsSync(shared)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a holder records its shared ledger beside the lane before its body runs, and a record that is no absolute path is ignored",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"shared");
 try{
  const lane=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  await withLaneRetentionV1({lane,shared,language:"en"},async()=>{expect(readFileSync(join(lane.paths.base,layout.destination),"utf8")).toBe(shared);});
  const again=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  writeFileSync(join(again.paths.base,layout.destination),"relative/shared");mkdirSync(join(again.paths.build,rules.ledger),{recursive:true});writeFileSync(join(again.paths.build,rules.ledger,R+"y.json"),"y");again.release();
  expect(trimPoolV1(cache,everything,Date.now()).removed).toBe(1);expect(existsSync(join(root,"relative"))).toBe(false);expect(existsSync(join(shared,rules.ledger,R+"y.json"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the retention is a synchronous step under the claim of the trimmer: its result is a value, not a promise",()=>{
 const root=scratch();
 try{
  const lane=join(root,"lane"),ledger=join(lane,rules.ledger);mkdirSync(ledger,{recursive:true});writeFileSync(join(ledger,R+"a.json"),"a");
  const summary=retainLaneOutputsV1({lane,shared:join(root,"shared"),laneId:"ab12",language:"en"});
  expect(typeof (summary as unknown as {then?:unknown}).then).toBe("undefined");expect(summary).toMatchObject({published:1,conflicts:[]});
 }finally{rmSync(root,{recursive:true,force:true});}
});
