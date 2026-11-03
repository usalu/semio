import sys, glob, os, re
from PIL import Image
d=sys.argv[1]; chunk=int(sys.argv[2]) if len(sys.argv)>2 else 7
for a in sorted(os.listdir(d)):
    ad=os.path.join(d,a)
    if not os.path.isdir(ad): continue
    for sc in ("50","100","200"):
        fs=sorted(glob.glob(os.path.join(ad,f"*--1-{sc}.png")))
        fs=[f for f in fs if not os.path.basename(f).startswith("_")]
        for n in range(0,len(fs),chunk):
            ims=[Image.open(f).convert("RGB") for f in fs[n:n+chunk]]
            w=max(i.width for i in ims); h=sum(i.height for i in ims)
            M=Image.new("RGB",(w,h),"white"); y=0
            for i in ims: M.paste(i,(0,y)); y+=i.height
            M.save(os.path.join(ad,f"_m{sc}-{n//chunk}.png"))
