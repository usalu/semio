#!/usr/bin/env python3
"""Applies the staged concurrent-session harness: group `core` creates the nine module files and runs the anchored edits of wire.py; group `trace` adds the lease trace emitter to the lease module (optional, the harness measures waits only as lower bounds without it); group `all` is both. Dry run by default; --write applies all or nothing. A create refuses when its target exists with other content, an edit refuses when its anchor is missing or ambiguous, every JSON result is parsed before anything is written, and every file is re-read right before it is replaced so a concurrent editor is never overwritten."""
import argparse,json,os,sys,tempfile
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
ap=argparse.ArgumentParser();ap.add_argument("--write",action="store_true");ap.add_argument("--root",default=names.R);ap.add_argument("--group",choices=["core","trace","all"],default="core")
a=ap.parse_args()
manifest=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"))
plan=[];problems=[]
def read(path):
    try:return open(path,encoding="utf8").read()
    except FileNotFoundError:return None
edits=[]
if a.group in ("core","all"):
    for flat,row in manifest.items():
        new=open(os.path.join(here,"staged",flat),encoding="utf8").read();path=os.path.join(a.root,row["destination"]);old=read(path)
        if len(path)>255:problems.append(f"path longer than 255: {path}")
        if old is None:plan.append(("create",path,None,new))
        elif old==new:plan.append(("same",path,old,new))
        else:problems.append(f"create target exists with other content: {path}")
    edits+=wire.EDITS
if a.group in ("trace","all"):edits+=wire.TRACE_EDITS
for rel,fn in edits:
    path=os.path.join(a.root,rel);old=read(path)
    if old is None:problems.append(f"edit target missing: {path}");continue
    try:new=fn(old)
    except wire.Drift as error:problems.append(str(error));continue
    if path.endswith(".json"):
        try:json.loads(new)
        except ValueError as error:problems.append(f"result is not JSON: {path}: {error}");continue
    plan.append(("same" if new==old else "edit",path,old,new))
for kind,path,old,new in plan:print(f"{kind:7} {os.path.relpath(path,a.root)[-150:]}")
if problems:
    print("REFUSED (nothing written):");[print(" -",p) for p in problems];sys.exit(2)
changes=[row for row in plan if row[0]!="same"]
print(f"{len(changes)} change(s), {len(plan)-len(changes)} already applied")
if not a.write:print("dry run: pass --write to apply");sys.exit(0)
staged=[]
for kind,path,old,new in changes:
    os.makedirs(os.path.dirname(path),exist_ok=True)
    handle,temp=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".conc-");os.close(handle)
    open(temp,"w",encoding="utf8").write(new);staged.append((path,temp,old))
try:
    for path,temp,old in staged:
        if read(path)!=old:raise SystemExit(f"drift during apply: {path}")
    for path,temp,old in staged:
        os.chmod(temp, os.stat(path).st_mode & 0o7777 if os.path.exists(path) else 0o644)
        os.replace(temp,path)
finally:
    for path,temp,old in staged:
        if os.path.exists(temp):os.remove(temp)
print("applied",len(staged))
