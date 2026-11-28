
const CROWD=Array.from({length:16},(_,at)=>`g1-m${String(at+1).padStart(2,"0")}`);
const crowdedFacts=():FleetFactsV1=>{
 const base=plain(cases.find(row=>row.id==="warden-reported-alarm")!.facts),metrics=definitions.Policy.const.rates.metrics,why={code:"no-headroom",en:"No headroom: every slot of this machine is taken by its own builds.",de:"Kein Spielraum: jeder Platz dieser Maschine ist von den eigenen Builds belegt."};
 const refusal={code:"stage-failed",en:"Stage workspace failed (exit status 1); retrying with back-off.",de:"Stufe workspace schlug fehl (Exit-Status 1); neuer Versuch mit Wartezeit."};
 const sources=["leases","capacity","bazel","storage","sealed","generation","scheduler","fleet","workers","memory","cas","offload","rates"].map(id=>id==="storage"?{id,ok:false,milliseconds:5000,error:"no answer within 5 s ".repeat(30)}:{id,ok:true,milliseconds:120}) as FleetFactsV1["sources"];
 const alarm=alarmOf("cancel",{en:"[warden] cancelled remote-unit pid 4242 (bun ".repeat(12),de:"[Wächter] abgebrochen remote-unit pid 4242 (bun ".repeat(12)});
 return {...base,sources,
  workers:{offlineSeconds:600,rows:CROWD.map((origin,at)=>({...workerRow(origin,at%3===0?"working":"ready"),platform:"macos",stage:"workspace",stageIndex:5,stageCount:5,attempts:at,refusal:at%2===0?refusal:null,tree:{reference:"g1-server",files:14,onlyThere:14,onlyHere:0,stale:true,verdict:"diverged"},recent:{hours:12,ok:4,failed:20,stalled:0,timedOut:0,seconds:13949},hostName:{state:"duplicate",label:"mac-mini",others:["g1-m02"]}}))} as never,
  memory:{local:null,origins:Object.fromEntries(CROWD.map(origin=>[origin,memoryOf(origin,stateOf({level:"guard",lastAlarms:[alarm],overhead:{tickMillisecondsP95:400,cpuBasisPoints:900,residentMegabytes:111}},{swapUsedMegabytes:6000}))])) as never},
  rates:{windowsSeconds:[900,3600,86400],origins:Object.fromEntries(CROWD.map((origin,at)=>[origin,Object.fromEntries(metrics.map((id,index)=>[id,[at+index+1,(at+index+1)*4,(at+index+1)*96]]))]))},
  fleet:{...base.fleet!,treeCode:"0123456789abcdef",machines:CROWD.map((origin,at)=>({origin,platform:"macos-arm64-0123456789abcdef",pressure:at%4===0?"warn":"normal",load:3.5,cores:8,running:1,queued:0,headroom:at%2===0,ageSeconds:5,worker:at%2===0?"idle":"refusing",offloadedIn:100+at,offloadedOut:200+at,refused:at,cacheHit:1000,cacheMiss:2000,cacheSkipped:30,cacheSkipReasons:{"spool-full":20,"read-only":10},...at%2===0?{}:{why},...at===3?{code:"ffffffffffffffff"}:{}})) as never},
 } as FleetFactsV1;
};
const crowdReaders=(facts:FleetFactsV1):FleetReadersV1=>({leases:async()=>({}),capacity:async()=>({}),bazel:async()=>({}),storage:async()=>({}),sealed:async()=>({}),generation:async()=>({}),scheduler:async()=>({}),fleet:async()=>({fleet:facts.fleet}),workers:async()=>({workers:facts.workers}),memory:async()=>({memory:facts.memory}),cas:async()=>({}),offload:async()=>({offload:facts.offload}),rates:async()=>({rates:facts.rates})});
const sectionNames=definitions.Policy.const.summary.sections;

test("size law: the default summary of a 16 origin fleet in which every origin is anomalous stays under 16 KB as JSON and as text in both languages, and is valid against its closed schema while the detail is several times larger",()=>{
 const facts=crowdedFacts();expect(validateFleetFactsV1(facts)).toEqual([]);
 for(const language of ["en","de"] as const){
  const detail=summarizeFleetV1(facts,language);expect(validateFleetReportV1(detail)).toEqual([]);
  const summary=summarizeFleetReportV1(detail),json=renderFleetJsonV1(summary);
  expect(validateFleetSummaryV1(summary)).toEqual([]);expect(validateFleetSummaryV1(JSON.parse(json))).toEqual([]);expect(JSON.parse(json)).toEqual(plain(summary));
  expect(Buffer.byteLength(json),`json ${language}`).toBeLessThan(16*1024);
  const text=formatFleetSummaryV1(language,summary).join("\n");expect(Buffer.byteLength(text),`text ${language}`).toBeLessThan(16*1024);
  expect(JSON.stringify(detail,null,2).length).toBeGreaterThan(4*json.length);
  expect(Object.keys(summary.rates!.origins)).toEqual(CROWD);for(const metrics of Object.values(summary.rates!.origins as Record<string,Record<string,number[]>>))expect(Object.keys(metrics).length).toBeLessThanOrEqual(definitions.Policy.const.summary.maximumOriginMetrics);
  expect(summary.findings.every((row:{count:number;subjects:string[];more:number})=>row.subjects.length+row.more<=row.count)).toBe(true);
  expect(summary.findings.length).toBeLessThan(detail.findings.length);
 }
});

test("detail parity: the summary is derived from the detail alone; totals, counts, rates, findings and workers agree on every corpus case, the crowded fleet and seeded cases",()=>{
 const reports=[...cases.map(row=>row.report as FleetReport),...python(5,20).map(row=>row.report as FleetReport),summarizeFleetV1(crowdedFacts(),"en")];
 for(const detail of reports){
  const summary=summarizeFleetReportV1(detail,20);expect(validateFleetSummaryV1(summary)).toEqual([]);
  expect(summary.summary).toEqual(detail.summary);expect(summary.runs.count).toBe(detail.runs.length);expect(summary.memory.grants).toBe(detail.memory.grants);
  expect(summary.findings.reduce((sum:number,row:{count:number})=>sum+row.count,0)).toBe(detail.findings.length);
  for(const row of summary.findings as {id:string;severity:string;subjects:string[];count:number}[]){const own=detail.findings.filter(finding=>finding.id===row.id&&finding.severity===row.severity);expect(own.length).toBe(row.count);for(const subject of row.subjects)expect(own.some(finding=>finding.subject===subject)).toBe(true);}
  if(detail.workers!==undefined){
   const rows=detail.workers.rows as {origin:string}[],shown=[...summary.workers.rows.map((row:{origin:string})=>row.origin),...summary.workers.healthy];
   expect(shown.sort()).toEqual(rows.map(row=>row.origin).sort());expect(summary.workers.counts).toEqual(detail.workers.counts);
   for(const row of summary.workers.rows as {origin:string;liveness:string;attempts:number}[]){const own=rows.find(candidate=>candidate.origin===row.origin) as unknown as {liveness:string;attempts:number};expect([row.liveness,row.attempts]).toEqual([own.liveness,own.attempts]);}
  }else expect(summary.workers).toBeUndefined();
  if(detail.fleet!==undefined){
   const fleet=detail.fleet as {machines:{origin:string}[];offloadedIn:number;refused:number};
   expect([summary.fleet.machines,summary.fleet.offloadedIn,summary.fleet.refused]).toEqual([fleet.machines.length,fleet.offloadedIn,fleet.refused]);
   for(const row of summary.fleet.attention as {origin:string}[])expect(fleet.machines.some(machine=>machine.origin===row.origin)).toBe(true);
  }else expect(summary.fleet).toBeUndefined();
  if(detail.rates!==undefined){
   const rates=detail.rates as {fleet:Record<string,number[]>;origins:Record<string,Record<string,number[]>>},anomalous=new Set<string>(definitions.Policy.const.summary.anomalousMetrics);
   const totals=Object.values(rates.fleet).filter(values=>values.some(value=>value>0)).length;
   expect(Object.keys(summary.rates.fleet).length).toBe(Math.min(totals,definitions.Policy.const.summary.maximumFleetMetrics));
   for(const [id,values] of Object.entries(summary.rates.fleet as Record<string,number[]>))expect(values).toEqual(rates.fleet[id]);
   let wanted=totals-Object.keys(summary.rates.fleet).length;
   for(const [origin,metrics] of Object.entries(rates.origins)){const rows=Object.entries(metrics).filter(([id,values])=>anomalous.has(id)&&values[0]!>0),shown=summary.rates.origins[origin]??{};expect(Object.keys(shown).length).toBe(Math.min(rows.length,definitions.Policy.const.summary.maximumOriginMetrics));wanted+=rows.length-Object.keys(shown).length;for(const [id,values] of Object.entries(shown as Record<string,number[]>))expect(values).toEqual(metrics[id]);}
   expect(summary.rates.omitted).toBe(wanted);
  }else expect(summary.rates).toBeUndefined();
  for(const name of sectionNames){const selection=selectFleetSectionsV1(detail,[name]);expect(validateFleetSelectionV1(selection)).toEqual([]);expect((selection.sections as Record<string,unknown>)[name]).toEqual((detail as unknown as Record<string,unknown>)[name]);}
 }
});

test("origin filter and section gating: one origin keeps only its worker, machine, counters and findings and still validates; the sections of the text partition the full text; an unknown origin or section is refused in both languages",()=>{
 const detail=summarizeFleetV1(crowdedFacts(),"en"),one=filterFleetReportV1(detail,"g1-m03");
 expect(validateFleetReportV1(one)).toEqual([]);expect((one.workers!.rows as {origin:string}[]).map(row=>row.origin)).toEqual(["g1-m03"]);expect((one.fleet!.machines as {origin:string}[]).map(row=>row.origin)).toEqual(["g1-m03"]);expect(Object.keys(one.rates!.origins)).toEqual(["g1-m03"]);
 expect(one.findings.length).toBeGreaterThan(0);expect(one.findings.every(row=>row.subject==="g1-m03"||row.values.origin==="g1-m03")).toBe(true);expect(fleetOriginsV1(detail)).toEqual(CROWD);
 const selection=selectFleetSectionsV1(one,["workers","findings"],"g1-m03");expect(validateFleetSelectionV1(selection)).toEqual([]);expect(Object.keys(selection.sections)).toEqual(["workers","findings"]);expect(selection.origin).toBe("g1-m03");
 expect(()=>filterFleetReportV1(detail,"nope")).toThrow(/Unknown origin nope[\s\S]*Unbekannter Origin nope/u);
 for(const language of ["en","de"] as const){
  const full=formatFleetReportV1(language,detail,20),parts=sectionNames.flatMap(name=>formatFleetReportV1(language,detail,20,new Set([name])));
  expect([...parts].sort()).toEqual([...full].sort());
  expect(formatFleetReportV1(language,detail,20,new Set(["workers"])).every(line=>/^ {2}g1-m|^(Workers|Worker):/u.test(line))).toBe(true);
 }
});

test("the command prints the summary by default and the detail, one origin or one section on request, always schema-validated, in both languages",async()=>{
 const facts=crowdedFacts(),readers=crowdReaders(facts),run=async(args:string[],language="en")=>{const lines:string[]=[];const code=await runFleetCliV1("/w",args,{SEMIO_LANGUAGE:language},line=>lines.push(line),{readers,now:()=>NOW});return {code,lines};};
 const summary=await run(["status","--json"]);expect(summary.lines.length).toBe(1);const parsed=JSON.parse(summary.lines[0]!);expect(parsed.view).toBe("summary");expect(validateFleetSummaryV1(parsed)).toEqual([]);expect(Buffer.byteLength(summary.lines[0]!)).toBeLessThan(16*1024);
 const detail=await run(["status","--json","--detail"]);expect(validateFleetReportV1(JSON.parse(detail.lines[0]!))).toEqual([]);expect(detail.lines[0]!.length).toBeGreaterThan(4*summary.lines[0]!.length);
 const origin=await run(["status","--json","--origin=g1-m05"]);const selected=JSON.parse(origin.lines[0]!);expect(validateFleetSelectionV1(selected)).toEqual([]);expect(selected.origin).toBe("g1-m05");expect(selected.sections.workers.rows.map((row:{origin:string})=>row.origin)).toEqual(["g1-m05"]);expect(Object.keys(selected.sections)).toEqual(sectionNames.filter(name=>selected.sections[name]!==undefined));
 const section=await run(["status","--json","--section=workers"]);const only=JSON.parse(section.lines[0]!);expect(Object.keys(only.sections)).toEqual(["workers"]);expect(only.sections.workers.rows.length).toBe(16);expect(only.origin).toBeUndefined();
 const text=await run(["status"]);expect(text.lines[0]).toMatch(/^Fleet /u);expect(text.lines.at(-1)).toMatch(/--detail/u);expect(text.lines.some(line=>/^ {2}g1-m01: .*stage workspace/u.test(line))).toBe(true);expect(text.lines.join("\n").length).toBeLessThan(16*1024);
 const german=await run(["status"],"de");expect(german.lines[0]).toMatch(/^Flotte /u);expect(german.lines.at(-1)).toMatch(/Einzelheiten/u);
 expect((await run(["status","--detail","--section=workers"])).lines[0]).toMatch(/^Workers: /u);expect((await run(["status","--origin=g1-m02","--section=workers"],"de")).lines.filter(line=>line.startsWith("  g1-m")).length).toBe(1);
 for(const bad of [["status","--origin="],["status","--detail","--detail"],["status","--section=workers","--section=rates"],["status","--origin=a","--origin=b"]])await expect(run(bad)).rejects.toThrow(/Usage|Verwendung/u);
 await expect(run(["status","--section=nope"])).rejects.toThrow(/Unknown section nope[\s\S]*Unbekannter Abschnitt nope/u);
 await expect(run(["status","--origin=nope"])).rejects.toThrow(/Unknown origin nope[\s\S]*Unbekannter Origin nope/u);
 expect(fleetArgumentsV1(["status","--detail","--origin=g1-m01","--section=rates","--limit=3","--json"])).toEqual({json:true,limit:3,detail:true,origin:"g1-m01",section:"rates"});
});
