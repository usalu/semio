import sys,glob,os,fitz
out=sys.argv[1]; os.makedirs(out,exist_ok=True)
for pat in sys.argv[2:]:
    for f in glob.glob(pat):
        doc=fitz.open(f); pix=doc[0].get_pixmap(dpi=130)
        pix.save(os.path.join(out,os.path.basename(f)[:-4]+".png"))
        print(os.path.basename(f))
