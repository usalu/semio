import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {requireTestToolV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../../🧬️schema/🔣️.json";
import fixture from "../../../🧪️tests/🧫️fixtures/🔣️.json";
import {WINDOWS_PROCESS_LISTING_V1,systemAncestryV1,windowsAncestryV1,windowsHostAncestryV1,windowsNativeAncestryV1,type AncestorV1} from "../../🟦️.ts";
import {createWindowsProcessHostV1,openWindowsProcessBindingV1,windowsFileTimeIsoV1,withWindowsProcessHostV1,type WindowsProcessHostV1,type WindowsProcessOptionsV1} from "../🟦️.ts";

const windows=schema.definitions.Policy.const.identity.ancestry.windows,oraclePath=join(import.meta.dir,"🐍️oracle","🐍️.py");
type Row={pid:number;ppid:number;started:string;command:string};
type Request=Readonly<{layout?:boolean;records?:readonly{pid:number;ppid:number;name:string}[];headers?:readonly string[];ticks?:readonly string[];iso?:readonly string[]}>;
const oracle=(request:Request):any=>{const result=spawnSync(requireTestToolV1("python"),["-I",oraclePath],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:1<<28});if(result.status!==0)throw Error(result.stderr);return JSON.parse(result.stdout);};
const split=(row:Row)=>{const at=row.command.lastIndexOf(" ");return at<0?{commandLine:"",name:row.command}:{commandLine:row.command.slice(0,at),name:row.command.slice(at+1)};};
const INVALID=18446744073709551615n,STATUS_ACCESS_DENIED=-1073741790;

type Scenario=Readonly<{refuse?:(pid:number)=>"open"|"commandline"|"times"|undefined;snapshot?:bigint;emptyFirst?:boolean;throwOnOpen?:boolean;throwOnClose?:boolean}>;
/** 🪟️ A model of the six kernel calls: it validates every argument (flags, access mask, information class, structure sizes, pointers, handles), writes the records the independent oracle encoded, and tracks every handle so a leak or a double close is a failure. */
function kernelModelV1(rows:readonly Row[],scenario:Scenario={}){
 const encoded=oracle({records:rows.map(row=>({pid:row.pid,ppid:row.ppid,name:split(row).name})),headers:rows.map(row=>split(row).commandLine),ticks:rows.map(row=>row.started)});
 const pointers=new Map<number,Buffer>(),handles=new Map<bigint,{kind:"snapshot"|"process";index:number;cursor:number}>(),state={next:0x200n,pointer:8192,opened:0,closed:0,errors:[] as string[],log:[] as string[]},byPid=new Map(rows.map((row,index)=>[row.pid,index] as const));
 const fail=(message:string)=>{state.errors.push(message);};
 const open=(kind:"snapshot"|"process",index:number)=>{const handle=state.next;state.next+=4n;handles.set(handle,{kind,index,cursor:0});return handle;};
 const need=(handle:bigint,kind:"snapshot"|"process")=>{const row=handles.get(handle);if(!row||row.kind!==kind){fail("bad "+kind+" handle "+handle);return undefined;}return row;};
 const bytes=(hex:string)=>Buffer.from(hex,"hex");
 const kernel:Record<string,(...args:any[])=>unknown>={
  CreateToolhelp32Snapshot:(flags:number,pid:number)=>{expect(flags).toBe(windows.snapshotProcesses);expect(pid).toBe(0);state.log.push("snapshot");return scenario.snapshot??open("snapshot",0);},
  Process32FirstW:(handle:bigint,entry:number)=>{const row=need(handle,"snapshot"),target=pointers.get(entry)!;if(!row)return 0;if(target.length!==windows.entry.bytes||target.readUInt32LE(0)!==windows.entry.bytes)fail("entry size not set");if(scenario.emptyFirst)return 0;row.cursor=1;bytes(encoded.records[0]).copy(target);return 1;},
  Process32NextW:(handle:bigint,entry:number)=>{const row=need(handle,"snapshot"),target=pointers.get(entry)!;if(!row||row.cursor>=rows.length)return 0;if(target.readUInt32LE(0)!==windows.entry.bytes)fail("entry size lost");target.fill(0);bytes(encoded.records[row.cursor++]!).copy(target);return 1;},
  OpenProcess:(access:number,inherit:number,pid:number)=>{expect(access).toBe(windows.queryLimitedInformation);expect(inherit).toBe(0);state.log.push("open "+pid);const index=byPid.get(pid);if(index===undefined||scenario.refuse?.(pid)==="open")return 0n;return open("process",index);},
  GetProcessTimes:(handle:bigint,creation:number,exit:number,kernelTime:number,user:number)=>{const row=need(handle,"process");if(!row)return 0;for(const pointer of[creation,exit,kernelTime,user])if(pointers.get(pointer)?.length!==8)fail("filetime buffer");if(scenario.refuse?.(rows[row.index]!.pid)==="times")return 0;pointers.get(creation)!.writeBigUInt64LE(BigInt(encoded.ticks[row.index]),0);return 1;},
  CloseHandle:(handle:bigint)=>{state.log.push("close");if(scenario.throwOnClose)throw Error("close failed");if(!handles.delete(handle)){fail("close of unknown handle "+handle);return 0;}return 1;},
 };
 const nt:Record<string,(...args:any[])=>unknown>={
  NtQueryInformationProcess:(handle:bigint,informationClass:number,buffer:number,length:number,returned:number)=>{
   const row=need(handle,"process"),target=pointers.get(buffer)!;if(!row)return -1073741816;expect(informationClass).toBe(windows.commandLine.informationClass);expect(length).toBe(target.length);expect(length).toBe(windows.commandLine.capacityBytes);expect(pointers.get(returned)!.length).toBe(4);
   if(scenario.refuse?.(rows[row.index]!.pid)==="commandline")return STATUS_ACCESS_DENIED;
   const data=bytes(encoded.headers[row.index]!);target.fill(0);data.copy(target);target.writeBigUInt64LE(BigInt(buffer+windows.commandLine.headerBytes),8);pointers.get(returned)!.writeUInt32LE(data.length,0);return 0;
  },
 };
 const expected={kernel32:{CreateToolhelp32Snapshot:{args:["u32","u32"],returns:"u64"},Process32FirstW:{args:["u64","ptr"],returns:"i32"},Process32NextW:{args:["u64","ptr"],returns:"i32"},OpenProcess:{args:["u32","i32","u32"],returns:"u64"},GetProcessTimes:{args:["u64","ptr","ptr","ptr","ptr"],returns:"i32"},CloseHandle:{args:["u64"],returns:"i32"}},ntdll:{NtQueryInformationProcess:{args:["u64","u32","ptr","u32","ptr"],returns:"i32"}}};
 const system={
  ptr:(view:ArrayBufferView)=>{state.pointer+=16;pointers.set(state.pointer,Buffer.from(view.buffer,view.byteOffset,view.byteLength));return state.pointer;},
  dlopen:(library:string,symbols:Record<string,unknown>)=>{
   if(scenario.throwOnOpen)throw Error("dlopen failed");
   const key=library.replace(/\.dll$/u,"") as keyof typeof expected;expect(Object.keys(expected)).toContain(key);expect(symbols).toEqual(expected[key]);state.opened++;
   const table=key==="kernel32"?kernel:nt;return{symbols:table,close:()=>{state.closed++;}};
  },
 };
 return{system,state,leaks:()=>handles.size};
}
const run=<T>(model:ReturnType<typeof kernelModelV1>,use:(host:WindowsProcessHostV1)=>T,extra:WindowsProcessOptionsV1={})=>withWindowsProcessHostV1(use,{open:()=>openWindowsProcessBindingV1(model.system as never),platform:"win32",architecture:"x64",ffi:()=>true,...extra});
const corpus=fixture.windows.listing as Row[];

test("the byte layouts and the FILETIME spelling equal the independent oracle (ctypes structures after the Windows headers, datetime arithmetic)",()=>{
 const layout=oracle({layout:true}).layout;
 expect(layout.pointerBytes).toBe(windows.pointerBytes);expect(layout.entry).toEqual(windows.entry);expect(layout.commandLine.headerBytes).toBe(windows.commandLine.headerBytes);expect(layout.commandLine.textOffset).toBe(windows.commandLine.headerBytes);
 expect(windows.commandLine.capacityBytes).toBeGreaterThanOrEqual(windows.commandLine.headerBytes+32767*2+2);expect(windows.commandLine.capacityBytes%16).toBe(0);
 expect(windows.snapshotProcesses).toBe(2);expect(windows.queryLimitedInformation).toBe(0x1000);expect(windows.commandLine.informationClass).toBe(60);expect(BigInt(windows.invalidHandle)).toBe(INVALID);
 const instants=["1970-01-01T00:00:00.0000010Z","2000-02-29T23:59:59.9999990Z","2026-10-20T06:00:00.0000000Z","2026-10-27T08:15:30.1234560Z","2262-04-11T23:47:16.8547750Z","9999-12-31T23:59:59.9999990Z",...corpus.map(row=>row.started)];
 const ticks=oracle({ticks:instants}).ticks as string[];
 for(const [index,text] of instants.entries())expect(windowsFileTimeIsoV1(BigInt(ticks[index]!)),text).toBe(text);
 const odd=[134_000_123_456_789n+133_000_000_000_000_000n,133_912_345_678_999_999n,133_912_345_679_000_001n,116_444_736_000_000_001n,116_444_736_000_000_009n,116_444_736_010_000_007n];
 const expected=oracle({iso:odd.map(String)}).iso as string[];
 for(const [index,value] of odd.entries())expect(windowsFileTimeIsoV1(value),String(value)).toBe(expected[index]);
 for(const bad of[0n,-1n,116_444_736_000_000_000n,116_444_735_999_999_999n,2_650_467_744_000_000_000n,18_446_744_073_709_551_615n])expect(windowsFileTimeIsoV1(bad),String(bad)).toBeUndefined();
});

test("the native chain equals the PowerShell chain of the Windows fixture corpus for every first process, every limit, the recycled id and the orphan",()=>{
 const cases:[number,Row[]][]=[[fixture.windows.first,corpus],[fixture.windows.orphanFirst,corpus],[6200,corpus],[5100,corpus],[1200,corpus],[4,corpus],[12345,corpus],[77,[fixture.windows.single as Row]]];
 for(const [first,rows] of cases){
  const model=kernelModelV1(rows),text=JSON.stringify(rows);
  for(const limit of[undefined,1,2,3,16]){
   const native=run(model,host=>windowsHostAncestryV1(host,first,limit)),shell=windowsAncestryV1(text,first,limit);
   expect(native,first+" limit "+limit).toEqual(shell);
  }
  expect(model.state.errors).toEqual([]);expect(model.leaks()).toBe(0);expect(model.state.opened).toBe(model.state.closed);
 }
 const model=kernelModelV1(corpus);expect(run(model,host=>windowsHostAncestryV1(host,fixture.windows.first))!.map(row=>row.pid)).toEqual(fixture.windows.pids);
 expect(run(model,host=>windowsHostAncestryV1(host,fixture.windows.orphanFirst))!.map(row=>row.pid)).toEqual(fixture.windows.orphanPids);
 expect(run(model,host=>windowsHostAncestryV1(host,fixture.windows.first))![0]).toEqual({pid:7300,started:"2026-10-20T08:06:00.0000000Z",command:"bun.exe repo cargo bun.exe"});
});

test("the listing asks the kernel exactly once for the snapshot, opens one handle per process it describes, closes every handle and both libraries, and keeps the row schema",()=>{
 const model=kernelModelV1(corpus),rows=run(model,host=>{const first=windowsHostAncestryV1(host,fixture.windows.first)!;windowsHostAncestryV1(host,fixture.windows.first);return first;})!;
 expect(rows.length).toBe(4);expect(model.state.log.filter(entry=>entry==="snapshot").length).toBe(1);expect(model.state.log.filter(entry=>entry.startsWith("open ")).map(entry=>Number(entry.slice(5)))).toEqual([...fixture.windows.pids,...fixture.windows.pids]);
 expect(model.state.log.filter(entry=>entry==="close").length).toBe(1+8);expect(model.leaks()).toBe(0);expect([model.state.opened,model.state.closed]).toEqual([2,2]);expect(model.state.errors).toEqual([]);
 const check=schema.definitions.Ancestor;for(const row of rows){expect(Object.keys(row).sort()).toEqual(Object.keys(check.properties).sort());expect(row.started.length).toBeLessThanOrEqual(check.properties.started.maxLength);expect(row.pid).toBeGreaterThanOrEqual(check.properties.pid.minimum);}
 const entries=run(kernelModelV1(corpus),host=>host.entries())!;expect(entries.map(row=>[row.pid,row.parent,row.name])).toEqual(corpus.map(row=>[row.pid,row.ppid,split(row).name]));
 expect(Object.isFrozen(entries)).toBe(true);
});

test("command lines survive as the system tells them: emoji, the longest line, a name that fills its field, a refused query, a missing line",()=>{
 const longest="x".repeat(32767),filled="n".repeat(259)+"é",rows:Row[]=[
  {pid:5000,ppid:4,started:"2026-10-20T08:00:00.0000000Z",command:"C:\\Users\\ü\\🧬 semio\\bun.exe run 🦑.ts bun.exe"},
  {pid:5001,ppid:5000,started:"2026-10-20T08:01:00.0000000Z",command:longest+" "+filled},
  {pid:5002,ppid:5001,started:"2026-10-20T08:02:00.0000000Z",command:"refused.exe"},
  {pid:5003,ppid:5002,started:"2026-10-20T08:03:00.0000000Z",command:"missing.exe"},
 ];
 const model=kernelModelV1(rows,{refuse:pid=>pid===5002?"commandline":undefined}),chain=run(model,host=>windowsHostAncestryV1(host,5003))!;
 expect(chain.map(row=>row.pid)).toEqual([5003,5002,5001,5000]);expect(chain[0]!.command).toBe("missing.exe");expect(chain[1]!.command).toBe("refused.exe");expect(chain[2]!.command).toBe(longest+" "+filled);expect(chain[3]!.command).toBe(rows[0]!.command);
 expect(Array.from(chain[3]!.command).includes("🦑")).toBe(true);expect(model.state.errors).toEqual([]);
 const probe=kernelModelV1(rows),binding=openWindowsProcessBindingV1(probe.system as never),lines=[(_process:bigint,buffer:Buffer)=>{buffer.fill(0);buffer.writeUInt16LE(3,0);return 0;},(_process:bigint,buffer:Buffer)=>{buffer.fill(0);buffer.writeUInt16LE(65535,0);return 0;},()=>-1073741820];
 for(const commandLine of lines)expect(createWindowsProcessHostV1({...binding,commandLine}).detail(5000)!.commandLine).toBe("");
 binding.release();expect(probe.leaks()).toBe(0);expect(probe.state.errors).toEqual([]);
});

test("a refusal degrades instead of lying: an unreadable ancestor, an empty or refused snapshot, a failing library, a failing use and an unsupported host all answer null and leak nothing",()=>{
 const refused=kernelModelV1(corpus,{refuse:pid=>pid===6200?"open":undefined});
 expect(run(refused,host=>windowsHostAncestryV1(host,fixture.windows.first))).toBeUndefined();expect(refused.leaks()).toBe(0);
 for(const stage of["times"as const,"open"as const]){const model=kernelModelV1(corpus,{refuse:()=>stage});expect(run(model,host=>windowsHostAncestryV1(host,fixture.windows.first))).toBeUndefined();expect(model.leaks()).toBe(0);}
 expect(windowsNativeAncestryV1(fixture.windows.first,16,{open:()=>openWindowsProcessBindingV1(kernelModelV1(corpus,{refuse:pid=>pid===5100?"open":undefined}).system as never),platform:"win32",architecture:"arm64",ffi:()=>true})).toBeNull();
 const invalid=kernelModelV1(corpus,{snapshot:INVALID}),empty=kernelModelV1(corpus,{emptyFirst:true}),broken=kernelModelV1(corpus,{throwOnOpen:true}),zero=kernelModelV1(corpus,{snapshot:0n});
 for(const model of[invalid,empty,zero])expect(run(model,host=>host.entries())).toBeNull();
 expect(empty.leaks()).toBe(0);expect([empty.state.opened,empty.state.closed]).toEqual([2,2]);expect(run(broken,host=>host.entries())).toBeNull();
 const throwing=kernelModelV1(corpus);expect(run(throwing,()=>{throw Error("use failed");})).toBeNull();expect([throwing.state.opened,throwing.state.closed]).toEqual([2,2]);
 const unclosable=kernelModelV1(corpus,{throwOnClose:true});expect(run(unclosable,host=>host.entries())).toBeNull();
 let opened=0;const spy=()=>{opened++;throw Error("must not open");};
 for(const extra of[{platform:"linux"},{platform:"darwin"},{architecture:"ia32"},{architecture:"arm"},{ffi:()=>false}])expect(run(kernelModelV1(corpus),host=>host.entries(),{open:spy,...extra})).toBeNull();
 expect(opened).toBe(0);
 expect(windowsNativeAncestryV1(1,16,{platform:"win32",architecture:"x64",ffi:()=>true,open:()=>openWindowsProcessBindingV1(kernelModelV1(corpus).system as never)})).toEqual([]);
});

test("the session resolver takes the native chain without starting a shell, and keeps the PowerShell program for hosts without it",()=>{
 const rows:readonly AncestorV1[]=[{pid:7,started:"2026-10-20T08:06:00.0000000Z",command:"bun.exe repo bun.exe"}];let asked:unknown[]=[];
 expect(systemAncestryV1("win32",7,3,(first,limit)=>{asked=[first,limit];return rows;})).toBe(rows);expect(asked).toEqual([7,3]);
 for(const word of["Win32_Process","ConvertTo-Json -Compress","ToUniversalTime","CommandLine"])expect(WINDOWS_PROCESS_LISTING_V1).toContain(word);
 if(process.platform!=="win32"){let calls=0;expect(systemAncestryV1("win32",7,3,()=>{calls++;return null;})).toEqual([]);expect(calls).toBe(1);}
 expect(windowsNativeAncestryV1(7,3,{platform:"linux"})).toBeNull();
});

test.skipIf(process.platform!=="win32")("on a real Windows host the kernel listing names the same chain as the CIM listing for this process, and costs far less than the shell (timings are printed for the record)",()=>{
 const begun=performance.now(),direct=windowsNativeAncestryV1(process.pid,16),nativeMs=performance.now()-begun;
 expect(direct).not.toBeNull();expect(direct![0]!.pid).toBe(process.pid);
 const root=process.env.SystemRoot,program=root?join(root,"System32","WindowsPowerShell","v1.0","powershell.exe"):"powershell.exe",shellBegun=performance.now();
 const shell=spawnSync(program,["-NoProfile","-NonInteractive","-Command",WINDOWS_PROCESS_LISTING_V1],{encoding:"utf8",windowsHide:true,maxBuffer:1<<26}),shellMs=performance.now()-shellBegun,viaShell=windowsAncestryV1(shell.stdout,process.pid,16);
 console.log(`[winps] windows process listing: kernel ${nativeMs.toFixed(2)} ms (${direct!.length} rows), PowerShell ${shellMs.toFixed(0)} ms (${viaShell.length} rows); first row started kernel ${direct![0]!.started} shell ${viaShell[0]?.started}`);
 expect(direct!.map(row=>row.pid)).toEqual(viaShell.map(row=>row.pid));
 for(const [index,row] of direct!.entries()){expect(row.command,String(row.pid)).toBe(viaShell[index]!.command);expect(Math.abs(Date.parse(row.started)-Date.parse(viaShell[index]!.started)),String(row.pid)).toBeLessThanOrEqual(1);}
 expect(systemAncestryV1("win32",process.pid,16)).toEqual(direct!);
 expect(nativeMs).toBeLessThan(shellMs);
},60000);
