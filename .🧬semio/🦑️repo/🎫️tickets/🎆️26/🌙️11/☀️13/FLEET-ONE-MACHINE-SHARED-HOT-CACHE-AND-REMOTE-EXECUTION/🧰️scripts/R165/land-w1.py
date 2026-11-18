import sys,os,json,hashlib,shutil
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/"
F=R+"💾️storage/🟦️.ts"; TD=R+"🧪️tests/🏎️ports"
D=os.path.dirname(os.path.abspath(__file__))
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
E=[('import {lstat,readdir} from "node:fs/promises";\n',''),
('stat:async(portable:string)=>{try{const facts=await lstat(path(portable),{bigint:true});','stat:async(portable:string)=>{try{const facts=lstatSync(path(portable),{bigint:true});'),
('children:async(portable:string)=>(await readdir(path(portable),{withFileTypes:true})).map(','children:async(portable:string)=>readdirSync(path(portable),{withFileTypes:true}).map('),
('/** 🔌️ Binds the two no-follow operations a ledger verification needs to one generation root. */','/** 🔌️ Binds the two no-follow operations a ledger verification needs to one generation root; both are synchronous system calls (a stat through the thread pool costs milliseconds on a loaded machine, a synchronous one microseconds) and the verification loop yields every `progressWork` rows. */')]
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    old=read(F);new=old
    for a,b in E:
        assert new.count(a)==1,("anchor",a[:40],new.count(a));new=new.replace(a,b)
    os.makedirs(backup,exist_ok=True);write(backup+"/0.bak",old);json.dump({"before":sha(old),"after":sha(new)},open(backup+"/manifest.json","w"))
    write(F,new);os.makedirs(TD,exist_ok=True);shutil.copy(D+"/ports-test.ts",TD+"/🟦️.ts");print("applied")
else:
    m=json.load(open(backup+"/manifest.json"));now=sha(read(F))
    if now==m["after"]:write(F,read(backup+"/0.bak"));print("restored")
    elif now==m["before"]:print("already original")
    else:print("CONFLICT");sys.exit(4)
    shutil.rmtree(TD,ignore_errors=True)
