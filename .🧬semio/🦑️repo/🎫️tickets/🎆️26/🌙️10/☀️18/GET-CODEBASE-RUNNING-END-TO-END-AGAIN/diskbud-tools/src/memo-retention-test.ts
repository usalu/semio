/** 🧹️ Contract: a graph memo directory stays within its byte budget. The plan equals an independent Python model on seeded states, never removes the newest slice, keeps what the count alone kept when the budget is not reached (the former rule), and the real collection removes exactly the planned files and announces a byte-driven removal in English and German.
 * @see ../../🧹️retention/🟦️.ts */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync,readdirSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GRAPH_MEMO_RETENTION_V1,formatGraphMemoCollectionV1,planGraphMemoCollectionV1,type GraphMemoCollectionV1,type GraphMemoFileV1,type GraphMemoRecordV1} from "../../🧹️retention/🟦️.ts";
import {GRAPH_MEMO_POLICY_V1,collectGraphMemoV1} from "../../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url));
const python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-ret-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));

const generator=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
type Scenario={records:GraphMemoRecordV1[];files:Record<string,GraphMemoFileV1>;now:number;retention:{recordsKept:number;minimumRecordsKept:number;maximumDirectoryBytes:number;graceMilliseconds:number}};
const scenario=(seed:number):Scenario=>{
 const next=generator(seed),pick=(limit:number)=>Math.floor(next()*limit),now=1_800_000_000_000,files:Record<string,GraphMemoFileV1>={},names:string[]=[];
 for(let index=0;index<2+pick(14);index++){const name=`f${index}.${["plan","rel","chk","lst"][pick(4)]}`;names.push(name);files[name]={bytes:pick(40)*1000+1,mtimeMs:now-pick(5)*40000};}
 const records:GraphMemoRecordV1[]=[];
 for(let index=0;index<1+pick(14);index++)records.push({name:`r${index}.json`,bytes:100+pick(900),mtimeMs:now-pick(12)*1000,owned:Array.from({length:pick(5)},()=>names[pick(names.length)]!)});
 return{records,files,now,retention:{recordsKept:1+pick(9),minimumRecordsKept:pick(3),maximumDirectoryBytes:pick(4)===0?1_000_000_000:pick(150)*1000,graceMilliseconds:60000}};
};
const seeded=Array.from({length:400},(_,index)=>scenario(index+1));

test("the plan equals the independent Python model on 400 seeded memo states",()=>{
 const input=join(sandbox,"scenarios.json");writeFileSync(input,JSON.stringify({cases:seeded}));
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),input],{encoding:"utf8",maxBuffer:1<<26});
 expect(run.status).toBe(0);
 const expected=JSON.parse(run.stdout) as GraphMemoCollectionV1[];
 expect(seeded.map(row=>planGraphMemoCollectionV1(row.records,row.files,row.now,row.retention))).toEqual(expected);
});

test("the newest slice always stays, the count and the budget bound the rest, and an infinite budget keeps the former count rule",()=>{
 for(const row of seeded){
  const plan=planGraphMemoCollectionV1(row.records,row.files,row.now,row.retention),newest=[...row.records].sort((a,b)=>b.mtimeMs-a.mtimeMs||(a.name<b.name?-1:1))[0]!;
  expect(plan.removeRecords).not.toContain(newest.name);
  expect(plan.keptRecords).toBeLessThanOrEqual(row.retention.recordsKept);
  expect(plan.keptBytes<=row.retention.maximumDirectoryBytes||plan.keptRecords<=Math.max(row.retention.minimumRecordsKept,1)).toBe(true);
  const former=planGraphMemoCollectionV1(row.records,row.files,row.now,{...row.retention,maximumDirectoryBytes:Number.POSITIVE_INFINITY});
  expect(former.keptRecords).toBe(Math.min(row.records.length,row.retention.recordsKept));
  expect(former.overBudget).toBe(0);
 }
});

test("the schema declares a budget that holds the largest single slice and two slices of the observed size",()=>{
 expect(GRAPH_MEMO_RETENTION_V1.maximumDirectoryBytes).toBeGreaterThan(GRAPH_MEMO_POLICY_V1.maximumPlanBytes+GRAPH_MEMO_POLICY_V1.maximumRelevanceBytes);
 expect(GRAPH_MEMO_RETENTION_V1.minimumRecordsKept).toBeLessThanOrEqual(GRAPH_MEMO_RETENTION_V1.recordsKept);
 expect(GRAPH_MEMO_RETENTION_V1.recordsKept).toBe(8);
});

test("the real collection removes exactly the planned files from a directory and reports a byte-driven removal in both languages",()=>{
 const directory=join(sandbox,"memo");Bun.spawnSync(["mkdir","-p",directory]);
 const write=(name:string,size:number,ageMs:number)=>{const path=join(directory,name);writeFileSync(path,"x".repeat(size));const at=(Date.now()-ageMs)/1000;utimesSync(path,at,at);};
 for(let index=0;index<4;index++){
  const plan=`p${index}`,relevance=`r${index}`,stamps=`s${index}`;
  writeFileSync(join(directory,`k${index}.json`),JSON.stringify({state:"sealed",plan:{sha256:plan},relevance:{id:relevance,stamps}}));
  const at=(Date.now()-index*1000)/1000;utimesSync(join(directory,`k${index}.json`),at,at);
  write(plan+GRAPH_MEMO_POLICY_V1.planSuffix,400_000,120_000);write(relevance+GRAPH_MEMO_POLICY_V1.relevanceSuffix,1000,120_000);write(relevance+GRAPH_MEMO_POLICY_V1.recheckSuffix,1000,120_000);write(stamps+GRAPH_MEMO_POLICY_V1.stampSuffix,1000,120_000);
 }
 write("orphan.plan",10,120_000);write("young.plan",10,1000);
 let seen="";const writer=process.stderr.write.bind(process.stderr);
 (process.stderr as unknown as {write:(text:string)=>boolean}).write=(text:string)=>{seen+=text;return true;};
 try{collectGraphMemoV1(directory,3);}finally{(process.stderr as unknown as {write:typeof writer}).write=writer;}
 const left=readdirSync(directory).sort();
 expect(left).toEqual(["k0.json","k1.json","k2.json","p0.plan","p1.plan","p2.plan","r0.chk","r0.rel","r1.chk","r1.rel","r2.chk","r2.rel","s0.lst","s1.lst","s2.lst","young.plan"].sort());
 expect(existsSync(join(directory,"orphan.plan"))).toBe(false);
 expect(seen).toBe("");
});

test("the real collection enforces the byte budget: the oldest slice and the files only it names go, the newest two stay, and both languages are announced once",()=>{
 const directory=join(sandbox,"memo-budget");Bun.spawnSync(["mkdir","-p",directory]);
 const old=(path:string,ageMs:number)=>{const at=(Date.now()-ageMs)/1000;utimesSync(path,at,at);};
 for(let index=0;index<3;index++){
  writeFileSync(join(directory,`k${index}.json`),JSON.stringify({state:"sealed",plan:{sha256:`p${index}`},relevance:{id:`r${index}`,stamps:`s${index}`}}));old(join(directory,`k${index}.json`),index*1000);
  for(const name of [`p${index}${GRAPH_MEMO_POLICY_V1.planSuffix}`,`r${index}${GRAPH_MEMO_POLICY_V1.relevanceSuffix}`,`r${index}${GRAPH_MEMO_POLICY_V1.recheckSuffix}`,`s${index}${GRAPH_MEMO_POLICY_V1.stampSuffix}`]){writeFileSync(join(directory,name),"x".repeat(100_000));old(join(directory,name),120_000);}
 }
 let seen="";const writer=process.stderr.write.bind(process.stderr);
 (process.stderr as unknown as {write:(text:string)=>boolean}).write=(text:string)=>{seen+=text;return true;};
 try{collectGraphMemoV1(directory,8,900_000);}finally{(process.stderr as unknown as {write:typeof writer}).write=writer;}
 expect(readdirSync(directory).sort()).toEqual(["k0.json","k1.json","p0.plan","p1.plan","r0.chk","r0.rel","r1.chk","r1.rel","s0.lst","s1.lst"].sort());
 expect(seen.split("\n").filter(Boolean)).toHaveLength(1);
 expect(seen).toContain("the graph memo exceeded its");expect(seen).toContain("Das Graph-Memo überschritt sein Budget");
});

test("a byte-driven removal keeps only the slices that fit and prints the English and German line",()=>{
 const plan=planGraphMemoCollectionV1([0,1,2].map(index=>({name:`k${index}.json`,bytes:150,mtimeMs:Date.now()-index*1000,owned:[`p${index}.plan`]})),Object.fromEntries([0,1,2].map(index=>[`p${index}.plan`,{bytes:600_000_000,mtimeMs:Date.now()-120_000}])),Date.now(),{...GRAPH_MEMO_RETENTION_V1,maximumDirectoryBytes:1_300_000_000});
 expect(plan.keptRecords).toBe(2);expect(plan.removeRecords).toEqual(["k2.json"]);expect(plan.removeFiles).toEqual(["p2.plan"]);expect(plan.overBudget).toBe(1);
 expect(formatGraphMemoCollectionV1(plan,"en",{maximumDirectoryBytes:1_300_000_000})).toBe("[repo graph] the graph memo exceeded its 1239.8 MiB budget: removed 1 older slice(s) and 1 unnamed file(s) (572.2 MiB); 2 slice(s) kept (1144.4 MiB)");
 expect(formatGraphMemoCollectionV1(plan,"de",{maximumDirectoryBytes:1_300_000_000})).toBe("[repo graph] Das Graph-Memo überschritt sein Budget von 1239.8 MiB: 1 ältere Ausschnitt(e) und 1 unbenannte Datei(en) entfernt (572.2 MiB); 2 Ausschnitt(e) behalten (1144.4 MiB)");
});
