import sys,re,os
ROOT="/Users/ueli/Documents/semio/"
sys.path.insert(0,os.path.dirname(__file__))
import testonly as T
for short in sys.argv[1:]:
    print("=====",short)
    for r in T.testonly(short):
        if r[0]=="embedded": continue
        p=ROOT+r[3] if not r[3].startswith("/") else r[3]
        txt=open(p).read()
        uses=[l.strip() for l in txt.split("\n") if re.match(r'\s*(pub(\([^)]*\))?\s+)?use ',l)]
        internal=[u for u in uses if re.search(r'\b(super|crate|self)::',u)]
        incl=re.findall(r'include_(?:str|bytes)!\(',txt)
        print(p.split("subsets/")[-1][-90:], r[1], "uses",len(uses),"internal",len(internal),"incl",len(incl), "supersglob" if "super::*" in txt else "")
        for u in internal[:6]: print("     ",u[:150])
