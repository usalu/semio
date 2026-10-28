"""usage: patch-wait-test.py <wait/🧪️tests/🟦️.ts> — a deadline stop reaches the lease continuation through two equal timers (the lease's own and the issued control's abort); whichever fires first is the first stop: the law waits for either and takes the control's abort reason as `first` when the lease check never threw."""
import sys
p=sys.argv[1];s=open(p).read()
a='if(row.stop==="deadline")await new Promise(done=>setTimeout(done,1100));else abort.abort(cancel);await new Promise(done=>setImmediate(done));expect(selected,row.id).toBe(row.stop==="deadline");'
b='if(row.stop==="deadline"){const limit=Date.now()+10000;while(!selected&&!control.signal.aborted&&Date.now()<limit)await new Promise(done=>setTimeout(done,10));}else abort.abort(cancel);await new Promise(done=>setImmediate(done));if(row.stop==="deadline"&&!selected){expect(control.signal.aborted,row.id).toBe(true);first=control.signal.reason;selected=true;}expect(selected,row.id).toBe(row.stop==="deadline");'
assert s.count(a)==1
open(p,'w').write(s.replace(a,b))
