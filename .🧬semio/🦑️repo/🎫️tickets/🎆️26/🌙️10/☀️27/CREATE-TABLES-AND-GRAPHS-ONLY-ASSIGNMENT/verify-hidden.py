"""🔎 Check every hidden edge and contour in table and graph."""
from pathlib import Path
import json
import sys
import pdfplumber

case=json.loads((Path(__file__).parent/"case.json").read_text())
with pdfplumber.open(sys.argv[1]) as pdf:
    objects=[o for kind in ("line","curve") for o in pdf.pages[0].objects.get(kind,[]) if o.get("dash") and any(float(v)>0 for v in o["dash"][0])]
    boxes=[(o["x0"],o["top"],o["x1"],o["bottom"]) for o in objects]
    components=[]
    pending=set(range(len(boxes)))
    def touches(a,b):
        return a[0]<=b[2]+1 and b[0]<=a[2]+1 and a[1]<=b[3]+1 and b[1]<=a[3]+1
    while pending:
        group={pending.pop()}
        while True:
            joined={i for i in pending if any(touches(boxes[i],boxes[j]) for j in group)}
            if not joined:break
            pending-=joined
            group|=joined
        components.append(group)
    assert len(components)==case["hiddenLineGroups"],f"Expected six hidden-line shape groups; found {len(components)}"
    point=72/72.27
    expected=[v*point for v in case["hiddenDashPt"]]
    for obj in objects:
        assert all(abs(a-b)<.002 for a,b in zip(obj["dash"][0],expected)),obj["dash"]
    widths={round(o["linewidth"]/(case["shapeLineWidthPt"]*point),2) for o in objects}
    assert widths=={.6,1},widths
    curved=0
    boxes_count=0
    for group in components:
        if any(any(p[0]=="c" for p in objects[i].get("path",[])) for i in group):
            assert len(group)==1
            curved+=1
            continue
        segments=[]
        for i in group:
            pts=objects[i].get("pts",[])
            segments.extend(zip(pts,pts[1:]))
        assert len(segments)==3,(group,segments)
        assert any(abs(b[1]-a[1])<.02 for a,b in segments)
        assert any(abs(abs((b[1]-a[1])/(b[0]-a[0]))-7/9)<.002 for a,b in segments if abs(b[0]-a[0])>.02)
        boxes_count+=any(abs(b[0]-a[0])<.02 for a,b in segments)
    assert curved==2 and boxes_count==2,(curved,boxes_count)
print("PASS: both renderings of all three shapes have complete hidden edges/contours; shared projection, pen width and dash style")
