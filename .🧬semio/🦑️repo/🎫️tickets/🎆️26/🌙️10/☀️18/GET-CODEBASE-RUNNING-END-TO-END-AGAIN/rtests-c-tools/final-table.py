import re,sys,os,collections
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G=C+"/🗑️generated/rtests-c"
latest={}
for d in sys.argv[1:]:
  p=G+"/"+d+"/summary.tsv"
  if not os.path.exists(p): continue
  for l in open(p):
    a=l.rstrip("\n").split("\t")[:6]
    n,rc,ps,fl,sec,file=a
    latest[file]=(d,int(n),rc,ps,fl)
CAT=[("lane: Original SDK / Nx namespace / storage reservation / graph-writer custody / process issuance",r"Original (immutable )?(SDK|Nx|issued|bounded|actual|artifact|prepared|default|short|historical|fixed|report|snapshot|installed|control|resource|process|incoming|compiler|exact|Script)|Live Original|Nested persistent image|Actual (incoming|ownership)|Exact (Original|same-wire)|Vite config requires its original|Vite intent must match|Closed (production|Cargo|Original)|Nx SDK|nx/package.json|bounded Nx"),
("dashboard binary not installed",r"Dashboard is not installed"),
("host/capacity (timeout or memory)",r"capacity\]|cancelled by SIGTERM|made no progress|stopped: cancelled"),
("env: sandbox path / short root / socket / ticket dir",r"SEMIO_TICKET_DIR|Short test sandbox|SUN_LEN|exceeds 256|escapes repository|SEMIO_TEST_ARTIFACT_DIR must"),
("ratchet / debt pin (owner must pay down)",r"language law finding|grew from|exceed the .*-line wiring|knownEntanglements|cache-correctness finding"),
]
rows=[]
cnt=collections.Counter()
for file,(d,n,rc,ps,fl) in sorted(latest.items()):
  if rc=="0": cnt["green"]+=1; continue
  txt=open("%s/%s/logs/%04d.log"%(G,d,n),errors="replace").read()
  cat="behavioral / fixture drift (needs owner)"
  if rc=="TIMEOUT": cat="host/capacity (timeout or memory)"
  else:
    for name,rx in CAT:
      if re.search(rx,txt): cat=name; break
  cnt["red"]+=1; cnt[cat]+=1
  short=file.replace("🧰️framework/🔨️modules/🏃️process/","P/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/","R/")
  first=next(iter(re.findall(r"^error: (.*)$",txt,re.M)),"")[:110]
  rows.append((cat,short,"%s/%s"%(ps,fl),first))
print(dict(cnt))
out=["| category | file | pass/fail | first error |","|---|---|---|---|"]+["| %s | `%s` | %s | %s |"%(c,f,pf,e.replace("|","/")) for c,f,pf,e in sorted(rows)]
open(G+"/final-table.md","w").write("\n".join(out)+"\n")
