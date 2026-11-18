#!/usr/bin/env python3
import sys,os,shutil,hashlib,importlib.util
d=os.path.dirname(os.path.abspath(__file__));s=importlib.util.spec_from_file_location("patch",d+"/patch.py");P=importlib.util.module_from_spec(s);s.loader.exec_module(P)
def sha(p): return hashlib.sha256(open(p,"rb").read()).hexdigest()
def apply(b):
    os.makedirs(b,exist_ok=True)
    for k,p in P.LIVE.items():
        if os.path.exists(p): shutil.copy2(p,os.path.join(b,k)); open(os.path.join(b,k+".sha"),"w").write(sha(p))
        else: assert k=="sealed",k; open(os.path.join(b,k+".absent"),"w").write("")
    new={}
    for k,p in P.LIVE.items(): new[k]=P.edit(k,open(p,encoding="utf8").read() if os.path.exists(p) else "")
    for k,p in P.LIVE.items():
        os.makedirs(os.path.dirname(p),exist_ok=True); open(p,"w",encoding="utf8").write(new[k]); open(os.path.join(b,k+".after"),"w").write(sha(p))
def restore(b):
    for k,p in P.LIVE.items():
        if os.path.exists(os.path.join(b,k+".absent")):
            os.remove(p); os.rmdir(os.path.dirname(p)); continue
        shutil.copy2(os.path.join(b,k),p); assert sha(p)==open(os.path.join(b,k+".sha")).read(),k
if __name__=="__main__":
    (apply if sys.argv[1]=="apply" else restore)(sys.argv[2]);print(sys.argv[1],"ok")
