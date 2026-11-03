import sys, glob, os, re, pymupdf as fitz
from PIL import Image, ImageDraw
d=sys.argv[1]; TARGET=int(sys.argv[2]) if len(sys.argv)>2 else 420
DPI={"50":500,"100":900,"200":1600}
MM=72/25.4
for s in glob.glob(os.path.join(d,"*","*.svg")):
    sc=s[:-4].rsplit("-",1)[1]
    p=s[:-4]+".png"
    if os.path.exists(p) and os.path.getmtime(p)>=os.path.getmtime(s): continue
    txt=open(s,encoding="utf8").read()
    pan=[(float(a),float(b)) for a,b in re.findall(r'<g transform="translate\(([\d.]+) ([\d.]+)\)">',txt)]
    sizes=[(float(a),float(b)) for a,b in re.findall(r'<rect id="staffage-background" x="0" y="0" width="([\d.]+)" height="([\d.]+)"',txt)]
    pg=fitz.open(s)[0]
    crops=[]
    for (tx,ty),(w,h) in zip(pan,sizes):
        clip=fitz.Rect(tx*MM,ty*MM,(tx+w)*MM,(ty+h*0.95)*MM)
        pm=pg.get_pixmap(dpi=DPI[sc]*0+int(DPI[sc]),clip=clip)
        im=Image.frombytes("RGB",(pm.width,pm.height),pm.samples)
        bg=im.getpixel((2,2))
        from PIL import ImageChops
        diff=ImageChops.difference(im,Image.new("RGB",im.size,bg)).convert("L").point(lambda v:255 if v>40 else 0)
        bb=diff.getbbox() or (0,0,im.width,im.height)
        m=6
        bb=(max(0,bb[0]-m),max(0,bb[1]-m),min(im.width,bb[2]+m),min(im.height,bb[3]+m))
        c=im.crop(bb); k=TARGET/max(c.size)
        c=c.resize((max(1,int(c.width*k)),max(1,int(c.height*k))),Image.LANCZOS)
        crops.append(c)
    W=sum(c.width for c in crops)+20*(len(crops)+1); H=max(c.height for c in crops)+24
    M=Image.new("RGB",(W,H),"white"); dr=ImageDraw.Draw(M); x=20
    name=os.path.basename(s).split("--")[1]
    for lab,c in zip(("E","F","P"),crops):
        M.paste(c,(x,20)); dr.text((x,4),f"{name} {lab}",fill="black"); x+=c.width+20
    M.save(p)
