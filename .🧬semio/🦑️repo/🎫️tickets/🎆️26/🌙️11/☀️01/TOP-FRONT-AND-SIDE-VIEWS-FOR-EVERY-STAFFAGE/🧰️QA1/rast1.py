import sys,fitz,glob
dpi=int(sys.argv[1])
for p in sys.argv[2:]:
    for svg in glob.glob(p):
        fitz.open(svg)[0].get_pixmap(dpi=dpi).save(svg[:-4]+'.png')
        print(svg[:-4]+'.png')
