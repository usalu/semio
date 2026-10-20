#!/usr/bin/env python3
"""Builds the verification overlay /private/tmp/conc-tests/ov: the staged new files at their final paths plus the few installed packages the tests import (copied, not linked). usage: overlay.py [--clean]"""
import os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import manifest as m
OV="/private/tmp/conc-tests/ov"
REAL="/Users/ueli/Documents/semio"
if "--clean" in sys.argv: shutil.rmtree(OV,ignore_errors=True)
for staged,rel in m.NEW.items():
    dst=OV+"/"+rel; os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copyfile(HERE+"/staged/"+staged,dst)
nm=OV+"/node_modules"
store=os.path.realpath(REAL+"/node_modules")+"/.bun"
for pkg in ["ajv","fast-deep-equal","json-schema-traverse","require-from-string","fast-uri"]:
    dst=nm+"/"+pkg
    if os.path.isdir(dst): continue
    cand=[d for d in os.listdir(store) if d.startswith(pkg+"@")]
    src=store+"/"+sorted(cand)[-1]+"/node_modules/"+pkg
    shutil.copytree(src,dst,symlinks=False)
open(OV+"/package.json","w").write('{"name":"ov","private":true}\n')
print("overlay ok")
