import sys,re,collections,json,os
C=sys.argv[1]; S=C+"/🗑️generated/repo-tests-b"
rows={}
def load(d,genuine):
  for l in open(S+"/%s/summary.tsv"%d):
    n,rc,p,f,sec,file=l.rstrip("\n").split("\t")
    rows.setdefault(file,[]).append(dict(src=d,n=int(n),rc=rc,p=p,f=f,sec=sec,genuine=genuine))
for d,g in (("run1",False),("gen1",True),("gen2",True),("gen3",True),("re1",True),("re2",True),("re4a",True),("re4b",True),("sw2a",True),("sw2b",True)): load(d,g)
fixed=set(l.strip() for l in open(C+"/repo-tests-b-tools/fixed.txt")) if os.path.exists(C+"/repo-tests-b-tools/fixed.txt") else set()
def verdict(file):
  r=rows[file]
  if file in fixed: return "GREEN-after-fix",None
  gen=[x for x in r if x["genuine"]]
  pick=gen[-1] if gen else r[-1]
  if any(x["genuine"] and x["rc"]=="0" for x in r): return "GREEN",pick
  if not gen and pick["rc"]=="0": return "GREEN(envelope)",pick
  return "RED",pick
def cause(x):
  txt=open("%s/%s/logs/%04d.log"%(S,x["src"],x["n"]),errors="replace").read()
  if "Owner projection policy digest" in txt: return "VSCODE-REMOVAL (owner-projection digest)"
  if "Original process birth already received" in txt: return "multi-test birth (needs harness)"
  if "Live Original resource control issuance required" in txt or "Live Original process issuance required" in txt: return "stale hand-built control/invocation"
  if "Original issued native storage reservation required" in txt or "Original actual Nx SDK required" in txt or "Original SDK required" in txt: return "needs real lane env (storage reservation/SDK)"
  if "profile `prototype` is not defined" in txt or "-Z` flag is only accepted" in txt: return "cargo env of the real lane"
  if x["rc"]=="TIMEOUT": return "TIMEOUT (>150 s)"
  if "Dashboard is not installed" in txt: return "dashboard (not mine)"
  m=re.findall(r"^error: (.*)$",txt,re.M)
  return (m[0][:110] if m else "see log")
out=collections.Counter(); lines=[]
for f in sorted(rows):
  v,x=verdict(f); out[v]+=1
  if v=="RED":
    short=f.replace("🧰️framework/🔨️modules/🏃️process/","P/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","R/")
    lines.append((cause(x),short,x["rc"],"%s/%s"%(x["p"],x["f"]),"%s#%d"%(x["src"],x["n"])))
print(dict(out))
bycause=collections.Counter(l[0] for l in lines)
for c,k in bycause.most_common(): print(k,c)
open(S+"/red-table.tsv","w").write("\n".join("\t".join(map(str,l)) for l in sorted(lines))+"\n")
