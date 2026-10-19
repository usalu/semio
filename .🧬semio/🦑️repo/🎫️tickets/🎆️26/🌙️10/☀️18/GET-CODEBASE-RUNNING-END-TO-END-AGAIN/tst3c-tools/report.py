import json,sys,collections
R="/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/C/"
G=R+"🗑️generated/tst3c-out/"
def load(n):
    d={}
    try:
        for l in open(G+n,encoding="utf-8"):
            r=json.loads(l); d[r["file"]]=r
    except FileNotFoundError: pass
    return d
before=load("plain.jsonl"); after=load("after-plain.jsonl"); hoist=load("after-hoist.jsonl"); art=load("after-art.jsonl")
notes=json.load(open(R+"tst3c-tools/notes.json",encoding="utf-8"))
def note(f):
    for k,v in notes:
        if k in f: return v
    return ["UNCLASSIFIED","",""]
def pf(r): return "-" if r is None else "%d/%d"%(r["pass_"],r["fail"])
def best(f):
    for d in (art,hoist):
        if f in d and d[f]["rc"]==0: return d[f]
    return hoist.get(f) or after.get(f)
out=[]
tot=lambda d:(len(d),sum(r["rc"]==0 for r in d.values()),sum(r["pass_"] for r in d.values()),sum(r["fail"] for r in d.values()))
print("before",tot(before)); print("after",tot(after))
final={f:best(f) for f in after}
print("final(after+hoist+art)",len(final),sum(r["rc"]==0 for r in final.values()),sum(r["pass_"] for r in final.values()),sum(r["fail"] for r in final.values()))
rows=[]
for f in sorted(after):
    b=before.get(f);a=after[f];h=final[f]
    if (b and b["rc"]==0) and a["rc"]==0: continue
    c=note(f)
    rows.append((f,pf(b),pf(a),pf(h),"green" if h["rc"]==0 else "red",c))
json.dump(rows,open(G+"report-rows.json","w",encoding="utf-8"),ensure_ascii=False)
cnt=collections.Counter(r[5][0] for r in rows if r[4]=="red")
print(cnt)
unc=[r[0] for r in rows if r[4]=="red" and r[5][0]=="UNCLASSIFIED"]
print("\n".join(unc))
