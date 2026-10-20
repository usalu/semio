import re,sys
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run,name=sys.argv[1],sys.argv[2]
n=int(sys.argv[3]) if len(sys.argv)>3 else 2
w=int(sys.argv[4]) if len(sys.argv)>4 else 420
t=open(C+"/🗑️generated/"+run+"/"+name+".log",encoding="utf-8",errors="replace").read().split("\n")
casc=("admission closed","birth already received","remains unsettled","closed with active")
shown=0;seen=set()
for i,l in enumerate(t):
    if re.match(r"^(error|TypeError|ReferenceError|SyntaxError|AssertionError|RangeError)",l) and not l.startswith("error: [script]") and not any(c in l for c in casc):
        if l[:120] in seen: continue
        seen.add(l[:120])
        ctx=[x for x in t[max(0,i-8):i] if re.match(r"^\s*\d+ \|",x)][-2:]
        print("---",l[:300])
        for c in ctx: print("   ",c[:w])
        for x in t[i+1:i+12]:
            if re.match(r"^\s+at ",x) and "node_modules" not in x: print("   ",x.strip().replace("/Users/ueli/Documents/semio/🧰️framework/","FW/")[:220])
        shown+=1
        if shown>=n: break
