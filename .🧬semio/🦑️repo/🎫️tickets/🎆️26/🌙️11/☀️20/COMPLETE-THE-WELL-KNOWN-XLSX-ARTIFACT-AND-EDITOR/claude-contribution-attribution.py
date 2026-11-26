import subprocess,json,re,sys,os
S=sys.argv[1]
fc=json.load(open(S+"/fc.json"))
src=open("/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🤖️generated/🧩️composition/🦀️.rs").read()
roots=re.findall(r'(semio_s_artifact_stdio_[a-z0-9_]+)::contribution',src)
def cl(p):
    o=subprocess.run(["cargo","tree","--offline","-p",p.replace('_','-'),"--target","wasm32-wasip2","-e","normal","--prefix","none"],capture_output=True,text=True).stdout
    return set(re.sub(r' v[0-9].*','',l).strip().replace('-','_') for l in o.splitlines() if l.strip())
C={r:cl(r) for r in roots}
# also text apps + plugin base
apps=set(l.strip() for l in open(S+"/c-apps.txt") if l.strip()); apps={a.replace('-','_') for a in apps}
C["TEXT_APPS"]=apps
w=lambda s:sum(fc.get(c,0) for c in s)
allr=list(C)
common=set.intersection(*[C[r] for r in allr if r!="TEXT_APPS"]) if True else set()
print("common to all contributions",len(common),w(common))
rows=[]
for r in allr:
    others=set().union(*[C[o] for o in allr if o!=r])
    ex=C[r]-others
    rows.append((w(ex),w(C[r]-common),r))
rows.sort(reverse=True)
for e,t,r in rows: print(f"{e:8d} exclusive {t:8d} total-ex-common  {r}")
print("union",w(set().union(*C.values())))
