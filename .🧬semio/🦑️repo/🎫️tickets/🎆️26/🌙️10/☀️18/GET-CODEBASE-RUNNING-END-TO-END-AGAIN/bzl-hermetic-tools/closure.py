#!/usr/bin/env python3
"""Closure helper: copies the import closure of entries (bun build metafile) plus extra files into an overlay root `s/`.
usage: closure.py <overlay-s-dir> --entry <repo-relative file>... --extra <repo-relative file>... [--list]"""
import argparse,json,os,shutil,subprocess,sys,tempfile
ROOT=os.environ.get("SEMIO_ROOT","/Users/ueli/Documents/semio")
ap=argparse.ArgumentParser();ap.add_argument("dest");ap.add_argument("--entry",action="append",default=[]);ap.add_argument("--extra",action="append",default=[]);ap.add_argument("--list",action="store_true")
a=ap.parse_args()
files=set(a.extra)
for e in a.entry:
    with tempfile.TemporaryDirectory(dir="/private/tmp") as t:
        meta=os.path.join(t,"m.json")
        r=subprocess.run(["bun","build","--target=bun","--outdir",os.path.join(t,"o"),"--metafile="+meta,os.path.join(ROOT,e)],cwd=ROOT,capture_output=True,text=True)
        if r.returncode:sys.exit(r.stderr)
        for k in json.load(open(meta))["inputs"]:
            p=os.path.normpath(os.path.join(ROOT,k)) if not os.path.isabs(k) else k
            files.add(os.path.relpath(p,ROOT))
files={f for f in files if not f.startswith("..")}
sib=set();seen=set()
def walk(base,depth):
    try: names=os.listdir(base)
    except OSError: return
    for n in names:
        p=os.path.join(base,n)
        if n=="🧬️schema" and os.path.isdir(p):
            for dp,dn,fn in os.walk(p):
                for m in fn: sib.add(os.path.relpath(os.path.join(dp,m),ROOT))
        elif depth<3 and os.path.isdir(p) and not os.path.islink(p) and n not in ("node_modules",".git",".cache"):
            walk(p,depth+1)
for f in files:
    d=os.path.dirname(f)
    if d in seen: continue
    seen.add(d); walk(os.path.join(ROOT,d),1)
files|=sib
if a.list:
    print("\n".join(sorted(files)));sys.exit()
for f in sorted(files):
    d=os.path.join(a.dest,f);os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(os.path.join(ROOT,f),d)
print(len(files),"files")
