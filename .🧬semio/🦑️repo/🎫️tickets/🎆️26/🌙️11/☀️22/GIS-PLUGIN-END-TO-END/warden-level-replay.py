import importlib.util,json,sys,collections
old_path,new_path,samples=sys.argv[1:4]
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
old=load("old",old_path);new=load("new",new_path)
rows=[json.loads(l) for l in open(samples)]
def host(r,pressure=None):
    return {k:r[k] for k in("totalMegabytes","availableMegabytes","swapUsedMegabytes","swapOutMegabytesPerSecond","compressedMegabytes","pressure","swapFreeMegabytes","swapVolumeFreeMegabytes")}|({"pressure":pressure} if pressure else {})
def run(module,policy,pressure_of,interval):
    previous={"level":"calm","calmTicks":0};levels=[];need=[]
    for r in rows:
        h=host(r,pressure_of(r));
        if interval:h["intervalMilliseconds"]=1000
        d=module.decide(policy,h,[],previous);previous={"level":d["level"],"calmTicks":d["calmTicks"],"swapMilliseconds":d.get("swapMilliseconds",0)};levels.append((d["level"],d["trigger"]))
    return collections.Counter(levels)
kernel_normal=lambda r:"normal" if r["availableMegabytes"]*10000>=r["totalMegabytes"]*1500 else r["pressure"]
print("samples",len(rows))
print("old law, recorded pressure:",dict(run(old,old.POLICY,lambda r:None,False)))
print("new law, recorded pressure (worst case, kernel warn throughout):",dict(run(new,new.POLICY,lambda r:None,True)))
print("new law, pressure re-derived without compressed (kernel normal):",dict(run(new,new.POLICY,kernel_normal,True)))
