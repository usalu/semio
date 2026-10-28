"""usage: patch-leases-drain.py <🔒️leases/🟦️.ts> — restores the drain of waitForLeaseContinuation: after a stop (deadline or abort) wins the race, the accepted continuation is awaited to settlement before the acquisition releases its database; the first stop stays the primary failure (law: lease continuations drain before acquisition releases its database after a genuine child stop)."""
import sys
p=sys.argv[1];s=open(p).read()
old='finally{void pending.catch(()=>undefined);options.signal.removeEventListener("abort",cancel);'
new='finally{try{await pending;}catch(error){if(!failed){failed=true;failure=error;}}options.signal.removeEventListener("abort",cancel);'
assert s.count(old)==1
open(p,'w').write(s.replace(old,new))
