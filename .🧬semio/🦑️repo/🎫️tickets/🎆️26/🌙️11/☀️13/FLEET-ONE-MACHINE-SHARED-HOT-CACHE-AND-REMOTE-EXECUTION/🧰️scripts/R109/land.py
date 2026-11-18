import hashlib,os,shutil,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import stage
S=stage.S;FILES=stage.FILES;B=stage.B;N=stage.N
TESTS={"tests/🆔️record-revision":B+"👁️observation/🧪️tests/🆔️record-revision","tests/🔔️keep-alive":N+"🌐️owner-pool/🧪️tests/🔔️keep-alive"}
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
def land():
    plan={}
    for k,live in FILES.items():
        bk=S+"backup/"+k;st=S+"stage/"+k
        if sha(live)==sha(bk):plan[k]=st;continue
        tmp=S+"merge/"+k;os.makedirs(os.path.dirname(tmp),exist_ok=True);shutil.copyfile(live,tmp)
        d=subprocess.run(["diff","-u",bk,st],capture_output=True,text=True).stdout
        r=subprocess.run(["patch","--no-backup-if-mismatch","-s",tmp],input=d,text=True,capture_output=True)
        assert r.returncode==0,("merge conflict",k,r.stdout[:300],r.stderr[:300])
        plan[k]=tmp;print("merged over a peer edit:",k)
    for k,src in plan.items():
        pre=S+"prelanding/"+k;os.makedirs(os.path.dirname(pre),exist_ok=True);shutil.copyfile(FILES[k],pre);shutil.copyfile(src,FILES[k])
    for k,live in TESTS.items():shutil.copytree(S+"stage/"+k,live,dirs_exist_ok=True)
def restore():
    for k,live in FILES.items():
        landed=S+"merge/"+k if os.path.exists(S+"merge/"+k) else S+"stage/"+k
        if sha(live)==sha(landed):shutil.copyfile(S+"prelanding/"+k,live)
        else:print("live differs from what was landed, not restoring",k)
    for k,live in TESTS.items():
        if os.path.realpath(live).startswith("/Users/ueli/Documents/semio/"):shutil.rmtree(live,ignore_errors=True)
if __name__=="__main__":
    land() if sys.argv[1]=="land" else restore()
    print(sys.argv[1],"ok")
