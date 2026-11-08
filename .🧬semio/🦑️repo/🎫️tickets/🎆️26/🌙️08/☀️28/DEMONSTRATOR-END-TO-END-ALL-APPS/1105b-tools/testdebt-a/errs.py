import re,sys
t=open(sys.argv[1]).read().split("\n\n")
want=sys.argv[2] if len(sys.argv)>2 else ""
seen=set()
for b in t:
    if not b.startswith("error"): continue
    if b.startswith("error: could not"): continue
    m=re.search(r"-->\s+(.*?):(\d+):(\d+)",b)
    if not m: print(b[:200]); continue
    loc=m.group(1).split("rust/")[-1].lstrip("./")
    while loc.startswith("../"): loc=loc[3:]
    if want not in loc: continue
    k=(loc,m.group(2),b.split("\n")[0])
    if k in seen: continue
    seen.add(k)
    print(f"{loc}:{m.group(2)}:{m.group(3)} {b.split(chr(10))[0][:150]}")
