#!/usr/bin/env python3
"""Package O: an offloaded action that one worker's policy refuses goes to another worker (a machine that has not yet taken new code must not decide for the fleet). Usage: patch-policy-retry.py <🌐️remote-execution dir> [--dry]"""
import sys, pathlib
RE = pathlib.Path(sys.argv[1]); DRY = "--dry" in sys.argv
S = RE / "🗓️scheduler/🟦️.ts"; T = RE / "🗓️scheduler/🧪️tests/🟦️.ts"
s = S.read_text(); t = T.read_text()
old = '''   if([10,14].includes(result.status.code)&&operation.failures+1<this.options.maximumAttempts){if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,result.status.message,true);}'''
new = '''   const policy=result.status.code===9&&operation.requester!==undefined&&result.status.message.startsWith("Action refused by the worker policy");
   if(([10,14].includes(result.status.code)||policy)&&operation.failures+1<this.options.maximumAttempts){if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,result.status.message,true);}'''
assert s.count(old) == 1
s = s.replace(old, new)
add = '''
test("an offloaded action that one worker's policy refuses runs on another worker; when every worker refuses it fails back at once with the refusal, and a non-offloaded action keeps the refusal as its result",async()=>{
 const {memory,scheduler}=harness({maximumAttempts:3}),requested=[{name:"semio-origin",value:"requester"}];
 const stale=scheduler.register(profile("stale")),fresh=scheduler.register(profile("fresh"));
 const run=submit(scheduler,memory.action(requested,"policy-1"));await Bun.sleep(20);
 const first=await polled(scheduler,stale.id);expect(first.task).not.toBeNull();
 scheduler.report(stale.id,first.task!.id,{version:1,outcome:"failed",status:{code:9,message:"Action refused by the worker policy: -Z embed-metadata=no is not allowed"},actionResult:null});
 const second=await polled(scheduler,fresh.id,2000);expect(second.task?.attempt).toBe(2);
 scheduler.report(fresh.id,second.task!.id,resultOf());
 await run.done;expect(run.rows.at(-1)!.outcome!.status.code).toBe(0);
 const all=harness({maximumAttempts:3}),only=all.scheduler.register(profile("only")),runAll=submit(all.scheduler,all.memory.action(requested,"policy-2"));await Bun.sleep(20);
 const given=await polled(all.scheduler,only.id);
 all.scheduler.report(only.id,given.task!.id,{version:1,outcome:"failed",status:{code:9,message:"Action refused by the worker policy: --sysroot is not allowed"},actionResult:null});
 await runAll.done;const status=runAll.rows.at(-1)!.outcome!.status;expect(status.message).toContain("Action refused by the worker policy: --sysroot is not allowed");expect(status.message).toContain("no other machine can take the action");
 const plain=harness({maximumAttempts:3}),one=plain.scheduler.register(profile("one")),runPlain=submit(plain.scheduler,plain.memory.action([],"policy-3"));await Bun.sleep(20);
 const taken=await polled(plain.scheduler,one.id);
 plain.scheduler.report(one.id,taken.task!.id,{version:1,outcome:"failed",status:{code:9,message:"Action refused by the worker policy: -o is not allowed"},actionResult:null});
 await runPlain.done;expect(runPlain.rows.at(-1)!.outcome!.status).toEqual({code:9,message:"Action refused by the worker policy: -o is not allowed"});
});
'''
t = t.rstrip("\n") + "\n" + add
print("edit scheduler + test")
if not DRY:
    S.write_text(s); T.write_text(t)
