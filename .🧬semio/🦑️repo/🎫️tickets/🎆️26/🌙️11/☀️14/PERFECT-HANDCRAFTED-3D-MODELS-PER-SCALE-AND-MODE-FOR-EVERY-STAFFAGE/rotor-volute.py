"""🌊 Forms the fixed suction plate with a central opening and opposite wings into the line ports."""
import json,math
from pathlib import Path
t=Path(__file__).resolve().parent;p=t/'🗑️generated'/'rotor'/'circulation-pump.json';a=json.loads(p.read_bytes());volumes=a['asset']['volumes']
for den,n in [(50,64),(100,48),(200,32),(500,24)]:
    v=volumes[str(den)];verts=[];tri=[]
    for r in [.0105,.0314,.0375,.046]:
        u=max(0,(r-.0314)/(.046-.0314))
        for z in [.03475,.03625]:
            for i in range(n):
                theta=i*math.tau/n;c=math.cos(theta);s=math.sin(theta)
                verts.append([round(.104+r*c+.0015*u,7),round(r*s-.003*u,7),round(z+.0159*u*c,7)])
    def face(a,b,c,d):tri.extend([[a,b,c],[a,c,d]])
    for i in range(n):
        j=(i+1)%n
        for k in range(3):
            a0=2*k*n; b0=a0+2*n
            face(a0+i,a0+j,b0+j,b0+i);face(a0+n+i,b0+n+i,b0+n+j,a0+n+j)
        face(i,n+i,n+j,j);face(6*n+i,6*n+j,7*n+j,7*n+i)
    assert not any(q['id']=='suction-divider'for q in v['body'])
    v['body'].append({'id':'suction-divider','kind':'polyhedron','material':'cast-iron','role':'body','bone':'housing','tone':.37,'vertices':verts,'triangles':tri})
    v['decisions'][0]['en']+=' A fixed, pierced suction plate bends down at the left inlet and up at the right outlet. Its unperforated land separates the two chambers outside the eye.'
    v['decisions'][0]['de']+=' Eine feststehende Saugplatte mit Mittelöffnung fällt am linken Zulauf ab und steigt am rechten Ablauf an. Außerhalb des Saugauges trennt die geschlossene Fläche die beiden Kammern.'
p.write_text(json.dumps(a,ensure_ascii=False))
