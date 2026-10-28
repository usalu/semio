/** 🌜 Acceptance conformance: the budgets of the build system's acceptance targets are a closed, bilingual table; recorded official runs of the gate (g34-g46: a breach and a pass of every product family) are judged against it and against an independent Python oracle; the scenarios, the byte-exact edits, the crash journal, the exact clock and the physical footprint are checked with scripted ports and a real child process; the nightly workflow and the operations catalogue are checked against the parser. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import {parse as parseYaml} from "yaml";
import schema from "../@@SCHD@@/@@JSON@@" with {type:"json"};
import budgets from "../@@BUDGETSF@@" with {type:"json"};
import fixture from "./@@FIXD@@/@@JSON@@" with {type:"json"};
import slo from "../../@@SLOF@@" with {type:"json"};
import catalog from "../../../../@@OPS@@" with {type:"json"};
import {validateJsonSchemaSubset} from "@@VALIDATOR@@";
import {acceptanceDigestV1,acceptanceExpandV1,acceptanceFailureV1,acceptanceJudgeBudgetV1,acceptanceJudgeV1,acceptanceLineV1,acceptanceMutateV1,acceptanceObserverV1,acceptanceReplayV1,acceptanceReportV1,acceptanceRestoreV1,acceptanceSelectV1,acceptanceSetIssuesV1,acceptanceTallyV1,acceptanceVerdictV1,renderAcceptanceListV1,renderAcceptanceReportV1,type AcceptanceBudgetV1,type AcceptanceOutcomeV1,type AcceptanceRecordV1,type AcceptanceSetV1} from "../@@TS@@";
import {acceptanceRecoverV1,acceptanceRunV1,acceptanceSpawnV1,type AcceptanceJournalRowV1,type AcceptancePortsV1,type AcceptanceRunRequestV1,type AcceptanceRunResultV1} from "../@@RUN@@/@@TS@@";
import {acceptanceArgumentsV1,acceptanceCommandLineV1,acceptanceEnvironmentV1,acceptanceOptionNamesV1,runAcceptanceCommandV1,type AcceptanceCommandPortsV1} from "../@@CMD@@/@@TS@@";

const set=budgets as unknown as AcceptanceSetV1,policy=schema.definitions.Policy.const,plain=(value:unknown):any=>JSON.parse(JSON.stringify(value));
const ajv=new Ajv({strict:true,allowUnionTypes:true}),define=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const oracle=join(import.meta.dir,"@@ORACLED@@/@@PY@@"),fixturePath=join(import.meta.dir,"@@FIXD@@/@@JSON@@");
const root=((start:string):string=>{for(let path=start;;path=dirname(path)){if(existsSync(join(path,"package.json"))&&existsSync(join(path,"🧰️framework")))return path;if(dirname(path)===path)throw Error("repository root required");}})(import.meta.dir);
const scratch:string[]=[];
const temporary=():string=>{const folder=mkdtempSync(join(tmpdir(),"acc-"));scratch.push(folder);return folder;};
process.on("exit",()=>{for(const folder of scratch)rmSync(folder,{recursive:true,force:true});});
const python=(mode:string,data:unknown):any=>{const folder=temporary(),path=join(folder,"input.json");writeFileSync(path,JSON.stringify(data));const run=spawnSync("python3",["-I",oracle,mode,path],{encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const clone=():any=>plain(budgets);
const cases=fixture.cases as unknown as readonly any[];
const recordOf=(row:any,overrides:Partial<AcceptanceRecordV1>={}):AcceptanceRecordV1=>({...acceptanceReplayV1({scenario:row.scenario,role:row.role,index:0,command:"recorded",exit:row.exit,wallMs:row.wallMs,clock:row.clock,peakMb:row.peakResidentMb,measure:"resident",loadPerCore:null},acceptanceExpandV1(row.log)),...overrides});

test("the schema is valid and, with the budget table and the fixture, a closed document for ajv and for the repository validator; the table is consistent and every inconsistency is found",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 for(const [name,value] of [["Set",budgets],["Fixture",fixture]] as const){expect(define(name)(value)).toBe(true);expect(validateJsonSchemaSubset(schema.definitions[name],value,schema)).toEqual([]);}
 expect(acceptanceSetIssuesV1(set)).toEqual([]);
 const broken:[string,(value:any)=>void][]=[
  ["duplicate budget",value=>{value.budgets[1].id=value.budgets[0].id;}],
  ["unknown scenario",value=>{value.budgets[0].scenario="nowhere";}],
  ["a role the kind never runs",value=>{value.budgets[0].role="edit";}],
  ["a prime budget",value=>{value.budgets[0].role="prime";}],
  ["a statistic the samples cannot back",value=>{value.budgets.find((row:any)=>row.id==="course-edit-wall").statistic="p50";}],
  ["a fractional count",value=>{value.budgets.find((row:any)=>row.id==="course-noop-executed").maximum=0.5;}],
  ["identical texts",value=>{value.budgets[0].title.de=value.budgets[0].title.en;}],
  ["identical label",value=>{value.labels.title.de=value.labels.title.en;}],
  ["hot on a peer scenario",value=>{value.scenarios.find((row:any)=>row.kind==="peer").hot=3;}],
  ["peer without files",value=>{delete value.scenarios.find((row:any)=>row.kind==="peer").files;}],
  ["unknown product",value=>{value.scenarios[0].product="nope";}],
  ["a trigger that is no expression",value=>{value.peer.trigger="(";}],
  ["a path beyond the limit",value=>{value.products[0].edit.file="x".repeat(300);}],
  ["an open row",value=>{value.budgets[0].extra=1;}]
 ];
 for(const [name,change] of broken){const value=clone();change(value);expect(acceptanceSetIssuesV1(value).length,name).toBeGreaterThan(0);}
});

test("the budgets are the acceptance targets of the brief: course hot < 10 s and < 1.5 GB, course edit < 60 s, Rust edit < 120 s, dashboard no-op < 10 s, 0 re-executions on peer edits",()=>{
 const by=(id:string)=>set.budgets.find(row=>row.id===id)!;
 for(const [id,metric,maximum,enforcement] of [["course-noop-wall","wallMs",10000,"blocking"],["course-noop-footprint","peakFootprintMb",1536,"blocking"],["course-noop-executed","executedActions",0,"blocking"],["course-edit-wall","wallMs",60000,"blocking"],["course-revert-wall","wallMs",60000,"blocking"],["print-edit-wall","wallMs",120000,"blocking"],["dashboard-edit-wall","wallMs",120000,"advisory"],["dashboard-noop-wall","wallMs",10000,"blocking"],["dashboard-noop-footprint","peakFootprintMb",1536,"blocking"],["course-peer-unrelated-peer-executed","executedActions",0,"blocking"],["course-peer-scheduling-peer-executed","executedActions",0,"blocking"],["print-peer-unrelated-peer-executed","executedActions",0,"blocking"]] as const)
  expect(by(id),id).toMatchObject({metric,maximum,enforcement,statistic:"max"});
 expect(by("course-noop-footprint").maximum*1048576).toBe(1.5*1024**3);
 for(const scenario of set.scenarios.filter(row=>row.kind==="noop"))expect(scenario.hot,scenario.id).toBe(3);
 expect(set.scenarios.map(row=>row.id)).toEqual(["course-noop","print-noop","dashboard-noop","course-peer-unrelated","print-peer-unrelated","dashboard-peer-unrelated","course-peer-scheduling","course-edit","print-edit","dashboard-edit"]);
 for(const row of set.budgets.filter(entry=>entry.scenario.includes("-peer-")))expect(row.metric).toBe("executedActions");
});

test("every budget that names an objective of the bench SLO table restates its number exactly",()=>{
 const objectives=new Map((slo as any).slos.map((row:any)=>[row.id,row] as const));
 let restated=0;
 for(const budget of set.budgets.filter(row=>row.slo)){
  const objective:any=objectives.get(budget.slo!);
  expect(objective,budget.id).toBeDefined();
  const gate=budget.metric==="peakFootprintMb"?objective.gate/1048576:objective.gate;
  expect(budget.maximum,`${budget.id} vs ${budget.slo}`).toBe(gate);
  expect(objective.goal,budget.slo).toBe(objective.gate);
  restated++;
 }
 expect(restated).toBe(13);
});

test("the recorded logs of the gate are read like the independent oracle reads them: executed actions and memo service agree for every case",()=>{
 const expected=python("observe",fixture);
 expect(cases.length).toBeGreaterThanOrEqual(12);
 for(const row of cases){
  const observer=acceptanceObserverV1();
  for(const line of acceptanceExpandV1(row.log))observer.feed(0,line);
  const seen=observer.finish(row.wallMs);
  expect({executed:seen.executed,served:seen.served},row.id).toEqual(expected[row.id]);
  expect({executed:seen.executed,served:seen.served},row.id).toEqual({executed:row.expect.executed,served:row.expect.served});
 }
 expect(acceptanceExpandV1(["a",["b",3],"c"])).toEqual(["a","b","b","b","c"]);
});

test("recorded runs are judged as written down: every pass passes, every breach fails, a time within one grain of the budget and a resident set above it are unproven",()=>{
 for(const row of cases){
  const record=recordOf(row),mine=set.budgets.filter(budget=>budget.scenario===row.scenario&&budget.role===row.role),verdicts=Object.fromEntries(mine.map(budget=>[budget.id,acceptanceJudgeBudgetV1(budget,[record],{scenario:row.scenario,status:"ran",reason:null,restored:null,landed:null,merged:null}).status]));
  expect(verdicts,row.id).toEqual(row.expect.rows);
 }
 const kinds=new Set(cases.flatMap(row=>Object.values(row.expect.rows)));
 expect([...kinds].sort()).toEqual(["fail","pass","unproven"]);
 const grain=acceptanceJudgeBudgetV1(set.budgets.find(row=>row.id==="course-noop-wall")!,[recordOf(cases.find(row=>row.id==="course-hot-grain"))],undefined);
 expect([grain.status,grain.reason,grain.bound,grain.value]).toEqual(["unproven","clock-grain","upper",10100]);
 const loaded=acceptanceJudgeBudgetV1(set.budgets.find(row=>row.id==="course-noop-wall")!,[recordOf(cases.find(row=>row.id==="course-hot-breach"),{loadPerCore:4})],undefined);
 expect([loaded.status,loaded.reason]).toEqual(["unproven","host-saturated"]);
 const exact=acceptanceJudgeBudgetV1(set.budgets.find(row=>row.id==="course-noop-wall")!,[recordOf(cases.find(row=>row.id==="course-hot-grain"),{clock:{kind:"exact",grainMs:0}})],undefined);
 expect([exact.status,exact.bound]).toEqual(["fail","exact"]);
 const physical=acceptanceJudgeBudgetV1(set.budgets.find(row=>row.id==="course-noop-footprint")!,[recordOf(cases.find(row=>row.id==="course-hot-breach"),{measure:"footprint"})],undefined);
 expect([physical.status,physical.value]).toEqual(["fail",2478]);
});

test("a grid of budgets and run sets is judged exactly like the independent Python oracle",()=>{
 let seed=20261027;
 const next=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;},pick=<T>(values:readonly T[]):T=>values[Math.floor(next()*values.length)]!;
 const entries:any[]=[];
 for(let at=0;at<700;at++){
  const metric=pick(["wallMs","peakFootprintMb","executedActions"] as const),statistic=pick(["max","min","p50"] as const),maximum=metric==="executedActions"?pick([0,1,2]):pick([1000,10000,1536,60000]),size=pick([0,1,2,3,3,4]);
  const records=Array.from({length:size},(_,index)=>({scenario:"s",role:"hot",index,command:"x",exit:pick([0,0,0,0,1,null]),killed:pick([null,null,null,"timeout","abort"]),wallMs:pick([0.5,0.9,1,1.1,5000,9999,10000,10001,10100,15000,61000])*pick([1,1,1,0.5]),clock:pick([{kind:"exact",grainMs:0},{kind:"poll",grainMs:5000}]),peakMb:pick([null,100,1536,1537,3000]),measure:pick([null,"footprint","resident"]),executed:pick([null,0,1,2,3]),served:false,inferred:0,phases:{},loadPerCore:pick([null,0.1,1.49,1.5,3])}));
  entries.push({budget:{id:"b",title:{en:"a",de:"b"},scenario:"s",role:"hot",metric,statistic,maximum,enforcement:"blocking",source:"x"},records,outcome:pick([undefined,{scenario:"s",status:"ran"},{scenario:"s",status:"failed"},{scenario:"s",status:"skipped"}])});
 }
 const expected=python("judge",entries);
 entries.forEach((entry,at)=>{
  const row=acceptanceJudgeBudgetV1(entry.budget as AcceptanceBudgetV1,entry.records,entry.outcome&&{reason:null,restored:null,landed:null,merged:null,...entry.outcome});
  expect({status:row.status,reason:row.reason,value:row.value,n:row.n,bound:row.bound},JSON.stringify(entry)).toEqual(expected[at]);
 });
 expect(new Set(expected.map((row:any)=>row.status)).size).toBe(3);
 expect(new Set(expected.map((row:any)=>row.reason)).size).toBeGreaterThanOrEqual(7);
});

test("an edit appends exactly one line and the restore is byte-exact for empty, newline-less, CRLF and binary files; a foreign write survives; the oracle agrees",()=>{
 let seed=7;
 const next=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const samples:Uint8Array[]=[new Uint8Array(),Buffer.from("a"),Buffer.from("a\n"),Buffer.from("a\r\nb\r\n"),Buffer.from([0,255,10,13,200]),Buffer.from("🧬\n"),...Array.from({length:60},()=>Buffer.from(Array.from({length:Math.floor(next()*80)},()=>Math.floor(next()*256))))];
 const checked:any[]=[];
 for(const original of samples){
  const line=acceptanceLineV1(next()>0.5?"//":"#","abcdef12"),edited=acceptanceMutateV1(original,line),back=acceptanceRestoreV1(original,edited,line,edited.bytes);
  expect(back.exact).toBe(true);expect(back.merged).toBe(false);expect(Buffer.from(back.bytes!).equals(Buffer.from(original))).toBe(true);
  expect(acceptanceDigestV1(back.bytes!)).toBe(acceptanceDigestV1(original));
  checked.push({original:Buffer.from(original).toString("base64"),line,edited:Buffer.from(edited.bytes).toString("base64"),restored:Buffer.from(back.bytes!).toString("base64")});
  const foreign=Buffer.concat([Buffer.from(edited.bytes),Buffer.from("foreign\n")]),kept=acceptanceRestoreV1(original,edited,line,foreign);
  expect(kept.merged).toBe(true);expect(kept.exact).toBe(false);expect(Buffer.from(kept.bytes!).toString("utf8").endsWith("foreign\n")).toBe(true);
 }
 expect(python("edit",checked)).toEqual([]);
 expect(acceptanceRestoreV1(Buffer.from("x"),acceptanceMutateV1(Buffer.from("x"),"# [acceptance] abcdef\n"),"# [acceptance] abcdef\n",null)).toMatchObject({bytes:null,merged:true});
 expect(()=>acceptanceLineV1("//","BAD")).toThrow();
 expect(acceptanceLineV1("#","abcdef")).toBe("# [acceptance] abcdef\n");
});

type Script=(request:AcceptanceRunRequestV1,state:World)=>Promise<AcceptanceRunResultV1>|AcceptanceRunResultV1;
type World={files:Map<string,Uint8Array>;journal:AcceptanceJournalRowV1[];notes:string[];calls:string[];clock:{t:number};controller:AbortController};
const original=(file:string)=>Buffer.from(`original ${file}\n`);
const result=(row:any,lines?:readonly string[],overrides:Partial<AcceptanceRunResultV1>={}):AcceptanceRunResultV1=>{
 const observer=acceptanceObserverV1();
 for(const line of lines??acceptanceExpandV1(row.log))observer.feed(0,line);
 return {command:"fake",exit:row.exit,killed:null,wallMs:row.wallMs,clock:row.clock,peakMb:row.peakResidentMb,measure:"resident",endedMs:0,observation:observer.finish(row.wallMs),...overrides};
};
const world=(extra:string[]=[]):World=>{
 const files=new Map<string,Uint8Array>();
 for(const file of [...set.products.map(row=>row.edit.file),...set.peer.unrelated.map(row=>row.file),...set.peer.scheduling.map(row=>row.file),...extra])files.set(file,original(file));
 return {files,journal:[],notes:[],calls:[],clock:{t:0},controller:new AbortController()};
};
const ports=(state:World,script:Script):AcceptancePortsV1=>({
 async run(request,signal){
  state.calls.push(`${request.scenario}:${request.role}`);
  if(signal.aborted)throw signal.reason;
  const made=await script(request,state);
  state.clock.t+=1;
  return {...made,endedMs:state.clock.t};
 },
 files:{read:async path=>state.files.get(path)??null,write:async(path,bytes)=>{state.files.set(path,Buffer.from(bytes));}},
 journal:{read:async()=>state.journal,write:async rows=>{state.journal=[...rows];},clear:async()=>{state.journal=[];}},
 host:()=>({loadPerCore:0.2}),nonce:()=>"abcdef12",now:()=>state.clock.t,check:()=>{},progress:async()=>{},note:line=>{state.notes.push(line);},signal:state.controller.signal
});
const byId=(id:string)=>cases.find(row=>row.id===id)!;
const hotFor=(request:AcceptanceRunRequestV1)=>result(request.target.includes("dashboard")?byId("dashboard-hot-pass"):request.target.includes("print")?byId("print-hot-pass"):byId("course-hot-pass"));
const marked=(state:World,file:string)=>Buffer.from(state.files.get(file)!).toString("utf8").includes("[acceptance]");
const scenario=(id:string)=>set.scenarios.find(row=>row.id===id)!;

test("a no-op scenario primes twice, then runs the hot runs; a product that is hot is not primed again",async()=>{
 const state=world(),run=await acceptanceRunV1(set,[scenario("course-noop"),scenario("course-noop")],ports(state,hotFor));
 expect(state.calls).toEqual(["course-noop:prime","course-noop:prime","course-noop:hot","course-noop:hot","course-noop:hot","course-noop:hot","course-noop:hot","course-noop:hot"]);
 expect(run.outcomes.map(row=>row.status)).toEqual(["ran","ran"]);
 expect(run.records.filter(row=>row.role==="hot").map(row=>row.index)).toEqual([0,1,2,0,1,2]);
 expect(run.aborted).toBe(false);
});

test("an edit scenario edits one line, runs, restores the file byte for byte, runs the revert, and leaves the product to be primed again; the journal is gone",async()=>{
 const state=world(),file=set.products[0]!.edit.file,seen:string[]=[];
 const run=await acceptanceRunV1(set,[scenario("course-edit"),scenario("course-noop")],ports(state,(request,current)=>{seen.push(`${request.role}:${marked(current,file)}`);if(request.role==="edit")expect(current.journal.length).toBe(1);return hotFor(request);}));
 expect(seen.slice(0,4)).toEqual(["prime:false","prime:false","edit:true","revert:false"]);
 expect(state.calls.slice(0,4)).toEqual(["course-edit:prime","course-edit:prime","course-edit:edit","course-edit:revert"]);
 expect(state.calls[4]).toBe("course-noop:prime");
 expect(Buffer.from(state.files.get(file)!).equals(original(file))).toBe(true);
 expect(state.journal).toEqual([]);
 expect(run.outcomes[0]).toMatchObject({scenario:"course-edit",status:"ran",restored:true,landed:null,merged:false});
});

test("a peer edit lands when the first graph line appears, while the run executes; the follow-up runs after the byte-exact restore",async()=>{
 const state=world(),files=set.peer.unrelated.map(row=>row.file),during:boolean[][]=[];
 const run=await acceptanceRunV1(set,[scenario("course-peer-unrelated")],ports(state,async(request,current)=>{
  if(request.role!=="peer")return hotFor(request);
  const lines=acceptanceExpandV1(byId("course-hot-pass").log);
  lines.forEach((line,at)=>{if(at===0)during.push(files.map(file=>marked(current,file)));request.onLine(at,line);});
  await new Promise(accept=>setTimeout(accept,10));
  during.push(files.map(file=>marked(current,file)));
  return result(byId("course-hot-pass"),lines);
 }));
 expect(state.calls).toEqual(["course-peer-unrelated:prime","course-peer-unrelated:prime","course-peer-unrelated:control","course-peer-unrelated:peer","course-peer-unrelated:follow"]);
 expect(during).toEqual([[false,false],[true,true]]);
 for(const file of files)expect(Buffer.from(state.files.get(file)!).equals(original(file))).toBe(true);
 expect(run.outcomes[0]).toMatchObject({status:"ran",restored:true,landed:true,merged:false});
 const rows=acceptanceJudgeV1(set,["course-peer-unrelated"],run.records,run.outcomes);
 expect(rows.filter(row=>row.kind==="invariant").map(row=>[row.id,row.status])).toEqual([["course-peer-unrelated-exit","pass"],["course-peer-unrelated-restored","pass"],["course-peer-unrelated-landed","pass"]]);
 expect(rows.filter(row=>row.kind==="budget").every(row=>row.status==="pass")).toBe(true);
});

test("a peer edit that never lands is reported unproven and runs no follow-up; a foreign write during the stage survives the restore",async()=>{
 const quiet=world(),first=set.peer.unrelated[0]!.file;
 const silent=await acceptanceRunV1(set,[scenario("course-peer-unrelated")],ports(quiet,request=>result(byId("course-hot-pass"),["INFO: 1 process: 1 internal."])));
 expect(quiet.calls).toEqual(["course-peer-unrelated:prime","course-peer-unrelated:prime","course-peer-unrelated:control","course-peer-unrelated:peer"]);
 expect(silent.outcomes[0]).toMatchObject({status:"ran",landed:false,restored:null});
 const rows=acceptanceJudgeV1(set,["course-peer-unrelated"],silent.records,silent.outcomes);
 expect(rows.find(row=>row.id==="course-peer-unrelated-landed")).toMatchObject({status:"unproven",reason:"not-landed"});
 expect(rows.find(row=>row.id==="course-peer-unrelated-follow-executed")).toMatchObject({status:"unproven",reason:"no-samples"});
 const crowded=world();
 const merged=await acceptanceRunV1(set,[scenario("course-peer-unrelated")],ports(crowded,async(request,current)=>{
  if(request.role!=="peer")return hotFor(request);
  const lines=acceptanceExpandV1(byId("course-hot-pass").log);
  lines.forEach((line,at)=>request.onLine(at,line));
  await new Promise(accept=>setTimeout(accept,10));
  current.files.set(first,Buffer.concat([Buffer.from(current.files.get(first)!),Buffer.from("written by another agent\n")]));
  return result(byId("course-hot-pass"),lines);
 }));
 expect(Buffer.from(crowded.files.get(first)!).toString("utf8")).toBe(`original ${first}\nwritten by another agent\n`);
 expect(merged.outcomes[0]).toMatchObject({restored:false,merged:true});
 expect(acceptanceJudgeV1(set,["course-peer-unrelated"],merged.records,merged.outcomes).find(row=>row.id==="course-peer-unrelated-restored")).toMatchObject({status:"unproven",reason:"foreign-write"});
});

test("a failed run ends its scenario with the edit restored and no revert; a missing file fails the scenario; the next scenario still runs",async()=>{
 const state=world(),file=set.products[1]!.edit.file;
 const run=await acceptanceRunV1(set,[scenario("print-edit"),scenario("dashboard-noop")],ports(state,request=>request.role==="edit"?result(byId("print-edit-pass"),undefined,{exit:1}):hotFor(request)));
 expect(state.calls).toEqual(["print-edit:prime","print-edit:prime","print-edit:edit","dashboard-noop:prime","dashboard-noop:prime","dashboard-noop:hot","dashboard-noop:hot","dashboard-noop:hot"]);
 expect(Buffer.from(state.files.get(file)!).equals(original(file))).toBe(true);
 expect(run.outcomes.map(row=>[row.scenario,row.status,row.reason])).toEqual([["print-edit","failed","edit run failed"],["dashboard-noop","ran",null]]);
 const rows=acceptanceJudgeV1(set,["print-edit"],run.records,run.outcomes);
 expect(rows.find(row=>row.id==="print-edit-exit")).toMatchObject({status:"fail",value:1});
 expect(rows.find(row=>row.id==="print-edit-wall")).toMatchObject({status:"unproven",reason:"run-failed"});
 const lost=world();lost.files.delete(set.products[2]!.edit.file);
 const missing=await acceptanceRunV1(set,[scenario("dashboard-edit"),scenario("course-noop")],ports(lost,hotFor));
 expect(missing.outcomes[0]!.status).toBe("failed");expect(missing.outcomes[0]!.reason).toContain("does not exist");expect(missing.outcomes[1]!.status).toBe("ran");
});

test("a cancellation restores the edits, stops, and reports what was measured; a killed earlier run is recovered from the journal first",async()=>{
 const state=world(),file=set.products[0]!.edit.file;
 const run=await acceptanceRunV1(set,[scenario("course-edit"),scenario("course-noop")],ports(state,(request,current)=>{if(request.role==="edit"){current.controller.abort(Error("stop"));throw Error("killed");}return hotFor(request);}));
 expect(run.aborted).toBe(true);
 expect(Buffer.from(state.files.get(file)!).equals(original(file))).toBe(true);
 expect(state.journal).toEqual([]);
 expect(run.outcomes).toEqual([expect.objectContaining({scenario:"course-edit",status:"failed",reason:"aborted"})]);
 expect(state.calls).toEqual(["course-edit:prime","course-edit:prime","course-edit:edit"]);
 const left=world(),leftover="left over\n";
 left.files.set(file,Buffer.from(`edited\n${leftover}`));left.journal=[{file,original:Buffer.from(original(file)).toString("base64")}];
 expect(await acceptanceRecoverV1(ports(left,hotFor))).toBe(1);
 expect(Buffer.from(left.files.get(file)!).equals(original(file))).toBe(true);expect(left.journal).toEqual([]);expect(left.notes[0]).toContain("Restored");
 const again=world();again.files.set(file,Buffer.from("x"));again.journal=[{file,original:Buffer.from("y").toString("base64")}];
 await acceptanceRunV1(set,[scenario("course-noop")],ports(again,hotFor));
 expect(Buffer.from(again.files.get(file)!).toString()).toBe("y");
});

const commandPorts=(state:World,script:Script,log:{out:string[];files:Map<string,string>}):AcceptanceCommandPortsV1=>({
 write:line=>{log.out.push(line);},
 writeText:async(path,text)=>{log.files.set(path,text);},
 environment:{SEMIO_LANGUAGE:"en"},
 isoNow:()=>"2026-10-27T12:00:00.000Z",
 host:()=>({platform:"darwin",arch:"arm64",cpus:10,memoryBytes:32*1024**3,load1:1,loadPerCore:0.1,loadClass:"quiet",hostClass:"darwin-arm64-10"}),
 runs:()=>ports(state,script)
});

test("a passing run exits clean and reports in English and German as text, markdown and a closed JSON file; a breaching run writes the same files and then fails",async()=>{
 const state=world(),log={out:[] as string[],files:new Map<string,string>()};
 const report=await runAcceptanceCommandV1(["--scenario=kind:noop","--file=r.json","--markdown=r.md","--lang=en"],commandPorts(state,hotFor,log),set);
 expect(report.verdict).toBe("pass");expect(report.tally.blockingFailed).toBe(0);expect(report.tally.provenBlocking).toBeGreaterThanOrEqual(7);
 expect(log.out[0]).toContain("ACCEPTANCE PASS");
 expect(define("Report")(JSON.parse(log.files.get("r.json")!))).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.Report,JSON.parse(log.files.get("r.json")!),schema)).toEqual([]);
 expect(log.files.get("r.md")).toContain("| Budget |");
 const german={out:[] as string[],files:new Map<string,string>()};
 await runAcceptanceCommandV1(["--scenario=course-noop","--lang=de"],commandPorts(world(),hotFor,german),set);
 expect(german.out[0]).toContain("ABNAHME BESTANDEN");expect(german.out[0]).toContain("Architekturkurs: heißer Leerlauf");
 const slow={out:[] as string[],files:new Map<string,string>()};
 await expect(runAcceptanceCommandV1(["--scenario=course-noop","--file=slow.json","--lang=de"],commandPorts(world(),request=>result(byId("course-hot-breach")),slow),set)).rejects.toThrow(/1 blocking acceptance budgets are broken\n1 verbindliche Abnahmebudgets sind verletzt/u);
 expect(JSON.parse(slow.files.get("slow.json")!)).toMatchObject({verdict:"fail",language:"de",tally:{blockingFailed:1}});
 expect(slow.out[0]).toContain("ABNAHME VERLETZT");
 expect(slow.out[0]).toContain("verletzt");
});

test("edit and peer scenarios need --in-place: explicit selection is refused, `all` skips them and says so, a cancellation still writes the partial report",async()=>{
 const state=world(),log={out:[] as string[],files:new Map<string,string>()};
 await expect(runAcceptanceCommandV1(["--scenario=course-edit"],commandPorts(state,hotFor,log),set)).rejects.toThrow(/--in-place/u);
 const notes:string[]=[],everything=commandPorts(world(),hotFor,log);
 const report=await runAcceptanceCommandV1(["--json"],{...everything,runs:language=>({...everything.runs(language),note:line=>{notes.push(line);}})},set);
 expect(notes.join("\n")).toContain("Skipped without --in-place");
 expect(report.outcomes.filter(row=>row.status==="skipped").length).toBe(7);
 expect(report.rows.filter(row=>row.reason==="skipped").length).toBe(set.budgets.filter(row=>!set.scenarios.find(entry=>entry.id===row.scenario)!.kind.startsWith("noop")).length);
 expect(JSON.parse(log.out[0]!).kind).toBe("acceptance-report");
 await expect(runAcceptanceCommandV1(["--scenario=nope"],commandPorts(state,hotFor,log),set)).rejects.toThrow(/Unbekanntes Szenario/u);
 const stop=world(),cut={out:[] as string[],files:new Map<string,string>()};
 await expect(runAcceptanceCommandV1(["--scenario=course-noop","--file=cut.json"],commandPorts(stop,(request,current)=>{current.controller.abort(Error("cancelled by test"));throw Error("killed");},cut),set)).rejects.toThrow("cancelled by test");
 expect(JSON.parse(cut.files.get("cut.json")!).outcomes[0]).toMatchObject({status:"failed",reason:"aborted"});
});

test("a report that proved no blocking budget is not green; invariants are blocking but do not count as proof",()=>{
 const unknown=acceptanceReportOf([{...recordOf(byId("course-hot-breach"),{executed:null,peakMb:5000,loadPerCore:4}),index:0}]);
 expect(unknown.verdict).toBe("unproven");
 expect(acceptanceFailureV1(unknown)).toContain("No blocking acceptance budget was proven\nKein verbindliches Abnahmebudget wurde bewiesen");
 expect(acceptanceFailureV1(acceptanceReportOf([recordOf(byId("course-hot-pass"))]))).toBeNull();
 expect(acceptanceVerdictV1(acceptanceTallyV1([]))).toBe("unproven");
});
function acceptanceReportOf(records:AcceptanceRecordV1[]){
 const outcomes:AcceptanceOutcomeV1[]=[{scenario:"course-noop",status:"ran",reason:null,restored:null,landed:null,merged:null}],rows=acceptanceJudgeV1(set,["course-noop"],records,outcomes),host={platform:"linux",arch:"x64",cpus:2,memoryBytes:1,load1:null,loadPerCore:null,loadClass:"unknown" as const,hostClass:"linux-x64-2"};
 return acceptanceReportV1({startedAt:"a",finishedAt:"b",host,hostAfter:host,language:"en",selected:["course-noop"],records,outcomes,rows});
}

test("the report speaks both languages from the table alone: no raw key leaks, the two renderings have the same shape and differ in words, the list covers every budget",()=>{
 const records=[recordOf(byId("course-hot-pass")),recordOf(byId("course-hot-breach"),{index:1}),recordOf(byId("course-hot-grain"),{index:2})],report=acceptanceReportOf(records);
 for(const format of ["text","markdown"] as const){
  const en=renderAcceptanceReportV1(report,set,"en",format),de=renderAcceptanceReportV1(report,set,"de",format);
  for(const output of [en,de])expect(output).not.toMatch(/\binvariant[A-Z]\w+|\b(?:no-samples|run-failed|clock-grain|host-saturated|resident-only|not-landed|foreign-write)\b/u);
  expect(en.split("\n").length).toBe(de.split("\n").length);expect(en).not.toBe(de);
  expect(de).toContain("Abnahmebericht");expect(en).toContain("Build acceptance report");
 }
 const markdown=renderAcceptanceReportV1(report,set,"en","markdown");
 expect(markdown.split("\n").filter(line=>line.startsWith("| ")&&line.includes("Budget")).length).toBe(1);
 for(const language of ["en","de"] as const){const list=renderAcceptanceListV1(set,language,"text");for(const budget of set.budgets)expect(list).toContain(budget.id);}
 for(const [key,value] of Object.entries(set.labels))expect([key,value.en===value.de]).toEqual([key,false]);
 const used=new Set([...readFileSync(join(import.meta.dir,"../@@TS@@"),"utf8").matchAll(/\bt\("([A-Za-z-]+)"\)/gu)].map(match=>match[1]!));
 for(const key of used)expect(set.labels[key],key).toBeDefined();
});

test("the command line options are closed and shared with the catalogue; scenarios are selected by id, kind, product or all",()=>{
 const ok=(args:string[])=>acceptanceArgumentsV1(args);
 expect(ok([])).toMatchObject({scenario:"all",inPlace:false,json:false});
 expect(ok(["--scenario=course-noop,print-noop","--in-place","--file","a.json","--markdown=a.md","--json","--lang=de"])).toMatchObject({scenario:["course-noop","print-noop"],inPlace:true,file:"a.json",markdown:"a.md",json:true,language:"de"});
 for(const bad of [["--nope"],["--scenario"],["--scenario=a","--scenario=b"],["--in-place=1"],["--lang=fr"],["--json=1"],["x"],["--file","--json"]])expect(()=>ok(bad),JSON.stringify(bad)).toThrow(/Unbekannt|braucht|Wiederholte/u);
 expect(acceptanceSelectV1(set,["kind:peer"],true).run.map(row=>row.id)).toEqual(["course-peer-unrelated","print-peer-unrelated","dashboard-peer-unrelated","course-peer-scheduling"]);
 expect(acceptanceSelectV1(set,["product:course"],true).run.map(row=>row.id)).toEqual(["course-noop","course-peer-unrelated","course-peer-scheduling","course-edit"]);
 expect(acceptanceSelectV1(set,"all",false).run.map(row=>row.kind)).toEqual(["noop","noop","noop"]);
 expect(acceptanceSelectV1(set,"all",false).skipped.length).toBe(7);
 expect(acceptanceSelectV1(set,["course-edit","course-noop"],true).run.map(row=>row.id)).toEqual(["course-noop","course-edit"]);
 expect(()=>acceptanceSelectV1(set,["course-edit"],false)).toThrow(/--in-place[\s\S]*--in-place/u);
 expect(()=>acceptanceSelectV1(set,["kind:nope"],true)).toThrow(/Unknown scenario[\s\S]*Unbekanntes Szenario/u);
 const entry=(catalog as any).operations.find((row:any)=>row.name==="bench"),names=acceptanceOptionNamesV1();
 expect(entry.synopsis).toContain("acceptance");
 const sub=entry.subcommands.find((row:any)=>row.name==="acceptance");
 expect(sub).toBeDefined();expect(sub.synopsis.startsWith("bench acceptance")).toBe(true);expect(sub.summary.en).not.toBe(sub.summary.de);
 expect([...sub.flags].sort()).toEqual([...names.valued,...names.plain,...names.shared.filter(name=>name!=="--lang")].sort());
 for(const name of [...names.valued,...names.plain]){const flag=(catalog as any).flags.find((row:any)=>row.name===name);expect(flag,name).toBeDefined();expect(flag.operations).toContain("bench");expect(flag.value).toBe(names.valued.includes(name)?"required":"none");expect(flag.description.en).not.toBe(flag.description.de);expect(entry.flags).toContain(name);}
 for(const name of names.shared.filter(name=>name!=="--lang")){const flag=(catalog as any).flags.find((row:any)=>row.name===name);expect(flag.operations).toContain("bench");}
 expect(entry.examples.some((row:any)=>row.command.startsWith("repo bench acceptance"))).toBe(true);
});

test("official runs go through the door with the neutral run command, a clean child environment and only the declared build variables",()=>{
 expect(existsSync(join(root,policy.door))).toBe(true);
 const line=acceptanceCommandLineV1("/checkout","/bun","@a/b:build");
 expect(line).toEqual({command:"/bun",args:[join("/checkout",policy.door),"repo","--tool=bazel","run","@a/b:build"]});
 const env=acceptanceEnvironmentV1({PATH:"/bin",HOME:"/h",SEMIO_BUILD_GENERATION_PIN:"7",SEMIO_BUILD_MODE:"prototype",NX_DAEMON:"1",SEMIO_LANGUAGE:"de",SEMIO_REMOTE_CACHE_ENDPOINT:"https://cache",SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384",SEMIO_BUILD_BUDGET_MS:"9000000",SEMIO_SCRIPT_STALL_MS:"5"},"en");
 expect(env.PATH).toBe("/bin");expect(env.SEMIO_LANGUAGE).toBe("en");expect(env.SEMIO_REMOTE_CACHE_ENDPOINT).toBe("https://cache");expect(env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES).toBe("16384");expect(env.SEMIO_BUILD_BUDGET_MS).toBe("9000000");
 for(const dropped of ["SEMIO_BUILD_GENERATION_PIN","SEMIO_BUILD_MODE","NX_DAEMON","SEMIO_SCRIPT_STALL_MS"])expect(env[dropped],dropped).toBeUndefined();
});

test("a real child is measured on an exact clock against the oracle's clock, its physical footprint covers what it allocated, and its output is read; a timeout and a cancellation stop the whole tree",async()=>{
 const folder=temporary(),script=join(folder,"child.mjs");
 writeFileSync(script,'const keep=Buffer.alloc(300*1024*1024,1);console.log("graph: 3 projects served from the memo of 2 s ago");const begun=Date.now();await new Promise(accept=>setTimeout(accept,2200));console.log("INFO: 2 processes: 1 internal, 1 worker.");console.log(`self ${Date.now()-begun} ${keep.length}`);');
 const lines:string[]=[],made=await acceptanceSpawnV1(process.execPath,[script],folder,{PATH:process.env.PATH??""},{timeoutMs:120000,onLine:(_at,line)=>{lines.push(line);}},new AbortController().signal);
 expect(made.exit).toBe(0);expect(made.killed).toBeNull();expect(made.clock).toEqual({kind:"exact",grainMs:0});
 expect(made.wallMs).toBeGreaterThanOrEqual(2200);expect(made.wallMs).toBeLessThan(30000);
 expect(made.observation).toMatchObject({executed:1,served:true});expect(lines.length).toBe(3);
 expect(made.peakMb).not.toBeNull();expect(made.peakMb!).toBeGreaterThanOrEqual(250);expect(["footprint","resident"]).toContain(made.measure!);
 const pause="import time;time.sleep(1.5)",clock=python3Wall(pause),ours=await acceptanceSpawnV1("python3",["-I","-c",pause],folder,{PATH:process.env.PATH??""},{timeoutMs:60000,onLine:()=>{}},new AbortController().signal);
 expect(ours.wallMs).toBeGreaterThanOrEqual(1500);expect(Math.abs(ours.wallMs-clock)).toBeLessThan(1500);
 const slow=await acceptanceSpawnV1(process.execPath,["-e","setTimeout(()=>{},60000)"],folder,{PATH:process.env.PATH??""},{timeoutMs:600,onLine:()=>{}},new AbortController().signal);
 expect([slow.killed,slow.exit]).toEqual(["timeout",null]);expect(slow.wallMs).toBeLessThan(20000);
 const controller=new AbortController(),pending=acceptanceSpawnV1(process.execPath,["-e","setTimeout(()=>{},60000)"],folder,{PATH:process.env.PATH??""},{timeoutMs:120000,onLine:()=>{}},controller.signal);
 setTimeout(()=>controller.abort(Error("stop")),500);
 const cut=await pending;
 expect([cut.killed,cut.exit]).toEqual(["abort",null]);
 await expect(acceptanceSpawnV1(process.execPath,["-e",""],folder,{},{timeoutMs:1000,onLine:()=>{}},controller.signal)).rejects.toThrow("stop");
},120000);
test("real files and a real process: the child sees the peer edit while it runs and not before or after, the edit run sees the edit and the revert does not, every file ends byte-exact and the journal is gone",async()=>{
 const folder=temporary(),checkout=join(folder,"checkout"),stub=join(folder,"stub.mjs"),lines:string[]=[];
 const peerFiles=[...set.peer.unrelated.map(row=>row.file)],productFile=set.products[0]!.edit.file;
 for(const file of [...peerFiles,productFile]){mkdirSync(dirname(join(checkout,file)),{recursive:true});writeFileSync(join(checkout,file),`original ${file}\n`);}
 writeFileSync(stub,'import {readFileSync} from "node:fs";const [,,root,file]=process.argv;console.log("graph: 3 projects served from the memo of 2 s ago");await new Promise(accept=>setTimeout(accept,500));console.log("[stub] edited="+readFileSync(root+"/"+file,"utf8").includes("[acceptance]"));console.log("INFO: 1 process: 1 internal.");');
 const journalPath=join(checkout,policy.journalDirectory,policy.journalFile),at=(path:string)=>join(checkout,path);
 const real:AcceptancePortsV1={
  run:(request,signal)=>acceptanceSpawnV1(process.execPath,[stub,checkout,request.scenario.includes("peer")?peerFiles[0]!:productFile],checkout,{PATH:process.env.PATH??""},{timeoutMs:60000,onLine:(time,line)=>{lines.push(`${request.role}:${line}`);request.onLine(time,line);}},signal),
  files:{read:async path=>existsSync(at(path))?readFileSync(at(path)):null,write:async(path,bytes)=>{writeFileSync(at(path),bytes);}},
  journal:{read:async()=>existsSync(journalPath)?JSON.parse(readFileSync(journalPath,"utf8")).rows:[],write:async rows=>{mkdirSync(dirname(journalPath),{recursive:true});writeFileSync(journalPath,JSON.stringify({version:1,rows}));},clear:async()=>{rmSync(journalPath,{force:true});}},
  host:()=>({loadPerCore:null}),nonce:()=>"0123456789ab",now:()=>performance.now(),check:()=>{},progress:async()=>{},note:()=>{},signal:new AbortController().signal
 };
 const run=await acceptanceRunV1(set,[scenario("course-peer-unrelated"),scenario("course-edit")],real);
 expect(run.outcomes.map(row=>[row.scenario,row.status,row.restored,row.landed])).toEqual([["course-peer-unrelated","ran",true,true],["course-edit","ran",true,null]]);
 expect(lines.filter(line=>line.includes("[stub]"))).toEqual(["prime:[stub] edited=false","prime:[stub] edited=false","control:[stub] edited=false","peer:[stub] edited=true","follow:[stub] edited=false","edit:[stub] edited=true","revert:[stub] edited=false"]);
 for(const file of [...peerFiles,productFile])expect(readFileSync(join(checkout,file),"utf8")).toBe(`original ${file}\n`);
 expect(existsSync(journalPath)).toBe(false);
 for(const record of run.records){expect(record.exit).toBe(0);expect(record.wallMs).toBeGreaterThanOrEqual(500);expect(record.executed).toBe(0);expect(record.served).toBe(true);expect(record.clock.kind).toBe("exact");}
 expect(run.records.map(record=>`${record.scenario}:${record.role}`)).toEqual(["course-peer-unrelated:prime","course-peer-unrelated:prime","course-peer-unrelated:control","course-peer-unrelated:peer","course-peer-unrelated:follow","course-edit:edit","course-edit:revert"]);
 const rows=acceptanceJudgeV1(set,["course-peer-unrelated","course-edit"],run.records,run.outcomes);
 expect(rows.filter(row=>row.kind==="invariant").every(row=>row.status==="pass")).toBe(true);
 expect(rows.filter(row=>row.id.endsWith("executed")).every(row=>row.status==="pass")).toBe(true);
},120000);
function python3Wall(code:string):number{
 const run=spawnSync("python3",["-I","-c","import subprocess,sys,time\nbegun=time.perf_counter()\nsubprocess.run(['python3','-I','-c',sys.argv[1]],check=True)\nprint((time.perf_counter()-begun)*1000)",code],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 return Number(run.stdout);
}

test("the nightly workflow runs the acceptance per product on a pinned macOS runner by schedule and by hand, in place, fails the blocking row on a breach, and keeps the reports",()=>{
 const path=join(root,".github","workflows","acceptance.yml");
 expect(existsSync(path)).toBe(true);
 const workflow:any=parseYaml(readFileSync(path,"utf8"),{strict:true,uniqueKeys:true});
 expect(workflow.on.workflow_dispatch).toBeDefined();
 expect(workflow.on.schedule).toHaveLength(1);expect(workflow.on.schedule[0].cron).toMatch(/^\d{1,2} \d{1,2} \* \* \*$/u);
 expect(workflow.on.pull_request).toBeUndefined();expect(workflow.on.push).toBeUndefined();
 expect(workflow.permissions).toEqual({contents:"read"});
 const job=workflow.jobs.acceptance,rows:any[]=job.strategy.matrix.include;
 expect(job.strategy["fail-fast"]).toBe(false);
 expect(String(job["runs-on"])).toContain("macos-26");expect(String(job["runs-on"])).not.toMatch(/latest/u);
 expect(rows.map(row=>row.product).sort()).toEqual(set.products.map(row=>row.id).sort());
 expect(rows.find(row=>row.product==="course").advisory).toBe(false);
 for(const row of rows){expect(row.minutes).toBeGreaterThanOrEqual(90);expect(row.minutes).toBeLessThanOrEqual(360);expect(Number(row.budget)).toBeLessThan(row.minutes*60000);expect(typeof row.advisory).toBe("boolean");}
 expect(job["continue-on-error"]).toContain("matrix.advisory");expect(job["timeout-minutes"]).toContain("matrix.minutes");
 const steps:any[]=job.steps,measure=steps.find(step=>String(step.run??"").includes("repo bench acceptance"));
 expect(measure).toBeDefined();expect(measure["continue-on-error"]).toBeUndefined();
 for(const flag of ["--in-place","--file=","--markdown=","--scenario="])expect(measure.run,flag).toContain(flag);
 expect(measure.env.SCENARIO).toBe("product:${{ matrix.product }}");
 const summary=steps.find(step=>String(step.run??"").includes("ACCEPTANCE_MARKDOWN")&&String(step.run).includes("GITHUB_STEP_SUMMARY")),upload=steps.find(step=>String(step.uses??"").startsWith("actions/upload-artifact@"));
 expect(summary.if).toContain("!cancelled()");expect(upload.if).toContain("!cancelled()");expect(upload.with.name).toBe("acceptance-macos-${{ matrix.product }}");
 for(const step of steps.filter(row=>row.uses&&!String(row.uses).startsWith("./")))expect(String(step.uses),step.uses).toMatch(/@[0-9a-f]{40}( |$)/u);
 expect(steps.findIndex(step=>step===measure)).toBeGreaterThan(steps.findIndex(step=>String(step.run??"").includes("repo tools setup")));
 for(const step of steps.filter(row=>/^repo (?:tools setup|dependencies sync)\b/mu.test(String(row.run??""))))expect(step["timeout-minutes"],step.name).toBeLessThanOrEqual(60);
 expect(workflow.env.SEMIO_LANGUAGE).toBe("en");
});

test("the files of the module stay far below the path limit, so a checkout on Windows can hold them",()=>{
 const base=join(import.meta.dir,".."),walk=(folder:string):string[]=>readdirSync(folder,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?walk(join(folder,entry.name)):[join(folder,entry.name)]);
 const longest=Math.max(...walk(base).map(path=>[...relative(root,path)].length));
 expect(longest).toBeLessThan(200);
});
