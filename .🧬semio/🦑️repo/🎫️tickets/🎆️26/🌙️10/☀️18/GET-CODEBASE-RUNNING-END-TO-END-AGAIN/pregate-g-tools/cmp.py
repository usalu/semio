"""compare laws jsonl vs baseline: usage cmp.py BASE NEW"""
import json,sys
def load(p):
    d={}
    for l in open(p):
        r=json.loads(l);d[r["file"]]=r
    return d
b,n=load(sys.argv[1]),load(sys.argv[2])
reg=[];imp=[]
for f,r in n.items():
    o=b.get(f)
    if o is None:
        if r["exit"]:reg.append((f,"new file red",r))
        continue
    if r["fail"]>o["fail"] or (r["exit"] and not o["exit"]) or r["pass"]<o["pass"]-0 and r["exit"]:reg.append((f,"base %s/%s"%(o["pass"],o["fail"]),r))
    elif o["exit"] and not r["exit"]:imp.append(f)
print(f"files {len(n)} (base {len(b)}), pass {sum(r['pass'] for r in n.values())} fail {sum(r['fail'] for r in n.values())} (base pass {sum(r['pass'] for r in b.values())} fail {sum(r['fail'] for r in b.values())}); red files {sum(1 for r in n.values() if r['exit'])} (base {sum(1 for r in b.values() if r['exit'])})")
for f,why,r in reg:print("REGRESSION",why,"->",r["pass"],"/",r["fail"],r["mode"],"/".join(f.split("/")[-4:]))
for r in reg[:6]:print("   tail:",(r[2].get("tail","")[:600]).replace("\n"," | "))
print("fixed vs base:",len(imp))
