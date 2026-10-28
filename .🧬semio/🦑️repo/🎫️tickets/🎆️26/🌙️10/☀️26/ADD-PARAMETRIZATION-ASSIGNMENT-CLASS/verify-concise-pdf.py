"""🔎 Independently audit concise headings and the native worked example PDF."""
import re
import sys
from collections import Counter
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

def verify(path):
    reader = PdfReader(path)
    assert len(reader.pages) == 3
    with pdfplumber.open(path) as pdf:
        page = pdf.pages[0]
        text = page.extract_text() or ""
        german = "Fingerübung Parametrisierung" in text
        heading = "Fingerübung Parametrisierung" if german else "Finger Exercise: Parametrization"
        first = "Beispiel" if german else "Example"
        second = "Wohnkapseln" if german else "Dwelling Capsules"
        assert re.search(r"(?m)^" + re.escape(heading) + r"\s+1$", text)
        assert re.search(r"(?m)^" + first + r"\s+1\.1$", text)
        page2 = pdf.pages[1].extract_text() or ""
        assert re.search(r"(?m)^" + second + r"\s+1\.2$", page2)
        assert not re.search(r"(?m)^" + re.escape(heading) + r"$", text), "Duplicate title banner"
        assert not re.search(r"(?m)^(?:Name|Datum|Date|Matrikel|Student)", text)
        horizontal = [e for e in page.edges if e["orientation"] == "h" and e["width"] > 400]
        bounds = sorted({round(e["top"], 2) for e in horizontal})
        found = []
        for top,bottom in zip(bounds,bounds[1:]):
            vertical = [e for e in page.edges if e["orientation"] == "v" and abs(e["top"]-top)<.5 and abs(e["bottom"]-bottom)<.5]
            xs = sorted({round(e["x0"],2) for e in vertical})
            if len(xs) == 5:
                found.append((top,bottom,xs))
        assert len(found) == 4, f"Worked matrix needs one header and three rows: {found}"
        expected = [[2,3,3],[1,1,2],[1,1,1]]
        for index,(top,bottom,xs) in enumerate(found[1:]):
            for col,value in enumerate(expected[index],1):
                cell = page.crop((xs[col]+.5,top+.5,xs[col+1]-.5,bottom-.5))
                assert (cell.extract_text() or "").strip() == str(value)
        lines = [line for line in page.lines if abs(line.get("linewidth",0)-.8)<.02 and len(line.get("pts",[]))==2]
        endpoints = Counter(tuple(round(v,2) for v in p) for line in lines for p in line["pts"])
        assert len(lines) == 3 and len(endpoints) == 3 and set(endpoints.values()) == {2}, "Example must be native K3"
        compact = re.sub(r"\s+", "", text).replace("−","-")
        for delta in ["(+1,0,0)","(0,+1,0)","(+1,+1,0)"]:
            assert delta in compact, f"Missing worked change {delta}"
        assert "(0,0,0)" in compact
    print(f"PASS {Path(path).name}: concise numbered heading tree; filled 3×3 parameter values; native K3; three correct signed changes")

if __name__ == "__main__":
    for path in sys.argv[1:]:
        verify(path)
