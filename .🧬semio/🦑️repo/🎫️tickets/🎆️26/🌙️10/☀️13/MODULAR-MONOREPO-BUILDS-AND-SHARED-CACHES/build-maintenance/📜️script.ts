import {join,resolve} from "node:path";
import {writeFile,mkdir,statfs} from "node:fs/promises";
import {spawn} from "node:child_process";
import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1,captureOwnedProcess,terminateOwnedChildTree} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withCargoCacheMaintenanceV1,scanCargoBuildUnits,planCachePrune,planCachePressurePruneV1,deleteUnit} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts";
import {CACHE_POLICY} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔍️discovery/📂️source/🟦️.ts";
import {cargoDirectories} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";
import{assertNativeCacheDescriptorCustodyV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts";
const command=process.argv[2];if(!["preview","prune"].includes(command!))throw Error("Expected preview or prune");
const workspace=resolve(import.meta.dir,"../../../../../../../.."),buildDirectory=cargoDirectories(workspace).build;
const lockProgram=`import os,sys,stat
fds=[]
try:
 for path in sys.argv[1:]:
  fd=os.open(path,os.O_RDWR|getattr(os,"O_NOFOLLOW",0));fds.append(fd)
  actual=os.fstat(fd);named=os.stat(path,follow_symlinks=False)
  if not stat.S_ISREG(actual.st_mode) or (actual.st_dev,actual.st_ino)!=(named.st_dev,named.st_ino):raise RuntimeError("Physical compiler lock identity changed")
  if os.name=="nt":
   import ctypes,msvcrt
   from ctypes import wintypes
   class Overlapped(ctypes.Structure):
    _fields_=[("Internal",ctypes.c_size_t),("InternalHigh",ctypes.c_size_t),("Offset",wintypes.DWORD),("OffsetHigh",wintypes.DWORD),("hEvent",wintypes.HANDLE)]
   lock=ctypes.windll.kernel32.LockFileEx;lock.argtypes=[wintypes.HANDLE,wintypes.DWORD,wintypes.DWORD,wintypes.DWORD,wintypes.DWORD,ctypes.POINTER(Overlapped)];lock.restype=wintypes.BOOL
   if not lock(msvcrt.get_osfhandle(fd),2,0,0xffffffff,0xffffffff,ctypes.byref(Overlapped())):raise ctypes.WinError()
  else:
   import fcntl
   fcntl.flock(fd,fcntl.LOCK_EX)
 print("held",flush=True)
 sys.stdin.read()
finally:
 for fd in reversed(fds):os.close(fd)
`;
async function withPhysicalProfileLocks(paths:readonly string[],control:ReturnType<typeof scriptCompilerAdmissionControlV1>,run:(guard:Readonly<{pid:number;check():void}>)=>Promise<void>):Promise<void>{
 checkResourceLeaseControl(control);
 const child=spawn("python3",["-u","-c",lockProgram,...paths],{cwd:workspace,stdio:["pipe","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true});
 let error="",announced=false,stopped:unknown,failure:unknown;const finished=new Promise<number|null>(accept=>child.once("close",accept));
 const ready=new Promise<void>((accept,reject)=>{child.stdout.on("data",bytes=>{if(String(bytes).trim()==="held"){announced=true;accept();}});child.stderr.on("data",bytes=>{error=(error+String(bytes)).slice(-8192);});child.once("error",reject);child.once("close",code=>{if(!announced)reject(stopped??Error(`Physical compiler lock acquisition failed ${code}: ${error}`));});});
 const abort=()=>{stopped=control.signal.reason;terminateOwnedChildTree(child);};control.signal.addEventListener("abort",abort,{once:true});
 try{
  if(control.signal.aborted)abort();
  while(!announced){const remaining=checkResourceLeaseControl(control);await Promise.race([ready,new Promise<void>(accept=>setTimeout(accept,Math.min(remaining??500,500)))]);checkResourceLeaseControl(control);if(!announced){await control.publish({resource:"cache:physical-compiler-locks",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}}
  console.log(`[DEBUG] Physical Cargo profile locks held pid=${child.pid} paths=${JSON.stringify(paths)}`);
  const check=()=>{checkResourceLeaseControl(control);if(child.exitCode!==null||child.signalCode!==null||child.killed)throw Error("Physical compiler lock holder terminated before maintenance settled");};
  check();await run({pid:child.pid!,check});check();
 }catch(error){failure=error;throw error;}finally{control.signal.removeEventListener("abort",abort);child.stdin.end();if(!announced||child.exitCode!==null)terminateOwnedChildTree(child);const status=await finished;if(status!==0&&stopped===undefined&&failure===undefined)throw Error(`Physical compiler lock lifetime failed ${status}: ${error}`);}
}
await receiveScriptProcessInvocation(process.env,async invocation=>{
 const control=scriptCompilerAdmissionControlV1(invocation,"cache:inactive-build-units",600000);
 const available=async()=>{checkResourceLeaseControl(control);const fs=await statfs(buildDirectory,{bigint:true});checkResourceLeaseControl(control);const bytes=Number(fs.bavail*fs.bsize);if(!Number.isSafeInteger(bytes)||bytes<0)throw Error("Finite actual filesystem headroom required");return bytes;};
 await withCargoCacheMaintenanceV1({buildDirectory,leaseDirectory:join(workspace,".cache/framework/leases"),control,wait:false},async()=>{
  const processes=await captureOwnedCommandV1("ps",["-axo","pid,comm"],workspace,{control});
  if(processes.split("\n").some(line=>/(?:^|\s|\/)(?:rustc|cargo|cargo-nextest|clang|clang\+\+|ld)$/u.test(line.trim())))throw Error("External compiler process must finish before maintenance");
  const originalAvailableBytes=await available(),units=await scanCargoBuildUnits(buildDirectory,control),policy=CACHE_POLICY.storage.cargo,pressurePolicy=CACHE_POLICY.storage.pressure;
  const area=planCachePrune([{name:"cargo-build",budgetBytes:policy.budgetBytes,unusedAgeMs:policy.unusedAgeMs,units}],Date.now(),CACHE_POLICY.storage.guardAgeMs).areas[0]!;
  const pressure=planCachePressurePruneV1(units,Date.now(),await available(),pressurePolicy,originalAvailableBytes),candidates=pressure.pressured?pressure.units:[...area.ageDeletions,...area.budgetDeletions];
  const artifacts=join(import.meta.dir,"../🗑️generated/build-maintenance"),plan=join(artifacts,`${command}-${process.pid}.json`);
  await mkdir(artifacts,{recursive:true});await writeFile(plan,JSON.stringify({area,pressure,originalAvailableBytes}));
  console.log(`[DEBUG] Inactive Cargo build plan ${JSON.stringify({mode:command,units:units.length,totalBytes:area.totalBytes,retainedBytes:area.retainedBytes,guardedBytes:area.guardedOverBudgetBytes,candidates:candidates.length,pressured:pressure.pressured,plannedBytes:pressure.plannedBytes,originalAvailableBytes,targetUnmetBytes:pressure.targetUnmetBytes,plan})}`);
  if(command==="preview")return;
  if(candidates.length===0){console.log("[DEBUG] Inactive Cargo build maintenance completed {\"count\":0,\"bytes\":0}");return;}
  const locks=[...new Set(candidates.map(unit=>join(buildDirectory,unit.path.slice(0,unit.path.indexOf("/build/")),".cargo-build-lock")))].sort();
  await withPhysicalProfileLocks(locks,control,async guard=>{
  const opened=await captureOwnedProcess("lsof",["-nP","-Fpn"],{invocation,cwd:workspace,env:process.env,budgetMs:600000,maxOutputBytes:16777216,stdoutPath:join(artifacts,`opened-${process.pid}.txt`),stderrPath:join(artifacts,`errors-${process.pid}.txt`)});
  console.log(`[DEBUG] Physical compiler descriptor probe ${JSON.stringify({status:opened.status,signal:opened.signal,reason:opened.reason,stdoutBytes:Buffer.byteLength(opened.stdout),stderrBytes:Buffer.byteLength(opened.stderr),guardPid:guard.pid})}`);
  assertNativeCacheDescriptorCustodyV1(opened,buildDirectory,{pid:guard.pid,paths:locks},"newline-pn");
  let bytes=0,count=0;
  for(let pass=0;;pass++){
  if(pressure.pressured&&(bytes>=pressurePolicy.maximumDeletionBytes||await available()>=pressurePolicy.targetAvailableBytes))break;
  guard.check();const currentUnits=await scanCargoBuildUnits(buildDirectory,control),current=planCachePrune([{name:"cargo-build",budgetBytes:policy.budgetBytes,unusedAgeMs:policy.unusedAgeMs,units:currentUnits}],Date.now(),CACHE_POLICY.storage.guardAgeMs).areas[0]!,free=await available(),remainingDeletion=pressurePolicy.maximumDeletionBytes-bytes,currentPressure=remainingDeletion>0?planCachePressurePruneV1(currentUnits,Date.now(),free,{...pressurePolicy,maximumDeletionBytes:remainingDeletion},originalAvailableBytes):undefined,selected=pressure.pressured?(currentPressure?.units??[]):[...current.ageDeletions,...current.budgetDeletions];
  const admitted=selected.filter(unit=>locks.includes(join(buildDirectory,unit.path.slice(0,unit.path.indexOf("/build/")),".cargo-build-lock")));
  await writeFile(join(artifacts,`locked-prune-${process.pid}-${pass}.json`),JSON.stringify({current,pressure:currentPressure,availableBytes:free,originalAvailableBytes}));
  console.log(`[DEBUG] Physical Cargo lock rescan candidates=${admitted.length} deferredUnprotected=${selected.length-admitted.length} availableBytes=${free}`);
  if(!admitted.length||pressure.pressured&&free>=pressurePolicy.targetAvailableBytes)break;
  for(const unit of admitted){
   guard.check();
   if(pressure.pressured&&(await available()>=pressurePolicy.targetAvailableBytes||unit.bytes>pressurePolicy.maximumDeletionBytes-bytes))break;
   await deleteUnit(buildDirectory,unit,control);bytes+=unit.bytes;count++;
   console.log(`[DEBUG] Inactive Cargo build removed ${JSON.stringify({path:unit.path,count,bytes})}`);
  }
  if(!pressure.pressured)break;
  }
  console.log(`[DEBUG] Inactive Cargo build maintenance completed ${JSON.stringify({count,bytes,availableBytes:await available(),pressured:pressure.pressured})}`);
  });
 });
});
