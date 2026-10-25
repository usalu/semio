import re,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log=sys.argv[1]; frag=sys.argv[2]; lim=int(sys.argv[3]) if len(sys.argv)>3 else 60
seen=set();n=0
for l in open(log,errors="replace"):
    m=re.match(r"(.*?\.rs):(\d+):(\d+): error(\[E\d+\])?: (.*)",l.rstrip())
    if m and frag in m.group(1):
        k=(m.group(1),m.group(2),m.group(3),m.group(5))
        if k in seen: continue
        seen.add(k); n+=1
        if n<=lim: print(re.sub(r".*📦️packages/🦀️rust/","",m.group(1))[-80:],m.group(2)+":"+m.group(3),(m.group(4) or ""),m.group(5)[:230])
print("unique",n)
