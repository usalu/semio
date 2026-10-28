from pathlib import Path
import sys
import re
import json
import pdfplumber
import pypdfium2 as pdfium
from pypdf import PdfReader
from PIL import Image

source=Path(sys.argv[1])
out=Path(__file__).parent/"🗑️generated"/"review"
out.mkdir(parents=True,exist_ok=True)
reader=PdfReader(source)
expected=json.loads((Path(__file__).parent/"case.json").read_text())
assert len(reader.pages)==expected["pages"]
assert reader.trailer["/Root"]["/Lang"]=="de-DE"
text="\n".join(page.extract_text() or "" for page in reader.pages)
for word in ["Schlecht","Zyklus","Parametrisieren","Notieren","Prüfen","positive reelle","Achsen und"]:
    assert word not in text,word
for word in ["Fingerübung Parametrisierung","Beispiel","Wohnkapseln","Parameter","Form"]:
    assert word in text,word
with pdfplumber.open(source) as doc:
    for page in doc.pages:
        assert all(-.5<=c["x0"]<=c["x1"]<=page.width+.5 and -.5<=c["top"]<=c["bottom"]<=page.height+.5 for c in page.chars)
    for index in [0,2]:
        edges=[c for c in doc.pages[index].curves if c.get("stroke") and any(p[0]=="c" for p in c.get("path",[]))]
        assert len(edges)>=6 if index==0 else len(edges)>=260
    assert len(doc.pages[1].lines)>100
pdf=pdfium.PdfDocument(str(source))
thumbs=[]
for index,page in enumerate(pdf):
    im=page.render(scale=1.6).to_pil()
    im.save(out/f"page-{index+1}.png")
    small=im.copy();small.thumbnail((420,620));thumbs.append(small)
canvas=Image.new("RGB",(1260,620),"#dddddd")
for index,im in enumerate(thumbs):canvas.paste(im,(index*420,0))
canvas.save(out/"contact.png")
pdf.close()
print(f"PASS: {len(reader.pages)} pages; no prose; labels and native graphs retained; all text inside page")
