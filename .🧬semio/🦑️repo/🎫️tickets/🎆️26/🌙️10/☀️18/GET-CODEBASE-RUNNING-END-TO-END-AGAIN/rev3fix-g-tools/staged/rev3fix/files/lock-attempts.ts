/** 🔁️ Contract: the physical lock answers "busy" only for the contention code and refuses only after a failure that persists. The error code of a failed lock call is read by a second foreign call, so anything that runs between the two (a garbage collection, a timer) may replace it; a failure whose code is not the contention code is therefore repeated up to the declared number of attempts before it is refused, and the refusal keeps the last code. The oracle is a scripted system: the sequence of codes the kernel answers with. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createPosixPhysicalLockBackendV1,createWindowsPhysicalLockBackendV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";

const protocol=schema.definitions.Protocol.const,win=schema.definitions.Windows.const;

function windows(locks:readonly boolean[],codes:readonly number[]){
 let call=0,asked=0;const system={ptr:()=>1,dlopen:()=>({close:()=>undefined,symbols:{
  CreateFileW:()=>100n,GetFileInformationByHandle:()=>1,CloseHandle:()=>1,UnlockFileEx:()=>1,
  LockFileEx:()=>locks[Math.min(call++,locks.length-1)]?1:0,
  GetLastError:()=>codes[Math.min(asked++,codes.length-1)]!,
 }})};
 return{backend:createWindowsPhysicalLockBackendV1(system as never),calls:()=>call};
}

test("the number of attempts is declared and at least two, so one clobbered error code never decides",()=>{
 expect(protocol.attempts).toBeGreaterThanOrEqual(2);
});

test("Windows: contention answers busy at once, without a repeat",()=>{
 const{backend,calls}=windows([false],[win.contentionError]);backend.retain("C:\\ws\\a.lock");
 expect(backend.tryLock()).toBe(false);expect(calls()).toBe(1);backend.close();
});

test("Windows: a clobbered error code is repeated, and the repeat decides: busy when the range is held, locked when it was free",()=>{
 const busy=windows([false,false],[0,win.contentionError]);busy.backend.retain("C:\\ws\\a.lock");
 expect(busy.backend.tryLock()).toBe(false);expect(busy.calls()).toBe(2);busy.backend.close();
 const free=windows([false,true],[5]);free.backend.retain("C:\\ws\\a.lock");
 expect(free.backend.tryLock()).toBe(true);expect(free.calls()).toBe(2);free.backend.close();
});

test("Windows: a failure that persists is refused after the declared attempts and keeps its code",()=>{
 const{backend,calls}=windows([false],[5]);backend.retain("C:\\ws\\a.lock");
 expect(()=>backend.tryLock()).toThrow("admission failed (Windows error 5)");expect(calls()).toBe(protocol.attempts);backend.close();
});

function posix(results:readonly number[],errors:readonly number[]){
 let call=0,asked=0;const system={dlopen:()=>({close:()=>undefined,symbols:{flock:()=>results[Math.min(call++,results.length-1)]!,__error:()=>1}}),read:{i32:()=>errors[Math.min(asked++,errors.length-1)]!}};
 return{backend:createPosixPhysicalLockBackendV1(system as never,"darwin","libc"),calls:()=>call};
}
const lockFile=()=>{const directory=realpathSync(mkdtempSync(join(tmpdir(),"lock-attempts-"))),path=join(directory,"a.lock");writeFileSync(path,"x");return{directory,path};};

test("POSIX: the same rules with errno: contention is busy at once, a clobbered errno is repeated, a persistent failure is refused",()=>{
 const{directory,path}=lockFile();
 try{
  const held=posix([-1],[protocol.contentionErrno.darwin]);held.backend.retain(path);expect(held.backend.tryLock()).toBe(false);expect(held.calls()).toBe(1);
  const clobbered=posix([-1,-1],[0,protocol.contentionErrno.darwin]);clobbered.backend.retain(path);expect(clobbered.backend.tryLock()).toBe(false);expect(clobbered.calls()).toBe(2);
  const free=posix([-1,0],[0]);free.backend.retain(path);expect(free.backend.tryLock()).toBe(true);expect(free.calls()).toBe(2);
  const broken=posix([-1],[9]);broken.backend.retain(path);expect(()=>broken.backend.tryLock()).toThrow("Physical compiler lock admission failed");expect(broken.calls()).toBe(protocol.attempts);
  for(const row of[held,clobbered,free,broken])row.backend.close();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
