import pymupdf,sys,re
d=pymupdf.open(sys.argv[1])
n=d.xref_length()
t3=0;glyphs=0;gbytes=0;content=0;cbytes=0;forms=0
fonts=set()
for x in range(1,n):
    try: o=d.xref_object(x,compressed=False)
    except Exception: continue
    if "/Subtype/Type3" in o.replace(" ","") and "/CharProcs" in o:
        t3+=1
for p in d:
    for c in p.get_contents():
        content+=1; cbytes+=len(d.xref_stream(c))
    for f in p.get_fonts(full=True):
        fonts.add(f[0])
print("objects",n,"pages",len(d),"type3 fonts",t3,"content streams",content,"content bytes",cbytes,"page fonts distinct",len(fonts))
# glyph procs
import collections
cp=0;cb=0
for x in range(1,n):
    try: o=d.xref_object(x,compressed=False)
    except Exception: continue
    m=re.search(r"/CharProcs\s*(\d+) 0 R",o) 
    if m:
        co=d.xref_object(int(m.group(1)),compressed=False)
        refs=re.findall(r"/([^\s/]+)\s+(\d+) 0 R",co)
        for _,r in refs:
            cp+=1; cb+=len(d.xref_stream(int(r)))
print("glyph programs",cp,"bytes",cb)
