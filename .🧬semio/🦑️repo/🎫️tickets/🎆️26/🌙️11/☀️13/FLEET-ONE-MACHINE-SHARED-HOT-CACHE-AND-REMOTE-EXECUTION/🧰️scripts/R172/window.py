import os,subprocess,sys,time,datetime,re
L="/Users/ueli/Documents/semio/"
T=L+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R172"
BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
B=L+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
RUN=[B+"🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests",B+"🌡️host-capacity/🧹️disk-recovery/🗃️replica-store/🧪️tests",B[:-9]+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🌱️regrow",B+"🌡️host-capacity/🧹️disk-recovery/🧪️tests",B+"🌡️host-capacity/💽️disk/🧪️tests",B+"🗄️cas/🧪️tests",B+"🧹️cache-gc/🧪️tests/💰️budget",B+"🧹️cache-gc/🧪️tests/♻️quota",B+"🧹️cache-gc/🧪️tests/🚀️launch",B+"🌡️host-capacity/🧪️tests",B+"🌡️host-capacity/🧪️tests/🤫️quiet"]
env={k:v for k,v in os.environ.items() if not k.startswith("SEMIO_REMOTE_CACHE") and not k.startswith("SEMIO_FLEET_")}
now=lambda:datetime.datetime.utcnow()
if os.environ.get("R172_NOWAIT")!="1":
    while not (now().minute%30<5): time.sleep(1)
lock=T+"/landing.lock";waited=0
while True:
    try: os.mkdir(lock);break
    except FileExistsError:
        time.sleep(2);waited+=2
        if waited>=200: print("never got the lock:",open(lock+"/owner").read());sys.exit(1)
open(lock+"/owner","w").write("claude-R172 %sZ\n"%now().strftime("%Y-%m-%dT%H:%M:%S"))
start=now();print("lock taken",start.time(),"after",waited,"s",flush=True)
bk=S+"/bk"+start.strftime("%H%M")
def release():
    try: os.remove(lock+"/owner")
    except Exception: pass
    os.rmdir(lock)
def restore(): print(subprocess.run(["python3",S+"/land.py","restore",L[:-1]+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",bk],capture_output=True,text=True).stdout)
r=subprocess.run(["python3",S+"/land.py","apply",L[:-1]+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",bk],capture_output=True,text=True);print(r.stdout,r.stderr)
if r.returncode!=0: release();sys.exit(2)
bad=[]
for d in RUN:
    if (now()-start).total_seconds()>165: bad.append("time budget before "+d);break
    p=subprocess.run([BUN,"test","./🟦️.ts","--timeout","120000"],cwd=d,env=env,capture_output=True,text=True,stdin=subprocess.DEVNULL)
    o=p.stdout+p.stderr;fails=re.findall(r"^\s*(\d+) fail",o,re.M);ps=re.findall(r"^\s*(\d+) pass",o,re.M)
    print(d.replace(B,""),"pass",ps[-1] if ps else "?","fail",fails[-1] if fails else "?",flush=True)
    if p.returncode!=0 or (fails and fails[-1]!="0"): bad.append(d.replace(B,""))
if bad:
    print("RED",bad);restore();release();print("restored",now().time());sys.exit(3)
print("GREEN",start.time(),now().time());release()
