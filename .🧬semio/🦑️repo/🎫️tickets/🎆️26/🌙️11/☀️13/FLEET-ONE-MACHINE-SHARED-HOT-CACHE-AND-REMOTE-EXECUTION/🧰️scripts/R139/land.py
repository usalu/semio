import os,sys,shutil,subprocess,hashlib
ROOT="/Users/ueli/Documents/semio/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R139/"
M,BASE,BK=SC+"m/",SC+"base/",SC+"backup/"
def files():
    for dp,_,fs in os.walk(M):
        for f in fs:yield os.path.relpath(os.path.join(dp,f),M)
def same(a,b):
    try:return open(a,"rb").read()==open(b,"rb").read()
    except FileNotFoundError:return False
def put(path,data):
    tmp=path+".r139"
    with open(tmp,"wb") as h:h.write(data)
    os.replace(tmp,path)
def apply():
    for rel in sorted(files()):
        live,mine,base,bk=ROOT+rel,M+rel,BASE+rel,BK+rel
        os.makedirs(os.path.dirname(bk),exist_ok=True)
        if os.path.exists(live) and not os.path.exists(bk):shutil.copy2(live,bk)
        if not os.path.exists(live) or same(live,base):put(live,open(mine,"rb").read());print("copied",rel[-60:]);continue
        if same(live,mine):print("already",rel[-60:]);continue
        r=subprocess.run(["git","merge-file","-p",live,base,mine],capture_output=True)
        if r.returncode!=0:print("CONFLICT",rel,r.returncode);sys.exit(1)
        put(live,r.stdout);print("merged",rel[-60:])
    for rel in sorted(files()):print(hashlib.sha256(open(ROOT+rel,"rb").read()).hexdigest()[:12],rel[-50:])
def restore():
    for rel in sorted(files()):
        bk=BK+rel
        if os.path.exists(bk):put(ROOT+rel,open(bk,"rb").read());print("restored",rel[-60:])
{"apply":apply,"restore":restore}[sys.argv[1]]()
