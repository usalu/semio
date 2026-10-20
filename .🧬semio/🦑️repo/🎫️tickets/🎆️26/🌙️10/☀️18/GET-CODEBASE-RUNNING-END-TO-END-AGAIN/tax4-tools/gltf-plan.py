# 🔬 Plans the gltf pairing repair: fixture leaf renames (assert_case leaf differs from the schema leaf) and implementation case renames (case dir differs from the fixture case). usage: python3 gltf-plan.py <out.json>
import re,os,subprocess,sys,json,collections
G="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any"
F=f"{G}/🧪️tests/🧫️fixtures/🧬️mutations"; M=f"{G}/🧬️schema/🧬️mutations/"
fs=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",M],capture_output=True,text=True).stdout.split("\n") if os.path.isfile(f) and f.endswith("/🦀️.rs") and "/🧪️tests/" in f[len(M):]]
leafmv={};casemv=[];odd=[]
for f in fs:
    rel=f[len(M):]; leaf,rest=rel.split("/🧪️tests/",1); parts=rest.split("/")
    if len(parts)!=2: continue
    case=parts[0]
    t=open(f,encoding="utf8").read()
    m=re.search(r'assert_case\("([^"]+)"\)',t)
    if not m: odd.append((rel,"no assert_case")); continue
    dom,fleaf,fcase=m.group(1).split("/")
    sleaf=leaf.split("/")[-1]
    if leaf.split("/")[0]!=dom: odd.append((rel,"domain differs",m.group(1)))
    if fleaf!=sleaf: leafmv[(dom,fleaf)]=sleaf
    if case!=fcase: casemv.append({"leaf":leaf,"from":case,"to":fcase,"assertLeaf":fleaf})
print("leaf renames",len(leafmv),"case renames",len(casemv),"odd",len(odd))
for k,v in leafmv.items(): print("  leaf",k,"->",v)
for o in odd[:10]: print("  odd",o)
dups=collections.Counter((c["leaf"],c["to"]) for c in casemv)
print("dups",[k for k,n in dups.items() if n>1])
bad=[c for c in casemv if os.path.exists(f"{M}{c['leaf']}/🧪️tests/{c['to']}")]
print("targets occupied",len(bad))
json.dump({"leafmv":[[d,a,b] for (d,a),b in leafmv.items()],"casemv":casemv,"odd":odd},open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=1)
