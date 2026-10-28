/** 🎛️ Contract: the queued lease port of the claim registry hands the process lease module the signal of the issued control whenever it has one (the module refuses a control next to any other signal, `Original lease signal identity required`), while a caller's own abort still ends its wait at once, a lease granted after that is released exactly once, and the waiters of one resource share one queued ticket instead of queueing a new one per poll.  The refusal rule is that of `remaining` in the process module's leases, copied into the stub.
 * @see ../../🟨️.mjs — the port */
import{expect,test}from"bun:test";
import{createQueuedClaimLeasePortV1}from"../../🟨️.mjs";

interface Request{readonly resource:string;readonly mode:string;readonly signal:AbortSignal;readonly control?:{readonly signal:AbortSignal}}
const issued=()=>({signal:new AbortController().signal});
const lender=()=>{
 const asked:Request[]=[],grants:(()=>void)[]=[],released:number[]=[];
 const acquire=(request:Request)=>{
  asked.push(request);
  if(request.control&&request.control.signal!==request.signal)return Promise.reject(Error("Original lease signal identity required"));
  return new Promise<{release():void}>(accept=>{grants.push(()=>accept({release:()=>released.push(asked.length)}));});
 };
 return{asked,grants,released,acquire:acquire as never};
};
const settle=async()=>{for(let turn=0;turn<5;turn++)await Promise.resolve();await Bun.sleep(1);};

test("a wait or a hold under an issued control asks the lease module with the control's own signal, whatever signal the caller derived",async()=>{
 const control=issued(),bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,directory:"d",control});
 const derived=new AbortController(),waited=port.wait("r",{signal:derived.signal}),held=port.hold("h",{signal:derived.signal});
 await settle();
 expect(bank.asked.map(request=>request.signal)).toEqual([control.signal,control.signal]);
 for(const grant of bank.grants)grant();
 await waited;(await held).release();
});

test("a derived signal that aborts ends the wait at once and the lease granted late is released exactly once",async()=>{
 const control=issued(),bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,directory:"d",control});
 const derived=new AbortController(),held=port.hold("h",{signal:derived.signal});
 await settle();derived.abort(Error("stop"));
 await expect(held).rejects.toThrow("stop");
 bank.grants[0]!();await settle();
 expect(bank.released).toHaveLength(1);
});

test("the waiters of one resource share one queued ticket however often they poll",async()=>{
 const control=issued(),bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,directory:"d",control});
 const polls=[0,1,2,3,4].map(()=>{const derived=new AbortController(),wait=port.wait("r",{signal:derived.signal}).catch(()=>undefined);queueMicrotask(()=>derived.abort());return wait;});
 await Promise.all(polls);
 expect(bank.asked).toHaveLength(1);
 bank.grants[0]!();await settle();
 expect(bank.released).toHaveLength(1);
});

test("without a control the caller's own signal reaches the lease module, so an abandoned ticket leaves the queue",async()=>{
 const bank=lender(),port=createQueuedClaimLeasePortV1({acquire:bank.acquire,directory:"d"}),own=new AbortController(),held=port.hold("h",{signal:own.signal});
 await settle();
 expect(bank.asked.map(request=>request.signal)).toEqual([own.signal]);
 bank.grants[0]!();(await held).release();
});
