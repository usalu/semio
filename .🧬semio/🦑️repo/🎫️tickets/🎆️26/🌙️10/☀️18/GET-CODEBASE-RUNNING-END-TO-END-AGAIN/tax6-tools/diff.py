# 🔬 Diffs two aggregated finding files by (code,path): prints removed and added findings with counts per code. usage: python3 diff.py <before> <after> [scope]
import json,sys,os,collections
C=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
def load(n):
    d=json.load(open(f"{C}/🗑️generated/tax6/{n}.json",encoding="utf8"))
    return {(r["code"],r["path"]):r for r in d["findings"] if len(sys.argv)<4 or r["scope"]==sys.argv[3]}
a,b=load(sys.argv[1]),load(sys.argv[2])
rem=[k for k in a if k not in b]; add=[k for k in b if k not in a]
print("removed",collections.Counter(c for c,_ in rem)); print("added",collections.Counter(c for c,_ in add))
for c,p in add[:25]:print("  +",c,p,"|",b[(c,p)]["message"][:120])
