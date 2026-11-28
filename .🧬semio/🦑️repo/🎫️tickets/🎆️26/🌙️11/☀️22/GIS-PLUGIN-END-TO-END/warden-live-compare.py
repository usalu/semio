import importlib.util,json,subprocess,sys
old_path,new_path=sys.argv[1:3]
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
old=load("old",old_path);new=load("new",new_path)
out=subprocess.run(["repo","capacity","status","--json","--lang=en"],capture_output=True,text=True).stdout
d=json.loads(out[out.find("{"):out.rfind("}")+1]);w=d["warden"];h=w["host"]
keys=("totalMegabytes","availableMegabytes","swapUsedMegabytes","swapOutMegabytesPerSecond","compressedMegabytes","pressure","swapFreeMegabytes","swapVolumeFreeMegabytes")
base={k:h[k] for k in keys}
def show(title,host,previous_old,previous_new,seconds=None):
    o=old.decide(old.POLICY,host,[],previous_old);n=new.decide(new.POLICY,dict(host,intervalMilliseconds=1000),[],previous_new)
    def hold(m,r):
        st={"hold":r["hold"],"atMs":1000,"level":r["level"],"guardMegabytes":r["guardMegabytes"],"host":{"availableMegabytes":host["availableMegabytes"]}}
        return m.holds({"state":st,"nowMs":1000,"needMegabytes":256,"operation":"cargo"}) if m is new else r["hold"]
    print(f"{title}\n   sample: avail {host['availableMegabytes']} MB, swap-out {host['swapOutMegabytesPerSecond']} MB/s, compressed {host['compressedMegabytes']} MB, swap used {host['swapUsedMegabytes']} MB, kernel {host['pressure']}")
    print(f"   old: level {o['level']} trigger {o['trigger']}; 256 MB cargo unit held: {o['hold']}")
    print(f"   new: level {n['level']} trigger {n['trigger']}; 256 MB cargo unit held: {hold(new,n)}")
calm={"level":"calm","calmTicks":0}
show("live sample now (warden state.json, via repo capacity status)",base,calm,calm)
show("live sample now with the previous tick already at guard",base,{"level":"guard","calmTicks":0},{"level":"guard","calmTicks":0})
cancel={"totalMegabytes":16384,"availableMegabytes":2731,"swapUsedMegabytes":2940,"swapOutMegabytesPerSecond":529.75,"compressedMegabytes":7860,"pressure":"warn","swapFreeMegabytes":1155,"swapVolumeFreeMegabytes":36000}
show("04:36:43 cancel sample (warden.log: cancelled cargo 588 MB), previous tick guard",cancel,{"level":"guard","calmTicks":0},{"level":"guard","calmTicks":0})
paging=dict(base,availableMegabytes=4159,swapOutMegabytesPerSecond=2.5,compressedMegabytes=7761,pressure="warn")
show("04:3x admission sample (4159 MB available, 2.5 MB/s, compressed 7761, derived pressure warn)",paging,{"level":"guard","calmTicks":0},{"level":"guard","calmTicks":0})
