import os,re,sys,subprocess,shutil
sys.path.insert(0,os.path.dirname(__file__))
from ed import edit,ROOT
FW=ROOT+"/🧰️framework/"
RE_URL=re.compile(r'(?:readModuleTreeSource|readFileSync|await Bun\.file)\(\s*new URL\((["\'])([^"\']+/🦀️\.rs)\1\s*,\s*import\.meta\.url\)(?:\s*,\s*["\']utf-?8["\']|\s*\)\.text\(\))?\s*\)')
RE_URL2=re.compile(r'(?:await\s+)?Bun\.file\(\s*new URL\((["\'])([^"\']+/🦀️\.rs)\1\s*,\s*import\.meta\.url\)\s*\)\.text\(\)')
def family_dirs(test,text):
    base=os.path.dirname(os.path.join(ROOT,test))
    out=[]
    for m in list(RE_URL.finditer(text))+list(RE_URL2.finditer(text)):
        rel=m.group(2)
        tgt=os.path.normpath(os.path.join(base,rel))
        d=os.path.dirname(tgt)
        out.append((m.group(0),m.group(1),rel,d))
    return out
def famify(test):
    p=os.path.join(ROOT,test); t=open(p,encoding="utf-8").read(); orig=t
    n=0
    for full,q,rel,d in family_dirs(test,t):
        if "🧪️tests" in d.split("/")[-1:] : continue
        reldir=os.path.relpath(d,os.path.dirname(p))
        new=f'readSourceFamilySource(new URL({q}{reldir}/{q},import.meta.url))'
        if full in t:
            t=t.replace(full,new); n+=1
    if n==0: return 0
    open(p,"w",encoding="utf-8").write(t)
    edit(test,[],True,"readSourceFamilySource")
    return n
if __name__=="__main__":
    for tst in sys.argv[1:]: print(famify(tst),tst[-80:])
