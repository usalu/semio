import sys,re,collections
S=sys.argv[1]
rows=[l.rstrip("\n").split("\t") for l in open(S+"/summary.tsv")]
for n,rc,p,f,sec,file in rows:
  if rc=="0": continue
  txt=open("%s/logs/%04d.log"%(S,int(n)),errors="replace").read()
  errs=re.findall(r"^error: (.*)$",txt,re.M)
  c=collections.Counter(e[:140] for e in errs)
  short=file.replace("🧰️framework/🔨️modules/🏃️process/","P/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","R/")
  print(n,rc,"%s/%s"%(p,f),short[:90]); 
  for e,k in c.most_common(3): print("    %dx %s"%(k,e))
