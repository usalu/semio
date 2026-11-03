import sys
from PIL import Image
out=sys.argv[1]; cols=int(sys.argv[2]); fs=sys.argv[3:]
ims=[Image.open(f).convert("RGB") for f in fs]
rows=[ims[i:i+cols] for i in range(0,len(ims),cols)]
W=max(sum(i.width for i in r) for r in rows); H=sum(max(i.height for i in r) for r in rows)
c=Image.new("RGB",(W,H),"white");y=0
for r in rows:
    x=0
    for i in r: c.paste(i,(x,y)); x+=i.width
    y+=max(i.height for i in r)
c.save(out); print(c.size)
