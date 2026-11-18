import json,subprocess
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/"
def rd(p):return open(p,encoding="utf8").read()
def wr(p,s):open(p,"w",encoding="utf8").write(s)
def sub(s,a,b,count=1):
    assert s.count(a)==count,(a[:70],s.count(a))
    return s.replace(a,b)
s=rd(S+"stage/owner-pool.ts")
s=sub(s,"byCores=foreground.waiters>0?booked:idle===null?Math.max(booked,minimum):host.cores;","byCores=idle!==null?(foreground.waiters>0?input.children+Math.floor(Math.max(0,idle-reserved)):host.cores):foreground.waiters>0?booked:Math.max(booked,minimum);")
wr(S+"stage/owner-pool.ts",s)
o=rd(S+"stage/oracle.py")
o=sub(o,'''    by_cores = booked if fg["waiters"] > 0 else (max(booked, minimum) if idle is None else h["cores"])''','''    by_cores = (case["children"] + math.floor(max(0, idle - reserved)) if fg["waiters"] > 0 else h["cores"]) if idle is not None else (booked if fg["waiters"] > 0 else max(booked, minimum))''')
wr(S+"stage/oracle.py",o)
D=S+"stage/fixture.json"
d=json.load(open(D,encoding="utf8"))
out=subprocess.run(["python3","-I",S+"stage/oracle.py",D],capture_output=True,text=True,check=True).stdout
o=json.loads(out)
orig=json.load(open(S+"base/fixture.json",encoding="utf8"))
for i,a in enumerate(o["target"]):
    if i<len(orig["expect"]["target"]):
        if a!=orig["expect"]["target"][i] or o["limit"][i]!=orig["expect"]["limit"][i]:print("CHANGED",i,d["target"][i]["name"][:50],orig["expect"]["target"][i],"->",a,o["limit"][i])
    else:print("NEW",i,d["target"][i]["name"][:50],a,o["limit"][i])
d["expect"]["target"]=o["target"];d["expect"]["limit"]=o["limit"];d["expect"]["childMegabytes"]=o["childMegabytes"]
json.dump(d,open(D,"w",encoding="utf8"),indent=1,ensure_ascii=False)
