import sys, glob, os, json
from PIL import Image
d=sys.argv[1]; defaults=json.loads(sys.argv[2])
for a,p in defaults.items():
    ims=[]
    for sc in ("50","100","200"):
        f=os.path.join(d,a,f"{a}--{p}--1-{sc}.png")
        if os.path.exists(f): ims.append(Image.open(f).convert("RGB"))
    f=os.path.join(d,a,f"{a}--thermal-{p}--1-50.png")
    ims.append(Image.open(f).convert("RGB"))
    w=max(i.width for i in ims); h=sum(i.height for i in ims)
    M=Image.new("RGB",(w,h),"white"); y=0
    for i in ims: M.paste(i,(0,y)); y+=i.height
    M.save(os.path.join(d,a,"_default.png"))
