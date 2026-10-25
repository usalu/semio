import re,sys
log=sys.argv[1]; short=len(sys.argv)>2
lines=open(log).read().split("\n")
buf=[]
for l in lines:
    if re.search(r': error(\[|:)',l): buf.append(l)
    m=re.match(r'error: could not compile `([^`]+)` \(([^)]*)\)',l)
    if m:
        print("=====",m.group(1),m.group(2),len(buf))
        seen=set()
        for b in buf:
            b=re.sub(r'^.*?(🏅️standards|🔨️modules|🧰️framework|✏️s/|🧪️tests)', r'\1', b)
            b=re.sub(r'(/\./)+','/',b)
            b=re.sub(r'(📦️packages/🦀️rust/)?(\.\./)+','',b,count=1)
            k=b[:200] if short else b
            if k in seen: continue
            seen.add(k)
            print("  ",b[:240 if short else 400])
        buf=[]
