/** 🐧️ Contract: the machine a fleet read judges is the machine, never the memory budget of the process that asks. On the Linux worker g1-m04 (15848 MB, MemAvailable 7.8 to 10.5 GB in 12 tries) the graph service lives in its own `semio-service-*` cgroup with a hard limit; `process.constrainedMemory()` there answered that limit, the read took it for the host and printed `it can give about 0 MB per project ... infer 0 (0 per wave)` in 7 of 7 tries. Only a limit above the process's own groups (the user slice, the session) bounds the machine; a structural limit lowers the projects per wave before it lowers the grant; the free memory of the moment decides nothing; the plan line prints the readings.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fleetAdmittedMegabytesV1,fleetGrantMegabytesV1,inferableProjectsV1,wantedPolicyV1,wantedTextV1} from "../../🟦️.ts";
import {ownerGrantV1,ownerCostPolicyV1} from "../../💰️cost/🟦️.ts";
import {fleetHostReadingV1} from "../../../../🧠️memory/🟨️.mjs";
import {cgroupMachineLimitBytesV1} from "../../../../🧱️job-limits/🐧️cgroup/🟦️.ts";

const MiB=1048576,python=Bun.which("python3")??Bun.which("python");
type Chain={own:string;files:Record<string,string>};
const ports=(chain:Chain)=>({root:"/sys/fs/cgroup",own:()=>chain.own,read:(path:string)=>{const text=chain.files[path.replace("/sys/fs/cgroup","")];if(text===undefined)throw Object.assign(Error("ENOENT"),{code:"ENOENT"});return text;}}) as never;
const G=1024*MiB;
const SERVICE_UNDER_USER_SLICE:Chain={own:"/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph",files:{
 "/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph/memory.max":String(4096*MiB),"/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph/memory.high":String(Math.floor(4096*MiB*0.9)),
 "/user.slice/user-1000.slice/user@1000.service/app.slice/memory.max":"max","/user.slice/user-1000.slice/user@1000.service/memory.max":"max","/user.slice/user-1000.slice/memory.max":"max","/user.slice/memory.max":"max","/memory.max":"max"}};
const SLICE_LOAD_LEAF:Chain={own:"/user.slice/user-1000.slice/user@1000.service/app.slice/semio-jobs/job-4711",files:{
 "/user.slice/user-1000.slice/user@1000.service/app.slice/semio-jobs/job-4711/memory.max":String(1024*MiB),"/user.slice/user-1000.slice/user@1000.service/app.slice/semio-jobs/memory.max":String(3*G),"/user.slice/user-1000.slice/user@1000.service/app.slice/memory.max":"max"}};
const USER_SLICE_8G:Chain={own:"/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph",files:{
 "/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph/memory.max":String(4096*MiB),"/user.slice/user-1000.slice/user.slice/memory.max":"max","/user.slice/user-1000.slice/memory.max":String(8*G),"/user.slice/user-1000.slice/user@1000.service/memory.max":"max"}};
const USER_HIGH_6G:Chain={own:"/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph",files:{
 "/user.slice/user-1000.slice/user@1000.service/app.slice/semio-service-graph/memory.max":String(3*G),"/user.slice/user-1000.slice/user@1000.service/memory.max":"max","/user.slice/user-1000.slice/user@1000.service/memory.high":String(6*G)}};
const UNLIMITED:Chain={own:"/user.slice/user-1000.slice/session-2.scope",files:{"/user.slice/user-1000.slice/session-2.scope/memory.max":"max"}};

const none={root:"/nope",own:()=>{throw Error("no cgroup");},read:()=>{throw Error("no cgroup");}} as never;
const read=(chain:Chain|undefined,total:number,availableKiB:number|null)=>fleetHostReadingV1({ports:chain===undefined?none:ports(chain),total:()=>total*MiB,environment:{},available:()=>availableKiB===null?undefined:availableKiB*1024});
const decide=(hostMegabytes:number)=>{const reading={hostMegabytes},admitted=fleetAdmittedMegabytesV1(reading),fit=inferableProjectsV1(admitted),share=fleetGrantMegabytesV1(reading),grant=ownerGrantV1(share,wantedPolicyV1);return{admitted,wave:fit.waveProjects,projects:fit.projects,perProject:Math.round(grant.managedMegabytes*ownerCostPolicyV1.footprintFactor)};};
const oracle=(host:number)=>{
 const run=spawnSync(python!,["-I","-c","import json,sys\nh=json.load(sys.stdin)['host']\nhalf=h//2\nadmitted=max(0,min(half,h-2048))\nwave=min(48,max(0,int((admitted-2200)//((7200-2200)/48))))\nwave=0 if wave<8 else wave\nshare=min(half,9399) if wave>0 else admitted\nmanaged=int(max(0,int(share-3800))/2/1.25)\nprint(json.dumps({'admitted':admitted,'wave':wave,'projects':wave*4,'perProject':managed*2}))"],{input:JSON.stringify({host}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {admitted:number;wave:number;projects:number;perProject:number};
};

type Row={name:string;chain:Chain|undefined;total:number;availableKiB:number|null;host:number;limit:number|null;wave:number};
const rows:Row[]=[
 {name:"g1-m04 try 6 start: 15848 MB, MemAvailable 10473644 kB, service group limited to 4096 MB (serviceLimitMegabytes)",chain:SERVICE_UNDER_USER_SLICE,total:15848,availableKiB:10473644,host:15848,limit:null,wave:48},
 {name:"g1-m04 try 12 start: MemAvailable 10244952 kB",chain:SERVICE_UNDER_USER_SLICE,total:15848,availableKiB:10244952,host:15848,limit:null,wave:48},
 {name:"g1-m04 lowest sample of try 12: MemAvailable 7828948 kB",chain:SERVICE_UNDER_USER_SLICE,total:15848,availableKiB:7828948,host:15848,limit:null,wave:48},
 {name:"g1-m04 slice load leaf of 1024 MB inside a 3 GB jobs group",chain:SLICE_LOAD_LEAF,total:15848,availableKiB:9000000,host:15848,limit:null,wave:48},
 {name:"Linux worker whose user slice is capped at 8 GB",chain:USER_SLICE_8G,total:15848,availableKiB:9000000,host:8192,limit:8192,wave:18},
 {name:"Linux worker whose user service has memory.high 6 GB",chain:USER_HIGH_6G,total:15848,availableKiB:9000000,host:6144,limit:6144,wave:8},
 {name:"Linux worker in an unlimited session scope",chain:UNLIMITED,total:15848,availableKiB:9000000,host:15848,limit:null,wave:48},
 {name:"macOS 16 GB Mac, idle (no cgroup)",chain:undefined,total:16384,availableKiB:11000000,host:16384,limit:null,wave:48},
 {name:"macOS 16 GB Mac, busy (no cgroup)",chain:undefined,total:16384,availableKiB:3000000,host:16384,limit:null,wave:48},
 {name:"Linux 16 GB worker, starved moment, no cgroup",chain:undefined,total:15848,availableKiB:900000,host:15848,limit:null,wave:48},
];

for(const row of rows)test("the reading of "+row.name+" is the machine and the decision equals the independent computation",()=>{
 const reading=read(row.chain,row.total,row.availableKiB);
 expect({host:reading.hostMegabytes,limit:reading.limitMegabytes,physical:reading.physicalMegabytes}).toEqual({host:row.host,limit:row.limit,physical:row.total});
 const got=decide(reading.hostMegabytes);
 expect(got.wave).toBe(row.wave);
 expect(got).toEqual(oracle(reading.hostMegabytes));
 if(reading.hostMegabytes>=15000){expect(got.perProject).toBeGreaterThanOrEqual(3000);expect(got.perProject).toBeLessThanOrEqual(3600);}
});

test("the fault: the host taken as the process's own 4096 MB group is refused with 0 MB per project and a wave of 0, exactly what g1-m04 printed 7 times",()=>{
 const faulty=decide(4096);expect(faulty.wave).toBe(0);expect(faulty.perProject).toBe(0);
 const fixed=decide(read(SERVICE_UNDER_USER_SLICE,15848,10473644).hostMegabytes);expect(fixed.wave).toBe(48);expect(fixed.perProject).toBeGreaterThan(3000);
});

test("law: a structural limit above the process lowers the projects per wave before it lowers the grant; the free memory of the moment lowers nothing",()=>{
 const capped=decide(read(USER_SLICE_8G,15848,9000000).hostMegabytes),open=decide(read(SERVICE_UNDER_USER_SLICE,15848,9000000).hostMegabytes);
 expect(capped.wave).toBeLessThan(open.wave);expect(capped.wave).toBeGreaterThan(0);
 const idle=read(SERVICE_UNDER_USER_SLICE,15848,11000000),starved=read(SERVICE_UNDER_USER_SLICE,15848,900000);
 expect(decide(idle.hostMegabytes)).toEqual(decide(starved.hostMegabytes));
});

test("the readings carry what the plan line prints, and the line says them in both languages",()=>{
 const reading=read(SERVICE_UNDER_USER_SLICE,15848,10473644);
 expect(reading.availableMegabytes).toBe(10228);
 const fields={host:reading.hostMegabytes,physical:reading.physicalMegabytes,limit:"none",available:reading.availableMegabytes+" MB"};
 expect(wantedTextV1("readings","en",fields)).toBe("graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit none), available 10228 MB");
 expect(wantedTextV1("readings","de",{...fields,limit:"keine"})).toBe("Graph: Speicherwerte: Maschine 15848 MB (physisch 15848 MB, cgroup-Grenze keine), verfügbar 10228 MB");
});

test("the cgroup walk skips every group this system creates and reads memory.high beside memory.max",()=>{
 expect(cgroupMachineLimitBytesV1(ports(SERVICE_UNDER_USER_SLICE))).toBeUndefined();
 expect(cgroupMachineLimitBytesV1(ports(SLICE_LOAD_LEAF))).toBeUndefined();
 expect(cgroupMachineLimitBytesV1(ports(USER_SLICE_8G))).toBe(8*G);
 expect(cgroupMachineLimitBytesV1(ports(USER_HIGH_6G))).toBe(6*G);
 expect(cgroupMachineLimitBytesV1(none)).toBeUndefined();
});

test("the first plan line of a fleet read is preceded by the readings it used, in order, and the grant of a 15848 MB machine is the share of the machine",async()=>{
 const {mkdtempSync,rmSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path"),{fleetReadStaleGenerationV1}=await import("../../../../🕸️graph/🤝️prefetch/🟦️.ts");
 const root=mkdtempSync(join(tmpdir(),"r30-"));
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr),before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES="15848";
 try{await fleetReadStaleGenerationV1({root,request:{generation:3,dirty:Array.from({length:600},(_,at)=>`p${String(at).padStart(5,"0")}`),roots:{},structural:false,implementation:"a".repeat(64)},invocation:{control:{signal:new AbortController().signal}} as never,language:"en",environment:{PATH:process.env.PATH,HOME:root,SEMIO_GRAPH_FLEET_WAIT_MILLISECONDS:"50"}});}catch{}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;rmSync(root,{recursive:true,force:true});}
 const text=lines.join(""),readings=text.indexOf("graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit "),plan=text.indexOf("graph: 600 stale projects");
 expect(readings,text).toBeGreaterThanOrEqual(0);expect(plan).toBeGreaterThan(readings);expect(text).toContain("this machine can infer 192 (48 per wave)");
});
