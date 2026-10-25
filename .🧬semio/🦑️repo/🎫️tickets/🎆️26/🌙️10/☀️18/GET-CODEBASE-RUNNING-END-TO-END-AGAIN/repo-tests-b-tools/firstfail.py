import sys,re
C=sys.argv[1]; S=C+"/🗑️generated/repo-tests-b"; pref=tuple(sys.argv[2:])
for l in open(S+"/red-table.tsv"):
  x=l.rstrip("\n").split("\t")
  if pref and not x[0].startswith(pref): continue
  d,n=x[4].split("#"); lines=open("%s/%s/logs/%04d.log"%(S,d,int(n)),errors="replace").read().split("\n")
  for i,t in enumerate(lines):
    if t.startswith("(fail)"):
      err=""
      for j in range(i-1,max(i-40,0),-1):
        if lines[j].startswith(("error:","ENOENT","Error","TypeError")) or re.match(r"^\w*Error",lines[j]) or lines[j].startswith("error"): err=lines[j]; break
      print(x[1].replace("R/📚️library/","")[-80:],"|",t[:90],"|",err[:150]); break
