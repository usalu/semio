#!/usr/bin/env python3
"""Compares the error findings of a scan result with the staged ledger the way the law test does. usage: check_ledger.py SCAN.json LEDGER.json"""
import json,sys,collections
scan=json.load(open(sys.argv[1],encoding="utf8")); ledger=json.load(open(sys.argv[2],encoding="utf8"))
found=collections.Counter((x["file"],x["rule"]) for x in scan["findings"] if x["severity"]=="error")
debt={(e["file"],e["rule"]):e["count"] for e in ledger["entries"]}
fresh=[(k,v,debt.get(k)) for k,v in found.items() if debt.get(k)!=v]; stale=[k for k in debt if k not in found]
print("fresh",len(fresh),"stale",len(stale))
for f in fresh[:20]: print("FRESH",f)
for s in stale[:20]: print("STALE",s)
sys.exit(1 if fresh or stale else 0)
