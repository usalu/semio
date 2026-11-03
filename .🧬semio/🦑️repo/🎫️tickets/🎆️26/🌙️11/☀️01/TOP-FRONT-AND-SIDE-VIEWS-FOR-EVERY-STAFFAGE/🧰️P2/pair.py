import sys,fitz
# pair.py out.png dpi x0 y0 x1 y1 a.svg b.svg ... (fractions of the sheet; the crop of every svg is placed side by side)
out,dpi=sys.argv[1],int(sys.argv[2]); x0,y0,x1,y1=map(float,sys.argv[3:7]); files=sys.argv[7:]
pix=[]
for f in files:
    p=fitz.open(f)[0]; r=p.rect
    clip=fitz.Rect(r.x0+r.width*x0,r.y0+r.height*y0,r.x0+r.width*x1,r.y0+r.height*y1)
    pix.append(p.get_pixmap(dpi=dpi,clip=clip))
W=sum(p.width for p in pix)+10*(len(pix)-1); H=max(p.height for p in pix)
page=fitz.open(); pg=page.new_page(width=W,height=H)
x=0
for p in pix:
    pg.insert_image(fitz.Rect(x,0,x+p.width,p.height),pixmap=p); x+=p.width+10
page[0].get_pixmap(dpi=72).save(out)
