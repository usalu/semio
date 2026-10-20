#!/usr/bin/env python3
"""Checks that every relative import specifier introduced by edits.json resolves to a file of the overlay tree, and that every edited file still transpiles. usage: verify_imports.py OVERLAY"""
import json,os,re,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__)); OV=sys.argv[1].rstrip("/")
bad=0
for e in json.load(open(HERE+"/edits.json",encoding="utf8")):
    path=OV+"/"+e["target"]; base=os.path.dirname(path)
    for p in e["pairs"]:
        for spec in re.findall(r'from\s*"(\.[^"]+)"',p["new"]):
            if spec in p["old"]: continue
            if not os.path.exists(os.path.normpath(os.path.join(base,spec))): print("UNRESOLVED",spec,"in",e["target"][-70:]); bad+=1
    r=subprocess.run(["bun","build","--no-bundle",path],capture_output=True,text=True)
    if r.returncode!=0: print("TRANSPILE FAIL",e["target"][-70:],r.stderr[:200]); bad+=1
print("verified",len(json.load(open(HERE+"/edits.json",encoding="utf8"))),"targets; problems",bad)
sys.exit(1 if bad else 0)
