import json,subprocess
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/"
def rd(p):return open(p,encoding="utf8").read()
def wr(p,s):open(p,"w",encoding="utf8").write(s)
def sub(s,a,b,count=1):
    assert s.count(a)==count,(a[:70],s.count(a))
    return s.replace(a,b)
s=rd(S+"stage/owner-pool.ts")
s=sub(s,"foreground:Readonly<{cpu:number;unrealizedMegabytes:number;waiters:number;oldestWaitMilliseconds?:number}>","foreground:Readonly<{cpu:number;unrealizedMegabytes:number;waiters:number;waitingMegabytes?:number;oldestWaitMilliseconds?:number}>")
s=sub(s,"const held=foreground.waiters>0?foreground.unrealizedMegabytes:Math.min(foreground.unrealizedMegabytes,Math.floor(host.totalMegabytes*policy.ownHoldShareBasisPoints/10000)),spare=","const held=foreground.waiters>0&&foreground.waitingMegabytes===undefined?foreground.unrealizedMegabytes:Math.min(foreground.unrealizedMegabytes,Math.floor(host.totalMegabytes*policy.ownHoldShareBasisPoints/10000))+(foreground.waitingMegabytes??0),spare=")
s=sub(s,"while nothing of it waits, all of them once it waits: a request that cannot be admitted makes the pool yield within `yieldAfterMilliseconds` anyway, so holding more in advance only idles the machine;","plus exactly what its waiting requests ask for (`waitingMegabytes`; all of the reservations when that is not known): a request that cannot be admitted makes the pool yield within `yieldAfterMilliseconds` anyway, and a request the gate holds back on paper while the memory is free (a reservation of 14 GB of which 1 GB was touched) is no reason to idle the machine;")
wr(S+"stage/owner-pool.ts",s)
c=rd(S+"stage/sidecar-pool.ts")
c=sub(c,"waiters:view.waiters.length,","waiters:view.waiters.length,waitingMegabytes:Math.ceil(view.waiters.reduce((sum,waiter)=>sum+(waiter.request.minimumMegabytes??waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes),0)),")
wr(S+"stage/sidecar-pool.ts",c)
o=rd(S+"stage/oracle.py")
o=sub(o,'''    held = fg["unrealizedMegabytes"] if fg["waiters"] > 0 else min(fg["unrealizedMegabytes"], total * p["ownHoldShareBasisPoints"] // 10000)''','''    held = fg["unrealizedMegabytes"] if fg["waiters"] > 0 and "waitingMegabytes" not in fg else min(fg["unrealizedMegabytes"], total * p["ownHoldShareBasisPoints"] // 10000) + fg.get("waitingMegabytes", 0)''')
wr(S+"stage/oracle.py",o)
D=S+"stage/fixture.json"
d=json.load(open(D,encoding="utf8"))
for c in d["target"]:
    if c["name"].startswith("a 16 GB worker whose read waits"):
        c["name"]="a 16 GB worker whose read waits and asks for 3000 MB: the capped hold plus the asked memory is held back and the booked cores count"
        c["host"]["foreground"]["waitingMegabytes"]=3000
names={c["name"] for c in d["target"]}
new=[
 {"name":"a read whose farm waits for 1536 MB the gate holds back on paper: the machine has 19 GB free, the pool keeps the asked memory free and the rest of the budget","host":{"cores":10,"totalMegabytes":32768,"availableMegabytes":19038,"pressure":"normal","load":30.41,"swapOutMegabytesPerSecond":0,"foreground":{"cpu":9,"unrealizedMegabytes":15939,"waiters":1,"waitingMegabytes":1536,"oldestWaitMilliseconds":0},"idleCores":6},"children":1,"childMegabytes":1064},
 {"name":"a waiting request of unknown size holds the whole reservation","host":{"cores":10,"totalMegabytes":32768,"availableMegabytes":19038,"pressure":"normal","load":2,"swapOutMegabytesPerSecond":0,"foreground":{"cpu":2,"unrealizedMegabytes":15939,"waiters":1,"oldestWaitMilliseconds":0},"idleCores":6},"children":1,"childMegabytes":1064},
]
for c in new:
    if c["name"] not in names:d["target"].append(c)
json.dump(d,open(D,"w",encoding="utf8"),indent=1,ensure_ascii=False)
out=subprocess.run(["python3","-I",S+"stage/oracle.py",D],capture_output=True,text=True,check=True).stdout
o=json.loads(out)
orig=json.load(open(S+"base/fixture.json",encoding="utf8"))
for i,a in enumerate(o["target"]):
    if i<len(orig["expect"]["target"]):
        if a!=orig["expect"]["target"][i] or o["limit"][i]!=orig["expect"]["limit"][i]:print("CHANGED",i,d["target"][i]["name"][:50],orig["expect"]["target"][i],"->",a,o["limit"][i])
    else:print("NEW",i,d["target"][i]["name"][:50],a,o["limit"][i])
d["expect"]["target"]=o["target"];d["expect"]["limit"]=o["limit"];d["expect"]["childMegabytes"]=o["childMegabytes"]
json.dump(d,open(D,"w",encoding="utf8"),indent=1,ensure_ascii=False)
