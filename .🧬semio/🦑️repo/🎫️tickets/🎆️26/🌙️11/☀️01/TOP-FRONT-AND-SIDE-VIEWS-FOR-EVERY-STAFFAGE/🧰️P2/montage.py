import sys,glob,os,re,fitz
src,out=sys.argv[1],sys.argv[2]; os.makedirs(out,exist_ok=True)
groups={}
for f in sorted(glob.glob(os.path.join(src,"*--views-dark-line-1-*.svg"))):
    m=re.match(r"(.+?)--(.+)--views-dark-line-1-(\d+)\.svg",os.path.basename(f))
    groups.setdefault((m.group(1),m.group(3)),[]).append(f)
for (asset,scale),files in groups.items():
    docs=[fitz.open(f) for f in files]
    w=max(d[0].rect.width for d in docs); h=max(d[0].rect.height for d in docs)
    cols=2; rows=(len(files)+1)//2
    pdf=fitz.open(); page=pdf.new_page(width=w*cols,height=h*rows)
    for i,d in enumerate(docs):
        pdfbytes=d.convert_to_pdf(); sub=fitz.open("pdf",pdfbytes)
        r=d[0].rect; x=(i%cols)*w; y=(i//cols)*h
        page.show_pdf_page(fitz.Rect(x,y,x+r.width,y+r.height),sub,0)
    page.get_pixmap(dpi=110).save(os.path.join(out,f"{asset}-1-{scale}.png")); print(asset,scale,len(files))
