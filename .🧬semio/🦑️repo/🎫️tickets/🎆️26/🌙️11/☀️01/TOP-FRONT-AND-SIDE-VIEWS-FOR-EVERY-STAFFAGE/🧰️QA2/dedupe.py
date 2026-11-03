import glob,re,os,sys,zlib
d=sys.argv[1]
def norm(p):
    p=re.sub(r"<text[^>]*>.*?</text>","",p,flags=re.S)
    p=re.sub(r'<rect id="staffage-background"[^>]*/>',"",p)
    p=re.sub(r'<svg [^>]*?viewBox="[^"]*"[^>]*>',"",p)
    p=re.sub(r'width="[\d.]+mm"|height="[\d.]+mm"',"",p)
    return zlib.crc32(p.encode())
kept=0
for a in sorted(os.listdir(d)):
    ad=os.path.join(d,a)
    if not os.path.isdir(ad): continue
    for sc in ("50","100","200"):
        files=glob.glob(os.path.join(ad,f"*--1-{sc}.svg"))
        def key(f):
            n=os.path.basename(f).split("--")[1]
            return (0 if "@" not in n and not n.startswith("thermal") else (2 if n.startswith("thermal") else 1), n)
        seen=set()
        for f in sorted(files,key=key):
            n=os.path.basename(f).split("--")[1]
            if n.startswith("thermal"): continue
            parts=re.split(r'<g transform="translate\([^)]*\)">',open(f,encoding="utf8").read())[1:]
            h=[norm(p) for p in parts]
            k=tuple(h) if sc=="50" else tuple(h[1:])
            if k in seen:
                os.remove(f)
            else:
                seen.add(k); kept+=1
print("kept",kept)
