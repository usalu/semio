"""🎍 Shape reed branchlets and keeled blades with measured residuals along their authored margins."""
import math
import runpy
import statistics
from pathlib import Path

ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
add,sub,mul,dot,mesh=(b[k] for k in ('add','sub','mul','dot','mesh'))
errors={}

def mean(row):return [sum(v[i] for v in row)/len(row) for i in range(3)]
def residual(rings,j):
    a,p,c=rings[j-1:j+2];start,end=mean(a),mean(c);direction=sub(end,start)
    t=min(1,max(0,dot(sub(mean(p),start),direction)/max(1e-20,dot(direction,direction))))
    return max(math.dist(point,add(mul(a[i%len(a)],1-t),mul(c[i%len(c)],t))) for i,point in enumerate(p))

def author(scale,mode,s):
    if s.get('kind')!='polyhedron' or mode!='body':return False
    v=s['vertices'];mat=s['material'];role=s['role']
    if scale=='50' and mat=='bamboo-leaf' and role=='texture' and len(v)==14:
        rings=[[v[0]],v[1:5],v[5:9],v[9:13],[v[-1]]]
        widest=max(range(1,4),key=lambda i:math.dist(rings[i][0],rings[i][2]))
        err,j=min((residual(rings,j),j) for j in (1,2,3) if j!=widest)
        if err>.0025:return False
        del rings[j]
        errors.setdefault('blade',[]).append(err)
    elif scale=='50' and mat=='bamboo-culm' and role=='detail' and len(v) in (14,86):
        n=6 if len(v)==14 else 12
        rings=[v[i:i+n:2] for i in range(0,len(v)-2,n)]
    elif scale=='100' and mat=='bamboo-leaf' and role=='body' and len(v) in (50,58):
        rings=[v[i:i+8:2] for i in range(0,len(v)-2,8)]
        widest=max(range(len(rings)),key=lambda i:math.dist(rings[i][0],rings[i][2]))
        err,j=min((residual(rings,j),j) for j in range(1,len(rings)-1) if j!=widest)
        if err<=.009:
            del rings[j]
            errors.setdefault('spray',[]).append(err)
    else:return False
    mesh(s,rings)
    return True

for p in (next(b['root'].glob('*/bamboo-clump/*.json')),ticket/'volumes/vegetation/bamboo-clump.json'):
    errors.clear();edit(p,author)
    for kind,e in errors.items():print('[DEBUG]',kind,len(e),round(1000*max(e),3),round(1000*statistics.median(e),3))

