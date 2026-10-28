import json,statistics,sys
ep=json.load(open(sys.argv[1]))["episodes"][0]
base=[{"id":j["id"],"a":j["arrival"],"s":j["service"],"c":j["class"],"obs":j["observedWait"]} for j in ep["jobs"]]
dev=[{"id":f"dev{i}","a":60*60*(i+1),"s":300,"c":"interactive","obs":None} for i in range(6)]
def sim(js,adv):
    js=sorted(js,key=lambda j:j["a"]);t=0;pend=[];done={};i=0
    while i<len(js) or pend:
        while i<len(js) and js[i]["a"]<=t:pend.append(js[i]);i+=1
        if not pend:t=js[i]["a"];continue
        pend.sort(key=lambda j:(j["a"]-adv[j["c"]],j["id"]));j=pend.pop(0);done[j["id"]]=t-j["a"];t+=j["s"]
    return done
def p95(v):v=sorted(v);return v[min(len(v)-1,int(len(v)*0.95))]
fifo=sim(base,{"interactive":0,"agent":0,"background":0})
obs=[(j["obs"],fifo[j["id"]]) for j in base]
print("FIFO replay vs observed wait: mean %.0f vs %.0f min, corr %.2f"%(statistics.mean(f for _,f in obs)/60,statistics.mean(o for o,_ in obs)/60,__import__("numpy").corrcoef([o for o,_ in obs],[f for _,f in obs])[0][1] if False else 0))
for name,w in [("fifo",0),("15m",900),("30m",1800),("45m",2700),("60m",3600),("90m",5400),("120m",7200)]:
    adv={"interactive":w,"agent":0,"background":-w};r=sim(base+dev,adv);agg={}
    for j in base+dev:agg.setdefault(j["c"],[]).append(r[j["id"]])
    print(name.ljust(5),{k:"mean %d p95 %d max %d min"%(statistics.mean(v)/60,p95(v)/60,max(v)/60) for k,v in agg.items()})
