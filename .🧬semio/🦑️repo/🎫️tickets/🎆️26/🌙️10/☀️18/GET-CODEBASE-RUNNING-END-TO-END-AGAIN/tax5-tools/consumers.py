# 🧭 Classifies every case of every generated leaf case table by what executes it. usage: consumers.py [--json out]
# table-driven: a runner file reads the leaf's `🔣️.cases.json`; wired-row: the case is an `Examples` row of an exhaustive feature (declared by the feature);
# literal: a runner file (rs/ts/py/feature, outside fixtures and tables) names both the leaf and the case; none: no runner names the case.
import json,os,re,sys,glob,collections
ROOT="/Users/ueli/Documents/semio"
leaves=json.load(open(os.path.dirname(os.path.abspath(__file__))+"/leaves.json",encoding="utf8"))
def fold(s): return s.replace("️","")
def slug(s): return re.sub(r"^[^a-z0-9]+","",fold(s))
cache={}
def runner_texts(plugin):
    if plugin in cache: return cache[plugin]
    out=[]
    for dp,dn,fn in os.walk(plugin):
        dn[:]=[d for d in dn if d not in ("🧫️fixtures","node_modules","target")]
        for f in fn:
            if f.endswith((".rs",".ts",".tsx",".py",".feature")):
                p=os.path.join(dp,f)
                try: out.append((p,fold(open(p,encoding="utf8").read())))
                except Exception: pass
    cache[plugin]=out; return out
unpaired=set()
base=os.path.dirname(os.path.abspath(__file__))
for f in glob.glob(base+"/../🗑️generated/tax5-inv-base/inv-*.json"):
    for v in json.load(open(f,encoding="utf8"))["violations"]:
        if v["code"]=="mutation-fixture-unpaired": unpaired.add(v["path"])
rows=[]
for leaf in leaves:
    subset=ROOT+"/"+leaf["subset"]; owner=leaf["owner"]; plugin=ROOT+"/"+leaf["subset"].split("/🗿️artifacts/")[0]
    table=subset+"/🧬️schema/🧬️mutations/"+owner+"/🧪️tests/🔣️.cases.json"
    cases=json.load(open(table,encoding="utf8"))["cases"]
    fixtures=subset+"/🧪️tests/🧫️fixtures/🧬️mutations/"+owner
    leafname=slug(owner.split("/")[-1])
    texts=runner_texts(subset.split("/🏅️standards")[0])
    table_readers=[p for p,t in texts if "cases.json" in t and os.path.dirname(p).startswith(subset+"/🧬️schema/🧬️mutations/"+owner)]
    for case in cases:
        if "--all" not in sys.argv and (leaf["subset"]+"/🧪️tests/🧫️fixtures/🧬️mutations/"+owner+"/"+case) not in unpaired: continue
        hits=[p for p,t in texts if fold(case) in t and leafname in t]
        is_row=any(p.endswith("🥒️.feature") and leafname in t and fold(case) in t for p,t in texts if p.startswith(subset+"/🧪️tests/"))
        kind="table-driven" if table_readers else ("feature-row" if is_row else ("literal" if hits else "none"))
        rows.append({"plugin":leaf["subset"].split("/🗿️artifacts/")[0].split("/")[-1],"owner":owner,"case":case,"class":kind,"runners":[h.split("/🧬️schema/")[-1][-70:] if "/🧬️schema/" in h else h[-70:] for h in hits[:3]]})
summary=collections.Counter((r["plugin"],r["class"]) for r in rows)
for k in sorted(summary): print(summary[k],k)
print("total",len(rows),collections.Counter(r["class"] for r in rows))
if "--json" in sys.argv: json.dump(rows,open(sys.argv[sys.argv.index("--json")+1],"w",encoding="utf8"),ensure_ascii=False,indent=0)
