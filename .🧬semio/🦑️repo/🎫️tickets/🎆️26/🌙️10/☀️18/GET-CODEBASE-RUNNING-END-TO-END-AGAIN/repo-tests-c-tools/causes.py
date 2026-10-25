import sys,re
S=sys.argv[1]; lo=int(sys.argv[2]) if len(sys.argv)>2 else 0; hi=int(sys.argv[3]) if len(sys.argv)>3 else 9999
for l in open(S+"/summary.tsv"):
  n,rc,p,f,sec,file=l.rstrip("\n").split("\t")
  if rc=="0" or not lo<=int(n)<=hi: continue
  txt=open("%s/logs/%04d.log"%(S,int(n)),errors="replace").read()
  short=file.replace("🧰️framework/🔨️modules/🏃️process/","P/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","R/")
  print("##",n,rc,"%s/%s"%(p,f),short)
  errs=re.findall(r"^(?:error|ResolveMessage|TypeError|Error|SyntaxError)[:\s].*$",txt,re.M)
  for e in dict.fromkeys(errs).keys(): print("   E:",e[:260]); 
  for m in re.findall(r"^\(fail\) .*$",txt,re.M)[:6]: print("   F:",m[:200])
