#!/usr/bin/env python3
import sys,os,shutil,hashlib,importlib.util
d=os.path.dirname(os.path.abspath(__file__));s=importlib.util.spec_from_file_location("patch",d+"/patch.py");P=importlib.util.module_from_spec(s);s.loader.exec_module(P)
M=os.environ.get("R170_MIRROR")
PATHS={k:(os.path.join(M,k+os.path.splitext(p)[1]) if M else p) for k,p in P.LIVE.items()}
def sha(p): return hashlib.sha256(open(p,"rb").read()).hexdigest()
def apply(b):
    os.makedirs(b,exist_ok=True);new={}
    for k,p in PATHS.items():
        shutil.copy2(p,os.path.join(b,k));open(os.path.join(b,k+".sha"),"w").write(sha(p))
        new[k]=P.edit(k,open(p,encoding="utf8").read())
    for k,p in PATHS.items():
        open(p,"w",encoding="utf8").write(new[k]);open(os.path.join(b,k+".after"),"w").write(sha(p))
def restore(b):
    for k,p in PATHS.items():
        shutil.copy2(os.path.join(b,k),p);assert sha(p)==open(os.path.join(b,k+".sha")).read(),k
if __name__=="__main__":
    (apply if sys.argv[1]=="apply" else restore)(sys.argv[2]);print(sys.argv[1],"ok")
