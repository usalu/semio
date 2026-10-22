#!/usr/bin/env python3
"""Writes the wire.py-patched copies of the existing files the staged tests import to /private/tmp/perf-slo/tsc and a map for tscheck.cjs."""
import json,os,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
out="/private/tmp/perf-slo/tsc";os.makedirs(out,exist_ok=True);m={}
for i,(rel,fn) in enumerate(wire.EDITS):
    if rel.endswith(names.CATALOG):
        dst=os.path.join(out,f"{i}-"+os.path.basename(rel));open(dst,"w",encoding="utf8").write(fn(open(os.path.join(names.R,rel),encoding="utf8").read()));m[os.path.join(names.R,rel)]=dst
json.dump(m,open(os.path.join(out,"map.json"),"w"));print(m)
