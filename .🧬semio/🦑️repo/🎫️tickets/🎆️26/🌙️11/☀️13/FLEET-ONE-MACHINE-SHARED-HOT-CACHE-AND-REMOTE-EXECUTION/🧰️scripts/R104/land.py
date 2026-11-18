import hashlib,os,shutil,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R104/"
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
N=L+"🏗️build/🕸️graph/📥️inference/📦️project-nodes/"
FILES={f:N+f for f in["🌐️owner-pool/🧬️schema/🔣️.json","🌐️owner-pool/🟦️.ts","🌐️owner-pool/🏃️runner/🟨️.mjs","🌐️owner-pool/📣️publisher/🟦️.ts","🌐️owner-pool/🧒️child/🟨️.mjs","🌐️owner-pool/🧪️tests/🧫️fixtures/🧮️driver.mjs","🟨️.mjs"]}
FILES["sidecar-pool.ts"]=L+"🏗️build/🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts"
TESTDIR="🌐️owner-pool/🧪️tests/🌊️early-wave"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
import subprocess,tempfile
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
    shutil.copytree(S+"stage/"+TESTDIR,N+TESTDIR,dirs_exist_ok=True)
def restore():
    for k,live in FILES.items():
        landed=S+"merge/"+k if os.path.exists(S+"merge/"+k) else S+"stage/"+k
        if sha(live)==sha(landed):shutil.copyfile(S+"prelanding/"+k,live)
        else:print("live differs from what was landed, not restoring",k)
    shutil.rmtree(N+TESTDIR,ignore_errors=True)
if __name__=="__main__":
    land() if sys.argv[1]=="land" else restore()
    print(sys.argv[1],"ok")
