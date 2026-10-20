# 🔬 Summarizes a directory of per-scope inventories by code; with a second directory prints the multiset difference. usage: summarize.py <dir> [<other dir>]
import json,glob,os,sys,collections
def load(d):
    rows=[]
    for f in sorted(glob.glob(os.path.join(d,"inv-*.json"))):
        j=json.load(open(f,encoding="utf8"))
        for v in j["violations"]: rows.append((j["scope"],v["code"],v["path"],v.get("message","")))
    return rows
a=load(sys.argv[1])
print("total",len(a)); 
for k,v in collections.Counter(r[1] for r in a).most_common(): print(f"{v:6d} {k}")
if len(sys.argv)>2:
    b=load(sys.argv[2])
    ca=collections.Counter((r[1],r[2],r[3]) for r in a); cb=collections.Counter((r[1],r[2],r[3]) for r in b)
    gone=ca-cb; new=cb-ca
    print("\nCLEARED",sum(gone.values()),collections.Counter(k[0] for k in gone.elements()))
    print("NEW",sum(new.values()),collections.Counter(k[0] for k in new.elements()))
    for k in list(new)[:40]: print("  new:",k)
