# [scratch CVF-A] build one candidate, measure every named pose separately through the server, print violations per scale and pose
import json, sys, os, subprocess, time, re
from cva import *
G = f"{T}/🗑️generated/CVF-A"; SRV = os.environ.get("SRV", "srv")
BUILD = {"labrador-dog": "build_dog.py", "domestic-cat": "build_cat.py", "horse": "build_horse.py", "pigeon": "build_pigeon.py", "roe-deer": "build_deer.py", "dairy-cow": "build_cow.py"}
def request(name, file, limit=80):
    res = f"{G}/{SRV}/res/{name}.txt"
    if os.path.exists(res): os.remove(res)
    tmp = f"{G}/{SRV}/req/{name}.json.tmp"
    open(tmp, "w").write(json.dumps({"file": file, "physics": False, "samples": 2, "limit": limit}))
    os.rename(tmp, f"{G}/{SRV}/req/{name}.json")
    return res
def wait(res, timeout=300):
    t0 = time.time()
    while not os.path.exists(res):
        if time.time()-t0 > timeout: return "TIMEOUT"
        time.sleep(0.5)
    return open(res).read()
def main(aid, scales, build=True, summary=False):
    if build:
        r = subprocess.run(["python3", BUILD[aid]], capture_output=True, text=True)
        if r.returncode: print(r.stdout, r.stderr); sys.exit(1)
    doc = json.load(open(f"{G}/cand/{aid}.json")); poses = [p["id"] for p in doc["asset"]["poses"] if not p.get("key")]
    os.makedirs(f"{G}/cand/var", exist_ok=True); pend = {}
    for pid in poses:
        d = json.loads(json.dumps(doc))
        for p in d["asset"]["poses"]:
            if p["id"] != pid and not p.get("key"): p["key"] = True
        f = f"{G}/cand/var/{aid}@{pid}.json"; json.dump(d, open(f, "w"), ensure_ascii=False)
        pend[pid] = request(f"v-{aid}-{pid}", f)
    total = {}
    for pid, res in pend.items():
        txt = wait(res); cur = None
        for l in txt.splitlines():
            m = re.match(r"1:(\d+) suitable=\S+ .*violations=(\d+)", l)
            if m: cur = int(m.group(1)); total.setdefault(cur, {})[pid] = []; continue
            if (l.startswith("   gap") or l.startswith("   feature") or l.startswith("   stroke") or l.startswith("   hatch")) and cur: total[cur][pid].append(l.strip())
            if l.startswith("  problem") and "suitab" not in l and "too-few-poses" not in l: print(pid, l)
            if l.startswith("ERROR"): print(txt[:600])
    for sc in sorted(total, reverse=True):
        if scales and sc not in scales: continue
        n = sum(len(v) for v in total[sc].values()); print(f"=== 1:{sc}: {n} violations")
        if not summary:
            for pid, lines in total[sc].items():
                if lines: print(f"  -- {pid}"); [print("     " + x) for x in lines]
        else:
            print("  per pose:", {pid: len(l) for pid, l in total[sc].items()})
if __name__ == "__main__":
    aid = sys.argv[1]; scales = [int(x) for x in sys.argv[2].split(",")] if len(sys.argv) > 2 and sys.argv[2] != "all" else None
    main(aid, scales, build="--nobuild" not in sys.argv, summary="--summary" in sys.argv)
