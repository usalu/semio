import re,sys,collections
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run,summ=sys.argv[1],sys.argv[2]
rows=sorted(l.rstrip("\n").split("\t") for l in open(C+"/env-pass2-tools/"+summ,encoding="utf-8"))
casc=("Original process admission closed","Original process birth already received","Previous build test law remains unsettled","Build test receiver closed with active callback")
for r in rows:
    t=open(C+"/🗑️generated/"+run+"/"+r[0]+".log",encoding="utf-8",errors="replace").read()
    firsts=[]
    for m in re.finditer(r"^((?:error|TypeError|ReferenceError|SyntaxError|AssertionError|RangeError)[^\n]*)",t,re.M):
        s=m.group(1)
        if s.startswith("error: [script]") or any(c in s for c in casc): continue
        firsts.append(s[:150])
    c=collections.Counter(firsts)
    print(r[0],r[1],r[2],r[3],"|",(" || ".join(f"{k} x{v}" if v>1 else k for k,v in list(c.items())[:3])))
