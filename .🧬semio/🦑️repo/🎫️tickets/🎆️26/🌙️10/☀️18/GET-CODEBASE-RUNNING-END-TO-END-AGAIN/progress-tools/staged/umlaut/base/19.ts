/** 🔬️ Contract: the capacity gate prices a process tree by its memory footprint (what the processes hold privately plus what was swapped or compressed out of them), not by its resident set: on macOS a process keeps its resident high-water mark after it freed memory, and the learned peaks follow what it really held. Per platform the observer is data (`Observers` of the schema) read from fixtures: macOS `footprint` (the kernel's phys_footprint), Linux `smaps_rollup` (proportional set size plus swapped), Windows the private working set table; a source that cannot be read falls back to the resident set and says so once in every language. Oracle: a Python parser of the same outputs (standard library only) and, on macOS, the kernel's own `proc_pid_rusage` read by a child through ctypes. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {emptyHostEstimateStoreV1,readHostEstimateStoreV1,recordHostOutcomeV1,updateHostEstimateStoreV1} from "../../📐️estimates/🟦️.ts";
import {readHostCapacityPolicyV1} from "../../📏️decision/🟦️.ts";
import {observeTreeMegabytesV1,parseFootprintV1,parseSmapsRollupV1,treeMegabytesV1,treePidsV1,type HostCommandRunnerV1} from "../../📡️sampling/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import decision from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const here=fileURLToPath(new URL(".",import.meta.url)),python=testPythonV1(),observers=schema.definitions.Observers.const,policy=readHostCapacityPolicyV1(decision.policy);
const oracle=python?JSON.parse(spawnSync(python,["-I",join(here,"🐍️oracle","🐍️.py"),join(here,"🧫️fixtures","🔣️.json")],{encoding:"utf8",maxBuffer:64*1024*1024}).stdout):undefined;
const noNotice=():void=>{throw Error("no notice expected");};

test("the observers are data: the footprint is the measure of every platform and each platform names its source and its fallback",()=>{
 expect(observers.measure).toBe("footprint");
 expect(Object.keys(observers.platforms).sort()).toEqual(["darwin","linux","win32"]);
 expect(observers.platforms.darwin.footprint.command).toBe("footprint");expect(observers.platforms.darwin.footprint.arguments).toEqual(["-f","bytes","--noCategories"]);expect(observers.platforms.darwin.table.command).toBe("ps");
 expect(observers.platforms.linux.footprint).toEqual({file:"smaps_rollup",fields:["Pss","SwapPss"]});expect(observers.platforms.linux.resident.file).toBe("status");
 expect(observers.platforms.win32.footprint.arguments[3]).toContain("WorkingSetPrivate");expect(observers.platforms.win32.resident.arguments[3]).toContain("WorkingSetSize");
 expect(observers.platforms.win32.footprint.arguments.slice(0,3)).toEqual(observers.platforms.win32.resident.arguments.slice(0,3));
});

test("the macOS footprint output parses to the kernel's phys_footprint by pid, equal to the frozen values and to the Python oracle",()=>{
 fixture.darwin.cases.forEach((row,index)=>{
  const parsed=parseFootprintV1(row.output),expected=fixture.expect.darwin[index]!.bytes as Record<string,number>;
  expect(Object.fromEntries([...parsed].map(([pid,bytes])=>[String(pid),bytes])),row.name).toEqual(expected);
  if(oracle)expect(oracle.darwin[index].bytes,row.name).toEqual(expected);
 });
 expect(parseFootprintV1("").size).toBe(0);expect(parseFootprintV1("footprint: Unable to find pid for process matching '1'\n").size).toBe(0);
});

test("the Linux smaps_rollup parses to proportional set size plus swapped proportional set size, and a first field that is absent is no reading",()=>{
 fixture.linux.smaps.forEach((text,index)=>{
  expect(parseSmapsRollupV1(text)??null,String(index)).toBe(fixture.expect.linux.smaps[index]);
  if(oracle)expect(oracle.linux.smaps[index]).toBe(fixture.expect.linux.smaps[index]);
 });
 expect(parseSmapsRollupV1("Pss: 10 kB\nSwapPss: 5 kB\n",["Pss"])).toBe(10);
});

test("the tree of a process is its descendants, root first, and the resident sum is unchanged",()=>{
 const rows=[{pid:1,parent:0,rssKilobytes:1024},{pid:2,parent:1,rssKilobytes:2048},{pid:3,parent:1,rssKilobytes:4096},{pid:4,parent:2,rssKilobytes:1024},{pid:9,parent:8,rssKilobytes:99999}];
 expect([...treePidsV1(rows,1)].sort()).toEqual([1,2,3,4]);expect(treePidsV1(rows,1)[0]).toBe(1);expect(treePidsV1(rows,7)).toEqual([]);
 expect(treeMegabytesV1(rows,1)).toBe(8);expect(treeMegabytesV1(rows,2)).toBe(3);expect(treeMegabytesV1(rows,7)).toBeNull();
});

test("on macOS one footprint call prices the whole tree, a process it did not print is priced by its resident set, and a refusal falls back to the resident set once in every language",async()=>{
 for(const [index,row] of fixture.darwin.cases.entries()){
  const calls:string[][]=[],lines:string[]=[],run:HostCommandRunnerV1=async(command,args)=>{calls.push([command,...args]);return command==="ps"?row.ps:row.output;};
  const observed=await observeTreeMegabytesV1(row.root,run,"darwin",undefined,line=>{lines.push(line);});
  expect(observed,row.name).toBe(fixture.expect.darwin[index]!.megabytes);
  if(oracle)expect(observed,row.name+" (oracle)").toBe(oracle.darwin[index].megabytes);
  expect(calls[0],row.name).toEqual(["ps","-A","-o","pid=,ppid=,rss="]);
  if(observed!==null){
   const members=treePidsV1([...row.ps.matchAll(/^\s*(\d+)\s+(\d+)\s+(\d+)\s*$/gmu)].map(match=>({pid:Number(match[1]),parent:Number(match[2])})),row.root);
   expect(calls[1],row.name).toEqual(["footprint","-f","bytes","--noCategories",...members.flatMap(pid=>["-p",String(pid)])]);
  }else expect(calls.length,"a root that is not running asks nothing more").toBe(1);
  const refused=Object.keys(fixture.expect.darwin[index]!.bytes).length===0&&observed!==null;
  expect(lines.length,row.name).toBe(refused?2:0);
  if(refused){expect(lines[0]).toContain("footprint of running process trees cannot be read on darwin");expect(lines[1]).toContain("Speicher-Fussabdruck");}
 }
 const big=fixture.darwin.cases[3]!,resident=await observeTreeMegabytesV1(big.root,async(command)=>command==="ps"?big.ps:(()=>{throw Error("resident asks no footprint");})(),"darwin",undefined,noNotice,"resident");
 expect(resident,"the resident set of the process that freed its memory").toBe(7208);
 expect(fixture.expect.darwin[3]!.megabytes,"its footprint").toBe(2700);
});

test("a footprint call that fails after printing some processes keeps what it printed",async()=>{
 const row=fixture.darwin.cases[1]!,run:HostCommandRunnerV1=async(command)=>{if(command==="ps")return row.ps;throw Object.assign(new Error("exit 1"),{stdout:row.output});};
 expect(await observeTreeMegabytesV1(row.root,run,"darwin",undefined,noNotice)).toBe(fixture.expect.darwin[1]!.megabytes);
 const lines:string[]=[],gone:HostCommandRunnerV1=async(command)=>{if(command==="ps")return row.ps;throw Error("spawn footprint ENOENT");};
 expect(await observeTreeMegabytesV1(row.root,gone,"darwin",undefined,line=>{lines.push(line);})).toBe(4);
 expect(lines.length).toBe(2);expect(lines[0]).toContain("spawn footprint ENOENT");
});

test("on Linux each member is priced by its smaps_rollup, one that cannot be read by its resident set, and a host without smaps_rollup says so once in every language",async()=>{
 const run:HostCommandRunnerV1=async()=>{throw Error("no external tool on Linux");};
 for(const [index,row] of fixture.linux.trees.entries()){
  const files=row.files as Record<string,string>,proc={read:async(path:string)=>files[path]??null,list:async()=>row.listing};
  const lines:string[]=[],observed=await observeTreeMegabytesV1(row.root,run,"linux",proc,line=>{lines.push(line);});
  expect(observed,row.name).toBe(fixture.expect.linux.trees[index]);
  if(oracle)expect(observed,row.name+" (oracle)").toBe(oracle.linux.trees[index]);
  expect(lines,row.name).toEqual([]);
 }
 const row=fixture.linux.trees[0]!,bare=Object.fromEntries(Object.entries(row.files).filter(([path])=>!path.endsWith("smaps_rollup")));
 const lines:string[]=[],proc={read:async(path:string)=>bare[path]??null,list:async()=>row.listing};
 expect(await observeTreeMegabytesV1(row.root,run,"linux",proc,line=>{lines.push(line);}),"the resident sum 4000+8000+1000 KB").toBe(12);
 expect(lines.length).toBe(2);expect(lines[0]).toContain("smaps_rollup unreadable");
 expect(await observeTreeMegabytesV1(row.root,run,"linux",{read:async(path:string)=>row.files[path as keyof typeof row.files]??null,list:async()=>row.listing},noNotice,"resident")).toBe(12);
});

test("on Windows the private working set table prices the tree, a table that is empty or fails falls back to the working set once in every language, and a blind host observes nothing",async()=>{
 for(const [index,root] of fixture.win32.roots.entries()){
  const calls:string[][]=[],run:HostCommandRunnerV1=async(command,args)=>{calls.push([command,...args]);return fixture.win32.table;};
  expect(await observeTreeMegabytesV1(root,run,"win32",undefined,noNotice),String(root)).toBe(fixture.expect.win32[index]);
  expect(calls).toEqual([["powershell.exe",...observers.platforms.win32.footprint.arguments]]);
  if(oracle)expect(oracle.win32[index]).toBe(fixture.expect.win32[index]);
 }
 const lines:string[]=[],calls:string[][]=[],run:HostCommandRunnerV1=async(command,args)=>{calls.push([command,...args]);return args[3]===observers.platforms.win32.footprint.arguments[3]?"":fixture.win32.table;};
 expect(await observeTreeMegabytesV1(100,run,"win32",undefined,line=>{lines.push(line);})).toBe(76);
 expect(calls.map(call=>call[4])).toEqual([observers.platforms.win32.footprint.arguments[3],observers.platforms.win32.resident.arguments[3]]);expect(lines.length).toBe(2);expect(lines[0]).toContain("empty process table");
});

test("the learned peaks are stamped with the measure: a store of another measure is empty, a written store reads back",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"ram-footprint-"));
 try{
  const unstamped=JSON.stringify(recordHostOutcomeV1(policy,emptyHostEstimateStoreV1(),["run"],{operation:"run",jobs:1,peakMegabytes:4000,defaultMegabytes:3584,activeMilliseconds:1000,endedAtMs:5,peakAtMilliseconds:500}));
  writeFileSync(join(directory,schema.definitions.Protocol.const.estimatesFile),unstamped);
  expect(readHostEstimateStoreV1(directory),"peaks that were read from resident sets do not mix with footprints").toEqual(emptyHostEstimateStoreV1());
  expect(await updateHostEstimateStoreV1(directory,policy,store=>recordHostOutcomeV1(policy,store,["run"],{operation:"run",jobs:1,peakMegabytes:900,defaultMegabytes:3584,activeMilliseconds:1000,endedAtMs:6,peakAtMilliseconds:500}))).toBe(true);
  const stored=readHostEstimateStoreV1(directory);expect(stored.classes["run"]!.samples.length).toBe(1);
  expect((JSON.parse(await Bun.file(join(directory,schema.definitions.Protocol.const.estimatesFile)).text()) as {measure:string}).measure).toBe("footprint");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test.skipIf(process.platform!=="darwin"||!python)("on macOS the observed footprint of a child equals what the kernel reports to the child itself (proc_pid_rusage through ctypes)",async()=>{
 const script=["import ctypes,sys","lib=ctypes.CDLL('/usr/lib/libproc.dylib')","hold=bytearray(200*1024*1024)","for index in range(0,len(hold),4096): hold[index]=1","buffer=ctypes.create_string_buffer(512)","lib.proc_pid_rusage(__import__('os').getpid(),2,buffer)","print(int.from_bytes(buffer.raw[72:80],'little'),flush=True)","sys.stdin.read()"].join("\n");
 const child=Bun.spawn([python!,"-I","-c",script],{stdin:"pipe",stdout:"pipe",stderr:"ignore"});
 try{
  const reader=child.stdout.getReader(),first=await reader.read(),reported=Number(new TextDecoder().decode(first.value).trim()),observed=await observeTreeMegabytesV1(child.pid);
  expect(reported,"the child's own phys_footprint").toBeGreaterThan(190*1048576);
  expect(Math.abs(observed!-reported/1048576),`observed ${observed} MB, kernel ${reported/1048576} MB`).toBeLessThanOrEqual(16);
 }finally{child.kill();}
});

test.skipIf(process.platform!=="darwin")("on macOS a process that freed its memory keeps its resident set but its footprint falls: the footprint is what the gate must learn",async()=>{
 const script="let big=Array.from({length:3000000},(_,i)=>({i,s:'x'+i}));big=null;Bun.gc(true);Bun.shrink?.();console.log('ready');await new Promise(done=>process.stdin.once('end',done));";
 const child=Bun.spawn([process.execPath,"-e",script],{stdin:"pipe",stdout:"pipe",stderr:"ignore"});
 try{
  await child.stdout.getReader().read();
  const footprint=await observeTreeMegabytesV1(child.pid),resident=await observeTreeMegabytesV1(child.pid,undefined,undefined,undefined,undefined,"resident");
  expect(resident!,`resident ${resident} MB`).toBeGreaterThan(150);
  expect(footprint!,`footprint ${footprint} MB against resident ${resident} MB`).toBeLessThan(resident!/2);
 }finally{child.kill();}
});
