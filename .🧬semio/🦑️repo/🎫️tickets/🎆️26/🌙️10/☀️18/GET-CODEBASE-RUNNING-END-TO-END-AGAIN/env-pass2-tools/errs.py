import re,sys,glob,os
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
rows=[l.rstrip("\n").split("\t") for l in open(C+"/env-pass2-tools/summary2.tsv",encoding="utf-8")]
rows.sort()
only=sys.argv[1:] 
for r in rows:
    n=r[0]
    if only and n not in only: continue
    t=open(C+"/🗑️generated/env-pass2-run/"+n+".log",encoding="utf-8",errors="replace").read()
    errs=[]
    for m in re.finditer(r"^(?:error|TypeError|ReferenceError|SyntaxError|AssertionError)[^\n]*",t,re.M):
        s=m.group(0)[:150]
        if s not in errs and not s.startswith("error: [script]"): errs.append(s)
    print(n,r[2],r[3],r[4].split("/")[-3:-1] if False else os.path.basename(os.path.dirname(r[4]))+"/"+os.path.basename(r[4]))
    for e in errs[:3]: print("    ",e)
