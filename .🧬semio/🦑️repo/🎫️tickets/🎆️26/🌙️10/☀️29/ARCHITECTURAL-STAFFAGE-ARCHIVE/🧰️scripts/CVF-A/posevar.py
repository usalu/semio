# [scratch CVF-A] writes <id>@<pose>.json variants of a candidate where only one pose is measured (others flagged key) and runs the check on each via the server
import json, sys, os, subprocess
C = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-A/cand"
aid = sys.argv[1]; scale = sys.argv[2] if len(sys.argv) > 2 else None
doc = json.load(open(f"{C}/{aid}.json"))
poses = [p["id"] for p in doc["asset"]["poses"] if not p.get("key")]
os.makedirs(f"{C}/var", exist_ok=True)
for pid in poses:
    d = json.loads(json.dumps(doc))
    for p in d["asset"]["poses"]:
        if p["id"] != pid and not p.get("key"): p["key"] = True
    json.dump(d, open(f"{C}/var/{aid}@{pid}.json", "w"), ensure_ascii=False)
    out = subprocess.run(["./req.sh", f"v-{aid}-{pid}", f"{C}/var/{aid}@{pid}.json"], capture_output=True, text=True, env=dict(os.environ, SRV="srv", LIMIT="60", WAIT="120")).stdout
    print("=== pose", pid)
    for l in out.splitlines():
        if l.startswith("1:") or l.startswith("   gap") or l.startswith("   feature") or "problem" in l and "suitab" not in l: 
            if scale is None or l.startswith("   ") or l.startswith(f"1:{scale}") : print(l)
