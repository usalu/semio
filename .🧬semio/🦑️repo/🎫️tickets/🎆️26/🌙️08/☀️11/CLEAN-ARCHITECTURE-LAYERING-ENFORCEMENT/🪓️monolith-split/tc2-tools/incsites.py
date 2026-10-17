import sys,re,os,glob,collections
sys.path.insert(0,os.path.dirname(__file__))
import testonly as T
ROOT="/Users/ueli/Documents/semio/"
shorts=T.CRATES
shapes=collections.Counter(); ex={}
for short in sorted(shorts):
    for r in T.testonly(short):
        if r[0]=="embedded": continue
        p=ROOT+r[3]
        txt=open(p).read()
        for m in re.finditer(r'include_(?:str|bytes)!\s*\(',txt):
            line_start=txt.rfind("\n",0,m.start())+1
            line=txt[line_start:txt.find("\n",m.start())]
            shape=re.sub(r'"[^"]*"','"…"',line.strip())[:140]
            shape=re.sub(r'\b[A-Z][A-Z0-9_]{2,}\b','NAME',shape)
            shapes[shape]+=1; ex.setdefault(shape,(short,p))
for s,c in shapes.most_common(): print(c,s)
