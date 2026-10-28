"""📏 Measure named ledger delta widths with independent font libraries."""
from pathlib import Path
import json
from PIL import ImageFont
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont as ReportFont
import pdfplumber
root=Path.cwd()
ticket=Path(__file__).parent
fontpath=root/"🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/dist/fonts/Anta-Regular.ttf"
font=ImageFont.truetype(str(fontpath),size=900)
pdfmetrics.registerFont(ReportFont("AssignmentAnta",str(fontpath)))
with pdfplumber.open(root/"output/pdf/Fingeruebung-Parametrisierung.pdf") as pdf:
    arrows=[c for c in pdf.pages[0].chars if c["text"]=="→" and c["top"]>500]
    arrow=arrows[0]["width"] if arrows else 8.966
def widths(text,size):
    plain=text.replace("→","")
    independent=font.getlength(plain)*size/900
    third=pdfmetrics.stringWidth(plain,"AssignmentAnta",size)
    assert abs(independent-third)<.05,(independent,third)
    return (independent+text.count("→")*arrow*size/9)*25.4/72
rows=[]
for language,phrases in {
"de":["Quader → Pyramide","Pyramide → Quader","Pyramide → ellipt. Zylinder","ellipt. Zylinder → Pyramide","Quader → ellipt. Zylinder","ellipt. Zylinder → Quader"],
"en":["Box → Pyramid","Pyramid → Box","Pyramid → ellipt. cylinder","ellipt. cylinder → Pyramid","Box → ellipt. cylinder","ellipt. cylinder → Box"]}.items():
    for index,phrase in enumerate(phrases):
        prefix="" if index<2 else "L+1; B+1; H+1; " if index in [2,4] else "L-1; B-1; H-1; "
        text=prefix+phrase
        rows.append({"language":language,"text":text,"width9mm":round(widths(text,9),4),"width8_5mm":round(widths(text,8.5),4),"ruleWidthMm":64.6})
output=ticket/"🗑️generated/delta-widths"
output.mkdir(parents=True,exist_ok=True)
(output/"widths.json").write_text(json.dumps(rows,indent=2,ensure_ascii=False),encoding="utf8")
for row in rows:print(json.dumps(row,ensure_ascii=True))
print("PASS Pillow FreeType and ReportLab widths agree; native PDF arrow width",arrow)
