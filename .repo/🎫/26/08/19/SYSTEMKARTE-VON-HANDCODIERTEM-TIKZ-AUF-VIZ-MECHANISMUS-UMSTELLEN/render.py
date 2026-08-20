import fitz
doc = fitz.open(r"E:\semio\mit-bestand\bericht\forschungsbericht\dist\zz-verify-skalierung.pdf")
for i in range(len(doc)):
    pix = doc[i].get_pixmap(dpi=200)
    pix.save(rf"E:\semio\.repo\🎫\26\08\19\SYSTEMKARTE-VON-HANDCODIERTEM-TIKZ-AUF-VIZ-MECHANISMUS-UMSTELLEN\skalierung-page{i+1}.png")
print("pages:", len(doc))
