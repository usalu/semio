import pymupdf,sys,re
d=pymupdf.open(sys.argv[1])
n=d.xref_length()
def refs(x):
    try: o=d.xref_object(x,compressed=False)
    except Exception: return []
    return [int(m) for m in re.findall(r"(\d+) 0 R",o)]
def closure(x):
    seen={x};st=[x]
    while st:
        c=st.pop()
        for r in refs(c):
            if r not in seen: seen.add(r);st.append(r)
    return seen
t3=[]
for x in range(1,n):
    try: o=d.xref_object(x,compressed=False)
    except Exception: continue
    if re.search(r"/Subtype\s*/Type3",o): t3.append(x)
print("type3",len(t3))
sizes=[len(closure(x)) for x in t3]
print("closure sizes",sizes)
# claimed cumulative closure union: glyph role deps = own closure + union of claimed fonts (all fonts claimed incl. non-T3)
allfonts=set()
for p in d:
    for f in p.get_fonts(full=True): allfonts.add(f[0])
u=set()
for f in sorted(allfonts): u|=closure(f)
print("fonts",len(allfonts),"union closure",len(u))
