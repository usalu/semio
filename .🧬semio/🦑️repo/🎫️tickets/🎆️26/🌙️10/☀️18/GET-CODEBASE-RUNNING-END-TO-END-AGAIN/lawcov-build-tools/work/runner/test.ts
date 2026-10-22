/** 🧺️ Contract: the cache runner measures trees with bounded, link-safe walks, names what holds a tree, removes without following links, finds undeclared entries and plans a collection exactly like an independent Python oracle and the language-agnostic corpus in `./🔣️.json`, closed by `./🧬️schema/🔣️.json`. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {linkSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {cacheTextV1,formatCacheBytesV1,readCacheLedgerV1} from "@LIB/⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cachePidAliveV1,cacheRunnerPortsV1,cacheSqliteFreeV1,collectCacheV1,measureCacheTreeV1,probeCacheHeldV1,removeCacheTreeV1,undeclaredEntriesV1,type CacheRunnerPortsV1} from "@UNIT/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

type Entry={kind:"file"|"dir"|"symlink"|"hardlink";path:string;size?:number;ageHours?:number;to?:string;of?:string};
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE"),HOUR=3600000,now=Date.now(),ledger=readCacheLedgerV1(corpus.ledger);
let workspace="";
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const stamp=(path:string,ageHours:number):void=>{const time=(now-ageHours*HOUR)/1000;utimesSync(path,time,time);};
const rel=(base:string,path:string):string=>relative(base,path).split("\\").join("/");
function build(locate:(path:string)=>string,entries:readonly Entry[],outside:string):void{
 const later:[string,number][]=[];
 for(const entry of entries){
  const full=locate(entry.path);
  if(entry.kind==="file"){mkdirSync(dirname(full),{recursive:true});writeFileSync(full,"x".repeat(entry.size!));stamp(full,entry.ageHours!);}
  else if(entry.kind==="dir"){mkdirSync(full,{recursive:true});if(entry.ageHours!==undefined)later.push([full,entry.ageHours]);}
  else if(entry.kind==="symlink"){mkdirSync(dirname(full),{recursive:true});symlinkSync(join(outside,...entry.to!.slice("@outside".length).split("/").filter(Boolean)),full);}
  else{mkdirSync(dirname(full),{recursive:true});linkSync(locate(entry.of!),full);}
 }
 for(const[path,age]of later)stamp(path,age);
}
const within=(base:string)=>(path:string):string=>path===""?base:join(base,...path.split("/"));
const spaces=(name:string):string=>{const path=join(workspace,name);mkdirSync(path,{recursive:true});return path;};
const listing=(base:string):string[]=>readdirSync(base,{recursive:true,encoding:"utf8"}).map(path=>path.split("\\").join("/")).sort();
function ports(extra:Partial<CacheRunnerPortsV1>={}):CacheRunnerPortsV1{return cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>undefined,exclusive:async()=>({release:()=>undefined}),...extra});}
const lines=():{write(line:string):void;language:"en";out:string[]}=>{const out:string[]=[];return{write:line=>{out.push(line);},language:"en",out};};

beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"cache-runner-")));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the corpus is closed by its schema and its ledger is a valid cache ledger",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,measure:[{...corpus.measure[0],limit:0}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,remove:[{...corpus.remove[0],target:"../x"}]})).toBe(false);
 expect(ledger.areas.length).toBeGreaterThan(0);
});

test("measuring agrees with the corpus and with the Python walk: allocated bytes, hardlinks once, links never, sqlite files, queues, recency and the entry limit",async()=>{
 const rows=corpus.measure.map((row,index)=>{
  const base=spaces("m"+index),tree=within(join(base,"t")),outside=join(base,"outside");mkdirSync(outside,{recursive:true});
  build(within(outside),(row.outside??[]) as Entry[],outside);build(tree,row.entries as Entry[],outside);
  return{row,base,path:row.missing?join(base,"nothing"):"singleFile" in row?join(base,"t",row.singleFile!):join(base,"t"),limit:row.limit};
 });
 const answers=ask({op:"measure",rows:rows.map(item=>({path:item.path,limit:item.limit}))});
 for(const[index,item]of rows.entries()){
  const {row}=item,expected=row.expected,oracle=answers[index],measure=await measureCacheTreeV1(item.path,ports(),item.limit),root=item.path,label=row.name;
  expect(measure.present,label).toBe(expected.present);expect(oracle.present,label).toBe(expected.present);
  expect(measure.entries,label).toBe(expected.entries);expect(oracle.entries,label).toBe(expected.entries);
  expect(measure.partial,label).toBe(expected.partial);expect(oracle.partial,label).toBe(expected.partial);
  expect([...measure.sqlite].map(path=>path===root?rel(dirname(root),path):rel(root,path)).sort(),label).toEqual(expected.sqlite);
  expect([...measure.queues].map(path=>rel(root,path)).sort(),label).toEqual(expected.queues);
  expect(oracle.sqlite.map((path:string)=>path===root?rel(dirname(root),path):rel(root,path)).sort(),label).toEqual(expected.sqlite);
  expect(oracle.queues.map((path:string)=>rel(root,path)).sort(),label).toEqual(expected.queues);
  if(expected.countedGroups===null)continue;
  expect(measure.bytes,label).toBe(oracle.bytes);
  const counted=oracle.counted.map((path:string)=>path===root?rel(dirname(root),path):rel(root,path)) as string[];
  expect(counted.length,label).toBe(expected.countedGroups.length);
  for(const group of expected.countedGroups)expect(group.filter(name=>counted.includes(name)).length,label+" "+group.join("|")).toBe(1);
  expect(measure.bytes,label).toBe(oracle.counted.reduce((sum:number,path:string)=>sum+(lstatSync(path).blocks*512||lstatSync(path).size),0));
  if(expected.recencyAgeHours===null)expect(Math.abs(measure.recencyMs-oracle.recencyMs),label).toBeLessThan(1);
  else{
   expect(Math.abs(measure.recencyMs-(now-expected.recencyAgeHours*HOUR)),label).toBeLessThan(2000);
   expect(Math.abs(oracle.recencyMs-(now-expected.recencyAgeHours*HOUR)),label).toBeLessThan(2000);
  }
 }
});

test("a walk of any size reports progress at most once per interval and stops at once when cancelled",async()=>{
 const base=spaces("progress");for(let index=0;index<400;index++)writeFileSync(join(base,"f"+index),"x");
 let clock=0;const stages:string[]=[];
 await measureCacheTreeV1(base,ports({now:()=>clock+=1,progress:async stage=>{stages.push(stage);}}),1000,"walk");
 expect(stages.length).toBe(1);expect(stages[0]).toBe("walk");
 const controller=new AbortController(),aborted=ports({signal:controller.signal,progress:async()=>{controller.abort(Error("stop now"));}});
 await expect(measureCacheTreeV1(base,aborted,1000,"walk")).rejects.toThrow("stop now");
 const before=new AbortController();before.abort(Error("never started"));
 await expect(measureCacheTreeV1(base,ports({signal:before.signal}),1000)).rejects.toThrow("never started");
});

test("what holds a tree matches the corpus and the Python reading of the ticket rule",async()=>{
 const rows=corpus.held.map((row,index)=>{const root=spaces("h"+index);build(within(root),row.entries as Entry[],root);return{row,root};});
 const answers=ask({op:"held",rows:rows.map(item=>({root:item.root,alive:item.row.alive,heldSqlite:item.row.heldSqlite}))});
 for(const[index,{row,root}]of rows.entries()){
  const probe=ports({alive:pid=>row.alive.includes(pid),sqliteFree:async file=>!row.heldSqlite.includes(rel(root,file))}),measure=await measureCacheTreeV1(root,probe,100000),held=await probeCacheHeldV1(measure,probe);
  const named=held===undefined?null:held.startsWith("sqlite:")?"sqlite:"+rel(root,held.slice(7)):held;
  expect(named,row.name).toBe(row.expected);expect(answers[index],row.name).toBe(row.expected);
 }
});

test("a SQLite lock held by another engine is seen by the exclusive probe and released locks free the file",async()=>{
 for(const mode of["exclusive","shared"]){
  const file=join(spaces("lock-"+mode),"lease.sqlite"),holder=spawn(python,["-I",oracleFile,"lock",file,mode],{stdio:["pipe","pipe","inherit"]});
  try{
   await new Promise<void>((accept,reject)=>{holder.stdout.once("data",chunk=>String(chunk).startsWith("READY")?accept():reject(Error("no READY")));holder.once("error",reject);});
   expect(await cacheSqliteFreeV1(file),mode).toBe(false);
  }finally{holder.stdin.end();await new Promise(accept=>holder.once("exit",accept));}
  expect(await cacheSqliteFreeV1(file),mode).toBe(true);
 }
 expect(await cacheSqliteFreeV1(join(spaces("lock-none"),"absent.sqlite"))).toBe(true);
 const junk=join(spaces("lock-junk"),"junk.sqlite");writeFileSync(junk,"this is not a database");expect(await cacheSqliteFreeV1(junk)).toBe(true);
});

test("process liveness agrees with the operating system through Python",()=>{
 const dead=spawnSync(process.execPath,["-e","0"]).pid!,pids=[process.pid,process.ppid,dead];
 const answers=ask({op:"alive",pids});
 expect(pids.map(cachePidAliveV1)).toEqual(answers);expect(answers.slice(0,2)).toEqual([true,true]);expect(answers[2]).toBe(false);
});

test("the default ports are complete, frozen and overridable",()=>{
 const base=cacheRunnerPortsV1({signal:new AbortController().signal,progress:async()=>undefined,exclusive:async()=>undefined}),custom=cacheRunnerPortsV1({signal:base.signal,progress:base.progress,exclusive:base.exclusive,now:()=>7,alive:()=>true});
 expect(Object.isFrozen(base)).toBe(true);expect(typeof base.now()).toBe("number");expect(base.alive).toBe(cachePidAliveV1);expect(base.sqliteFree).toBe(cacheSqliteFreeV1);
 expect(custom.now()).toBe(7);expect(custom.alive(1)).toBe(true);
});

test("removal never follows a link and leaves everything outside the tree exactly as the Python digest saw it",async()=>{
 const cases=corpus.remove.map((row,index)=>{
  const base=spaces("r"+index),outside=join(base,"outside");mkdirSync(outside,{recursive:true});
  build(within(outside),(row.outside??[]) as Entry[],outside);if(!row.missing)build(within(join(base,"t")),row.entries as Entry[],outside);
  return{row,outside,target:join(base,...row.target.split("/"))};
 });
 const before=ask({op:"digest",paths:cases.map(item=>item.outside)});
 for(const item of cases){await removeCacheTreeV1(item.target,ports(),"remove");expect(()=>lstatSync(item.target),item.row.name).toThrow();}
 const after=ask({op:"digest",paths:cases.map(item=>item.outside)});
 for(const[index,item]of cases.entries()){
  expect(after[index],item.row.name).toEqual(before[index]);
  for(const kept of item.row.outside??[])if(kept.kind==="file")expect(readFileSync(join(item.outside,...kept.path.split("/")),"utf8").length,item.row.name).toBe(kept.size!);
 }
});

test("removal is cancellable between batches and keeps what it did not reach",async()=>{
 const base=spaces("cancel");for(let index=0;index<200;index++)writeFileSync(join(base,"f"+index),"x");
 let clock=0;const controller=new AbortController();let calls=0;
 const cancelling=ports({signal:controller.signal,now:()=>clock+=1000,progress:async()=>{if(++calls===3)controller.abort(Error("stop removal"));}});
 await expect(removeCacheTreeV1(base,cancelling,"remove")).rejects.toThrow("stop removal");
 const left=readdirSync(base).length;expect(left).toBeGreaterThan(0);expect(left).toBeLessThan(200);
});

test("undeclared entries match the corpus and the Python classification, descending only through containers, and a file where a container is expected is refused in both languages",async()=>{
 const built=corpus.undeclared.map((row,index)=>{
  const base=spaces("u"+index),repository=join(base,"r","cache"),native=join(base,".cache");
  build(path=>path.startsWith("n:")?within(native)(path.slice(2)):within(repository)(path),row.entries as Entry[],base);return{row,base};
 });
 for(const{row,base}of built){
  const oracle=ask({op:"undeclared",workspace:base,ledger:corpus.ledger}),failure=(row as {error?:{root:string;path:string}}).error;
  if(failure===undefined){
   expect(await undeclaredEntriesV1(base,ledger,"repository"),row.name).toEqual(row.expected.repository);
   expect(await undeclaredEntriesV1(base,ledger,"native"),row.name).toEqual(row.expected.native);
   expect(oracle,row.name).toEqual(row.expected);continue;
  }
  const path=failure.root+":"+failure.path,message=cacheTextV1("en","error.container",{path})+"\n"+cacheTextV1("de","error.container",{path});
  await expect(undeclaredEntriesV1(base,ledger,failure.root as "repository"),row.name).rejects.toThrow(message);
  expect(oracle[failure.root],row.name).toEqual({error:path});expect(message.split("\n").length,row.name).toBe(2);expect(message,row.name).toContain(path);
  const before=listing(base);await expect(collectCacheV1(base,ledger,{dryRun:false},ports(),lines()),row.name).rejects.toThrow(message);
  expect(listing(base),row.name+" the refusal removes nothing").toEqual(before);
 }
});

test("a collection plans and removes exactly what the corpus and the Python planner say, and a dry run touches nothing",async()=>{
 for(const[index,row]of corpus.collect.entries()){
  const base=spaces("c"+index),repository=join(base,"r","cache"),native=join(base,".cache");
  build(path=>path.startsWith("n:")?within(native)(path.slice(2)):within(repository)(path),row.entries as Entry[],base);
  const sqliteFree=async(file:string)=>!row.heldSqlite.includes(rel(base,file)),probe=ports({alive:pid=>row.alive.includes(pid),sqliteFree}),request={op:"collect",workspace:base,ledger:corpus.ledger,now,alive:row.alive,heldSqlite:row.heldSqlite};
  const summarize=(result:Awaited<ReturnType<typeof collectCacheV1>>)=>({removed:Object.fromEntries(result.removed.map(item=>[item.target+"/"+item.name,item.reason])),kept:Object.fromEntries(result.plans.flatMap(plan=>plan.kept).map(item=>[item.target+"/"+item.name,item.reason]))});
  const everything=listing(base),plan=ask({...request,dryRun:true}),applied=ask({...request,dryRun:false}),out=lines();
  const dry=await collectCacheV1(base,ledger,{dryRun:true},probe,out);
  expect(summarize(dry),row.name).toEqual({removed:row.removed,kept:row.kept});expect({removed:plan.removed,kept:plan.kept},row.name).toEqual({removed:row.removed,kept:row.kept});
  expect(listing(base),row.name).toEqual(everything);expect(plan.remaining,row.name).toEqual(everything);
  expect(dry.freedBytes,row.name).toBe(dry.plans.flatMap(item=>item.removals).reduce((sum,item)=>sum+item.bytes,0));
  expect(out.out.at(-1),row.name).toBe(cacheTextV1("en","gc.summary.plan",{count:dry.removed.length,bytes:formatCacheBytesV1(dry.freedBytes),kept:dry.kept}));
  for(const select of row.only){
   const part=await collectCacheV1(base,ledger,{dryRun:true,only:new Set(select.only)},probe,lines());
   expect(summarize(part),row.name+" only "+select.only.join()).toEqual({removed:select.removed,kept:select.kept});
  }
  const done=await collectCacheV1(base,ledger,{dryRun:false},probe,lines());
  expect(summarize(done),row.name).toEqual({removed:row.removed,kept:row.kept});expect(done.failed,row.name).toEqual([]);
  expect(listing(base),row.name).toEqual(applied.remaining);
  for(const path of row.absentAfterApply)expect(()=>statSync(join(base,...path.split("/"))),row.name+" "+path).toThrow();
  for(const path of row.presentAfterApply)expect(()=>statSync(join(base,...path.split("/"))),row.name+" "+path).not.toThrow();
  const again=await collectCacheV1(base,ledger,{dryRun:false},probe,lines());
  expect(again.removed,row.name+" a second run finds nothing left to do").toEqual([]);
 }
});

test("a held lease keeps a whole area, a granted lease is released and a cancelled collection stops between units",async()=>{
 const leased=readCacheLedgerV1({...corpus.ledger,areas:[...corpus.ledger.areas.filter(area=>area.id!=="t1"),{id:"leased",root:"repository",kind:"tree",directory:"leased",budget:{bytes:100000},collect:{unusedAgeMs:3600000,quietMs:0,custody:"lease",lease:"L:leased"},writers:["w"],purpose:{en:"p",de:"q"}}]});
 const base=spaces("lease"),repository=join(base,"r","cache");build(within(repository),[{kind:"file",path:"leased/a/f",size:10,ageHours:48},{kind:"file",path:"leased/b/f",size:10,ageHours:48}],base);
 const asked:string[]=[],released:string[]=[];
 const refusing=ports({exclusive:async resource=>{asked.push(resource);return undefined;}}),first=await collectCacheV1(base,leased,{dryRun:false,only:new Set(["leased"])},refusing,lines());
 expect(first.removed).toEqual([]);expect(first.plans[0]!.kept.length).toBe(0);expect(asked).toEqual(["L:leased"]);expect(readdirSync(join(repository,"leased")).sort()).toEqual(["a","b"]);
 const granting=ports({exclusive:async resource=>({release:()=>{released.push(resource);}})}),second=await collectCacheV1(base,leased,{dryRun:false,only:new Set(["leased"])},granting,lines());
 expect(second.removed.map(item=>item.name).sort()).toEqual(["a","b"]);expect(released).toEqual(["L:leased"]);
 build(within(repository),[{kind:"file",path:"leased/c/f",size:10,ageHours:48},{kind:"file",path:"leased/d/f",size:10,ageHours:48}],base);
 const controller=new AbortController();let seen=0,clock=0;
 const stopping=ports({signal:controller.signal,now:()=>now+(clock+=1000),progress:async stage=>{if(stage.startsWith("remove:")&&++seen===1)controller.abort(Error("stop collecting"));},exclusive:async resource=>({release:()=>{released.push(resource);}})});
 await expect(collectCacheV1(base,leased,{dryRun:false,only:new Set(["leased"])},stopping,lines())).rejects.toThrow("stop collecting");
 expect(released.length).toBe(2);expect(readdirSync(join(repository,"leased")).length).toBeGreaterThan(0);
});
