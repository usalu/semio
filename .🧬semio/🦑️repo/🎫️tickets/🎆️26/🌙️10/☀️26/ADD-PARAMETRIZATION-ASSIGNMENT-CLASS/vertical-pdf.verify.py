"""📐 Measure actual example text ink against native cell centers."""
import sys,json
from pathlib import Path
from importlib.util import spec_from_file_location,module_from_spec
import pdfplumber
import pypdfium2 as pdfium
from collections import Counter
spec=spec_from_file_location("mixed",Path(__file__).parent/"verify-mixed-pdf.py")
mixed=module_from_spec(spec)
spec.loader.exec_module(mixed)
MM=72/25.4

def measure(path):
    with pdfplumber.open(path) as pdf:
        page=pdf.pages[0]
        xs,ys=mixed.matrix(page)
        document=pdfium.PdfDocument(str(path))
        raster=document[0].render(scale=4).to_pil().convert("L")
        result=[]
        for row in range(1,5):
            for col in range(4):
                box=(xs[col]+1,ys[row]+1,xs[col+1]-1,ys[row+1]-1)
                chars=page.crop(box).chars
                assert chars,(row,col)
                text="".join(c["text"] for c in chars)
                nominal=(ys[row]+ys[row+1])/2
                pixel_box=tuple(round(value*4) for value in box)
                image=raster.crop(pixel_box)
                pixels=image.load()
                background=Counter(image.tobytes()).most_common(1)[0][0]
                occupied=[y for y in range(image.height) if any(abs(pixels[x,y]-background)>50 for x in range(image.width))]
                assert occupied,(row,col,text,image.size,image.getextrema(),[(c["non_stroking_color"],c["top"],c["bottom"]) for c in chars])
                ink_top=pixel_box[1]/4+min(occupied)/4
                ink_bottom=pixel_box[1]/4+(max(occupied)+1)/4
                offset=((ink_top+ink_bottom)/2-nominal)/MM
                result.append({"row":row,"column":col,"text":text,"offsetMm":round(offset,4),"inkHeightMm":round((ink_bottom-ink_top)/MM,4),"nominalGlyphOffsetMm":round(((min(c["top"] for c in chars)+max(c["bottom"] for c in chars))/2-nominal)/MM,4)})
        return result

root=next(parent for parent in Path(__file__).parents if (parent/"🧰️framework/🛍️products/📓️print").exists())
fixture=json.loads((root/"🧰️framework/🛍️products/📓️print/🧪️tests/📝️parametrization-assignment/🧫️cases.json").read_text(encoding="utf-8"))
contract=fixture["workedExample"]["matrix"]
for path in [arg for arg in sys.argv[1:] if arg!="--measure"]:
    rows=measure(path)
    print(json.dumps({"path":str(path),"cells":rows},ensure_ascii=False))
    assert len(rows)==contract["centeredFilledCells"]
    if "--measure" not in sys.argv:
        failures=[row for row in rows if abs(row["offsetMm"])>contract["maxInkCenterOffsetMm"]]
        assert not failures,("Visible ink not vertically centered",failures)
        print("PASS",Path(path).name,"all sixteen filled text ink centers within",contract["maxInkCenterOffsetMm"],"mm of unchanged native row midpoints")
