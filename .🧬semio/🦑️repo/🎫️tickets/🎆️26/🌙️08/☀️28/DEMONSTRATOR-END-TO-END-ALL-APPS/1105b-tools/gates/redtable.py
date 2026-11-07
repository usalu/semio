import re,sys
s=open(sys.argv[1],errors="replace").read()
crates=dict((m.group(1),int(m.group(3))) for m in re.finditer(r"error: could not compile `([^`]*)` \(([a-z ]*)\) due to (\d+) previous error",s))
first={}
for m in re.finditer(r"(error(?:\[E\d+\])?: [^\n]*)\n\s*--> ([^\n]*?:\d+:\d+)",s):
    if "could not" in m.group(1):continue
    first.setdefault(m.group(2).split("/packages/")[-1][-110:],m.group(1)[:80])
for c,n in sorted(crates.items()):print(f"| {c} | {n} |")
print("---first errors (dedup)");
for k,v in list(first.items())[:40]:print(v,"@",k)
