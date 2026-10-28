import pdfplumber
from pypdf import PdfReader
p=r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/🗑️generated/Fingeruebung-Parametrisierung.pdf"
with pdfplumber.open(p) as pdf:
    for i,page in enumerate(pdf.pages):
        content=[c for c in page.chars if c["top"]>70 and c["bottom"]<790]
        print(i+1,"last body bottom mm",round(max(c["bottom"] for c in content)/2.834645669,2))
r=PdfReader(p)
for font in r.pages[0]["/Resources"]["/Font"].values():
    f=font.get_object()
    if "/FontDescriptor" in f:
        descriptor=f["/FontDescriptor"].get_object()
        print("embedded",str(f.get("/BaseFont")),"/FontFile2" in descriptor)
print("pages",len(r.pages),"lang",r.trailer["/Root"].get("/Lang"))
