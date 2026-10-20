#!/usr/bin/env python3
"""Builds a COPY overlay of the module closure plus the staged files at their final paths. usage: overlay.py OUT_DIR [--production] (the production closure of the edited modules, unpatched: run apply.py --root OUT_DIR --write on it)"""
import json,os,shutil,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import manifest as m
REPO="/Users/ueli/Documents/semio"
out=sys.argv[1]
EXTRA=[m.S+"/📤️publication/🧬️schema/"+chr(0x1F523)+"\ufe0f.json",m.S+"/🧹️cleanup/🎛️command/🧬️schema/"+chr(0x1F523)+"\ufe0f.json"]
ENTRIES=[m.BOOT+"/"+m.JS,m.S+"/🍎️image/📡️roster/"+m.JS,m.PROCESS+"/🔒️leases/"+m.JS,m.S+"/📁️directory/"+m.JS,m.PROCESS+"/🎛️owned-execution/"+m.JS]
if "--production" in sys.argv:ENTRIES+=[m.S+"/"+m.JS,m.S+"/🧹️cleanup/🎛️command/"+m.JS,m.S+"/🧱️fixed/"+m.JS,m.S+"/🍎️image/"+m.JS,m.S+"/🌱️birth/🧾️claim/"+m.JS]
meta=out+".meta.json"
if os.path.isdir(out):shutil.rmtree(out)
os.makedirs(out)
files=set()
for entry in ENTRIES:
    subprocess.run(["bun","build","--target=bun","--metafile="+meta,"--outdir="+out+".bundle",entry],cwd=REPO,check=True,capture_output=True)
    files|=set(json.load(open(meta))["inputs"].keys())
shutil.rmtree(out+".bundle",ignore_errors=True);os.remove(meta)
files|=set(EXTRA)
for rel in sorted(files):
    dst=out+"/"+rel;os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(REPO+"/"+rel,dst)
for staged,rel in m.NEW.items():
    src=m.ST+"/"+staged
    if os.path.exists(src):
        dst=out+"/"+rel;os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(src,dst)
nm=out+"/node_modules";os.makedirs(nm)
real=os.path.realpath(REPO+"/node_modules/ajv");siblings=os.path.dirname(real);seen=[]
for name in sorted(os.listdir(siblings)):
    shutil.copytree(siblings+"/"+name,nm+"/"+name,symlinks=False);seen.append(name)
print("overlay",out,len(files),"repo files +",sum(os.path.exists(m.ST+"/"+k) for k in m.NEW),"staged; packages",sorted(seen))
