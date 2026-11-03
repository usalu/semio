import sys, glob, os
from PIL import Image
root = sys.argv[1]
for d in sorted(glob.glob(os.path.join(root, "*"))):
    if not os.path.isdir(d): continue
    ims=[]
    for sc in ("50","100","200"):
        for f in glob.glob(os.path.join(d, f"*-1-{sc}.png")):
            ims.append(Image.open(f).convert("RGB"))
    if not ims: continue
    w=max(i.width for i in ims); h=sum(i.height for i in ims)
    out=Image.new("RGB",(w,h),"white"); y=0
    for i in ims: out.paste(i,(0,y)); y+=i.height
    out.thumbnail((2000,2400)); out.save(os.path.join(root, os.path.basename(d)+"-stack.png"))
