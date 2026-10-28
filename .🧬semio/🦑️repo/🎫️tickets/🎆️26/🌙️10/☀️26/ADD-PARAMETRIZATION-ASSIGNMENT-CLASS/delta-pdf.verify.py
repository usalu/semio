"""🔢 Verify named nonzero ledger changes and complete native text width."""
import json,re,sys,hashlib
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

root=next(parent for parent in Path(__file__).parents if (parent/"🧰️framework/🛍️products/📓️print").exists())
fixture=json.loads((root/"🧰️framework/🛍️products/📓️print/🧪️tests/📝️parametrization-assignment/🧫️cases.json").read_text(encoding="utf-8"))
MM=72/25.4
normalize=lambda text: re.sub(r"\s+","",text).replace("−","-")

def verify(path):
    reader=PdfReader(path)
    names={"box":"Quader","pyramid":"Pyramide","elliptical-cylinder":"ellipt. Zylinder"} if reader.trailer["/Root"].get("/Lang")=="de-DE" else {"box":"Box","pyramid":"Pyramid","elliptical-cylinder":"elliptic cylinder"}
    with pdfplumber.open(path) as pdf:
        page=pdf.pages[0]
        rules=sorted([line for line in page.lines if abs(line.get("linewidth",0)-.3)<.02 and 50<line["width"]/MM<70 and line["height"]<.1],key=lambda line:(round(line["x0"],1),line["top"]))
        assert len(rules)==6,len(rules)
        columns=sorted({round(line["x0"],1) for line in rules})
        assert len(columns)==2
        expected_rule=(columns[1]-columns[0])*fixture["workedExample"]["ledger"]["ruleWidthFractionOfColumn"]
        assert all(abs(line["width"]-expected_rule)<.15 for line in rules),("native relative rule width",expected_rule)
        pairs=[(1,2),(2,3),(3,1),(2,1),(3,2),(1,3)]
        edges={(edge["from"],edge["to"]):edge for edge in fixture["workedExample"]["edges"]}
        widths=[]
        for line,pair in zip(rules,pairs):
            edge=edges[pair]
            numeric=[name+("+" if value>0 else "")+str(value) for name,value in zip(fixture["workedExample"]["ledger"]["dimensionNames"],edge["numeric"]) if value!=0]
            expected="; ".join(numeric+[names[edge["category"][0]]+" → "+names[edge["category"][1]]])
            right=min(page.width-19*MM,line["x1"]+MM)
            chars=[char for char in page.chars if line["x0"]-.2<=char["x0"]<right and line["top"]-4*MM<char["top"]<line["top"]]
            value="".join(char["text"] for char in chars)
            assert normalize(value)==normalize(expected),(pair,value,expected)
            assert not re.search(r"\([0-9,+-]+\)|[LBH][+-]?0",normalize(value)),value
            assert max(char["x1"] for char in chars)<=line["x1"]+.2,("overflow beyond writing rule",pair,value)
            for index,char in enumerate(chars):
                if char["text"]==";":
                    following=next(other for other in chars[index+1:] if other["text"].strip())
                    assert following["x0"]-char["x1"]>.25*MM,("semicolon lacks visible space",pair,value)
            minimum=min(char["size"] for char in chars)
            assert minimum>=fixture["workedExample"]["ledger"]["valueFontPt"]*72/72.27-.03,("font below nine points",minimum)
            assert max(char["bottom"] for char in chars)<=line["top"]+.2,("overlaps writing rule",pair)
            width=(max(char["x1"] for char in chars)-line["x0"])/MM
            widths.append(width)
        for x in {round(line["x0"],1) for line in rules}:
            ys=sorted(line["top"] for line in rules if round(line["x0"],1)==x)
            assert all(abs((b-a)/MM-fixture["workedExample"]["ledger"]["rowHeightMm"])<.02 for a,b in zip(ys,ys[1:]))
    print(json.dumps({"path":str(path),"entries":6,"namedNonzeroNumericEntries":4,"categoryOnlyEntries":2,"maxWidthMm":max(widths),"ruleWidthMm":rules[0]["width"]/MM,"sha256":hashlib.sha256(Path(path).read_bytes()).hexdigest()},ensure_ascii=False))

for source in sys.argv[1:]:
    verify(source)
