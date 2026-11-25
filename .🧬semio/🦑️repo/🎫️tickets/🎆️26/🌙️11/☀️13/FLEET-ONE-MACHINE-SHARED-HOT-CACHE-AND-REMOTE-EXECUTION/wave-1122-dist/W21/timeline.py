"""🧭️ W21: prints the changes of a probe timeline (phase, lock holder, identity file, spawn stamp, spool) one line each."""
import json,sys
prev=None
for line in open(sys.argv[1]):
    r=json.loads(line)
    if 'summary' in r: continue
    key=(r['phase'],r['holder'],r['identity'],r.get('stamp'),len(r['jobs']))
    if key!=prev: print(r['t'],r['phase'],r['holder'],r['identity'],r.get('stamp'),len(r['jobs']),r['jobs'][:2]); prev=key
