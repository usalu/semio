# ✍️ Applies an out dir (mirror of PLUG) to the repo: re-reads and compares every pre-state before writing, keeps the pre-cut originals for the differential baseline.
import sys, os, shutil, hashlib
from common import *
out = sys.argv[1]; pre = sys.argv[2]            # out dir, pre-state sha manifest (path -> sha) produced by run4 --manifest
import json
manifest = json.load(open(pre))
shutil.rmtree(SCR + '/precut', ignore_errors=True)
bad = 0
for rel, sha in manifest.items():
    p = PLUG + '/' + rel
    cur = hashlib.sha256(open(p, 'rb').read()).hexdigest() if os.path.exists(p) else None
    if cur != sha: print('CHANGED SINCE PLAN', rel); bad += 1
if bad: sys.exit('abort: pre-state changed')
n = 0
for dp, dn, fn in os.walk(out):
    for f in fn:
        src = os.path.join(dp, f); rel = os.path.relpath(src, out); dst = PLUG + '/' + rel
        if os.path.exists(dst):
            pc = SCR + '/precut/' + rel; os.makedirs(os.path.dirname(pc), exist_ok=True); shutil.copyfile(dst, pc)
        os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(src, dst); n += 1
open(GEN + '/pwp4-touched.txt', 'w').write('\n'.join(sorted(os.path.relpath(os.path.join(dp, f), out) for dp, dn, fn in os.walk(out) for f in fn)) + '\n')
print('applied', n, 'files')
