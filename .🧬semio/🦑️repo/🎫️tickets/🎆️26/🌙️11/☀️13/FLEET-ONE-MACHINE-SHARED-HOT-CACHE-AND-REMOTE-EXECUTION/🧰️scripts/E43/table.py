import json,glob,os,collections
d=glob.glob("/Users/ueli/Documents/semio/.*semio/*repo/*tickets/*26/*11/*13/FLEET-ONE*/wave-1122/*generated/E43/")[0]
def load(n):
    out={}
    p=d+n
    if os.path.exists(p):
        for l in open(p):
            r=json.loads(l);out[r["file"]]=r
    return out
before=load("before.jsonl")
after=dict(before)
for n in("pass2.jsonl","after-touched.jsonl","final.jsonl"):
    after.update(load(n))
def status(r):
    if r["rc"]==0:return "pass"
    if r["rc"]=="TIMEOUT":return "timeout"
    if r["rc"]==2:return "helper"
    return "fail"
def summ(t):
    c=collections.Counter(status(r) for r in t.values())
    p=sum(int(r.get("pass") or 0) for r in t.values() if status(r)!="helper");f=sum(int(r.get("fail") or 0) for r in t.values() if status(r)!="helper")
    return c,p,f
cb,pb,fb=summ(before);ca,pa,fa=summ(after)
print("files",len(before),"before",dict(cb),"tests pass/fail",pb,fb)
print("after",dict(ca),"tests pass/fail",pa,fa)
rows=[]
for f in before:
    b,a=before[f],after[f]
    if status(b)!=status(a) or (b.get("pass"),b.get("fail"))!=(a.get("pass"),a.get("fail")):
        rows.append((f.split("⚡️caching/")[-1],status(b),b.get("pass"),b.get("fail"),status(a),a.get("pass"),a.get("fail")))
print("changed",len(rows))
for r in rows:print("|".join(str(x) for x in r))
print("STILL RED")
for f,r in sorted(after.items()):
    if status(r) in("fail","timeout"):
        print(status(r),r.get("pass"),r.get("fail"),f.split("⚡️caching/")[-1],"::"," ".join((r.get("errs") or [""])[:1])[:110])
