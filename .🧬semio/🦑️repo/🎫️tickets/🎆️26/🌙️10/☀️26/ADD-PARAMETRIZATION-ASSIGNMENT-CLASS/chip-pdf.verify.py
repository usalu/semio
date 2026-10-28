"""🔎 Verify unnamed table chips through native PDF text and path geometry."""
import sys
from pathlib import Path
import pdfplumber
from pypdf import PdfReader
from importlib.util import spec_from_file_location,module_from_spec

def load(name,file):
    spec=spec_from_file_location(name,Path(__file__).parent/file)
    module=module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

worked=load("worked","verify-mixed-pdf.py")
task=load("task","verify-matrix-pdf.py")
MM=72/25.4

def verify(path):
    reader=PdfReader(path)
    assert len(reader.pages)==3
    language=reader.trailer["/Root"].get("/Lang")
    label="Tabelle" if language=="de-DE" else "Table"
    with pdfplumber.open(path) as pdf:
        x,y=worked.matrix(pdf.pages[0])
        matrices=[(pdf.pages[0],x,y)]+[(pdf.pages[1],x,y) for x,y in sorted(task.grids(pdf.pages[1]),key=lambda grid:grid[1][0])]
        assert len(matrices)==3
        for (page,xs,ys),number in zip(matrices,["1.1.a","1.2.a","1.2.b"]):
            top=ys[0]
            zone=page.crop((xs[0]-.5,top-8*MM,xs[-1]+.5,top+.05))
            words=zone.extract_words()
            text=" ".join(word["text"] for word in words)
            assert label in text and number in text,(number,text)
            left=[word for word in words if word["x0"]<(xs[0]+xs[-1])/2]
            assert not left,("left name chip",number,left)
            edges=[edge for edge in page.edges if edge["top"]>=top-8*MM and edge["bottom"]<=top+.2]
            ghost=[edge for edge in edges if edge["orientation"]=="v" and edge["height"]>1 and edge["x0"]<(xs[0]+xs[-1])/2]
            assert not ghost,("left ghost tab",number,ghost)
            right=[edge for edge in edges if edge["orientation"]=="v" and edge["height"]>1 and edge["x0"]>(xs[0]+xs[-1])/2]
            assert len(right)>=2,("right frame connection",number,right)
            assert any(abs(edge["bottom"]-top)<.6 for edge in right),("detached right chip",number)
            print("PASS",Path(path).name,number,"empty left name zone; automatic right chip; connected frame")
        heading="Beispiel" if label=="Tabelle" else "Example"
        assert heading in (pdf.pages[0].extract_text() or "")
        assert all("Table A" not in (p.extract_text() or "") and "Table B" not in (p.extract_text() or "") and "Tabelle A" not in (p.extract_text() or "") and "Tabelle B" not in (p.extract_text() or "") for p in pdf.pages)
    worked.verify(path)
    task.verify(path)

for source in sys.argv[1:]:
    verify(source)
