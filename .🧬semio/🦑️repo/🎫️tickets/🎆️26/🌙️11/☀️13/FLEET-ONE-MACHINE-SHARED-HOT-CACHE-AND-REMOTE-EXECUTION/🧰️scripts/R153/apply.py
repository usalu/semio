# apply.py <group> apply|restore  -- apply refuses when live base sha changed
import sys,json,os,shutil
sys.path.insert(0,os.path.dirname(__file__))
from stagelib import *
g,mode=sys.argv[1],sys.argv[2];d=f"{S}/g-{g}";man=json.load(open(d+"/manifest.json"));bk=d+"/backup";os.makedirs(bk,exist_ok=True)
if mode=="apply":
    for m in man:
        if sha(open(m["path"],"rb").read())!=m["base"]: print("LIVE CHANGED",m["path"]);sys.exit(3)
    for m in man: shutil.copyfile(m["path"],f'{bk}/{m["i"]}.old')
    for m in man: open(m["path"],"wb").write(open(f'{d}/{m["i"]}.new',"rb").read())
    print("applied",len(man))
else:
    for m in man: shutil.copyfile(f'{bk}/{m["i"]}.old',m["path"])
    print("restored",len(man))
