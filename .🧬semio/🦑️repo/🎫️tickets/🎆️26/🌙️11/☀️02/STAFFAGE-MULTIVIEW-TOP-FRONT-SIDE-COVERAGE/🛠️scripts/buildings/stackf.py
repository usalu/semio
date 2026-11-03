import sys, glob, os
from PIL import Image
out=sys.argv[1]; files=sys.argv[2:]
ims=[Image.open(f).convert("RGB") for f in files]
w=max(i.width for i in ims); h=sum(i.height for i in ims)
o=Image.new("RGB",(w,h),"white"); y=0
for i in ims: o.paste(i,(0,y)); y+=i.height
o.thumbnail((2000,2600)); o.save(out)
