import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir,uptime} from "node:os";
import {join} from "node:path";
import {leaseRegistryFolderV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {cacheLedgerV1,cacheRootDirectoryV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {DOCTOR_POLICY_V1,validateDoctorV1,type DoctorFactsV1,type DoctorPolicyV1} from "../../🟦️.ts";
import {analyzeDoctorV1} from "../../🧮️analysis/🟦️.ts";
import {doctorPortsV1,type DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import {DoctorCancelledV1,collectDoctorFactsV1,runDoctorV1,selectDoctorProbesV1} from "../../🏃️runner/🟦️.ts";
import {DOCTOR_PROBES_V1,DOCTOR_REPAIRS_V1,type DoctorProbeContextV1,type DoctorProbeV1} from "../../🔎️probes/🟦️.ts";
import {toolPinDriftV1} from "../../🔎️probes/🔧️tools/🟦️.ts";
import {cachesProbeV1,deadPublicationsV1} from "../../🔎️probes/⚡️caches/🟦️.ts";
import {capacityProbeV1} from "../../🔎️probes/🎫️capacity/🟦️.ts";
import {dependenciesProbeV1} from "../../🔎️probes/📦️dependencies/🟦️.ts";
import {graphProbeV1,newestImplementationMillisecondsV1} from "../../🔎️probes/🕸️graph/🟦️.ts";
import {leasesProbeV1} from "../../🔎️probes/🔒️leases/🟦️.ts";
import {processesProbeV1} from "../../🔎️probes/🏃️processes/🟦️.ts";
import {sessionsProbeV1} from "../../🔎️probes/🪪️sessions/🟦️.ts";
import {storageProbeV1} from "../../🔎️probes/💾️storage/🟦️.ts";
import {toolsProbeV1} from "../../🔎️probes/🔧️tools/🟦️.ts";

const roots:string[]=[],children:{kill:()=>void}[]=[];
const fresh=():string=>{const root=mkdtempSync(join(tmpdir(),"dr-"));roots.push(root);return root;};
afterAll(()=>{for(const child of children)try{child.kill();}catch{}for(const root of roots)rmSync(root,{recursive:true,force:true});});
const dead=():number=>Bun.spawnSync(["true"]).pid;
const sleeper=():number=>{const child=Bun.spawn(["sleep","60"]);children.push(child);return child.pid;};
const hash=(text:string):string=>createHash("sha256").update(text).digest("hex");
const ports=(root:string,extra:Partial<DoctorPortsV1>={}):DoctorPortsV1=>doctorPortsV1(root,{signal:new AbortController().signal,environment:{...process.env,SEMIO_LANGUAGE:"en"},...extra});
const context=(over:Partial<DoctorProbeContextV1>={}):DoctorProbeContextV1=>({policy:DOCTOR_POLICY_V1,known:new Set(),libraryRoot:fresh(),deadlineMs:Date.now()+60000,...over});
const write=(path:string,text:string="x",ageSeconds=0):void=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,text);if(ageSeconds>0){const time=(Date.now()-ageSeconds*1000)/1000;utimesSync(path,time,time);}};
const partitionDirectory=(root:string,id:string):string=>join(root,...(budget.partitions as Record<string,{directory:string}>)[id]!.directory.split("/"));
const read=async(probe:DoctorProbeV1,port:DoctorPortsV1,ctx:DoctorProbeContextV1=context())=>{const result=await probe.read(port,ctx);if("unavailable" in result)return result;return result.sections;};

test("storage: receipts, backing files, mounts and publication residue are read without opening anything",async()=>{
 const root=fresh();
 write(join(partitionDirectory(root,"graph"),"image.json"),'{"version":1}');write(join(partitionDirectory(root,"graph"),"fixed.dmg"),"x".repeat(1000));mkdirSync(join(partitionDirectory(root,"graph"),"m"));
 write(join(partitionDirectory(root,"dependencies"),"image.json"),"{not json");
 write(join(partitionDirectory(root,"tools"),"image.json"),'{"version":1}');
 write(join(partitionDirectory(root,"products"),".publication","intent.json"),"{}",700);
 const sections=await read(storageProbeV1,ports(root,{platform:"darwin"})) as {storage:NonNullable<DoctorFactsV1["storage"]>};
 const byId=Object.fromEntries(sections.storage.partitions.map(row=>[row.id,row]));
 expect(Object.keys(byId).sort()).toEqual(Object.keys(budget.partitions).sort());
 expect(byId.graph).toMatchObject({state:"detached",backing:"present",imageBytes:1000,image:"ok",publication:"none"});
 expect(byId.dependencies).toMatchObject({image:"damaged",state:"absent",backing:"missing"});
 expect(byId.tools).toMatchObject({image:"ok",state:"absent",backing:"missing"});
 expect(byId.products).toMatchObject({publication:"intent",image:"none"});
 expect(byId.products!.publicationAgeSeconds!).toBeGreaterThanOrEqual(699);
 expect(byId.nx).toMatchObject({state:"absent",image:"none",backing:"none"});
 expect(sections.storage.hostFreeBytes!).toBeGreaterThan(0);
 expect(validateDoctorV1("StorageFacts",sections.storage)).toEqual([]);
 const ids=analyzeDoctorV1({version:1,atMs:Date.now(),platform:"darwin",probes:[],storage:sections.storage}).findings.map(row=>`${row.id}:${row.subject}`);
 expect(ids).toContain("storage.image-damaged:dependencies");
 expect(ids).toContain("storage.backing-missing:tools");
 expect(ids).toContain("storage.image-detached:graph");
 expect(ids).toContain("storage.publication-pending:products");
 const linux=await read(storageProbeV1,ports(root,{platform:"linux"})) as {storage:NonNullable<DoctorFactsV1["storage"]>};
 expect(linux.storage.partitions.find(row=>row.id==="graph")).toMatchObject({state:"directory",backing:"none"});
});

test("leases: holders, waiters with their blockers, dead rows and folders of vanished lease directories; the repair keeps everything alive",async()=>{
 const root=fresh(),registry=join(fresh(),"registry"),environment={...process.env,SEMIO_LEASE_REGISTRY_DIRECTORY:registry};
 const directory=join(root,".cache","b","l","m","data");mkdirSync(directory,{recursive:true});
 const holderPid=process.pid,waiterPid=sleeper(),deadPid=dead(),resource="cargo:registry:/w/home",now=Date.now();
 const entry=(pid:number,holds:unknown[],waits:unknown[],updated=now)=>({version:1,pid,startedAtMs:1000+pid,updatedAtMs:updated,command:"/w/script.ts repo cargo test",directory,holds,waits});
 const folder=leaseRegistryFolderV1(registry,directory);
 write(join(folder,"directory.json"),JSON.stringify({version:1,directory}));
 write(join(folder,`${holderPid}-${1000+holderPid}.json`),JSON.stringify(entry(holderPid,[{resource,mode:"exclusive",sinceMs:now-100000,task:0}],[])));
 write(join(folder,`${waiterPid}-${1000+waiterPid}.json`),JSON.stringify(entry(waiterPid,[],[{resource,mode:"exclusive",sinceMs:now-70000,task:0,queued:false}])));
 write(join(folder,`${deadPid}-${1000+deadPid}.json`),JSON.stringify(entry(deadPid,[],[])));
 const gone=join(fresh(),"vanished");rmSync(join(gone,".."),{recursive:true,force:true});
 const goneFolder=leaseRegistryFolderV1(registry,gone);write(join(goneFolder,"directory.json"),JSON.stringify({version:1,directory:gone}));write(join(goneFolder,`${deadPid}-1.json`),JSON.stringify({...entry(deadPid,[],[]),directory:gone}));
 const inside=join(root,"somewhere","missing"),insideFolder=leaseRegistryFolderV1(registry,inside);write(join(insideFolder,"directory.json"),JSON.stringify({version:1,directory:inside}));
 const queue=join(directory,`${hash(resource)}.queue`);write(join(queue,`${(now-60000).toString(36).padStart(10,"0")}-${deadPid.toString(36).padStart(7,"0")}-abcdef`),JSON.stringify({version:1,mode:"exclusive",pid:deadPid,sharedCohort:false}));
 const port=ports(root,{environment});
 const sections=await read(leasesProbeV1,port) as {leases:NonNullable<DoctorFactsV1["leases"]>};
 const facts=sections.leases;
 expect(validateDoctorV1("LeasesFacts",facts)).toEqual([]);
 expect(facts.holders.filter(row=>row.resource===resource).map(row=>[row.pid,row.mode,row.owner])).toEqual([[holderPid,"exclusive","cargo"]]);
 const waiter=facts.waiters.find(row=>row.pid===waiterPid)!;
 expect(waiter).toMatchObject({resource,mode:"exclusive",class:"cargo-coordination",owner:"cargo"});
 expect([...waiter.blockers].sort()).toEqual([holderPid,deadPid].sort());
 expect(facts.deadHolders).toEqual([deadPid]);
 expect(waiter.waitedSeconds).toBeGreaterThanOrEqual(69);
 expect(facts.registry).toMatchObject({folders:3,orphanFolders:1,liveEntries:2});
 expect(facts.registry.deadEntries).toBe(2);
 expect(facts.deadTickets).toBe(1);
 const repair=DOCTOR_REPAIRS_V1.get("leases-registry")!;
 expect(await repair.run(port,context(),true)).toMatchObject({removed:3});
 expect(existsSync(goneFolder)&&existsSync(join(folder,`${deadPid}-${1000+deadPid}.json`))).toBe(true);
 const done=await repair.run(port,context(),false);
 expect(done.removed).toBe(3);
 expect(existsSync(goneFolder)).toBe(false);
 expect(existsSync(join(folder,`${deadPid}-${1000+deadPid}.json`))).toBe(false);
 expect(existsSync(join(folder,`${holderPid}-${1000+holderPid}.json`))&&existsSync(join(folder,`${waiterPid}-${1000+waiterPid}.json`))).toBe(true);
 expect(existsSync(insideFolder)).toBe(true);
 expect(await repair.run(port,context(),false)).toMatchObject({removed:0,status:"nothing"});
 const missing=await leasesProbeV1.read(ports(fresh(),{environment}),context());
 expect("unavailable" in missing).toBe(true);
});

test("caches: only publications of processes that do not exist and were quiet are found and removed",async()=>{
 const root=fresh(),base=join(partitionDirectory(root,"graph"),"m","inference","selections");
 const gone=dead(),goneFresh=dead(),alive=process.pid,outside=fresh();
 const make=(scope:string,name:string,age:number)=>{write(join(base,scope,"inference",name,"a","b.bin"),"y".repeat(5000),age);const time=(Date.now()-age*1000)/1000;utimesSync(join(base,scope,"inference",name),time,time);};
 make("s1",`p-${gone}`,300);make("s1",`p-${goneFresh}`,5);make("s2",`p-${alive}`,300);make("s2",`q-${gone}`,300);
 write(join(outside,"keep.txt"),"keep");symlinkSync(outside,join(base,"s1","inference",`p-${dead()}`));
 const escaped=fresh(),escapedPid=dead();write(join(escaped,`p-${escapedPid}`,"f.bin"),"e".repeat(3000),300);utimesSync(join(escaped,`p-${escapedPid}`),(Date.now()-300000)/1000,(Date.now()-300000)/1000);
 mkdirSync(join(base,"s4"),{recursive:true});symlinkSync(escaped,join(base,"s4","inference"));
 mkdirSync(join(root,".cache","zzz-undeclared"),{recursive:true});
 const ledger=cacheLedgerV1(),retirement=ledger.retirements[0]!;mkdirSync(join(cacheRootDirectoryV1(root,ledger,retirement.root),...retirement.directory.split("/")),{recursive:true});
 const port=ports(root);
 expect(deadPublicationsV1(port,DOCTOR_POLICY_V1.caches).map(row=>row.pid)).toEqual([gone]);
 const sections=await read(cachesProbeV1,port,context({policy:DOCTOR_POLICY_V1})) as {caches:NonNullable<DoctorFactsV1["caches"]>};
 expect(sections.caches).toMatchObject({deadPublications:1,deadPublicationPartial:false});
 expect(sections.caches.deadPublicationBytes).toBeGreaterThanOrEqual(5000);
 expect(sections.caches.retired).toContain(`${retirement.root}:${retirement.directory}`);
 expect(sections.caches.undeclared).toContain("native:zzz-undeclared");
 expect(validateDoctorV1("CachesFacts",sections.caches)).toEqual([]);
 const repair=DOCTOR_REPAIRS_V1.get("caches-dead-publications")!;
 expect(await repair.run(port,context(),true)).toMatchObject({removed:1});
 expect(existsSync(join(base,"s1","inference",`p-${gone}`))).toBe(true);
 const done=await repair.run(port,context(),false);
 expect(done.removed).toBe(1);expect(done.bytes).toBeGreaterThanOrEqual(5000);
 expect(existsSync(join(base,"s1","inference",`p-${gone}`))).toBe(false);
 for(const kept of [join(base,"s1","inference",`p-${goneFresh}`),join(base,"s2","inference",`p-${alive}`),join(base,"s2","inference",`q-${gone}`),join(outside,"keep.txt"),escaped])expect(existsSync(kept),kept).toBe(true);
 expect(readdirSync(escaped).length).toBe(1);
 expect(await repair.run(port,context(),false)).toMatchObject({removed:0,status:"nothing"});
 const aborted=new AbortController();aborted.abort();
 make("s3",`p-${dead()}`,300);
 await expect(repair.run(ports(root,{signal:aborted.signal}),context(),false)).rejects.toBeDefined();
});

test("graph: the pointer, its manifest and whether the implementation is newer are read from the generation store",async()=>{
 const root=fresh(),library=fresh(),mount=join(partitionDirectory(root,"graph"),"m"),store=join(mount,"generations");
 expect("unavailable" in await graphProbeV1.read(ports(root),context({libraryRoot:library}))).toBe(true);
 mkdirSync(mount,{recursive:true});
 const graph=async()=>(await read(graphProbeV1,ports(root),context({libraryRoot:library})) as {graph:NonNullable<DoctorFactsV1["graph"]>}).graph;
 expect((await graph()).pointer).toBe("absent");
 write(join(store,"current.json"),"{broken");expect((await graph()).pointer).toBe("damaged");
 write(join(store,"current.json"),'{"version":1,"generation":2,"manifest":null,"parent":null}',100);
 expect(await graph()).toMatchObject({pointer:"present",generation:2,manifest:"none",owners:null});
 const sha="a".repeat(64);write(join(store,"current.json"),`{"version":1,"generation":3,"manifest":"${sha}","parent":null}`,100);
 expect((await graph()).manifest).toBe("missing");
 write(join(store,"m",sha+".json"),"{broken");expect((await graph()).manifest).toBe("damaged");
 write(join(store,"m",sha+".json"),JSON.stringify({owners:{a:{},b:{},c:{}}}));
 const folder=DOCTOR_POLICY_V1.graph.implementationRoots[0]!.split("/");write(join(library,...folder,"x.ts"),"export {}",0);write(join(library,...folder,"🧪️tests","y.ts"),"export {}",0);
 const present=await graph();
 expect(present).toMatchObject({pointer:"present",generation:3,manifest:"present",owners:3});
 expect(present.implementationNewerSeconds!).toBeGreaterThanOrEqual(99);
 expect(validateDoctorV1("GraphFacts",present)).toEqual([]);
 const older=(Date.now()-3600*1000)/1000;utimesSync(join(library,...folder,"x.ts"),older,older);utimesSync(join(library,...folder,"🧪️tests","y.ts"),Date.now()/1000,Date.now()/1000);
 expect((await graph()).implementationNewerSeconds).toBe(0);
 expect(newestImplementationMillisecondsV1(fresh(),DOCTOR_POLICY_V1.graph)).toBeNull();
});

test("dependencies: pointers reach ready, unfinished or missing generations; superseded ones and newer scope inputs are counted",async()=>{
 const root=fresh(),mount=join(partitionDirectory(root,"dependencies"),"m");
 expect("unavailable" in await dependenciesProbeV1.read(ports(root),context())).toBe(true);
 mkdirSync(join(mount,"j"),{recursive:true});
 const section=async()=>(await read(dependenciesProbeV1,ports(root),context()) as {dependencies:NonNullable<DoctorFactsV1["dependencies"]>}).dependencies;
 expect(await section()).toMatchObject({pointers:"absent",scopes:[]});
 const generation=(seed:string):string=>hash(seed),published=Date.now()-3600*1000;
 const pointer=(scope:string,seed:string,bytes=1000)=>write(join(mount,"pointer",hash(scope).slice(0,16)+".pointer.json"),JSON.stringify({version:1,purpose:"dependency-generation-pointer",scope,generation:generation(seed),epoch:1,publishedAtMs:published,bytes,retired:[],sha256:hash(seed)}));
 pointer(".","root");pointer("✏️s","s");pointer("🎓️teaching","t",3000);
 write(join(mount,"j",generation("root").slice(0,20),"bindings.json"),"{}");
 mkdirSync(join(mount,"j",generation("s").slice(0,20)),{recursive:true});
 write(join(mount,"j",generation("old1").slice(0,20),"bindings.json"),"{}");write(join(mount,"j",generation("old2").slice(0,20),"bindings.json"),"{}");
 mkdirSync(join(mount,"j",generation("half").slice(0,20)),{recursive:true});const older=(Date.now()-7200*1000)/1000;utimesSync(join(mount,"j",generation("half").slice(0,20)),older,older);
 write(join(root,"bun.lock"),"lock");write(join(root,"✏️s","package.json"),"{}",7200);
 write(join(root,"🎓️teaching","package.json"),"{}");utimesSync(join(root,"🎓️teaching","package.json"),(published+1000)/1000,(published+1000)/1000);
 const facts=await section();
 expect(validateDoctorV1("DependenciesFacts",facts)).toEqual([]);
 const byScope=Object.fromEntries(facts.scopes.map(row=>[row.scope,row]));
 expect(byScope["."]).toMatchObject({directory:"ready",inputsNewer:"bun.lock"});
 expect(byScope["✏️s"]).toMatchObject({directory:"unready",inputsNewer:null});
 expect(byScope["🎓️teaching"]).toMatchObject({directory:"missing",inputsNewer:null});
 expect(facts).toMatchObject({pointers:"present",orphans:2,orphanEstimatedBytes:2*Math.round((1000+1000+3000)/3),unready:1});
 expect(facts.unreadyOldestSeconds!).toBeGreaterThanOrEqual(7199);
});

test("capacity: grants, waiters and the host sample come from the machine-wide gate directory",async()=>{
 const gate=fresh(),now=Date.now(),environment={...process.env,SEMIO_HOST_CAPACITY_DIRECTORY:gate};
 mkdirSync(join(gate,"grants"),{recursive:true});mkdirSync(join(gate,"queue"),{recursive:true});
 write(join(gate,"sample.json"),JSON.stringify({host:{version:1,atMs:now,platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes:9000,pressure:"warn",swapUsedMegabytes:5000,swapOutMegabytesPerSecond:0,load:3,openFiles:100,openFilesMaximum:1000,attachedImages:2},swappedOutMegabytes:0}));
 const bootAtMs=Math.round((now-uptime()*1000)/1000)*1000,grantId="grant-abcdefgh",waiterId="waiter-abcdefgh";
 write(join(gate,"grants",grantId+".json"),JSON.stringify({id:grantId,operation:"run",jobs:2,cpu:2,reservedMegabytes:4096,admittedAtMs:now-120000,observedMegabytes:512,pid:process.pid,bootAtMs}));
 const arrived=now-45000;
 write(join(gate,"queue",`${String(arrived).padStart(15,"0")}-${waiterId}.json`),JSON.stringify({request:{id:waiterId,operation:"cargo",priority:"batch",minimumJobs:1,preferredJobs:2,fixedMegabytes:3000,perJobMegabytes:500,cpuPerJob:1},arrivedAtMs:arrived,pid:process.pid}));
 const sections=await read(capacityProbeV1,ports(fresh(),{environment,now:()=>now})) as {host:NonNullable<DoctorFactsV1["host"]>;capacity:NonNullable<DoctorFactsV1["capacity"]>};
 expect(validateDoctorV1("HostFacts",sections.host)).toEqual([]);
 expect(validateDoctorV1("CapacityFacts",sections.capacity)).toEqual([]);
 expect(sections.host).toMatchObject({cores:8,availableMegabytes:9000,pressure:"warn",swapUsedMegabytes:5000,governor:"on"});
 expect(sections.host.floorMegabytes).toBeGreaterThan(0);
 expect(sections.capacity.grants).toEqual([{operation:"run",pid:process.pid,jobs:2,reservedMegabytes:4096,observedMegabytes:512,heldSeconds:120}]);
 expect(sections.capacity.waiters).toHaveLength(1);
 expect(sections.capacity.waiters[0]).toMatchObject({operation:"cargo",pid:process.pid,needMegabytes:3500,waitedSeconds:45});
 const off=await read(capacityProbeV1,ports(fresh(),{environment:{...environment,SEMIO_HOST_CAPACITY:"off"},now:()=>now})) as {host:NonNullable<DoctorFactsV1["host"]>};
 expect(off.host.governor).toBe("off");
});

test("tools: pins, missing tools and the probe budget through the registry when it exists",async()=>{
 expect(toolPinDriftV1("pinned","1.3.14","1.4.2")).toBe(true);
 expect(toolPinDriftV1("pinned","1.4.2","1.4")).toBe(false);
 expect(toolPinDriftV1("pinned","1.4","1.4.2")).toBe(false);
 expect(toolPinDriftV1("floating","1.0.0","2.0.0")).toBe(false);
 expect(toolPinDriftV1("pinned",null,"1.0.0")).toBe(false);
 expect(toolPinDriftV1("pinned","1.0.0","main")).toBe(false);
 const entry=(id:string,platforms:string[]=["darwin","linux","win32"],state="pinned")=>({id,platforms,pin:{state}});
 const registry={tools:[entry("repo"),entry("bun"),entry("gone"),entry("windows-only",["win32"]),entry("late")]};
 const versions:Record<string,string|null>={repo:"1.0.0",bun:"1.3.14",late:"9.9.9"};
 const api={TOOL_REGISTRY_V1:registry,toolRootV1:(directory:string)=>directory,resolveRegisteredToolV1:(tool:{id:string})=>({command:tool.id==="gone"?null:tool.id,pin:{version:tool.id==="bun"?"1.4.2":tool.id==="repo"?"1.0.0":tool.id==="late"?"9.9.9":null}}),probeToolV1:(tool:{id:string})=>versions[tool.id]??null};
 const optional=async(name:string)=>name==="tools"?api:undefined;
 const sections=await read(toolsProbeV1,ports(fresh(),{platform:"darwin",optional:optional as never}),context({deadlineMs:Date.now()+60000})) as {tools:NonNullable<DoctorFactsV1["tools"]>};
 expect(sections.tools.rows.map(row=>[row.id,row.state])).toEqual([["repo","ok"],["bun","drift"],["gone","missing"],["late","ok"]]);
 expect(validateDoctorV1("ToolsFacts",sections.tools)).toEqual([]);
 const late=await read(toolsProbeV1,ports(fresh(),{platform:"darwin",optional:optional as never}),context({deadlineMs:0})) as {tools:NonNullable<DoctorFactsV1["tools"]>};
 expect(late.tools.rows.every(row=>row.state==="unprobed")).toBe(true);
 expect("unavailable" in await toolsProbeV1.read(ports(fresh(),{optional:(async()=>undefined) as never}),context())).toBe(true);
});

test("sessions: counts through the session module, zero for a worktree without records, and the repair through the owner",async()=>{
 const root=fresh();mkdirSync(join(root,"store"));
 const rows=[{state:"live"},{state:"live"},{state:"idle"},{state:"gone"}];
 const calls:string[]=[];
 const core={agentSessionRegistryRootV1:()=>root,agentSessionWorktreeKeyV1:()=>"key",agentSessionStoreV1:(_root:string,key:string)=>({directory:join(root,key)}),listAgentSessionsV1:()=>rows,collectAgentSessionsV1:(_store:unknown,_reclaim:unknown,options:{dryRun?:boolean})=>{calls.push(options.dryRun?"dry":"real");return [{removed:!options.dryRun}];}};
 const build={agentSessionReclaimV1:()=>"reclaim"};
 const optional=async(name:string)=>name==="sessions-core"?core:name==="sessions-build"?build:undefined;
 const port=ports(root,{optional:optional as never});
 expect(await read(sessionsProbeV1,port)).toEqual({sessions:{live:0,idle:0,gone:0,damaged:0}});
 write(join(root,"key","a.json"),"{}");write(join(root,"key","b.json"),"{}");write(join(root,"key","c.json"),"{}");write(join(root,"key","d.json"),"{}");write(join(root,"key","e.json"),"{}");write(join(root,"key","collect.stamp"),"1");
 expect(await read(sessionsProbeV1,port)).toEqual({sessions:{live:2,idle:1,gone:1,damaged:1}});
 const repair=DOCTOR_REPAIRS_V1.get("agent-sessions")!;
 expect(await repair.run(port,context(),true)).toMatchObject({removed:1});
 expect(await repair.run(port,context(),false)).toMatchObject({removed:1});
 expect(calls).toEqual(["dry","real"]);
 expect("unavailable" in await sessionsProbeV1.read(ports(root,{optional:(async()=>undefined) as never}),context())).toBe(true);
 expect(await repair.run(ports(root,{optional:(async()=>undefined) as never}),context(),false)).toMatchObject({status:"unavailable"});
});

test("processes: matching processes, everything known and all descendants, never the doctor itself",async()=>{
 const table=[
  {pid:1,parent:0,ageSeconds:9999,rssMegabytes:1,cpuPercent:0,command:"/sbin/launchd"},
  {pid:10,parent:1,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"bun /w/script.ts repo run a"},
  {pid:11,parent:10,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"node worker.js"},
  {pid:12,parent:11,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"sleep 5"},
  {pid:20,parent:1,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"/usr/bin/python3 unrelated.py"},
  {pid:21,parent:20,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"child of unrelated"},
  {pid:30,parent:1,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"unrelated but known"},
  {pid:40,parent:1,ageSeconds:50,rssMegabytes:10,cpuPercent:1,command:"repo mcp stdio"},
  {pid:process.pid,parent:1,ageSeconds:5,rssMegabytes:10,cpuPercent:1,command:"bun /w/script.ts doctor"},
 ];
 const sections=await read(processesProbeV1,ports(fresh(),{processes:(async()=>table) as never}),context({known:new Set([30])})) as unknown as {processes:{pid:number}[]};
 expect(sections.processes.map(row=>row.pid)).toEqual([10,11,12,30]);
});

test("the runner reports a failing probe, cuts off a stalled one, runs probes in parallel and hands phase 2 the pids of phase 1",async()=>{
 const policy:DoctorPolicyV1={...DOCTOR_POLICY_V1,probe:{timeoutMilliseconds:500,timeoutOverrides:{caches:1200},parallelism:4}};
 const slow=(area:DoctorProbeV1["area"],ms:number):DoctorProbeV1=>({area,phase:1,read:async()=>{await new Promise(done=>setTimeout(done,ms));return {sections:{}};}});
 const seen:number[][]=[];
 const probes:DoctorProbeV1[]=[
  {area:"tools",phase:1,read:async()=>{throw Error("boom\nsecond line");}},
  {area:"storage",phase:1,read:()=>new Promise(()=>{})},
  slow("graph",250),slow("caches",800),
  {area:"leases",phase:1,read:async()=>({sections:{leases:{holders:[{resource:"r",hash:"a".repeat(64),mode:"exclusive",pid:77,heldSeconds:1,class:"c",owner:"o",command:"x"}],waiters:[],cycles:[],violations:[],registry:{folders:0,orphanFolders:0,deadEntries:0,liveEntries:0},deadTickets:0,deadHolders:[]}}})},
  {area:"processes",phase:2,read:async(_ports,ctx)=>{seen.push([...ctx.known]);return {unavailable:"nothing"};}},
 ];
 const started=Date.now(),facts=await collectDoctorFactsV1(ports(fresh()),{policy,probes});
 expect(Date.now()-started).toBeLessThan(1100);
 const row=Object.fromEntries(facts.probes.map(item=>[item.area,item]));
 expect(row.tools).toMatchObject({status:"failed",reason:"boom"});
 expect(row.storage!.status).toBe("timeout");
 expect(row.graph!.status).toBe("ok");
 expect(row.caches!.status).toBe("ok");
 expect(row.processes).toMatchObject({status:"unavailable",reason:"nothing"});
 expect(seen).toEqual([[77]]);
 expect(facts.probes.map(item=>item.area)).toEqual(["tools","storage","leases","graph","caches","processes"]);
 expect(validateDoctorV1("Facts",facts)).toEqual([]);
 const ids=analyzeDoctorV1(facts,{policy}).findings.map(item=>`${item.id}:${item.subject}`).sort();
 expect(ids).toEqual(["doctor.probe-failed:tools","doctor.probe-timeout:storage","doctor.probe-unavailable:processes"]);
});

test("the runner stops at once when cancelled and selects only the probes an area needs",async()=>{
 const controller=new AbortController();
 const probes:DoctorProbeV1[]=[{area:"tools",phase:1,read:async()=>{controller.abort();await new Promise(done=>setTimeout(done,50));return {sections:{}};}},{area:"storage",phase:1,read:async()=>({sections:{}})}];
 await expect(collectDoctorFactsV1(doctorPortsV1(fresh(),{signal:controller.signal}),{probes,policy:{...DOCTOR_POLICY_V1,probe:{timeoutMilliseconds:2000,timeoutOverrides:{},parallelism:1}}})).rejects.toBeInstanceOf(DoctorCancelledV1);
 const areas=(only:string[])=>selectDoctorProbesV1(DOCTOR_PROBES_V1,new Set(only as never)).map(probe=>probe.area);
 expect(areas(["storage"])).toEqual(["storage"]);
 expect(areas(["host"])).toEqual(["capacity"]);
 expect(areas(["processes"]).sort()).toEqual(["capacity","leases","processes"]);
 expect(selectDoctorProbesV1(DOCTOR_PROBES_V1,undefined)).toEqual(DOCTOR_PROBES_V1);
});

test("--fix end to end: a dry run changes nothing, the real run repairs, reads again and reports a clean state",async()=>{
 const root=fresh(),registry=join(fresh(),"registry"),environment={...process.env,SEMIO_LEASE_REGISTRY_DIRECTORY:registry,SEMIO_HOST_CAPACITY_DIRECTORY:fresh()};
 const base=join(partitionDirectory(root,"graph"),"m","inference","selections"),gone=dead();
 write(join(base,"s1","inference",`p-${gone}`,"f.bin"),"z".repeat(4000),300);const time=(Date.now()-300000)/1000;utimesSync(join(base,"s1","inference",`p-${gone}`),time,time);
 const directory=join(root,".cache","b","l","m","data");mkdirSync(directory,{recursive:true});
 const folder=leaseRegistryFolderV1(registry,directory);write(join(folder,"directory.json"),JSON.stringify({version:1,directory}));
 write(join(folder,`${gone}-1.json`),JSON.stringify({version:1,pid:gone,startedAtMs:1,updatedAtMs:Date.now(),command:"x",directory,holds:[],waits:[]}));
 const only=new Set(["caches","leases"] as const);
 const port=ports(root,{environment});
 const before=await runDoctorV1(port,{only:only as never,fix:false});
 expect(before.report.findings.map(row=>row.id)).toEqual(expect.arrayContaining(["caches.dead-publication","leases.registry-stale"]));
 const dry=await runDoctorV1(port,{only:only as never,fix:true,dryRun:true});
 expect(dry.report.repairs.map(row=>[row.id,row.dryRun,row.status,row.removed>0])).toEqual([["leases-registry",true,"done",true],["caches-dead-publications",true,"done",true]]);
 expect(existsSync(join(base,"s1","inference",`p-${gone}`))).toBe(true);
 const real=await runDoctorV1(port,{only:only as never,fix:true});
 expect(real.report.repairs.map(row=>[row.id,row.dryRun,row.status])).toEqual([["leases-registry",false,"done"],["caches-dead-publications",false,"done"]]);
 expect(existsSync(join(base,"s1","inference",`p-${gone}`))).toBe(false);
 expect(real.report.findings.map(row=>row.id)).not.toContain("caches.dead-publication");
 expect(real.report.findings.map(row=>row.id)).not.toContain("leases.registry-stale");
 expect(validateDoctorV1("Report",real.report)).toEqual([]);
 const again=await runDoctorV1(port,{only:only as never,fix:true});
 expect(again.report.repairs).toEqual([]);
});

test("every probe is declared once, in contract order, and every area but doctor and host has a probe",()=>{
 const areas=DOCTOR_PROBES_V1.map(probe=>probe.area);
 expect(new Set(areas).size).toBe(areas.length);
 expect(areas.sort()).toEqual(["tools","storage","leases","capacity","sessions","graph","dependencies","caches","processes"].sort());
 expect(DOCTOR_PROBES_V1.find(probe=>probe.area==="processes")!.phase).toBe(2);
});
