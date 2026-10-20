#!/usr/bin/env python3
"""Group 3: live wiring of the unit-wise check into `typecheck` (needs group 1, the engine). Dry run by default.
Creates the wrapper, selection schema, fixtures and tests (create-only) and applies the anchored edits of edits.json with the group 2 engine.
--apply writes everything or nothing; --undo restores edits and removes the created files that still equal the staged bytes."""
import os,sys,shutil,hashlib,subprocess,json
HERE=os.path.dirname(os.path.abspath(__file__)); R=os.path.abspath(os.path.join(HERE,*['..']*9))
if '--root' in sys.argv: R=sys.argv[sys.argv.index('--root')+1]
NEW=os.path.join(HERE,'new'); FX=os.path.join(HERE,'..','staged2','fixes.py')
L=os.path.join(R,'🧰️framework','🛍️products','🦑️repo','🔨️modules','📚️library')
def rels():
    for r,_,fs in os.walk(NEW):
        for f in sorted(fs): yield os.path.relpath(os.path.join(r,f),NEW)
def dg(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
def run(extra): return subprocess.run([sys.executable,FX,'--root',R,'--data',os.path.join(HERE,'edits.json')]+extra,capture_output=True,text=True)
a=sys.argv[1:]
if '--undo' in a:
    print(run(['--undo']).stdout.strip())
    for rel in rels():
        d=os.path.join(R,rel)
        if os.path.exists(d) and dg(d)==dg(os.path.join(NEW,rel)): os.remove(d);print('removed',rel[-60:])
    sys.exit(0)
engine=os.path.join(L,'🏗️build','🟦️typescript','🧩️units','🟦️.ts')
if not os.path.exists(engine): print('REFUSED: group 1 (engine) is not applied');sys.exit(1)
ops=[];refuse=[]
for rel in sorted(rels()):
    s=os.path.join(NEW,rel);d=os.path.join(R,rel)
    if os.path.exists(d): (ops.append(('same',rel)) if dg(d)==dg(s) else refuse.append(rel))
    else: ops.append(('create',rel))
for k,rel in ops: print(k,rel[-80:])
for rel in refuse: print('DRIFT',rel[-80:])
r=run([]);print(r.stdout.strip())
if refuse or r.returncode: sys.exit(1)
if '--apply' not in a: print('dry run: pass --apply');sys.exit(0)
made=[]
try:
    for k,rel in ops:
        if k!='create': continue
        d=os.path.join(R,rel);os.makedirs(os.path.dirname(d),exist_ok=True);tmp=d+'.tmp';shutil.copyfile(os.path.join(NEW,rel),tmp);os.replace(tmp,d);made.append(d)
    r=run(['--apply']);print(r.stdout.strip())
    if r.returncode: raise RuntimeError(r.stdout)
except Exception as e:
    for d in made: os.remove(d)
    print('rolled back',e);sys.exit(1)
print('applied',len(made),'files created')
