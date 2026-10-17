import json,os,re,sys
# 🔎️ peel.py <census-name>... : lists the direct child modules of a crate root with size and coupling (super::/crate:: references), the first step of choosing a cohesive module to carve out.
R="/Users/ueli/Documents/semio"
F=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
rows={r['name']:r for r in json.load(open(F+"/🗑️generated/hdr-near.json"))}
PATH_RX=re.compile(r'#\[path\s*=\s*"([^"]*)"\]\s*\n\s*((?:#\[[^\n]*\]\s*\n\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
for n in sys.argv[1:]:
    r=rows[n]; man=R+"/"+r['manifest']; mt=open(man).read()
    lp=re.search(r'\[lib\][^\[]*?path = "([^"]+)"',mt).group(1)
    root=os.path.normpath(os.path.join(os.path.dirname(man),lp)); rd=os.path.dirname(root)
    txt=open(root).read()
    print(f"== {n} root {os.path.relpath(root,R)[-50:]} {len(txt.splitlines())} lines {len(txt)} bytes")
    for m in PATH_RX.finditer(txt):
        p=os.path.normpath(os.path.join(rd,m.group(1)))
        if not os.path.exists(p): continue
        t=open(p).read()
        sup=len(re.findall(r'\bsuper::',t)); cr=len(re.findall(r'\bcrate::',t))
        glob=len(re.findall(r'use super::\*',t))
        cfg='cfg(test)' in m.group(0)
        print(f"  {m.group(3):40s} {len(t.splitlines()):5d} lines {len(t):7d} B  super={sup:3d} crate={cr:3d} glob={glob} {'TEST' if cfg else ''}")
