import json,math,re,sys
from pathlib import Path
from pypdf import PdfReader
import pdfplumber,pypdfium2 as pdfium
def coords(op):
    return tuple(x for value in op[1:] for x in (value if isinstance(value,(tuple,list)) else (value,)))
c=json.loads(Path(__file__).with_name('cases.json').read_text())
r=PdfReader(sys.argv[1]);assert len(r.pages)==2
for i,expected in enumerate([c['workedEdges'],c['selectedEdges']]):
    tokens=[]
    def visitor(text,cm,tm,font,size):
        if abs(size-8*72/72.27)<.1 and text.strip():tokens.append(text.strip())
    r.pages[i].extract_text(visitor_text=visitor)
    pairs=[[int(a),int(b)] for a,b in re.findall(r'(\d+)\s*→\s*(\d+)',' '.join(tokens))]
    assert pairs==expected,{'page':i+1,'pairs':pairs,'tokens':tokens}
doc=pdfium.PdfDocument(sys.argv[1])
for i,page in enumerate(doc):page.render(scale=1.6).to_pil().save(Path(sys.argv[1]).with_name('page-'+str(i+1)+'.png'))

with pdfplumber.open(sys.argv[1]) as pdf:
    for i,page in enumerate(pdf.pages):
        curves=[p for p in page.curves if not p['fill'] and abs(p['linewidth']-.8*72/72.27)<.02 and sum(v[0]=='c' for v in p['path'])==1]
        expected=len([c['workedEdges'],c['selectedEdges']][i]);assert len(curves)==expected,(i,len(curves))
        masks=[]
        for p in page.rects+page.curves:
            if not p['fill'] or p.get('non_stroking_color') not in [(1,),1,(1,1,1)]:continue
            ops=p.get('path',[]);points=[]
            if any(v[0]=='c' for v in ops):continue
            for v in ops:
                if v[0] in ['m','l']:points.append(coords(v))
            if len(points)>1 and points[0]==points[-1]:points.pop()
            if len(points)!=4:continue
            area=abs(sum(points[j][0]*points[(j+1)%4][1]-points[(j+1)%4][0]*points[j][1] for j in range(4)))/2
            if not 8*(72/25.4)**2<area<50*(72/25.4)**2:continue
            sides=[(points[(j+1)%4][0]-points[j][0],points[(j+1)%4][1]-points[j][1]) for j in range(4)]
            if any(abs(sum(sides[j][k]*sides[(j+1)%4][k] for k in range(2)))>0.01 for j in range(4)):continue
            masks.append(points)
        assert len(masks)==expected,{'page':i+1,'masks':len(masks),'expected':expected}
        mids=[]
        for p in curves:
            ops=p['path'];a=next(coords(v) for v in ops if v[0]=='m');v=next(coords(v) for v in ops if v[0]=='c');b,d,e=v[:2],v[2:4],v[4:6]
            m=tuple((a[k]+3*b[k]+3*d[k]+e[k])/8 for k in range(2))
            tangent=tuple(3*(b[k]-a[k])/4+3*(d[k]-b[k])/2+3*(e[k]-d[k])/4 for k in range(2))
            mids.append((m,tangent,(a,b,d,e)))
        used=set();min_gap=1000;max_mid=0;min_alignment=1
        for polygon in masks:
            center=tuple(sum(v[k] for v in polygon)/4 for k in range(2))
            idx=min(range(len(mids)),key=lambda j:math.dist(center,mids[j][0]));assert idx not in used;used.add(idx)
            m,tangent,pts=mids[idx];ln=math.hypot(*tangent);u=tuple(v/ln for v in tangent);n=(-u[1],u[0]);delta=tuple(center[k]-m[k] for k in range(2))
            along=abs(sum(delta[k]*u[k] for k in range(2)));assert along<.3*72/25.4,(i,along)
            max_mid=max(max_mid,along)
            side=max([(polygon[(j+1)%4][0]-polygon[j][0],polygon[(j+1)%4][1]-polygon[j][1]) for j in range(4)],key=lambda v:math.hypot(*v))
            alignment=abs(sum(side[k]*u[k] for k in range(2)))/math.hypot(*side);assert alignment>.999,alignment
            min_alignment=min(min_alignment,alignment)
            signed=[sum((v[k]-m[k])*n[k] for k in range(2)) for v in polygon]
            gap=min(abs(v) for v in signed) if min(signed)*max(signed)>0 else 0
            assert gap>.8*72/25.4,(i,gap)
            min_gap=min(min_gap,gap)
        print(json.dumps({'page':i+1,'labels':expected,'midpointMaxMm':max_mid*25.4/72,'minEdgeGapMm':min_gap*25.4/72,'minAlignment':min_alignment}))
