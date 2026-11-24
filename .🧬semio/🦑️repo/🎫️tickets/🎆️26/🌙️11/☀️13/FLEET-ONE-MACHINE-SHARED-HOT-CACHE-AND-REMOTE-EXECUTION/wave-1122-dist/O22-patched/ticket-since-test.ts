import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireSlotV1,tryAcquireSlotV1} from "../../🔒️lock/🟦️.ts";
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
test("a waiter that gives the queue up and comes back with its arrival time is served before the waiters that queued meanwhile",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-ticket-"));roots.push(root);
 const arrival=Date.now()-60_000,held=tryAcquireSlotV1(root,1)!,order:string[]=[];
 const later=acquireSlotV1(root,1).then(claim=>{order.push("later");claim.release();});
 await new Promise(done=>setTimeout(done,300));
 const returned=acquireSlotV1(root,1,{since:arrival}).then(claim=>{order.push("returned");claim.release();});
 await new Promise(done=>setTimeout(done,300));
 held.release();
 await Promise.all([later,returned]);
 expect(order).toEqual(["returned","later"]);
});
test("a ticket never takes a turn that lies in the future: a later arrival time is ignored",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-ticket-"));roots.push(root);
 const held=tryAcquireSlotV1(root,1)!,order:string[]=[];
 const first=acquireSlotV1(root,1).then(claim=>{order.push("first");claim.release();});
 await new Promise(done=>setTimeout(done,300));
 const second=acquireSlotV1(root,1,{since:Date.now()+3_600_000}).then(claim=>{order.push("second");claim.release();});
 await new Promise(done=>setTimeout(done,300));
 held.release();
 await Promise.all([first,second]);
 expect(order).toEqual(["first","second"]);
});
