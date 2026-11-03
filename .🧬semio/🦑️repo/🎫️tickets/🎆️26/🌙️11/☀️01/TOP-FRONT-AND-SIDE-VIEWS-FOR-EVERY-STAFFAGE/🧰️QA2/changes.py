import glob,re,os,sys,collections,zlib
d=sys.argv[1]; sc=sys.argv[2] if len(sys.argv)>2 else "50"
def norm(p):
    p=re.sub(r"<text[^>]*>.*?</text>","",p,flags=re.S)
    p=re.sub(r'<rect id="staffage-background"[^>]*/>',"",p)
    p=re.sub(r'<svg [^>]*?viewBox="[^"]*"[^>]*>',"",p)
    p=re.sub(r'width="[\d.]+mm"|height="[\d.]+mm"',"",p)
    return zlib.crc32(p.encode())
for a in sorted(os.listdir(d)):
    ad=os.path.join(d,a)
    if not os.path.isdir(ad): continue
    seq=collections.defaultdict(list)
    for f in glob.glob(os.path.join(ad,f"*--1-{sc}.svg")):
        n=os.path.basename(f).split("--")[1]
        if n.startswith("thermal"): continue
        s=open(f,encoding="utf8").read()
        parts=re.split(r'<g transform="translate\([^)]*\)">',s)[1:]
        c,_,t=n.partition("@")
        seq[c].append((float(t) if t else 0,[norm(p) for p in parts]))
    print("==",a)
    for c,l in seq.items():
        if len(l)<2: continue
        l.sort(key=lambda x:x[0]); out=[]
        for i in range(3):
            ids={}; out.append("".join(str(ids.setdefault(x[1][i],len(ids))%10) for x in l))
        print(" ",c,[x[0] for x in l][:0],"E",out[0],"F",out[1],"P",out[2])
