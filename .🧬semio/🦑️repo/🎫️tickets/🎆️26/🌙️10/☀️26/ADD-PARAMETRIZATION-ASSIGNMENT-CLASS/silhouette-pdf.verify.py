"""🪟 Verify native Diagram and Transitions chips and enclosing frame geometry."""
import json,sys,hashlib
from pathlib import Path
import pdfplumber
from pypdf import PdfReader
from importlib.util import spec_from_file_location,module_from_spec
spec=spec_from_file_location("directed",Path(__file__).parent/"verify-directed-pdf.py")
directed=module_from_spec(spec)
spec.loader.exec_module(directed)

root=next(parent for parent in Path(__file__).parents if (parent/"🧰️framework/🛍️products/📓️print").exists())
fixture=json.loads((root/"🧰️framework/🛍️products/📓️print/🧪️tests/📝️parametrization-assignment/🧫️cases.json").read_text(encoding="utf-8"))
MM=72/25.4

def verify(path):
    reader=PdfReader(path)
    language="de" if reader.trailer["/Root"].get("/Lang")=="de-DE" else "en"
    contract=fixture["silhouettes"]
    with pdfplumber.open(path) as pdf:
        assert len(pdf.pages)==contract["pages"]
        windows=[]
        for kind,key in [("Diagram","diagramNumbers"),("Transitions","transitionNumbers")]:
            label=contract["kindLabels"][language][kind]
            matches=[(index,page,word) for index,page in enumerate(pdf.pages) for word in page.extract_words() if word["text"]==label or word["text"].startswith(label+":")]
            assert len(matches)==2,("native kind chip count",kind,len(matches))
            for (index,page,word),number in zip(matches,contract[key]):
                assert index in [0,2],("wrong graph/transition page",kind,index)
                row=[other for other in page.extract_words() if abs(other["top"]-word["top"])<2]
                assert "".join(other["text"] for other in row)==label+":"+number,("automatic shared number",kind,number,row)
                assert not [other for other in row if other["x0"]<page.width/2],("custom left name chip",kind,row)
                assert word["x0"]>page.width/2,("kind chip is not automatic right chip",kind)
                vertical=[edge for edge in page.edges if edge["orientation"]=="v" and edge["height"]>10*MM and word["bottom"]-.5*MM<=edge["top"]<=word["bottom"]+5*MM]
                pairs=[(left,right) for left in vertical for right in vertical if left["x0"]<page.width/2<right["x0"] and right["x0"]-left["x0"]>160*MM and abs(left["top"]-right["top"])<.2 and abs(left["bottom"]-right["bottom"])<.2]
                assert pairs,("missing continuous native frame sides",kind,number)
                left,right=min(pairs,key=lambda pair:pair[0]["top"])
                bounds=(left["x0"],left["top"],right["x0"],left["bottom"])
                assert 19*MM<bounds[0]<21*MM and 189*MM<bounds[2]<191*MM,("native outer frame width",bounds)
                assert bounds[3]<277*MM,("frame crosses footer",kind,bounds)
                bottom=[edge for edge in page.edges if edge["orientation"]=="h" and abs(edge["top"]-bounds[3])<.5 and edge["x0"]<=bounds[0]+.5 and edge["x1"]>=bounds[2]-.5]
                assert bottom,("native frame bottom is disconnected",kind,number)
                right_band=[edge["x0"] for edge in vertical if abs(edge["top"]-bounds[1])<.2 and edge["x0"]>page.width/2]
                strokes=[edge for edge in page.edges if edge["orientation"]=="v" and edge.get("object_type")=="line"]
                cap=[edge for edge in strokes if edge["top"]<=word["top"] and edge["bottom"]>=word["bottom"] and edge["x0"]+edge.get("linewidth",0)/2>=min(right_band) and edge["x0"]-edge.get("linewidth",0)/2<=max(right_band)]
                assert cap,("automatic chip right stroke missing",kind,number)
                connected=[]
                for seed in cap:
                    component=[seed]
                    while True:
                        adjacent=[edge for edge in strokes if edge not in component and any(abs(edge["x0"]-other["x0"])<=(edge.get("linewidth",0)+other.get("linewidth",0))/2+.001 and edge["top"]-edge.get("linewidth",0)/2<=other["bottom"]+other.get("linewidth",0)/2+.001 and other["top"]-other.get("linewidth",0)/2<=edge["bottom"]+edge.get("linewidth",0)/2+.001 for other in component)]
                        if not adjacent:
                            break
                        component+=adjacent
                    connected+=component
                assert any(edge["top"]-edge.get("linewidth",0)/2<=bounds[1]<=edge["bottom"]+edge.get("linewidth",0)/2 and edge["x0"]+edge.get("linewidth",0)/2>=min(right_band) and edge["x0"]-edge.get("linewidth",0)/2<=max(right_band) for edge in connected),("automatic chip detached from right frame",kind,number)
                ghost=[edge for edge in strokes if edge["x0"]<page.width/2 and min(seed["top"] for seed in cap)-.001<=edge["top"]<word["bottom"] and edge["bottom"]>=word["top"]]
                assert not ghost,("empty left ghost tab",kind,number)
                bodychars=[char for char in page.chars if bounds[1]+.5<char["top"]<bounds[3]-.5]
                assert all(bounds[0]+.5<=char["x0"]<=char["x1"]<=bounds[2]-.5 and char["bottom"]<=bounds[3]-.5 for char in bodychars),("window text outside native frame",kind,number)
                if kind=="Transitions":
                    rules=[line for line in page.lines if abs(line.get("linewidth",0)-.3)<.02 and 50*MM<line["width"]<70*MM and bounds[1]<line["top"]<bounds[3]]
                    assert len(rules)==(6 if index==0 else 20),("transition list framing",number,len(rules))
                    assert all(bounds[0]+.5<line["x0"]<line["x1"]<bounds[2]-.5 for line in rules)
                else:
                    graph=[curve for curve in page.curves if abs(curve.get("linewidth",0)-.8)<.02 and curve.get("stroke") and any(part[0]=="c" for part in curve.get("path",[])) and bounds[1]<curve["top"]<bounds[3]]
                    assert len(graph)==(6 if index==0 else 20),("graph inside diagram",number,len(graph))
                    assert all(bounds[0]+.5<curve["x0"]<=curve["x1"]<bounds[2]-.5 and curve["bottom"]<bounds[3]-.5 for curve in graph),("graph outside native frame",number)
                    if index==2:
                        icons=directed.form_bounds(reader.pages[2])
                        assert len(icons)==16
                        centers=[(box[1]+box[3])/2 for box in icons]
                        assert abs((max(centers)-min(centers))/2/MM-fixture["graph"]["verticalRadiusMm"])<.1
                        digits=[char for char in bodychars if char["text"].isdigit()]
                        visible=(max([box[3] for box in icons]+[char["bottom"] for char in digits])-min([box[1] for box in icons]+[char["top"] for char in digits]))/MM
                        assert abs(visible-125.603)<.3,("native framed K16 visible height",visible)
                        print("PASS framed K16 vertical radius",round((max(centers)-min(centers))/2/MM,3),"mm; visible height",round(visible,3),"mm")
                windows.append({"kind":kind,"number":number,"page":index+1,"boundsMm":[round(value/MM,3) for value in bounds]})
        assert [window["page"] for window in windows]==[1,3,1,3]
        for index in [1,3]:
            same=[window for window in windows if window["page"]==index]
            assert same[0]["boundsMm"][3]<same[1]["boundsMm"][1], ("diagram overlaps transitions",index)
    print(json.dumps({"path":str(path),"windows":windows,"sha256":hashlib.sha256(Path(path).read_bytes()).hexdigest()},ensure_ascii=False))

for source in sys.argv[1:]:
    verify(source)
