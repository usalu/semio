#!/usr/bin/env python3
"""Applies the staged backend-contract set: creates the new module files and runs the anchored edits of wire.py (several edits of one file are chained on its running text). Dry run by default; --write applies all or nothing with hashed backups under ../🗑️generated/backend-contract-apply/<stamp>; --undo <stamp-dir> restores. A create refuses when its target exists with other content; an edit refuses when an anchor is missing or ambiguous (the file drifted: rebuild with gen-backends.py and build.py); paths over 255 characters are refused; every file is re-read right before it is replaced so a concurrent editor is never overwritten; refuses while a dependency sync or a chain is rewriting the graph unless --force-running."""
import argparse,hashlib,json,os,shutil,subprocess,sys,tempfile,time
from collections import OrderedDict
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
ap=argparse.ArgumentParser();ap.add_argument("--write",action="store_true");ap.add_argument("--root",default=names.R);ap.add_argument("--undo");ap.add_argument("--force-running",action="store_true")
a=ap.parse_args()
generated=os.path.normpath(os.path.join(here,"..","🗑️generated","backend-contract-apply"))
def read(path):
    try:return open(path,encoding="utf8").read()
    except FileNotFoundError:return None
def sha(text):return hashlib.sha256(text.encode()).hexdigest()
if a.undo:
    manifest=json.load(open(os.path.join(a.undo,"backup.json"),encoding="utf8"))
    for row in manifest["files"]:
        path=os.path.join(a.root,row["path"])
        if row["backup"] is None:
            if os.path.exists(path):os.remove(path)
        else:shutil.copyfile(os.path.join(a.undo,row["backup"]),path)
    print("restored",len(manifest["files"]));sys.exit(0)
if not a.force_running and a.write:
    out=subprocess.run(["ps","-axo","command"],capture_output=True,text=True).stdout
    busy=[l for l in out.splitlines() if "script.ts" in l and any(w in l for w in ("show projects","tools setup","dependencies sync"))]
    if busy:sys.exit("REFUSED: a long repository command is running (use --force-running to override):\n"+"\n".join(busy[:3]))
manifest=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"))
plan=[];problems=[]
for flat,row in manifest.items():
    new=open(os.path.join(here,"staged",flat),encoding="utf8").read();path=os.path.join(a.root,row["destination"]);old=read(path)
    if len(path)>255:problems.append(f"path longer than 255: {path}")
    if old is None:plan.append(("create",row["destination"],None,new))
    elif old==new:plan.append(("same",row["destination"],old,new))
    else:problems.append(f"create target exists with other content: {path}")
groups=OrderedDict()
for rel,fn in wire.EDITS:groups.setdefault(rel,[]).append(fn)
for rel,fns in groups.items():
    old=read(os.path.join(a.root,rel))
    if old is None:problems.append(f"edit target missing: {rel}");continue
    new=old
    try:
        for fn in fns:new=fn(new)
    except wire.Drift as error:problems.append(str(error));continue
    plan.append(("same" if new==old else "edit",rel,old,new))
for kind,rel,old,new in plan:print(f"{kind:7} {rel[-150:]}")
if problems:
    print("REFUSED (nothing written):");[print(" -",p) for p in problems];sys.exit(2)
changes=[row for row in plan if row[0]!="same"]
print(f"{len(changes)} change(s), {len(plan)-len(changes)} already applied")
if not a.write:print("dry run: pass --write to apply");sys.exit(0)
stamp=time.strftime("%Y%m%d-%H%M%S");backup=os.path.join(generated,stamp);os.makedirs(backup,exist_ok=True)
rows=[];staged=[]
for kind,rel,old,new in changes:
    path=os.path.join(a.root,rel);os.makedirs(os.path.dirname(path),exist_ok=True)
    handle,temp=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".bkc-");os.close(handle)
    open(temp,"w",encoding="utf8").write(new);staged.append((path,temp,old,rel))
    if old is None:rows.append({"path":rel,"backup":None})
    else:
        name=sha(rel)[:16];open(os.path.join(backup,name),"w",encoding="utf8").write(old);rows.append({"path":rel,"backup":name,"sha256":sha(old)})
json.dump({"files":rows},open(os.path.join(backup,"backup.json"),"w",encoding="utf8"),indent=1)
try:
    for path,temp,old,rel in staged:
        if read(path)!=old:raise SystemExit(f"drift during apply: {path}")
    for path,temp,old,rel in staged:
        os.chmod(temp,(os.stat(path).st_mode&0o7777) if os.path.exists(path) else 0o644)
        os.replace(temp,path)
finally:
    for path,temp,old,rel in staged:
        if os.path.exists(temp):os.remove(temp)
print("applied",len(staged),"; undo with: python3 apply.py --undo",backup)
print("next: python3 regen-help.py (regenerates the help snapshots), then the explicit-path tests listed in the result")
