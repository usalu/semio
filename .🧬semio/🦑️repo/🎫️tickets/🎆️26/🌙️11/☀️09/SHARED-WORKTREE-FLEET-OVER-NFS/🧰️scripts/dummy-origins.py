#!/usr/bin/env python3
"""usage: dummy-origins.py STORE GROUP COUNT [PREFIX] | STORE remove -- create COUNT valid origins (dummy-01 ..) with an empty segment, an empty head.json and home.json in STORE/v1/origins, or remove them again."""
import json, os, shutil, sys

store = sys.argv[1]
origins = os.path.join(store, "v1", "origins")
if sys.argv[2] == "remove":
    for n in os.listdir(origins):
        if "-" in n and n.split("-")[0].startswith("dummy"): shutil.rmtree(os.path.join(origins, n))
    sys.exit(0)
group, count = sys.argv[2], int(sys.argv[3])
prefix = sys.argv[4] if len(sys.argv) > 4 else "dummy"
for i in range(1, count + 1):
    name = "%s-%03d" % (prefix, i)
    d = os.path.join(origins, name, "segments"); os.makedirs(d, exist_ok=True)
    open(os.path.join(d, "000000000000.jsonl"), "w").close()
    json.dump({"version": 1, "segment": 0, "bytes": 0, "events": 0, "hlc": "1794285000000-000000-" + name}, open(os.path.join(origins, name, "head.json"), "w"))
    json.dump({"version": 1, "group": group}, open(os.path.join(origins, name, "home.json"), "w"))
