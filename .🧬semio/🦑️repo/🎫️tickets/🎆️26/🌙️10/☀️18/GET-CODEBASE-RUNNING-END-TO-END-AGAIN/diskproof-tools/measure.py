"""[lane diskproof] read-only streaming measurement of every disk consumer of the repository: physical (df of mounts, st_blocks of image files) and apparent (du -sk)."""
import json,os,subprocess,sys,tempfile,time
ROOT=os.environ.get("SEMIO_REPO","/Users/ueli/Documents/semio")
HOME=os.path.expanduser("~")
TMP=os.environ.get("TMPDIR",tempfile.gettempdir()).rstrip("/")
GIB=1024**3
def du(path):
    if not os.path.exists(path):return None
    r=subprocess.run(["nice","-n","15","du","-sk","-x",path],capture_output=True,text=True)
    try:return int(r.stdout.split()[0])*1024
    except Exception:return None
def df(path):
    try:
        s=os.statvfs(path);return {"total":s.f_blocks*s.f_frsize,"used":(s.f_blocks-s.f_bfree)*s.f_frsize,"free":s.f_bavail*s.f_frsize}
    except Exception:return None
def blocks(path):
    try:
        st=os.stat(path);return {"apparent":st.st_size,"physical":st.st_blocks*512}
    except Exception:return None
out={"at":int(time.time())}
images={"tools":"t","graph":"g","dependencies":"d","products":"p","diagnostics":"o","nx":"n","leases":"l"}
out["images"]={}
for name,k in images.items():
    d=os.path.join(ROOT,".cache/b",k)
    out["images"][name]={"file":blocks(os.path.join(d,"fixed.dmg")),"mount":df(os.path.join(d,"m"))}
rust=os.path.join(ROOT,".🧬semio/🦑️repo/⚡️cache/rust/fixed")
out["images"]["rust"]={"file":blocks(rust+".dmg") or blocks(os.path.join(os.path.dirname(rust),"rust","fixed.dmg")),"mount":df(os.path.join(rust,"m"))}
out["host"]=df("/")
J=os.path.join(HOME,"Library/Caches/semio")
areas=dict(
 hub=os.path.join(ROOT,".🧬semio/🌐hub"),
 mirrors=os.path.join(TMP,"sm"),mirrors_private_tmp_m="/private/tmp/m",
 tickets=os.path.join(ROOT,".🧬semio/🦑️repo/🎫️tickets"),
 journal=os.path.join(J,"repo"),library_semio=J,
 native_cache=os.path.join(ROOT,".cache"),
 repo_cache_loose=os.path.join(ROOT,".🧬semio/🦑️repo/⚡️cache"),
 stray_framework=os.path.join(ROOT,"🧰️framework/.cache"),
 generated_process=os.path.join(ROOT,"🧰️framework/🤖️generated/process"),
 old_journal_tmp=os.path.join(TMP,"semio-repo-501"),
)
only=sys.argv[2:] 
out["du"]={}
for k,v in areas.items():
    if only and k not in only:continue
    out["du"][k]=du(v)
open(sys.argv[1],"a",encoding="utf8").write(json.dumps(out,ensure_ascii=False)+"\n")
