"""H80 landing: copies the staged files of the scratch clone over the live files after proving that every live file still equals the base it was cloned from (new files must not exist), under the area locks, atomically (temp file in the same folder, fsync, rename; new files first). --check applies nothing."""
import hashlib,os,sys,tempfile,time
REPO="/Users/ueli/Documents/semio"
T=REPO+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
STAGE=sys.argv[1]
if "--release" in sys.argv:
    for a in ["sidecar","remote-execution","ready"]:
        d=T+"/landing.lock."+a
        try:
            if "H80" in open(d+"/owner").read():os.remove(d+"/owner");os.rmdir(d);print("released",a)
        except OSError:pass
    sys.exit(0)
SC="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar"
BASES={SC+"/🧬️schema/🔣️.json":"8c1f37eeb2699a57",SC+"/🟦️.ts":"5a319113463c1d81",SC+"/🧪️tests/🔁️restart/🟦️.ts":"31b3adaecf87aa5d",SC+"/🚦️ready/🟦️.ts":"33d7081d1a238cd5",SC+"/🚦️ready/🧬️schema/🔣️.json":"5cf02766e6e88bc5"}
NEW=[SC+"/📇️code-record/🟦️.ts",SC+"/🧪️tests/📇️code-record/🟦️.ts",SC+"/🚦️ready/🧪️tests/📨️timing-sender/🟦️.ts"]
STOP=SC+"/🚦️ready/🧪️tests/🛑️stop-sender/🟦️.ts"
STOP_ADDED=['expect(result.stopBy,"the stop row travels into the run record").toMatchObject({supervisor:424242,reason:"stall"});','expect(result.stopBy).toMatchObject({supervisor:process.pid,reason:"cancel"});']
AREAS=["ready","remote-execution","sidecar"]
def sha(path):return hashlib.sha256(open(path,"rb").read()).hexdigest()[:16]
def text(path):return open(path,encoding="utf8").read()
problems=[]
for rel,base in BASES.items():
    if sha(REPO+"/"+rel)!=base:problems.append(("live changed since clone",rel))
for rel in NEW:
    if os.path.exists(REPO+"/"+rel):problems.append(("new file exists",rel))
pristine=text(STAGE+"/"+STOP)
for add in STOP_ADDED:pristine=pristine.replace(add,"")
if pristine!=text(REPO+"/"+STOP):problems.append(("live changed since clone",STOP))
for rel in list(BASES)+NEW+[STOP]:
    if not os.path.exists(STAGE+"/"+rel):problems.append(("staged file missing",rel))
if problems:
    for row in problems:print("REFUSED",*row)
    sys.exit(1)
print("bases ok",len(BASES)+1,"modified,",len(NEW),"new")
if "--check" in sys.argv:sys.exit(0)
held=[]
def release():
    for area in reversed(held):
        d=T+"/landing.lock."+area
        try:os.remove(d+"/owner");os.rmdir(d)
        except OSError:pass
try:
    for area in AREAS:
        d=T+"/landing.lock."+area;start=time.time()
        while True:
            try:os.mkdir(d);break
            except FileExistsError:
                if time.time()-start>180:print("lock busy",area);release();sys.exit(2)
                time.sleep(15)
        open(d+"/owner","w").write("H80 "+time.strftime("%H:%M")+"\n");held.append(area)
    for rel in NEW+[STOP]+list(BASES):
        src=STAGE+"/"+rel;dst=REPO+"/"+rel;os.makedirs(os.path.dirname(dst),exist_ok=True)
        fd,tmp=tempfile.mkstemp(dir=os.path.dirname(dst),prefix=".h80-")
        os.write(fd,open(src,"rb").read());os.fsync(fd);os.close(fd)
        os.chmod(tmp,(os.stat(dst).st_mode&0o7777) if os.path.exists(dst) else 0o644);os.replace(tmp,dst);print("landed",rel[-60:])
finally:
    if "--keep-locks" not in sys.argv:release()
