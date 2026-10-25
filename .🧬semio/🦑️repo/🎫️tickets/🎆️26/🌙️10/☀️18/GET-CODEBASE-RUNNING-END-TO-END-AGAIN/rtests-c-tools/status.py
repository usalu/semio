import sys,re,collections
S=sys.argv[1]; verbose=len(sys.argv)>2
rows=[l.rstrip("\n").split("\t")[:6] for l in open(S+"/summary.tsv")]
cnt=collections.Counter("GREEN" if r[1]=="0" else "TIMEOUT" if r[1]=="TIMEOUT" else "RED" for r in rows)
print(dict(cnt))
for n,rc,p,f,sec,file in rows:
  if rc=="0": continue
  short=file.replace("🧰️framework/🔨️modules/🏃️process/","P/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","R/")
  print("##",n,rc,"%s/%s"%(p,f),short)
  if verbose:
    txt=open("%s/logs/%04d.log"%(S,int(n)),errors="replace").read()
    errs=re.findall(r"^(?:error|ResolveMessage|TypeError|Error|SyntaxError)[:\s].*$",txt,re.M)
    for e in list(dict.fromkeys(errs).keys())[:4]: print("   E:",e[:240])
