import sys,hashlib
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/"
P=B+"🗓️scheduler/🧪️tests/🟦️.ts"
s=open(P,encoding="utf-8").read();before=hashlib.sha256(s.encode()).hexdigest()[:12]
old=" return {memory,scheduler,events,advance:(seconds:number)=>{now+=seconds*1000;scheduler.sweep();}};"
if s.count(old)!=1:sys.exit("harness anchor")
s=s.replace(old," return {memory,scheduler,events,advance:(seconds:number)=>{now+=seconds*1000;scheduler.sweep();},elapse:(seconds:number)=>{now+=seconds*1000;}};")
s=s.rstrip("\n")+'''

test("a machine that registers again under the same name keeps its one entry and its id: no orphan entry that nothing heartbeats is left to be reported as lost",async()=>{
 const {scheduler,events,advance}=harness(),first=scheduler.register(profile("one")),second=scheduler.register(profile("one")),third=scheduler.register(profile("one"));
 expect(second.id).toBe(first.id);expect(third.id).toBe(first.id);
 expect(scheduler.status().workers).toHaveLength(1);
 expect(scheduler.status().counters.workersReused).toBe(2);
 expect(events.filter(event=>event.startsWith("worker-joined"))).toHaveLength(1);
 scheduler.heartbeat(first.id,[]);advance(5);scheduler.heartbeat(first.id,[]);advance(5);
 expect(events.filter(event=>event.startsWith("worker-lost"))).toHaveLength(0);
 const other=scheduler.register(profile("two"));expect(other.id).not.toBe(first.id);expect(scheduler.status().workers).toHaveLength(2);
});

test("a beat of the sweep timer that came back far too late judges no worker, because the heartbeats wait unread in a stalled process; the next beat judges on fresh marks",async()=>{
 const {scheduler,events,elapse}=harness(),alive=scheduler.register(profile("alive")),silent=scheduler.register(profile("silent"));
 elapse(40);scheduler.tick(1000);
 expect(scheduler.status().workers).toHaveLength(2);expect(scheduler.status().counters.sweepStalls).toBe(1);expect(events.filter(event=>event.startsWith("worker-lost"))).toHaveLength(0);
 scheduler.heartbeat(alive.id,[]);elapse(1);scheduler.tick(1000);
 expect(scheduler.status().workers.map(row=>row.id)).toEqual([alive.id]);expect(silent.id).not.toBe(alive.id);
 expect(events.filter(event=>event.startsWith("worker-lost"))).toHaveLength(1);expect(scheduler.status().counters.sweepStalls).toBe(1);
});

test("an action whose requester is gone is not placed during the linger and is counted as cancelled before start, not logged per cancel; the waits of placed actions are measured",async()=>{
 const {memory,scheduler,events,elapse}=harness({lingerSeconds:0.4}),gone=memory.action([],"gone"),kept=memory.action([],"kept"),run=submit(scheduler,gone);
 await Bun.sleep(20);run.abort.abort();await run.done;
 const worker=scheduler.register(profile("w"));
 expect((await polled(scheduler,worker.id,30)).task).toBeNull();
 await Bun.sleep(500);
 expect(scheduler.status().counters).toMatchObject({cancelled:1,cancelledQueued:1,cancelledExecuting:0});
 expect(events.some(event=>event.startsWith("cancelled"))).toBe(false);
 const next=submit(scheduler,kept);await Bun.sleep(20);elapse(3);
 const task=(await polled(scheduler,worker.id)).task!;expect(task.action).toEqual(kept);
 expect(scheduler.status().counters).toMatchObject({queueWaitSamples:1,queueWaitP50Milliseconds:3000,queueWaitP90Milliseconds:3000,queueWaitMaxMilliseconds:3000});
 expect(scheduler.report(worker.id,task.id,resultOf())).toBe(true);await next.done;
});
'''
open(P,"w",encoding="utf-8").write(s)
print(before,"->",hashlib.sha256(s.encode()).hexdigest()[:12])
