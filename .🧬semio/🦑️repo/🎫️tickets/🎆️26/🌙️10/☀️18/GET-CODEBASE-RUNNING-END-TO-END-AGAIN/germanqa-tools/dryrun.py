import json,sys,re,difflib
sys.path.insert(0,".")
import importlib, rules
importlib.reload(rules)
r=json.load(open("../🗑️generated/germanqa/pairs.json"))
seen={}
for x in r:
    d=x["de"]
    if not d or x["en"]==d: continue
    if "📖️help" in x["file"] and "fixtures" in x["file"]: continue
    if d in seen: continue
    t=x["path"].endswith("/title")
    n=rules.transform(d,t)
    seen[d]=n
ch=[(a,b) for a,b in seen.items() if a!=b]
print(len(ch),"changed of",len(seen),file=sys.stderr)
mode=sys.argv[1] if len(sys.argv)>1 else "diff"
for a,b in ch:
    sm=difflib.SequenceMatcher(None,a,b,autojunk=False)
    segs=[]
    for op,i1,i2,j1,j2 in sm.get_opcodes():
        if op!="equal": segs.append(f"[{a[max(0,i1-12):i2]}|{b[max(0,j1-12):j2]}]".replace("\n"," "))
    print(" ".join(segs)[:400])
