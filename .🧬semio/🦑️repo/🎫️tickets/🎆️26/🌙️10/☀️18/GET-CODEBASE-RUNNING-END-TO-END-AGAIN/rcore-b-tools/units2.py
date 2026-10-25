import re,sys
cur=[]
want=sys.argv[2:] 
for l in open(sys.argv[1],errors="replace"):
    l=l.rstrip()
    m=re.match(r"(.*?\.rs):(\d+):(\d+): error(\[E\d+\])?: (.*)",l)
    if m: cur.append((re.sub(r".*📦️packages/🦀️rust/","",m.group(1))[-75:],m.group(2),m.group(5)[:190])); continue
    m=re.match(r"error: could not compile `([^`]+)` \(([^)]*)\)",l)
    if m:
        if not want or m.group(1) in want:
            print("==",m.group(1),m.group(2)); seen=set()
            for e in cur:
                if e in seen: continue
                seen.add(e); print("  ",*e)
        cur=[]
