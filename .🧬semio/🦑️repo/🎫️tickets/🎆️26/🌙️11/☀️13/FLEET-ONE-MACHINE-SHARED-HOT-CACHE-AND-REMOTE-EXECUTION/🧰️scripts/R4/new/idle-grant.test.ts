/** 🍏️ Contract: a 16 GB worker that is idle gives each project about 3 300 to 3 500 MB and infers 48 projects per wave, on macOS (read from `vm_stat`) and on Linux (read from `MemAvailable`); a busy moment lowers the projects per wave first and leaves the grant, and only a machine that cannot run the smallest wave has a grant of 0 MB; "too heavy" names only projects whose measured cost exceeds the grant. The numbers are checked against an independent Python computation of the same policy.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fleetAdmittedMegabytesV1,fleetGrantMegabytesV1,inferableProjectsV1} from "../../🟦️.ts";
import {classifyOwnersV1,ownerGrantV1,ownerCostPolicyV1,type OwnerEstimateV1} from "../../💰️cost/🟦️.ts";
import {createHostAvailableBytesV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {wantedPolicyV1} from "../../🟦️.ts";

const MiB=1048576,python=Bun.which("python3")??Bun.which("python");
type Row={name:string;platform:"darwin"|"linux";total:number;reading:string;availableMegabytes:number};
const pages=(rows:Record<string,number>)=>Object.entries(rows).map(([name,count])=>`${name}:`.padEnd(34)+`${count}.`).join("\n");
const vmStat=(counts:{free:number;active:number;inactive:number;speculative:number;wired:number;purgeable:number;file:number;compressor:number})=>"Mach Virtual Memory Statistics: (page size of 16384 bytes)\n"+pages({"Pages free":counts.free,"Pages active":counts.active,"Pages inactive":counts.inactive,"Pages speculative":counts.speculative,"Pages throttled":0,"Pages wired down":counts.wired,"Pages purgeable":counts.purgeable,"File-backed pages":counts.file,"Anonymous pages":counts.active,"Pages occupied by compressor":counts.compressor})+"\n";
const meminfo=(totalKib:number,availableKib:number)=>`MemTotal:       ${totalKib} kB\nMemFree:         312000 kB\nMemAvailable:   ${availableKib} kB\nBuffers:          91000 kB\n`;
const mac=(name:string,counts:Parameters<typeof vmStat>[0]):Row=>({name,platform:"darwin",total:16384,reading:vmStat(counts),availableMegabytes:(counts.free+Math.max(counts.inactive+counts.speculative,counts.file+counts.purgeable))*16384/MiB});
const linux=(name:string,availableMegabytes:number):Row=>({name,platform:"linux",total:15848,reading:meminfo(15848*1024,availableMegabytes*1024),availableMegabytes});
const rows:Row[]=[
 mac("idle 16 GB Mac, warm cache",{free:15000,active:200000,inactive:620000,speculative:5000,wired:150000,purgeable:8000,file:330000,compressor:40000}),
 mac("16 GB Mac with a build running",{free:9000,active:420000,inactive:420000,speculative:3000,wired:170000,purgeable:6000,file:300000,compressor:80000}),
 mac("busy 16 GB Mac",{free:6000,active:520000,inactive:350000,speculative:2000,wired:190000,purgeable:4000,file:250000,compressor:120000}),
 mac("starved 16 GB Mac",{free:3000,active:600000,inactive:60000,speculative:1000,wired:210000,purgeable:900,file:40000,compressor:200000}),
 linux("idle 16 GB Linux worker",11000),linux("16 GB Linux worker with a build running",7000),linux("busy 16 GB Linux worker",5200),linux("starved 16 GB Linux worker",4300),
];
const read=(row:Row)=>row.platform==="darwin"?createHostAvailableBytesV1({platform:"darwin",run:()=>row.reading,total:()=>row.total*MiB,environment:{}})():createHostAvailableBytesV1({platform:"linux",read:()=>row.reading,total:()=>row.total*MiB,environment:{}})();
const oracle=(total:number,available:number)=>{
 const run=spawnSync(python!,["-I","-c","import json,sys\nr=json.load(sys.stdin)\nhalf=r['total']//2\nadmitted=max(0,min(half,r['available']-2048))\nwave=min(48,max(0,int((admitted-2200)//((7200-2200)/48))))\nwave=0 if wave<8 else wave\ngrantMemory=min(half,9399) if wave>0 else admitted\nmanaged=int(max(0,int(grantMemory-3800))/2/1.25)\nprint(json.dumps({'admitted':admitted,'wave':wave,'projects':wave*4,'grant':managed*2}))"],{input:JSON.stringify({total,available}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {admitted:number;wave:number;projects:number;grant:number};
};
const decide=(row:Row)=>{
 const bytes=read(row)!,availableMegabytes=Math.floor(bytes/MiB),reading={hostMegabytes:row.total,availableMegabytes},admitted=fleetAdmittedMegabytesV1(reading),grantMemory=fleetGrantMegabytesV1(reading),fit=inferableProjectsV1(admitted),grant=ownerGrantV1(grantMemory,wantedPolicyV1);
 return{availableMegabytes,admitted,fit,grant,shown:grant.managedMegabytes*ownerCostPolicyV1.footprintFactor};
};

for(const row of rows)test("the reading of "+row.name+" is the kernel's available memory and the decision equals the independent computation",()=>{
 const got=decide(row),expected=oracle(row.total,got.availableMegabytes);
 expect(Math.abs(got.availableMegabytes-row.availableMegabytes)).toBeLessThanOrEqual(1);
 expect({admitted:got.admitted,wave:got.fit.waveProjects,projects:got.fit.projects,grant:got.shown}).toEqual(expected);
});

test("law: an idle 16 GB worker gives 3000 to 3600 MB per project and infers 48 projects per wave, 192 in all, on macOS and on Linux",()=>{
 for(const row of rows.filter(item=>item.name.startsWith("idle"))){
  const got=decide(row);
  expect(got.shown,row.name).toBeGreaterThanOrEqual(3000);expect(got.shown,row.name).toBeLessThanOrEqual(3600);
  expect(got.fit,row.name).toEqual({waveProjects:48,projects:192});
 }
});

test("law: a busy moment lowers the projects per wave before it lowers the grant; the grant is 0 only where not even the smallest wave fits",()=>{
 const macs=rows.filter(item=>item.platform==="darwin").map(decide),linuxes=rows.filter(item=>item.platform==="linux").map(decide);
 for(const series of[macs,linuxes]){
  const[idle,loaded,busy,starved]=series as [ReturnType<typeof decide>,ReturnType<typeof decide>,ReturnType<typeof decide>,ReturnType<typeof decide>];
  expect(idle.fit.waveProjects).toBeGreaterThan(loaded.fit.waveProjects);expect(loaded.fit.waveProjects).toBeGreaterThan(busy.fit.waveProjects);
  expect(busy.fit.waveProjects).toBeGreaterThanOrEqual(wantedPolicyV1.minimumWaveProjects);
  expect([loaded.shown,busy.shown]).toEqual([idle.shown,idle.shown]);
  expect(starved.fit).toEqual({waveProjects:0,projects:0});expect(starved.shown).toBe(0);
 }
});

test("law: the idle Mac is not read as starved by its free pages or by its file cache alone (the former reading was under 5.8 GB)",()=>{
 const idle=rows[0]!,former=(15000+330000+8000)*16384/MiB;
 expect(former).toBeLessThan(5800);expect(decide(idle).availableMegabytes).toBeGreaterThan(9248);
});

test("law: too heavy names only projects whose measured cost exceeds the grant; unknown ones are never heavy, and only a machine that cannot run a wave calls every measured project heavy",()=>{
 const heavy=(row:Row)=>{
  const managed=decide(rows.find(item=>item.platform===row.platform)!).grant.managedMegabytes,estimates:Record<string,OwnerEstimateV1>={fits:{megabytes:managed-200,source:"recorded"},edge:{megabytes:managed,source:"recorded"},over:{megabytes:managed+1,source:"recorded"},seeded:{megabytes:3838,source:"seed"},guess:{megabytes:5000,source:"kind"},nobody:{megabytes:820,source:"unknown"}};
  return classifyOwnersV1(Object.keys(estimates),name=>estimates[name]!,decide(row).grant).heavy.map(item=>item.name);
 };
 for(const row of rows.slice(0,3).concat(rows.slice(4,7)))expect(heavy(row),row.name).toEqual(["seeded","over"]);
 for(const row of[rows[3]!,rows[7]!])expect(heavy(row),row.name+" can run no wave").toEqual(["seeded","over","edge","fits"]);
});
