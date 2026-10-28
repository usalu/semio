"""🔎 Validate standard print table chrome and the worked data grid."""
from pathlib import Path
import json
import re
import sys
import pdfplumber

case=json.loads((Path(__file__).parent/"case.json").read_text())
with pdfplumber.open(sys.argv[1]) as pdf:
    page=pdf.pages[0]
    text=page.extract_text() or ""
    compact=re.sub(r"\s+","",text)
    assert "Tabelle:1.1.a" in compact,"Worked example needs the standard table number"
    assert text.count(case["workedTableChrome"]["de"])==2,"Heading and table title must both be present"
    horizontal=[e for e in page.edges if e["orientation"]=="h" and e["width"]>400]
    boundaries=sorted({round(e["top"],2) for e in horizontal})
    rows=[]
    for top,bottom in zip(boundaries,boundaries[1:]):
        vertical=[e for e in page.edges if e["orientation"]=="v" and abs(e["top"]-top)<.5 and abs(e["bottom"]-bottom)<.5]
        xs=sorted({round(e["x0"],2) for e in vertical})
        if len(xs)==5:rows.append((top,bottom,xs))
    assert len(rows)==case["workedParameterRows"]+1,(len(rows),rows)
    assert all(row[2]==rows[0][2] for row in rows)
    assert 19.8<(rows[0][1]-rows[0][0])*25.4/72<20.5
    for row,expected in zip(rows[1:4],case["workedValues"]):
        top,bottom,xs=row
        for column,value in enumerate(expected,1):
            cell=page.crop((xs[column]+.5,top+.5,xs[column+1]-.5,bottom-.5))
            assert float((cell.extract_text() or "").strip())==value
print("PASS: standard print table title/number and connected 4-column, 5-row worked grid; all values retained")
