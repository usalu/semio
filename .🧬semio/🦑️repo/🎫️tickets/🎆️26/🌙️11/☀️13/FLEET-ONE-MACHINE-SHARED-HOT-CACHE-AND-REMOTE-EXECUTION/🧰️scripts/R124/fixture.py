import json,subprocess,sys
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🧫️fixtures/"
path=D+"🔣️.json"
d=json.load(open(path,encoding="utf8"))
d["policy"]["ownHoldShareBasisPoints"]=1500
names={c["name"] for c in d["target"]}
def host(cores,total,avail,cpu,unreal,waiters=0,idle=None,load=None,pressure="normal",swap=None,wait=None):
    f={"cpu":cpu,"unrealizedMegabytes":unreal,"waiters":waiters}
    if wait is not None:f["oldestWaitMilliseconds"]=wait
    h={"cores":cores,"totalMegabytes":total,"availableMegabytes":avail,"pressure":pressure,"load":load,"swapOutMegabytesPerSecond":swap,"foreground":f}
    if idle is not None:h["idleCores"]=idle
    return h
new=[
 {"name":"the server of 09:41: a read holds 9966 MB untouched, nine idle farm workers book every core, the host is loaded by other processes","host":host(10,32768,15851,11,9966,idle=0,load=43.45),"children":1,"childMegabytes":1064},
 {"name":"a 16 GB worker whose own read books every core while idle cores exist: the idle cores and the capped hold admit children","host":host(8,16384,9000,8,5200,idle=3,load=6),"children":2,"childMegabytes":1100},
 {"name":"a 16 GB worker whose read waits: the whole reservation is held back and the booked cores count","host":host(8,16384,9000,6,5200,waiters=2,idle=3,load=6,wait=1000),"children":2,"childMegabytes":1100},
 {"name":"a memory bound pool says the number that blocks the next child","host":host(8,16384,6000,0,3000,idle=6,load=2),"children":2,"childMegabytes":1237},
 {"name":"no read runs: the pool gets the whole budget above the floor","host":host(8,16384,11000,0,0,idle=8,load=0),"children":0,"childMegabytes":1000},
 {"name":"below the floor one child still runs and the free number is zero","host":host(8,16384,1500,0,0,idle=8,load=0),"children":1,"childMegabytes":1000},
]
for c in new:
    if c["name"] not in names:d["target"].append(c)
cm=[{"samples":[1800,1900,2000],"live":[450,500,520,540,600,650]},{"samples":[],"live":[500,600,700]},{"samples":[900,1000],"live":[]},{"samples":[1000],"live":[0,-5]}]
have=[json.dumps(c,sort_keys=True) for c in d["childMegabytes"]]
for c in cm:
    if json.dumps(c,sort_keys=True) not in have:d["childMegabytes"].append(c)
json.dump(d,open(path,"w",encoding="utf8"),indent=1,ensure_ascii=False)
out=subprocess.run(["python3","-I",D+"🐍️oracle.py",path],capture_output=True,text=True,check=True).stdout
o=json.loads(out)
d["expect"]["target"]=o["target"];d["expect"]["limit"]=o["limit"];d["expect"]["childMegabytes"]=o["childMegabytes"]
orig=json.load(open("/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/fixture.json.orig",encoding="utf8"))
for i,a in enumerate(o["target"]):
    if i<len(orig["expect"]["target"]) and a!=orig["expect"]["target"][i]:print("CHANGED",i,d["target"][i]["name"],orig["expect"]["target"][i],"->",a)
for i,a in enumerate(o["target"]):
    if i>=len(orig["expect"]["target"]):print("NEW",i,d["target"][i]["name"],a,o["limit"][i])
print(o["childMegabytes"][-8:])
json.dump(d,open(path,"w",encoding="utf8"),indent=1,ensure_ascii=False)
