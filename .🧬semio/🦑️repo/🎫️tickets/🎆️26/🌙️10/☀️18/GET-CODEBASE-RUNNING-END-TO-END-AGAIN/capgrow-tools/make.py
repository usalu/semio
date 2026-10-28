"""🏗️ capgrow: builds the flat batch `staged/<batch>` from a mirror holding the change and the live tree as base: python3 -I make.py <mirror> <batch> (files from files.txt; base = live bytes now, absent = new file)."""
import hashlib,json,os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__));LIVE="/Users/ueli/Documents/semio";mirror,batch=sys.argv[1],sys.argv[2]
out=os.path.join(HERE,"staged",batch);shutil.rmtree(os.path.join(out,"files"),ignore_errors=True);shutil.rmtree(os.path.join(out,"base"),ignore_errors=True)
os.makedirs(os.path.join(out,"files"));os.makedirs(os.path.join(out,"base"))
digest=lambda path:hashlib.sha256(open(path,"rb").read()).hexdigest() if os.path.exists(path) else None
manifest={}
for index,rel in enumerate(row for row in open(os.path.join(HERE,"files.txt"),encoding="utf8").read().split("\n") if row):
    name=f"f{index:03d}";shutil.copyfile(os.path.join(mirror,rel),os.path.join(out,"files",name))
    if os.path.exists(os.path.join(LIVE,rel)):shutil.copyfile(os.path.join(LIVE,rel),os.path.join(out,"base",name))
    manifest[rel]={"file":name,"base":digest(os.path.join(LIVE,rel)),"new":digest(os.path.join(mirror,rel))}
template=open(os.path.join(HERE,"..","xplat5-tools","stage.template.py"),encoding="utf8").read()
head,rest=template.split('usage:',1)
docstring='#!/usr/bin/env python3\n"""🌱️ Staged batch `'+batch+'` (lane capgrow): '+("a run admitted with fewer jobs may grow right after its admission; only a previous growth has to ramp first." if batch=="grownow" else "a run admitted with fewer jobs than it asked for grows when memory frees up, and a planned compile takes the cores its grant carries at the measured unit footprint.")+'\n'
open(os.path.join(out,"stage.py"),"w",encoding="utf8").write(docstring+'usage:'+rest)
json.dump(manifest,open(os.path.join(out,"manifest.json"),"w",encoding="utf8"),indent=1,ensure_ascii=False)
print(len(manifest),"files ->",out,"; new files:",sum(1 for row in manifest.values() if row["base"] is None))
