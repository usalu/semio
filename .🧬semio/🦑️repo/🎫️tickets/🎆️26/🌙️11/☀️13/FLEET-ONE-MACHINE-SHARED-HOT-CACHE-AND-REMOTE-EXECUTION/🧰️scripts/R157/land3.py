import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
MIRROR=os.environ.get("R157_MIRROR")
def loc(p): return p if MIRROR is None else MIRROR+"/"+os.path.basename(os.path.dirname(p))+"-"+os.path.basename(p)
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🏠️home"
FILES={"home":H+"/🟦️.ts","schema":H+"/🧬️schema/🔣️.json"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:80])
    return text.replace(old,new)
def home(t):
    t=one(t,'import {delimiter,dirname,join,sep} from "node:path";','import {hostname} from "node:os";\nimport {delimiter,dirname,join,sep} from "node:path";')
    return one(t,'env.NO_PROXY=H.noProxy;env.no_proxy=H.noProxy;','const bypass=`${H.noProxy},${hostname()}`;env.NO_PROXY=bypass;env.no_proxy=bypass;')
def schema(t):
    return one(t,'"noProxy": "127.0.0.1,localhost,::1"','"noProxy": "127.0.0.1,localhost,[::1],0.0.0.0"')
PATCH={"home":home,"schema":schema}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(loc(p)))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(loc(p));write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(loc(p),new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(loc(p)))
        if now==m["after"]: write(loc(p),read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
