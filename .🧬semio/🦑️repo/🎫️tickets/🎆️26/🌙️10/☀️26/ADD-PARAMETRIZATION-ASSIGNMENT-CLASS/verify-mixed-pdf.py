"""🔎 Verify mixed-form PDF dimensions, vector primitives and categorical transitions."""
import itertools
import math
import re
import sys
from collections import Counter
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

MM = 72 / 25.4


def matrix(page):
    horizontal = [edge for edge in page.edges if edge["orientation"] == "h" and edge["width"] > 400]
    groups = {}
    for edge in horizontal:
        groups.setdefault((round(edge["x0"], 1), round(edge["x1"], 1)), []).append(edge)
    for full in groups.values():
        ys = sorted({round(edge["top"], 2) for edge in full})
        rows = []
        for top, bottom in zip(ys, ys[1:]):
            vertical = [edge for edge in page.edges if edge["orientation"] == "v" and abs(edge["top"]-top) < .5 and abs(edge["bottom"]-bottom) < .5]
            xs = sorted({round(edge["x0"], 2) for edge in vertical})
            if len(xs) == 5:
                rows.append((top, bottom, xs))
        if len(rows) == 9 and all(row[2] == rows[0][2] for row in rows):
            return rows[0][2], [rows[0][0]] + [row[1] for row in rows]
    raise AssertionError("Expected a four-column nine-row worked example matrix")


def primitives(page, cell):
    left, top, right, bottom = cell
    return [obj for kind in ("line", "curve", "rect") for obj in page.objects.get(kind, [])
            if obj["x0"] > left+.2 and obj["x1"] < right-.2 and obj["top"] > top+.2 and obj["bottom"] < bottom-.2]


def verify(path):
    reader = PdfReader(path)
    assert len(reader.pages) == 3
    assert reader.trailer["/Root"].get("/Lang") in ("de-DE", "en-US")
    with pdfplumber.open(path) as pdf:
        page = pdf.pages[0]
        xs, ys = matrix(page)
        assert not [image for image in page.images if ys[0] < image["top"] < 450], "Worked example must be native vector"
        for row, values in enumerate(((2.5,2.5,3.5),(1.5,1.5,2.5),(2,2,3)), 1):
            for column, expected in enumerate(values, 1):
                text = page.crop((xs[column]+1, ys[row]+1, xs[column+1]-1, ys[row+1]-1)).extract_text()
                assert float(text) == expected, (row,column,text)
        border = lambda y: max(edge.get("linewidth",0) for edge in page.edges if edge["orientation"] == "h" and edge["width"] > 400 and abs(edge["top"]-y)<.05)
        header_pitch = 16+(border(ys[0])+border(ys[1]))/(2*MM)
        assert abs((ys[1]-ys[0])/MM-header_pitch) < .02, ("worked header height", (ys[1]-ys[0])/MM, header_pitch)
        task_headers = [edge for edge in pdf.pages[1].edges if edge["orientation"] == "h" and edge["width"]>400]
        task_ys = sorted({round(edge["top"],2) for edge in task_headers})
        assert sum(abs((b-a)-(ys[1]-ys[0])) < .02*MM for a,b in zip(task_ys,task_ys[1:])) == 2, "Both task headers must have the same pitch as the example"
        for row in range(5, 9):
            for column in range(4):
                assert not page.crop((xs[column]+1,ys[row]+1,xs[column+1]-1,ys[row+1]-1)).extract_text(), ("nonblank writing cell",row,column)
        for column in range(1,4):
            assert not page.crop((xs[column]+1,ys[0]+1,xs[column+1]-1,ys[1]-1)).extract_text(), "Worked header icons must be unnamed"
        assert all(abs((ys[row+1]-ys[row])/MM-(6+.4/MM)) < .05 for row in range(1,9))
        assert abs((xs[1]-xs[0])/MM-(27+4+.4/MM)) < .1
        forms = []
        for column in range(1,4):
            cell = (xs[column],ys[0],xs[column+1],ys[1])
            art = primitives(page, cell)
            assert art, "Missing worked header geometry"
            cubic = [obj for obj in art if any(command[0] == "c" for command in obj.get("path", []))]
            rects = [obj for obj in art if obj["object_type"] == "rect"]
            family = "elliptical-cylinder" if cubic else "box" if rects else "pyramid"
            forms.append(family)
            width = (max(obj["x1"] for obj in art)-min(obj["x0"] for obj in art))/MM
            height = (max(obj["bottom"] for obj in art)-min(obj["top"] for obj in art))/MM
            length, breadth, tall = ((2.5,1.5,2),(2.5,1.5,2),(3.5,2.5,3))[column-1]
            expected_width = (math.hypot(length,.45*breadth) if cubic else length+.45*breadth)*3
            expected_height = (tall+(.175 if family=="pyramid" else .35)*breadth)*3
            assert abs(width-expected_width) < .04, (family,"projected width",width,expected_width)
            assert abs(height-expected_height) < .04, (family,"projected height",height,expected_height)
            assert min(obj["top"] for obj in art)-ys[0] >= 2*MM-.1
            assert ys[1]-max(obj["bottom"] for obj in art) >= 2*MM-.1
            if cubic:
                solid=[obj for obj in cubic if not obj.get("dash") or not obj["dash"][0]]
                hidden=[obj for obj in cubic if obj.get("dash") and obj["dash"][0]]
                assert len(cubic)==3 and len(solid)==2 and len(hidden)==1, "Cylinder needs two solid rims and one dashed hidden lower rim"
                assert len(hidden[0]["dash"][0])==2 and all(value>0 for value in hidden[0]["dash"][0])
                lower=max(solid,key=lambda obj:obj["top"])
                upper=min(solid,key=lambda obj:obj["top"])
                assert upper["bottom"]<hidden[0]["top"] and hidden[0]["top"]<lower["top"]
                for first,last in [(hidden[0]["path"][0][1],lower["path"][-1][-1]),(hidden[0]["path"][-1][-1],lower["path"][0][1])]:
                    assert math.dist(first,last)<.02, "Hidden and visible lower rims must share tangent endpoints"
                assert abs(hidden[0]["x0"]-lower["x0"])<.02 and abs(hidden[0]["x1"]-lower["x1"])<.02
            elif family == "pyramid":
                assert any(len(obj.get("pts",[])) >= 5 for obj in art), "Pyramid needs a triangular apex silhouette"
        assert forms == ["box","pyramid","elliptical-cylinder"], forms
        for column in range(1,4):
            category = page.crop((xs[column]+1,ys[4]+1,xs[column+1]-1,ys[5]-1)).extract_text()
            assert category and not re.fullmatch(r"[\d.+-]+", category), "Form must remain a categorical word"
        graph_edges = [curve for curve in page.curves if abs(curve.get("linewidth",0)-.8)<.02 and curve.get("stroke") and any(command[0]=="c" for command in curve.get("path",[]))]
        assert len(graph_edges) == 6, "Worked graph needs six directed curved K3 arcs"
        assert {"1","2","3"} <= {word["text"] for word in page.extract_words() if word["top"]>ys[-1] and word["top"]<max(edge["bottom"] for edge in graph_edges)+35}
    print(f"PASS {Path(path).name}: four filled plus four blank parameter rows; shared 16 mm header and half-scale unnamed solids; native box/pyramid/elliptical-cylinder; independent projected geometry; directed K3; six separate ruled numeric/category changes")


if __name__ == "__main__":
    assert len(sys.argv)>1, "Supply candidate PDF paths"
    for argument in sys.argv[1:]:
        verify(argument)
