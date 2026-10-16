import sys, os, shutil, subprocess, difflib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host
from carve import REPO
OUT = REPO + '/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/phc-stage'
def dump(ws):
    if os.path.exists(OUT): shutil.rmtree(OUT)
    os.makedirs(OUT)
    n=0
    for p in ws.changed():
        if p in ws.deleted: continue
        rel = os.path.relpath(p, REPO).replace('/', '__')
        open(os.path.join(OUT, rel), 'w', encoding='utf-8').write(ws.cur[p])
        n+=1
    return n
if __name__=='__main__':
    ws, C, roots, base_ws = plan_host.prepare()
    print('dumped', dump(ws))
    import pickle
    for p in ws.changed():
        if p in ws.deleted: continue
        old = ws.orig.get(p)
        new = ws.cur[p]
        if old is None:
            print('NEW ', os.path.relpath(p, plan_host.H), len(new.split('\n')))
        else:
            d = list(difflib.unified_diff(old.split('\n'), new.split('\n'), lineterm='', n=0))
            add = sum(1 for l in d if l.startswith('+') and not l.startswith('+++'))
            rem = sum(1 for l in d if l.startswith('-') and not l.startswith('---'))
            print('MOD ', os.path.relpath(p, plan_host.H), f'+{add} -{rem}')
    for p in ws.deleted: print('DEL ', os.path.relpath(p, plan_host.H))
