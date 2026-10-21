#!/usr/bin/env python3
"""Anchored apply of the staged conc-tests set (per-run test sandbox, runtime guard, concurrency law, scratch-root fixes).
usage: apply.py [--root DIR] [--write] | apply.py [--root DIR] --undo
Default is a DRY RUN. Every anchor must occur exactly the declared number of times, new files must be absent or identical, the whole set is verified before the first write and rolled back on any failure; backups go to C/🗑️generated/conc-tests/backup (live) or ROOT.backup. Refuses on a live root while a dependency sync or `show projects` runs."""
import json,os,shutil,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import manifest as m
args=sys.argv[1:]
write="--write" in args; undo="--undo" in args
LIVE="/Users/ueli/Documents/semio"
root=args[args.index("--root")+1].rstrip("/") if "--root" in args else LIVE
live=os.path.realpath(root)==os.path.realpath(LIVE)
BK0=HERE+"/../🗑️generated/conc-tests/backup" if live else root+".backup"
import time
update="--update" in args
runs=sorted(os.listdir(BK0)) if os.path.isdir(BK0) else []
BK=BK0+"/"+(runs[-1] if undo and runs else time.strftime("%Y%m%d-%H%M%S"))
if live and write and "--allow-running" not in args:
    ps=subprocess.run(["ps","-axo","command"],capture_output=True,text=True).stdout
    if any(("dependencies sync" in l or "show projects" in l) and "apply.py" not in l for l in ps.split("\n")): sys.exit("refused: dependency sync or show projects running")
if undo:
    if not runs: sys.exit("nothing to undo")
    index=json.load(open(BK+"/index.json",encoding="utf8"))
    for path,name in index.items():
        if name is None:
            if os.path.exists(path):
                os.remove(path); print("removed",path[len(root)+1:][-90:])
                folder=os.path.dirname(path)
                while folder!=root:
                    try: os.rmdir(folder)
                    except OSError: break
                    folder=os.path.dirname(folder)
        else: shutil.copyfile(BK+"/"+name,path); print("restored",path[len(root)+1:][-90:])
    shutil.rmtree(BK); sys.exit(0)
changes=[]
for staged,rel in m.NEW.items():
    path=root+"/"+rel; data=open(HERE+"/staged/"+staged,encoding="utf8").read()
    if os.path.exists(path):
        have=open(path,encoding="utf8").read()
        if have!=data:
            if not update: sys.exit("refused: new file exists with other content (use --update for a staged revision): "+rel[-90:])
            import hashlib
            if hashlib.sha256(have.encode("utf8")).hexdigest()!=json.load(open(HERE+"/prev.json",encoding="utf8")).get(rel): sys.exit("refused: live file changed since staging: "+rel[-90:])
            changes.append((path,have,data)); continue
        print("already present:",rel[-80:]); continue
    changes.append((path,None,data))
for edit in json.load(open(HERE+"/edits.json",encoding="utf8")):
    path=root+"/"+edit["target"]
    if not os.path.exists(path): sys.exit("refused: target missing: "+edit["target"][-90:])
    text=open(path,encoding="utf8").read()
    if all(p["new"] in text for p in edit["pairs"]) and not any(p["old"] in text and p["old"] not in p["new"] for p in edit["pairs"]): print("already applied:",edit["target"][-80:]); continue
    out=text
    for p in edit["pairs"]:
        have=text.count(p["old"])
        if have!=p["count"]: sys.exit(f"refused: drift, anchor occurs {have} != {p['count']} times in {edit['target'][-90:]}: {p['old'][:100]!r}")
    for p in edit["pairs"]: out=out.replace(p["old"],p["new"])
    changes.append((path,text,out))
for path,old,new in changes: print("would write" if not write else "writing",("new  " if old is None else "edit ")+path[len(root)+1:][-100:])
print(len(changes),"file(s) change;",len(m.NEW),"new files declared")
if not write: print("dry run: nothing written (use --write)"); sys.exit(0)
if not changes: print("nothing to do"); sys.exit(0)
os.makedirs(BK,exist_ok=True); index={}; done=[]
try:
    for i,(path,old,new) in enumerate(changes):
        if old is None: index[path]=None
        else: name=f"{i:03d}.bak"; shutil.copyfile(path,BK+"/"+name); index[path]=name
        os.makedirs(os.path.dirname(path),exist_ok=True); open(path,"w",encoding="utf8").write(new); done.append((path,old))
except Exception as error:
    for path,old in reversed(done):
        if old is None: os.remove(path)
        else: open(path,"w",encoding="utf8").write(old)
    sys.exit(f"rolled back: {error}")
json.dump(index,open(BK+"/index.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print("applied; undo with: apply.py --undo")
