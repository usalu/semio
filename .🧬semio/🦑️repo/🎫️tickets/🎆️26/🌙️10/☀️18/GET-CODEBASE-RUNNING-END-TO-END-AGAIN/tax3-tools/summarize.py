# 🔬 Sums the per-scope inventory outputs by violation code against the audit baseline. usage: python3 summarize.py <dir-with-sim-*.json> <out-violations.json>
import json,glob,collections,sys
base=json.load(open("🗑️generated/tax3-run/sim-summary.json"))["base"]
after=collections.Counter(); rows=[]
for f in glob.glob(sys.argv[1]+"/sim-*.json"):
    d=json.load(open(f,encoding="utf8")); after.update(v["code"] for v in d["violations"]); rows+=d["violations"]
json.dump(rows,open(sys.argv[2],"w"),ensure_ascii=False)
print("before",sum(base.values()),"after",sum(after.values()))
for c in sorted(set(base)|set(after),key=lambda c:-base.get(c,0)): print(f"{c:52s} {base.get(c,0):6d} {after[c]:6d}")
for code in ("projection-member-unresolved","projection-catalog-coverage","projection-catalog-unrealized","mutation-fixture-unpaired","mutation-fixture-invalid"):
    print(code,dict(collections.Counter(v["path"].split("/")[2] for v in rows if v["code"]==code)))
