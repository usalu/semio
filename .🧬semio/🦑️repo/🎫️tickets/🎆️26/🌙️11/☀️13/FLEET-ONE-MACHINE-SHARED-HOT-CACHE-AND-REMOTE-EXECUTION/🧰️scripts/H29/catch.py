# -*- coding: utf-8 -*-
import json, os, subprocess, sys, time
ROOT="/Users/ueli/Documents/semio"
S="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H29"
probe="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🚪️gate/🟦️.ts"
big="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/🧪️tests/🟦️.ts"
def run(file,mode):
    env={**os.environ,"SEMIO_TEST_FILE_OFFLOAD":mode,"SEMIO_LANGUAGE":"en"}
    t=time.time()
    r=subprocess.run(["nice","-n","10",os.path.expanduser("~/.local/bin/repo"),"test-file","--json","--",file],cwd=ROOT,env=env,capture_output=True,text=True)
    try: d=json.loads(r.stdout)
    except Exception: d={}
    rows=[(x.get("status"),x.get("pass"),x.get("fail"),x.get("seconds"),x.get("ranOn")) for x in d.get("files",[])]
    return r.returncode,round(time.time()-t,1),rows,r.stderr
end=time.time()+float(sys.argv[1])
while time.time()<end:
    code,wall,rows,err=run(probe,"always")
    print("probe",code,wall,rows,flush=True)
    if rows and rows[0][4]:
        code,wall,rows,err=run(big,"always")
        print("BIG always",code,wall,rows,flush=True)
        open(S+"/big-always.err","w",encoding="utf-8").write(err)
        break
    time.sleep(20)
