import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {freemem,totalmem} from "node:os";
import {readHostCapacityPolicyV1,type HostCommandRunnerV1} from "../../🟦️.ts";
import {observeTreeMegabytesV1,parseDarwinCountersV1,parseLinuxCountersV1,parseProcessTableV1,readHostCountersV1,readHostSampleV1,reduceHostSampleV1,treeMegabytesV1} from "../../📡️sampling/🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1(decisionFixture.policy);

test("macOS counters parse from vm_stat, vm.swapusage and the pressure level exactly",()=>{
 for(const item of fixture.darwin)expect(parseDarwinCountersV1(item.vmStat,item.swapUsage,item.pressureLevel,item.filesTable,item.imageInfo),item.name).toEqual(item.expect as never);
});

test("Linux counters parse from meminfo, vmstat, PSI and the cgroup v2 limit files exactly",()=>{
 for(const item of fixture.linux)expect(parseLinuxCountersV1(item.meminfo,item.vmstat,item.pressure,item.cgroupMaximum,item.cgroupCurrent,item.fileNr,item.mounts),item.name).toEqual(item.expect as never);
});

test("Counters reduce to one host sample: the worst of the platform level and the free share wins, the container limit caps the host and the swap rate needs a predecessor",()=>{
 for(const item of fixture.reduce)expect(reduceHostSampleV1(policy,{platform:item.platform as never,cores:item.cores,totalMegabytes:item.totalMegabytes,atMs:item.atMs,load:item.load},item.counters as never,item.previous??undefined),item.name).toEqual(item.expect as never);
});

test("The process tree sums a process and all descendants from a ps table and answers null for an unknown pid",()=>{
 const rows=parseProcessTableV1(fixture.tree.table);
 expect(rows.length).toBe(6);
 for(const [pid,megabytes] of Object.entries(fixture.tree.expect))expect(treeMegabytesV1(rows,Number(pid)),pid).toBe(megabytes);
});

test("Every platform reads through injected ports, so Linux and Windows parse on any machine",async()=>{
 const files=new Map<string,string>([["/proc/meminfo",fixture.linux[0]!.meminfo],["/proc/vmstat",fixture.linux[0]!.vmstat],["/proc/pressure/memory",fixture.linux[0]!.pressure!],["/sys/fs/cgroup/memory.max",fixture.linux[0]!.cgroupMaximum!],["/sys/fs/cgroup/memory.current",fixture.linux[0]!.cgroupCurrent!],["/proc/sys/fs/file-nr",fixture.linux[0]!.fileNr!],["/proc/mounts",fixture.linux[0]!.mounts!]]);
 const counters=await readHostCountersV1({platform:"linux",run:async()=>{throw Error("no command expected");},read:async path=>files.get(path)??null});
 expect(counters).toEqual(fixture.linux[0]!.expect as never);
 const calls:string[]=[],run:HostCommandRunnerV1=async(command,args)=>{calls.push([command,...args].join(" "));return command==="vm_stat"?fixture.darwin[0]!.vmStat:command==="/usr/bin/hdiutil"?fixture.darwin[0]!.imageInfo!:args.includes("kern.num_files")?fixture.darwin[0]!.filesTable:args.includes("vm.swapusage")?fixture.darwin[0]!.swapUsage:fixture.darwin[0]!.pressureLevel;};
 expect(await readHostCountersV1({platform:"darwin",run,read:async()=>null})).toEqual(fixture.darwin[0]!.expect as never);
 expect(calls.sort()).toEqual(["/usr/bin/hdiutil info","sysctl -n kern.memorystatus_vm_pressure_level","sysctl -n vm.swapusage","sysctl kern.num_files kern.maxfiles","vm_stat"]);
 expect(await observeTreeMegabytesV1(102,async()=>fixture.tree.table,"darwin")).toBe(22);
 expect(await observeTreeMegabytesV1(102,async()=>{throw Error("no command expected");},"win32")).toBeNull();
});

test("The optional open file table and image count fall back to unknown when their probes fail, and never fail the sample",async()=>{
 const run:HostCommandRunnerV1=async(command,args)=>{if(command==="/usr/bin/hdiutil"||args.includes("kern.num_files"))throw Error("stalled");return command==="vm_stat"?fixture.darwin[0]!.vmStat:args.includes("vm.swapusage")?fixture.darwin[0]!.swapUsage:fixture.darwin[0]!.pressureLevel;};
 expect(await readHostCountersV1({platform:"darwin",run,read:async()=>null})).toMatchObject({openFiles:null,openFilesMaximum:null,attachedImages:null,availableBytes:fixture.darwin[0]!.expect.availableBytes});
 const only=new Map<string,string>([["/proc/meminfo",fixture.linux[0]!.meminfo],["/proc/vmstat",fixture.linux[0]!.vmstat]]);
 expect(await readHostCountersV1({platform:"linux",run:async()=>"",read:async path=>only.get(path)??null})).toMatchObject({openFiles:null,openFilesMaximum:null,attachedImages:null});
 expect(await readHostCountersV1({platform:"win32",run:async()=>"",read:async()=>null})).toMatchObject({openFiles:null,openFilesMaximum:null,attachedImages:null});
});

test("A missing Linux counter file fails closed",async()=>{
 await expect(readHostCountersV1({platform:"linux",run:async()=>"",read:async()=>null})).rejects.toThrow("Linux memory counters unavailable");
});

test.skipIf(process.platform!=="darwin")("macOS live sample is conservative against the kernel's own memory_pressure and consistent with os.freemem",async()=>{
 const first=await readHostSampleV1(policy,{platform:"darwin",run:async(command,args)=>{const out=spawnSync(command,[...args],{encoding:"utf8"});if(out.status!==0)throw Error(out.stderr);return out.stdout;},read:async()=>null});
 const kernel=spawnSync("memory_pressure",[],{encoding:"utf8"}).stdout,percent=Number(/System-wide memory free percentage: (\d+)%/u.exec(kernel)?.[1]),host=first.host;
 expect(host.totalMegabytes).toBe(Math.floor(totalmem()/1048576));
 expect(host.availableMegabytes).toBeGreaterThanOrEqual(Math.floor(freemem()/1048576)*0.5);
 expect(host.availableMegabytes).toBeLessThanOrEqual(host.totalMegabytes);
 expect(Number.isFinite(percent)).toBe(true);
 expect(host.availableMegabytes/host.totalMegabytes*100,"ours must not exceed the kernel's free share").toBeLessThanOrEqual(percent+5);
 const level=Number(spawnSync("sysctl",["-n","kern.memorystatus_vm_pressure_level"],{encoding:"utf8"}).stdout.trim());
 expect(host.pressure===(level===4?"critical":level===2?"warn":"normal")||host.pressure!=="normal").toBe(true);
 expect(host.load).toBeGreaterThanOrEqual(0);
 const files=spawnSync("sysctl",["-n","kern.num_files"],{encoding:"utf8"}).stdout.trim(),maximum=spawnSync("sysctl",["-n","kern.maxfiles"],{encoding:"utf8"}).stdout.trim();
 expect(host.openFilesMaximum).toBe(Number(maximum));
 expect(Math.abs(host.openFiles!-Number(files))).toBeLessThan(Number(maximum)/20);
 const plist=spawnSync("/usr/bin/hdiutil",["info","-plist"],{encoding:"utf8"}).stdout,independent=Number(spawnSync("plutil",["-extract","images","raw","-o","-","-"],{input:plist,encoding:"utf8"}).stdout.trim());
 expect(Number.isFinite(independent)).toBe(true);
 expect(Math.abs(host.attachedImages!-independent),"attached images agree with plutil's count of hdiutil's plist").toBeLessThanOrEqual(1);
});
