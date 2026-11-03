import json,os,sys
R="/Users/ueli/Documents/semio";C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rows=json.load(open(C+"/🗑️generated/1102/winplan-scan.json"))
for r in rows:
  if r['sup'] or r['kind']!='diff' or not r['check'].startswith("FAIL"):continue
  cur=None;plus={};minus={}
  for l in open(C+"/"+r['rel'],errors="replace").read().split("\n"):
    if l.startswith("+++ "):
      cur=l[4:].split("\t")[0];cur=cur[2:] if cur.startswith("b/") else cur;plus[cur]=[];minus[cur]=[]
    elif l.startswith("--- "):continue
    elif cur and l.startswith("+"):plus[cur].append(l[1:].strip())
    elif cur and l.startswith("-"):minus[cur].append(l[1:].strip())
  out=[]
  for f in plus:
    try:s=open(R+"/"+f,errors="replace").read()
    except Exception:out.append(f"{f[-30:]}:MISSING");continue
    lines=set(x.strip() for x in s.split("\n"))
    p=[x for x in plus[f] if x];m=[x for x in minus[f] if x]
    out.append(f"{os.path.basename(os.path.dirname(f))}: +{sum(x in lines for x in p)}/{len(p)} -still{sum(x in lines for x in m)}/{len(m)}")
  print(r['gen'],r['lane'],os.path.basename(r['rel']),"|"," ; ".join(out))
