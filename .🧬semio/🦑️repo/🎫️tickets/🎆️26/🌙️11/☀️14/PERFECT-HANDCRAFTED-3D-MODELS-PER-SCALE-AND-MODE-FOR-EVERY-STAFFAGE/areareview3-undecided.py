"""Reviewer 3: audit rows (area-audit.json) of the five categories whose current value is neither decided by reviewer 1 (value in its table) nor by reviewer 3, listed with the measured outer area when known.
Usage: python3 areareview3-undecided.py"""
import importlib.util, json, os
T = os.path.dirname(os.path.abspath(__file__))
s = importlib.util.spec_from_file_location("d3", os.path.join(T, "areareview3-decisions.py")); m = importlib.util.module_from_spec(s); s.loader.exec_module(m)
au = json.load(open(os.path.join(T, "area-audit.json")))
M = json.load(open(os.path.join(T, "🗑️generated", "areareview3", "measure.json")))
mine = ("vegetation", "animals", "humans", "appliances", "furniture")
r1 = {}
for l in open(os.path.join(T, "area-decisions.md"), encoding="utf8").read().split("\n")[33:367]:
    c = [x.strip() for x in l.strip().strip("|").split("|")]
    if len(c) >= 9:
        try: r1[(c[0].split(" ")[-1], c[1])] = float(c[6].split(" ")[0])
        except ValueError: r1[(c[0].split(" ")[-1], c[1])] = None
n = 0
for a in au["areas"]:
    if not any(x in a["category"] for x in mine): continue
    k = (a["slug"], a["region"])
    if k in m.R and (m.R[k][1] is None or abs(m.R[k][1] - a["new"]) < 1e-9): continue
    if k in r1 and (r1[k] is None or abs(r1[k] - a["new"]) < 1e-9): continue
    if a["model"] == "body": continue
    me = M.get(a["slug"], {}).get(a["region"], {}).get("outer")
    n += 1
    print(f"{a['slug']}/{a['region']} {a['kind']}/{a['model']} old {a['old']} new {a['new']} class {a['class']} measured {me and round(me, 4)}")
print(n, "undecided")
