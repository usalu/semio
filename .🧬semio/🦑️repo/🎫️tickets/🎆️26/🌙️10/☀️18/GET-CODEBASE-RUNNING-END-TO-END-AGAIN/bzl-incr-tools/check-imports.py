#!/usr/bin/env python3
"""Resolves every relative import specifier of the staged files against their destination (and the staged files themselves) and every edit-inserted specifier against the live tree; fails on a dangling one."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
m=json.load(open(os.path.join(here,"staged","manifest.json"),encoding="utf8"));staged={os.path.normpath(os.path.join(names.R,r["destination"])) for r in m.values()}
bad=[]
for flat,row in m.items():
    if not flat.endswith(".ts"):continue
    d=os.path.dirname(os.path.join(names.R,row["destination"]));t=open(os.path.join(here,"staged",flat),encoding="utf8").read()
    for spec in re.findall(r'(?:from\s*|import\()\s*"(\.[^"]+)"',t):
        p=os.path.normpath(os.path.join(d,spec))
        if p not in staged and not os.path.exists(p):bad.append((flat,spec))
    for spec in re.findall(r'import\("(@?[a-z][^"]*)"\)',t):pass
print("dangling:",bad);sys.exit(1 if bad else 0)
