"""🏹 Independently inspect directed graph cubic paths and native arrowheads in actual PDFs."""
import itertools
import math
import sys
from collections import Counter
from pathlib import Path
import pdfplumber
from pypdf import PdfReader
from pypdf.generic import ContentStream


MM = 72 / 25.4
SELECTED = [(i,i+1) for i in range(1,16)] + [(16,1),(1,5),(5,10),(10,13),(13,1)]


def product(a,b):
    return (a[0]*b[0]+a[1]*b[2],a[0]*b[1]+a[1]*b[3],a[2]*b[0]+a[3]*b[2],a[2]*b[1]+a[3]*b[3],a[4]*b[0]+a[5]*b[2]+b[4],a[4]*b[1]+a[5]*b[3]+b[5])


def form_bounds(page):
    matrix=(1,0,0,1,0,0)
    stack=[]
    bounds=[]
    resources=page["/Resources"].get_object().get("/XObject",{})
    for operands,operator in ContentStream(page.get_contents(),page.pdf).operations:
        if operator==b"q": stack.append(matrix)
        elif operator==b"Q": matrix=stack.pop()
        elif operator==b"cm": matrix=product(tuple(map(float,operands)),matrix)
        elif operator==b"Do":
            obj=resources[operands[0]]
            if obj.get("/Subtype")=="/Form":
                box=list(map(float,obj["/BBox"]))
                points=[(matrix[0]*x+matrix[2]*y+matrix[4],float(page.mediabox.height)-(matrix[1]*x+matrix[3]*y+matrix[5])) for x,y in itertools.product((box[0],box[2]),(box[1],box[3]))]
                bounds.append((min(x for x,y in points),min(y for x,y in points),max(x for x,y in points),max(y for x,y in points)))
    return bounds


def inside(point,box,padding=0):
    return box[0]-padding < point[0] < box[2]+padding and box[1]-padding < point[1] < box[3]+padding


def distance(point,box):
    return math.hypot(max(box[0]-point[0],0,point[0]-box[2]),max(box[1]-point[1],0,point[1]-box[3]))


def arcs(page,width):
    return [curve for curve in page.curves if abs(curve.get("linewidth",0)-width)<.02 and curve.get("stroke") and any(command[0]=="c" for command in curve.get("path",[]))]


def heads(page,width):
    return [curve for curve in page.curves if abs(curve.get("linewidth",0)-width)<.02 and curve.get("fill") and not any(command[0]=="c" for command in curve.get("path",[])) and 0.2<curve["width"]<9 and 0.2<curve["height"]<9]


def check_arcs(page,width,pairs,boxes):
    curves=[curve for curve in arcs(page,width) if not any(inside(curve["path"][0][1],box,.2) and inside(curve["path"][-1][-1],box,.2) for box in boxes)]
    assert len(curves)==len(pairs),("directed cubic count",width,len(curves),len(pairs))
    tips=heads(page,width)
    assert len(tips)==len(pairs),("native arrowhead count",width,len(tips),len(pairs))
    traces={}
    for curve,head,(a,b) in zip(curves,tips,pairs):
        path=curve["path"]
        start=path[0][1]
        end=[command[-1] for command in path if command[0]=="c"][-1]
        assert distance(start,boxes[a-1])<13,("source endpoint",a,b,start,boxes[a-1])
        assert distance(end,boxes[b-1])<13,("target endpoint",a,b,end,boxes[b-1])
        assert not any(inside(start,box) or inside(end,box) for box in boxes),("edge endpoint occluded by icon",a,b)
        points=[point for command in head["path"] for point in command[1:] if isinstance(point,tuple)]
        assert points and min(math.dist(end,point) for point in points)<8,("head absent at target",a,b)
        assert all(not any(inside(point,box) for box in boxes) for point in points),("arrowhead occluded by icon",a,b)
        traces[a,b]=tuple(path)
    for (a,b),path in traces.items():
        if (b,a) in traces:
            mid=path[1][1]
            reverse=traces[b,a][1][-2]
            assert math.dist(mid,reverse)>.5,("antiparallel paths coincide",a,b)
    return traces


def verify(path):
    reader=PdfReader(path)
    assert len(reader.pages)==3
    boxes=form_bounds(reader.pages[2])
    assert len(boxes)==16,("canonical native vector icon count",len(boxes))
    with pdfplumber.open(path) as pdf:
        page=pdf.pages[2]
        pairs=list(itertools.permutations(range(1,17),2))
        check_arcs(page,.35,pairs,boxes)
        check_arcs(page,.8,SELECTED,boxes)
        assert Counter(a for a,b in pairs)==Counter({i:15 for i in range(1,17)})
        assert Counter(b for a,b in pairs)==Counter({i:15 for i in range(1,17)})
        assert all((b,a) not in SELECTED for a,b in SELECTED)
        inner=SELECTED[16:]
        assert Counter(a for a,b in inner)==Counter({i:1 for i in [1,5,10,13]})
        assert Counter(b for a,b in inner)==Counter({i:1 for i in [1,5,10,13]})
        assert all(b==inner[(index+1)%4][0] for index,(a,b) in enumerate(inner))
        assert sum((b-a)%16 for a,b in inner)==16
        centers=[((box[0]+box[2])/2,(box[1]+box[3])/2) for box in boxes]
        assert abs((max(y for x,y in centers)-min(y for x,y in centers))/2/MM-53)<.1
        assert all(0<box[1]<box[3]<page.height for box in boxes),("graph page bounds",boxes)
        sides=[edge for edge in page.edges if edge["orientation"]=="v" and edge["top"]<min(box[1] for box in boxes) and edge["bottom"]>max(box[3] for box in boxes)]
        frames=[(left,right) for left in sides for right in sides if left["x0"]<min(box[0] for box in boxes) and right["x0"]>max(box[2] for box in boxes) and abs(left["top"]-right["top"])<.2 and abs(left["bottom"]-right["bottom"])<.2]
        assert frames,"K16 requires a native enclosing Diagram frame"
        left,right=min(frames,key=lambda pair:pair[0]["height"])
        graph_labels=[char for char in page.chars if char["text"].isdigit() and left["top"]<char["top"]<left["bottom"]]
        visible_left=min([box[0] for box in boxes]+[char["x0"] for char in graph_labels])
        visible_right=max([box[2] for box in boxes]+[char["x1"] for char in graph_labels])
        left_inset=(visible_left-left["x0"])/MM
        right_inset=(right["x0"]-visible_right)/MM
        assert 1<left_inset<3 and 1<right_inset<3 and abs(left_inset-right_inset)<.3,("native inner graph width",left_inset,right_inset)
        visible_height=(max([box[3] for box in boxes]+[char["bottom"] for char in graph_labels])-min([box[1] for box in boxes]+[char["top"] for char in graph_labels]))/MM
        assert abs(visible_height/83.6-1.5)<.03,("visible graph height ratio",visible_height/83.6)
        print("Graph visible height",round(visible_height,3),"mm; ratio",round(visible_height/83.6,5))
        entries=[]
        ledger_geometry=[]
        for line in page.lines:
            if abs(line.get("linewidth",0)-.3)<.02 and 20*MM<line["width"]<90*MM and line["height"]<.1:
                text=page.crop((max(0,line["x0"]-23*MM),line["top"]-5*MM,line["x0"]+1,line["top"]+1)).extract_text() or ""
                import re
                nums=re.findall(r"\d+",text)
                assert len(nums)==2 and "→" in text,("directed ledger arrow",text)
                entries.append(tuple(map(int,nums)))
                chars=page.crop((max(0,line["x0"]-23*MM),line["top"]-5*MM,line["x0"]+1,line["top"]+1)).chars
                ledger_geometry.append((line,min(char["x0"] for char in chars)))
        assert entries==SELECTED,("ledger ordered arcs",entries)
        ledger_geometry.sort(key=lambda entry:entry[0]["x0"])
        for left,right in zip(ledger_geometry[:10],ledger_geometry[10:]):
            assert (right[1]-left[0]["x1"])/MM>=2,("ledger column gutter below 2mm",(right[1]-left[0]["x1"])/MM)
        worked=pdf.pages[0]
        example_nodes=[(rect["x0"],rect["top"],rect["x1"],rect["bottom"]) for rect in worked.rects if rect.get("fill") and not rect.get("stroke") and 40<rect["width"]<120 and 25<rect["height"]<80]
        assert len(example_nodes)==3,("K3 opaque node bounds",example_nodes)
        top=min(example_nodes,key=lambda box:box[1])
        others=sorted((box for box in example_nodes if box!=top),key=lambda box:box[0],reverse=True)
        example_nodes=[top]+others
        check_arcs(worked,.8,[(1,2),(2,1),(2,3),(3,2),(1,3),(3,1)],example_nodes)
        graph_bottom=max(box[3] for box in example_nodes)
        graph_top=min(box[1] for box in example_nodes)-12
        graph_text="".join(char["text"] for char in worked.chars if graph_top<char["top"]<graph_bottom)
        assert "Form" not in graph_text and "Δ" not in graph_text and "(1,1,1)" not in graph_text, ("unexpected graph caption",graph_text)
        rules=[line for line in worked.lines if abs(line.get("linewidth",0)-.3)<.02 and 50<line["width"]/MM<70 and line["height"]<.1 and line["top"]>graph_bottom]
        assert len(rules)==6,("worked ruled ledger entries",len(rules))
        positions={round(line["x0"],1) for line in rules}
        assert len(positions)==2
        column_pitch=max(positions)-min(positions)
        assert all(abs(line["width"]-.76*column_pitch)<.15 for line in rules), "Ledger rules must use native inner column width"
        changes=[]
        value_metrics=[]
        categories=["Quader","Pyramide","ellipt.Zylinder"] if reader.trailer["/Root"].get("/Lang")=="de-DE" else ["Box","Pyramid","ellipticcylinder"]
        worked_pairs=[(1,2),(2,3),(3,1),(2,1),(3,2),(1,3)]
        for index,line in enumerate(rules):
            chars=[char for char in worked.chars if line["x0"]-.2<=char["x0"] and char["x1"]<=line["x1"]+.2 and line["top"]-5*MM<char["top"]<line["top"]]
            value="".join(char["text"] for char in chars).replace(" ","")
            assert value and "→" in value,("missing filled change",value)
            a,b=worked_pairs[index]
            assert categories[a-1]+"→"+categories[b-1] in value,("localized categorical change clipped or incorrect",a,b,value)
            assert max(char["x1"] for char in chars)<=line["x1"]+.2,("value exceeds writing rule",value)
            assert min(char["size"] for char in chars)>=8.5*72/72.27-.01,("ledger font below 8.5pt",min(char["size"] for char in chars))
            assert all(char["bottom"]<=line["top"]+.2 for char in chars),("value overlaps rule",value)
            changes.append(value)
            value_metrics.append((min(char["size"] for char in chars),(max(char["x1"] for char in chars)-line["x0"])/MM,(line["top"]-max(char["bottom"] for char in chars))/MM))
        for value,(a,b) in zip(changes,worked_pairs):
            sign="" if {a,b}=={1,2} else "+" if b==3 else "-"
            numeric=";".join(name+sign+"1" for name in ["L","B","H"])+";" if sign else ""
            expected=numeric+categories[a-1]+"→"+categories[b-1]
            assert value.replace("−","-")==expected,("named nonzero change",value,expected)
        print("Worked ledger: min font",round(min(row[0] for row in value_metrics),5),"PDF bp; max value width",round(max(row[1] for row in value_metrics),3),"mm; min rule clearance",round(min(row[2] for row in value_metrics),3),"mm")
        for x in positions:
            ys=sorted(line["top"] for line in rules if round(line["x0"],1)==x)
            assert all(abs((b-a)/MM-6.3)<.02 for a,b in zip(ys,ys[1:])),ys
        for line,_ in ledger_geometry:
            assert not [char for char in page.chars if line["x0"]<=char["x0"] and char["x1"]<=line["x1"] and line["top"]-4*MM<char["top"]<line["top"]],"Task ledger must remain blank"
    print("PASS",Path(path).name,"240 directed cubic arcs, 20 selected single directions, 266 native arrowheads; endpoint/icon clearance; ordered ledger; directed K3")


if __name__=="__main__":
    assert len(sys.argv)>1,"Supply candidate PDF paths"
    for path in sys.argv[1:]: verify(path)
