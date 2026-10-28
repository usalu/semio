import json,sys
from pathlib import Path
from pypdf import PdfReader
from pypdf.generic import ContentStream
import pypdfium2 as pdfium
r=PdfReader(sys.argv[1]);assert len(r.pages)==1
stack=[];dash=[];path=[];strokes=[]
for args,op in ContentStream(r.pages[0].get_contents(),r).operations:
    if op==b'q':stack.append(dash[:])
    elif op==b'Q':dash=stack.pop()
    elif op==b'd':dash=list(map(float,args[0]))
    elif op in (b'm',b'l',b'c',b'h'):path.append(op)
    elif op in (b'S',b's',b'B',b'b',b'B*',b'b*'):
        if dash:strokes.append((dash[:],path[:]))
        path=[]
    elif op in (b'n',b'f',b'F',b'f*'):path=[]
assert len(strokes)==5,len(strokes)
assert all(s[0]==strokes[0][0] for s in strokes)
assert len(strokes[0][0])==2 and strokes[0][0][0]>0
counts=[s[1].count(b'l') for s in strokes if b'c' not in s[1]]
assert counts==[2,1,2,1],counts
assert sum(b'c' in s[1] for s in strokes)==1
pdf=pdfium.PdfDocument(sys.argv[1])
pdf[0].render(scale=2).to_pil().save(Path(sys.argv[1]).with_suffix('.png'))
print(json.dumps({'hiddenPaths':5,'boxEdges':3,'pyramidEdges':3,'cylinderRearHalf':True,'dash':strokes[0][0]}))
