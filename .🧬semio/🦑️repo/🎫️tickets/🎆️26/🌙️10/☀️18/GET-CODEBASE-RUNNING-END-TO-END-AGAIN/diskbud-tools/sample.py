"""[lane diskbud] read-only size sampler: df of every mounted cache image, du -sk of the loose areas, one JSON line appended to the output file."""
import json,os,subprocess,sys,tempfile,time
ROOT=os.environ.get("SEMIO_REPO","/Users/ueli/Documents/semio")
HOME=os.path.expanduser("~")
TMP=tempfile.gettempdir()
def du(path):
    if not os.path.exists(path):return None
    r=subprocess.run(["nice","-n","15","du","-sk","-x",path],capture_output=True,text=True)
    try:return int(r.stdout.split()[0])
    except Exception:return None
def df(path):
    try:
        s=os.statvfs(path);return {"totalKiB":s.f_blocks*s.f_frsize//1024,"freeKiB":s.f_bavail*s.f_frsize//1024}
    except Exception:return None
mounts={"products":".cache/b/p/m","tools":".cache/b/t/m","dependencies":".cache/b/d/m","graph":".cache/b/g/m","diagnostics":".cache/b/o/m","rust":".🧬semio/🦑️repo/⚡️cache/rust/fixed/m","leases":".cache/b/l/m"}
J=os.path.join(HOME,"Library/Caches/semio/repo")
out={"at":int(time.time()),"df":{k:df(os.path.join(ROOT,v)) for k,v in mounts.items()}}
out["host"]=df("/")
loose={"journal-new":J,"journal-old-tmp":os.path.join(TMP,"semio-repo-501"),"journal-old-products":os.path.join(ROOT,".cache/b/p/m/tmp/semio-repo-501"),
 "mirrors":os.path.join(TMP,"sm"),"tmpdir":TMP,"library-semio":os.path.join(HOME,"Library/Caches/semio"),
 "tickets":os.path.join(ROOT,".🧬semio/🦑️repo/🎫️tickets"),"native-loose":os.path.join(ROOT,".cache/command"),"lease-registry":os.path.join(HOME,"Library/Caches/semio/lease-registry"),
 "unit-lanes":os.path.join(ROOT,".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l"),"unit-admissions":os.path.join(ROOT,".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/a"),"unit-objects":os.path.join(ROOT,".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/o"),
 "bazel-outputs-s0":os.path.join(ROOT,".cache/b/p/m/bazel/outputs/server/s0"),"bazel-outputs-s1":os.path.join(ROOT,".cache/b/p/m/bazel/outputs/server/s1"),"bazel-outputs-s2":os.path.join(ROOT,".cache/b/p/m/bazel/outputs/server/s2")}
out["du"]={k:du(v) for k,v in loose.items()}
mem=os.path.join(J,os.listdir(J)[0],"journal","graph-memo") if os.path.isdir(J) and os.listdir(J) else None
if mem and os.path.isdir(mem):out["memoFiles"]=len(os.listdir(mem))
open(sys.argv[1],"a",encoding="utf8").write(json.dumps(out,ensure_ascii=False)+"\n")
