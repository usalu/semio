import sys, glob, os
from PIL import Image
root=sys.argv[1]; sc=sys.argv[2]; names=sys.argv[3:]
ims=[]
for n in names:
    for f in glob.glob(os.path.join(root,n,f"*-1-{sc}.png")): ims.append(Image.open(f).convert("RGB"))
w=max(i.width for i in ims); h=sum(i.height for i in ims)
out=Image.new("RGB",(w,h),"white"); y=0
for i in ims: out.paste(i,(0,y)); y+=i.height
out.thumbnail((2000,2600)); out.save(os.path.join(root,"grid-"+"-".join(names[:1])+f"-{sc}.png"))
