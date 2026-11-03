import sys,fitz
from PIL import Image
svg,out,dpi=sys.argv[1],sys.argv[2],int(sys.argv[3])
fx0,fy0,fx1,fy1=[float(x) for x in sys.argv[4:8]]
pm=fitz.open(svg)[0].get_pixmap(dpi=dpi)
im=Image.frombytes("RGB",(pm.width,pm.height),pm.samples)
W,H=im.size
parts=[]
for c in range(3):
    x0=(c/3)+fx0/3; x1=(c/3)+fx1/3
    parts.append(im.crop((int(x0*W),int(fy0*H),int(x1*W),int(fy1*H))))
w=sum(p.width for p in parts); h=max(p.height for p in parts)
m=Image.new("RGB",(w,h),"white"); x=0
for p in parts: m.paste(p,(x,0)); x+=p.width
m.save(out)
