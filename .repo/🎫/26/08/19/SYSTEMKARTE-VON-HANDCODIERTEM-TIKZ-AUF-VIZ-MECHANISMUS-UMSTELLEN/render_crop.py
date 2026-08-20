import fitz
doc = fitz.open(r"E:\semio\mit-bestand\bericht\forschungsbericht\dist\zz-verify-skalierung.pdf")
page = doc[1]
clip = fitz.Rect(58, 730, 595, 800)
pix = page.get_pixmap(dpi=300, clip=clip)
pix.save(r"E:\semio\.repo\🎫\26\08\19\SYSTEMKARTE-VON-HANDCODIERTEM-TIKZ-AUF-VIZ-MECHANISMUS-UMSTELLEN\h1h2-crop.png")
