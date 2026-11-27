"""📜️ Derives the explicit client re-export list of the job-context facade: step root public names that appear in any file naming the job crates."""
import os,re,sys,json
root=os.getcwd()
step=open("🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs",encoding="utf8").read()
def strip(t):
    t=re.sub(r'//[^\n]*','',t)
    t=re.sub(r'"(?:\\.|[^"\\])*"','""',t)
    return t
s=strip(step)
names=set()
for m in re.finditer(r'(?m)^pub use ([^;]*);',s):
    body=m.group(1)
    for it in re.split(r'[{},]',body):
        it=it.strip()
        if not it or it=='*':continue
        it=it.split('::')[-1]
        if ' as ' in it:it=it.split(' as ')[-1]
        if re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*',it.strip()):names.add(it.strip())
    if '*' in body.replace('::*','::*'):print("GLOB",body,file=sys.stderr)
for m in re.finditer(r'(?m)^pub (?:unsafe |const |async )*(?:struct|enum|fn|trait|type|const|static|mod|union) ([A-Za-z_][A-Za-z_0-9]*)',s):
    names.add(m.group(1))
names-=set(["sealed"])
print("step names",len(names),file=sys.stderr)
pat=re.compile(r'semio_framework_job(?:_context)?\b')
ident=re.compile(r'[A-Za-z_][A-Za-z_0-9]*')
used=set()
skip={'node_modules','.git','.cache','.🧬semio','target','.next','dist'}
nfiles=0
for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d not in skip]
    for f in fn:
        if not f.endswith('.rs'):continue
        p=os.path.join(dp,f)
        try:t=open(p,encoding='utf8').read()
        except Exception:continue
        if not pat.search(t):continue
        if '/⏱️context/🪟️facade/' in p or '/⏱️context/🧱️step/' in p:continue
        nfiles+=1
        used|=set(ident.findall(t))
print("files",nfiles,file=sys.stderr)
tk=strip(open("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testkit/🦀️.rs",encoding="utf8").read())
tnames=set()
for m in re.finditer(r'(?m)^    pub use ([^;]*);',tk):
    for it in re.split(r'[{},]',m.group(1)):
        it=it.strip().split('::')[-1]
        if ' as ' in it:it=it.split(' as ')[-1]
        if re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*',it):tnames.add(it)
for m in re.finditer(r'(?m)^    pub (?:unsafe |const |async )*(?:struct|enum|fn|trait|type|const|static|mod) ([A-Za-z_][A-Za-z_0-9]*)',tk):tnames.add(m.group(1))
res=sorted(names&used)
json.dump({"testkit_all":sorted(tnames),"testkit_used":sorted(tnames&used)},open(sys.argv[1]+".testkit","w",encoding="utf8"),ensure_ascii=False)
json.dump({"all":sorted(names),"used":res},open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False)
print(len(res),"of",len(names),file=sys.stderr)
