import os,subprocess,sys,time,datetime,re,json
L="/Users/ueli/Documents/semio/"
T=L+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
D=os.path.dirname(os.path.abspath(__file__));S=os.environ["R164_SCRATCH"]
BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
B=L+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/"
RUN=[B+"🧪️tests/🤝️pool-admission",B+"🌐️owner-pool/🧪️tests/🧭️turn",B+"🌐️owner-pool/🤝️fleet/🧪️tests",B+"🧪️tests/🤝️selected-fleet",B+"🧪️tests/🩻️heavy-refusal",B+"🧪️tests/🧮️known-unit",B+"🧪️tests/🔭️look-job",B+"🧪️tests/📥️take",B+"🌐️owner-pool/🧪️tests/🏋️heavy",B+"🌐️owner-pool/🧪️tests/🔁️no-record",B+"🌐️owner-pool/🧪️tests/🔔️keep-alive",B+"🌐️owner-pool/🧪️tests/🎚️lanes",B+"🌐️owner-pool/🧪️tests/🎛️one-budget",B+"🌐️owner-pool/🧪️tests/🩺️pool-status-read"]
env={k:v for k,v in os.environ.items() if not k.startswith("SEMIO_REMOTE_CACHE") and not k.startswith("SEMIO_FLEET_")}
def minute(): return datetime.datetime.utcnow().minute
if os.environ.get("R164_NOWAIT")!="1":
    while not (minute()%30<5): time.sleep(1)
lock=T+"/landing.lock";waited=0
while True:
    try: os.mkdir(lock);break
    except FileExistsError:
        time.sleep(2);waited+=2
        if waited>=240: print("never got the lock:",open(lock+"/owner").read());sys.exit(1)
open(lock+"/owner","w").write("claude-R164 %sZ\n"%datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%S"))
start=datetime.datetime.utcnow();print("lock taken",start.time(),"after",waited,"s",flush=True)
bk=S+"/bk"+start.strftime("%H%M")
def release():
    try: os.remove(lock+"/owner")
    except Exception: pass
    os.rmdir(lock)
def restore():
    print(subprocess.run(["python3",D+"/patches.py","restore",bk],capture_output=True,text=True).stdout)
r=subprocess.run(["python3",D+"/patches.py","apply",bk],capture_output=True,text=True);print(r.stdout,r.stderr)
if r.returncode!=0: release();sys.exit(2)
bad=[]
for d in RUN:
    if (datetime.datetime.utcnow()-start).total_seconds()>170: bad.append("time budget before "+d);break
    p=subprocess.run([BUN,"test","./🟦️.ts","--timeout","120000"],cwd=d,env=env,capture_output=True,text=True,stdin=subprocess.DEVNULL)
    o=p.stdout+p.stderr;fails=re.findall(r"^\s*(\d+) fail",o,re.M);ps=re.findall(r"^\s*(\d+) pass",o,re.M)
    print(d.split("/")[-2] if "🧪️tests" in d.split("/")[-1] and False else d.split("/")[-1] or d.split("/")[-2],"pass",ps[-1] if ps else "?","fail",fails[-1] if fails else "?",flush=True)
    if p.returncode!=0 or (fails and fails[-1]!="0"): bad.append(d.split("/")[-1])
if bad:
    print("RED",bad);restore();release();print("restored",datetime.datetime.utcnow().time());sys.exit(3)
print("GREEN",start.time(),datetime.datetime.utcnow().time());release()
