#!/usr/bin/env python3
"""Builds a verification overlay under /private/tmp/bkc/ov: a root of symlinks to the live checkout in which only the staged files (and the anchored edits' results) are real files. Untouched directories stay symlinks, so nothing live is ever written. Usage: mkoverlay.py [overlay-dir]"""
import json,os,shutil,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
args=[a for a in sys.argv[1:] if not a.startswith("--")]
ov=args[0] if args else "/private/tmp/bkc/ov"
live=names.R
SKIP={".git",".cache",".🧬semio",".nx","target","dist"}
shutil.rmtree(ov,ignore_errors=True);os.makedirs(ov)
real=set()
def realize(rel):
    if rel in real:return
    if rel!='':realize(os.path.dirname(rel))
    path=os.path.join(ov,rel)
    if os.path.islink(path):os.remove(path)
    os.makedirs(path,exist_ok=True);real.add(rel)
    src=os.path.join(live,rel)
    for name in (os.listdir(src) if os.path.isdir(src) else []):
        if rel=="" and name in SKIP:continue
        target=os.path.join(path,name)
        if not os.path.lexists(target):os.symlink(os.path.join(src,name),target)
def put(rel,text):
    realize(os.path.dirname(rel))
    path=os.path.join(ov,rel)
    if os.path.islink(path):os.remove(path)
    open(path,"w",encoding="utf8").write(text)
if "--physical" in sys.argv:
    # physical APFS clones of the library and the process framework: the revision capture refuses symlinked ancestors
    for rel in (names.L,f"{names.FRAMEWORK}/{names.MODULES}"):
        realize(os.path.dirname(rel))
        target=os.path.join(ov,rel)
        if os.path.islink(target):os.remove(target)
        subprocess.run(["cp","-cR",os.path.join(live,rel),target],check=True)
manifest=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"))
for flat,row in manifest.items():put(row["destination"],open(os.path.join(here,"staged",flat),encoding="utf8").read())
import wire
from collections import OrderedDict
groups=OrderedDict()
for rel,fn in wire.EDITS:groups.setdefault(rel,[]).append(fn)
for rel,fns in groups.items():
    src=open(os.path.join(live,rel),encoding="utf8").read()
    for fn in fns:src=fn(src)
    put(rel,src)
FIX=os.path.join(names.B,names.T['OPERATIONS'],names.T['TESTS'],names.T['FIXTURES'],names.T['HELP'],names.JSON)
put(FIX,open(os.path.join(live,FIX),encoding='utf8').read())
for root_file in (names.n(0x1F4CB,'project.json'),'package.json'):
    put(root_file,open(os.path.join(live,root_file),encoding='utf8').read())
print(ov)
