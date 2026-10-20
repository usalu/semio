#!/usr/bin/env python3
"""Applies staged/ops.json to the repository (or to an overlay root): `check` (default, nothing written), `diff`, `apply [--write]`. Every anchor must occur exactly once or its replacement must already be present (idempotent); any drift refuses everything. All files are written or none (temporary file + rename, rollback on error)."""
import json,os,pathlib,subprocess,sys,tempfile,difflib
HERE=pathlib.Path(__file__).resolve().parent
def repo_root():
    p=HERE
    while p!=p.parent:
        if (p/".git").exists():return p
        p=p.parent
    raise SystemExit("repository root not found")
args=sys.argv[1:]
command=args[0] if args and not args[0].startswith("--") else "check"
root=pathlib.Path(args[args.index("--root")+1]) if "--root" in args else repo_root()
write="--write" in args
ops=json.loads((HERE/"staged/ops.json").read_text(encoding="utf8"))
plan=[];drift=[]
by_file={}
for op in ops["edits"]:by_file.setdefault(op["file"],[]).append(op)
for file,items in by_file.items():
    path=root/file
    if not path.exists():drift.append((file,"missing file"));continue
    text=path.read_text(encoding="utf8");new=text;done=0
    for op in items:
        a,r=op["anchor"],op["replacement"]
        if r in new:done+=1;continue
        n=new.count(a)
        if n!=1:drift.append((file,f"anchor occurs {n} times: {a[:70]!r}"));continue
        new=new.replace(a,r)
    if new!=text:plan.append((path,text,new))
for item in ops["newFiles"]:
    path=root/item["file"];data=(HERE/item["from"]).read_text(encoding="utf8")
    if len(str(path))>256:drift.append((item["file"],"path longer than 256"))
    if path.exists():
        if path.read_text(encoding="utf8")!=data:drift.append((item["file"],"exists with different content"))
        continue
    plan.append((path,None,data))
print(f"{len(plan)} file(s) pending, {len(drift)} drift")
for file,why in drift:print("DRIFT",file[-80:],why)
if command=="diff":
    for path,old,new in plan:
        sys.stdout.writelines(difflib.unified_diff((old or "").splitlines(True),new.splitlines(True),str(path)[-60:]+" (live)",str(path)[-60:]+" (staged)",n=1))
if command=="apply":
    if drift:raise SystemExit("refused: drift")
    if not write:print("dry run (add --write)");sys.exit(0)
    written=[]
    try:
        for path,old,new in plan:
            path.parent.mkdir(parents=True,exist_ok=True)
            fd,tmp=tempfile.mkstemp(dir=path.parent,prefix=".loop-hunt-");os.close(fd)
            pathlib.Path(tmp).write_text(new,encoding="utf8");os.chmod(tmp, os.stat(path).st_mode & 0o7777 if os.path.exists(path) else 0o644);os.replace(tmp,path);written.append((path,old))
    except Exception:
        for path,old in written:
            path.write_text(old,encoding="utf8") if old is not None else path.unlink()
        raise
    print("applied",len(written))
sys.exit(1 if drift else 0)
