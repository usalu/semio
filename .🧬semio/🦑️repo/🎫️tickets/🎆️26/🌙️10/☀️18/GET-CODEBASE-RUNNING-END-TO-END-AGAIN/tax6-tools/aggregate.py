# 🔬 Sums the tax6 per-scope inventories into one findings file and prints totals per scope and code. usage: python3 aggregate.py <inv dir name> <out name>
import json,glob,collections,sys,os
C=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
rows=[];scopes={}
for f in sorted(glob.glob(f"{C}/🗑️generated/tax6/{sys.argv[1]}/inv-*.json")):
    d=json.load(open(f,encoding="utf8"))
    scopes[d["scope"]]=len(d["violations"])
    for v in d["violations"]: v["scope"]=d["scope"]; rows.append(v)
json.dump({"scopes":scopes,"total":len(rows),"findings":rows},open(f"{C}/🗑️generated/tax6/{sys.argv[2]}.json","w"),ensure_ascii=False,indent=1)
print("total",len(rows),scopes)
by=collections.Counter((v["scope"],v["code"]) for v in rows)
for (s,c),n in sorted(by.items(),key=lambda x:(-x[1])):print(f"{n:6d} {s:16s} {c}")
