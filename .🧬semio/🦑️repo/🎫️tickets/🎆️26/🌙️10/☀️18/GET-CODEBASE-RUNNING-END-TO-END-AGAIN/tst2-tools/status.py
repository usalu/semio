import json,os,re,sys,collections
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/tst2-out/"
order=["pri-bun","rest-bun","pri-rerun1","rest-rerun1","treeify1"]+[x for x in sys.argv[1:]]
last={}; first={}
for name in order:
    p=G+name+".jsonl"
    if not os.path.exists(p): continue
    for l in open(p,encoding="utf-8"):
        r=json.loads(l); last[r["file"]]=r
        first.setdefault(r["file"],r)
def kind(r):
    if r["rc"]==0: return "OK"
    t=open(r["log"],encoding="utf-8",errors="replace").read()
    m=re.search(r"Cannot find (?:package|module) '([^']+)'",t)
    if m:
        s=m.group(1)
        if s.startswith(("..","/")): return "BROKEN-IMPORT "+s[-60:]
        if "@semio-tech" in s: return "NO-DIST "+s
        return "MISSING-DEP "+(("/".join(s.split("/")[:2])) if s.startswith("@") else s.split("/")[0])
    return "ASSERT/ENV"
if __name__=="__main__":
    ok=sum(1 for r in last.values() if r["rc"]==0)
    print("suites",len(last),"ok",ok,"bad",len(last)-ok,"| first-pass ok",sum(1 for r in first.values() if r["rc"]==0))
    c=collections.Counter(kind(r).split(" ")[0] for r in last.values() if r["rc"]!=0)
    print(c)
    json.dump({f:dict(kind=kind(r),**{k:v for k,v in r.items() if k!='log'},log=r.get("log")) for f,r in last.items()},open(G+"status.json","w"),ensure_ascii=False)
