#!/usr/bin/env python3
"""take.py <relpath>...: copies live files (relative to /Users/Shared/semio) into Z/src and records their sha256 in Z/base.json (only when not yet recorded)."""
import os,sys,json,hashlib,shutil
Z=os.path.dirname(os.path.abspath(__file__));LIVE='/Users/Shared/semio'
bp=os.path.join(Z,'base.json');base=json.load(open(bp)) if os.path.exists(bp) else {}
for rel in sys.argv[1:]:
    src=os.path.join(LIVE,rel);dst=os.path.join(Z,'src',rel)
    if rel in base:print('kept',rel);continue
    os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(src,dst);bdst=os.path.join(Z,'base',rel);os.makedirs(os.path.dirname(bdst),exist_ok=True);shutil.copyfile(src,bdst)
    base[rel]=hashlib.sha256(open(src,'rb').read()).hexdigest();print('took',rel,base[rel][:12])
json.dump(base,open(bp,'w'),ensure_ascii=False,indent=1)
