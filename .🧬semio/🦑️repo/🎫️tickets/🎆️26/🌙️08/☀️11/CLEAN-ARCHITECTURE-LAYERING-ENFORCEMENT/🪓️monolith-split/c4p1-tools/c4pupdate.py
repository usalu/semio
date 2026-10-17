"""🔁 c4p1 updater: re-writes installed files whose staged text changed, only when the live file is still byte-identical to what this slice installed (journal `installed_sha`).
usage: c4pupdate.py [key...]"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

keys = sys.argv[1:] or ORDER
jp = GEN + "/c4p1-installed.json"
journal = json.load(open(jp, encoding="utf-8"))
for key in keys:
    j = journal[key]
    allowed = {os.path.abspath(ROOT + "/" + r) for r in j["files"]}
    for r in j["files"]:
        staged = read(f"{STAGE}/{key}/{r}")
        live = read(ROOT + "/" + r)
        if staged == live:
            continue
        assert sha(live) == j["installed_sha"][r], f"{key}: {r} changed since it was installed; refusing"
        repo_write(ROOT + "/" + r, staged, allowed)
        j["installed_sha"][r] = sha(staged)
        print(f"[{key}] updated {r.split('/')[-2]}")
json.dump(journal, open(jp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
