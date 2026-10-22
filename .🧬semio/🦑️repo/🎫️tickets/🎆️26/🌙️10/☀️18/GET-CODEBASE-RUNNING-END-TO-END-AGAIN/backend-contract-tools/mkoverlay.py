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
    for rel in (names.L,f"{names.FRAMEWORK}/{names.MODULES}",f"{names.FRAMEWORK}/{names.PRODUCTS}/{names.REPO}/{names.MODULES}/{names.n(0x1F4BB,'client')}"):
        realize(os.path.dirname(rel))
        target=os.path.join(ov,rel)
        if os.path.islink(target):os.remove(target)
        subprocess.run(["cp","-cR",os.path.join(live,rel),target],check=True)
if "--with-tool-iface" in sys.argv:
    # the staged tool interface (git adapter and runToolV1) is a dependency of the engine's inventory
    source=os.path.join(os.path.dirname(here),"tool-iface-tools","src","files",names.FRAMEWORK,names.MODULES,names.n(0x1F3C3,"process"),names.n(0x1F9F0,"tools"))
    target=os.path.join(ov,names.FRAMEWORK,names.MODULES,names.n(0x1F3C3,"process"),names.n(0x1F9F0,"tools"))
    realize(os.path.dirname(os.path.relpath(target,ov)))
    shutil.rmtree(target,ignore_errors=True);shutil.copytree(source,target)
if "--with-tool-iface" in sys.argv:
    # the staged runtime seam (glob, identity) is a dependency of the engine and of the tool interface
    runtime=names.n(0x2699,"runtime");source=os.path.join(os.path.dirname(here),"runtime-seam-tools","staged","new",names.FRAMEWORK,names.MODULES,runtime)
    target=os.path.join(ov,names.FRAMEWORK,names.MODULES,runtime)
    realize(os.path.dirname(os.path.relpath(target,ov)))
    shutil.rmtree(target,ignore_errors=True);shutil.copytree(source,target)
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
