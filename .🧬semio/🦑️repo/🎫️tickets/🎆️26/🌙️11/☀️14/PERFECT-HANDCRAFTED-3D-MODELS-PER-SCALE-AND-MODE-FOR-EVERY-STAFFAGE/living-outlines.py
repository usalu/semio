"""🌿 Check authored plant extrusion footprints without allocating any triangle buffers."""
import json
import math
from pathlib import Path

def turn(a,b,c):
    return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])

root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
count=0
for path in sorted(root.glob('*vegetation/*/*.json')):
    a=json.loads(path.read_text())['asset']
    for scale,model in a['volumes'].items():
        for mode in ('body','thermal','structure'):
            for s in model[mode]:
                if s.get('kind')!='extrude':continue
                key=(path.parent.name,scale,mode,s['id'])
                assert abs(s['to']-s['from'])>1e-10,key
                for pts in [s['profile'],*s.get('holes',[])]:
                    pts=pts[:-1] if pts[0]==pts[-1] else pts
                    edges=list(zip(pts,pts[1:]+pts[:1]))
                    assert abs(sum(a[0]*b[1]-b[0]*a[1] for a,b in edges))>1e-12,key
                    for i,(a,b) in enumerate(edges):
                        for j in range(i+2,len(edges)-(1 if i==0 else 0)):
                            c,d=edges[j]
                            assert not(turn(a,b,c)*turn(a,b,d)<-1e-18 and turn(c,d,a)*turn(c,d,b)<-1e-18),key
                    count+=1
print('[DEBUG]',count,'nonzero simple extrusion profiles')

