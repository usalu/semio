import copy
import json
import sys

sys.stdout.reconfigure(encoding="utf8")
root = sys.argv[1]
apply = len(sys.argv) > 2 and sys.argv[2] == "apply"


def load(n):
    p = f"{root}/{n}/🔣️.json"
    return p, json.load(open(p, encoding="utf8"))


def save(p, d):
    open(p, "w", encoding="utf8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


jp, jd = load("jogger")
rest = [x for x in jd["asset"]["thermal"]["respiration"]["patterns"] if x["id"] == "rest"][0]
print(json.dumps(rest, ensure_ascii=False))
for n in ("cyclist", "worker-carrying-box"):
    p, d = load(n)
    print(n, json.dumps(d["asset"]["thermal"]["respiration"]["patterns"][0], ensure_ascii=False))
    print(" refs", [r["id"] for r in d["asset"]["provenance"]["references"]])
print("jogger refs", [r["id"] for r in jd["asset"]["provenance"]["references"]])
if apply:
    a = jd["asset"]
    pats = a["thermal"]["respiration"]["patterns"]
    order = [2, 1, 0]
    a["thermal"]["respiration"]["patterns"] = [pats[i] for i in order]
    remap = {old: new for new, old in enumerate(order)}
    for c in a["clips"]:
        for t in c["tracks"]:
            if t["target"] == "breathing":
                for k in t["keys"]:
                    k["v"] = remap[k["v"]]
    save(jp, jd)
    for n in ("cyclist", "worker-carrying-box"):
        p, d = load(n)
        a = d["asset"]
        have = {r["id"] for r in a["provenance"]["references"]}
        new = copy.deepcopy(rest)
        if "reference" in new and new["reference"] not in have:
            src = [r for r in jd["asset"]["provenance"]["references"] if r["id"] == new["reference"]][0]
            a["provenance"]["references"].append(src)
        a["thermal"]["respiration"]["patterns"].insert(0, new)
        for c in a["clips"]:
            for t in c["tracks"]:
                if t["target"] == "breathing":
                    for k in t["keys"]:
                        k["v"] += 1
        save(p, d)
