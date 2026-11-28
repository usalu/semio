import os,sys,hashlib,time,shutil
ROOT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
STAGE=sys.argv[1];BASES=sys.argv[2];mode=sys.argv[3]  # mode: bases | land
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
files=[]
for d,_,fs in os.walk(STAGE):
    for f in fs:
        if f.endswith(".h85.tmp"):continue
        files.append(os.path.relpath(os.path.join(d,f),STAGE))
files.sort()
if mode=="bases":
    open(BASES,"w",encoding="utf8").write("".join(f"{sha(os.path.join(ROOT,f)) if os.path.exists(os.path.join(ROOT,f)) else 'NEW'}\t{f}\n" for f in files));print(len(files));sys.exit()
bases={b:a for a,b in (l.rstrip("\n").split("\t",1) for l in open(BASES,encoding="utf8"))}
areas={"caching-cargo":lambda f:f.startswith("⚡️caching"),"fleet-status":lambda f:"fleet-status" in f,"remote-execution":lambda f:"remote-execution" in f}
need=sorted(a for a,m in areas.items() if any(m(f) for f in files))
held=[]
try:
    for a in need:
        waited=0
        while True:
            try:os.mkdir(f"{T}/landing.lock.{a}");break
            except FileExistsError:
                time.sleep(5);waited+=5
                if waited>=240:print("lock busy",a);open(f"{T}/landing.queue.{a}","a").write(f"H85 {time.strftime('%H:%M')} waiting\n");sys.exit(3)
        open(f"{T}/landing.lock.{a}/owner","w").write(f"H85 {time.strftime('%H:%M')}\n");held.append(a)
    for f in files:
        live=os.path.join(ROOT,f);b=bases[f]
        if b=="NEW":assert not os.path.exists(live),f
        else:assert sha(live)==b,"stale base "+f
    for f in files:
        live=os.path.join(ROOT,f);os.makedirs(os.path.dirname(live),exist_ok=True);shutil.copyfile(os.path.join(STAGE,f),live+".h85.tmp");
    for f in files:
        live=os.path.join(ROOT,f);os.replace(live+".h85.tmp",live)
    print("landed",len(files),time.strftime("%H:%M:%S"))
finally:
    for a in reversed(held):shutil.rmtree(f"{T}/landing.lock.{a}",ignore_errors=True)
