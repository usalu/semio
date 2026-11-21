"""🦆 Express the mallard's primary flight feathers in the carpometacarpal frame at their actual thickness."""
import json
import math
import runpy
from pathlib import Path
ticket=Path(__file__).parent
edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
f=json.loads((ticket/'living-duck-wings.json').read_text())
def sub(a,b):return [x-y for x,y in zip(a,b)]
def unit(v):return [x/math.hypot(*v) for x in v]
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def frame(v):
    u=unit(v);a=math.atan2(u[1],u[0]);b=math.asin(u[2])
    return [u,[-math.sin(a),math.cos(a),0],[-math.cos(a)*math.sin(b),-math.sin(a)*math.sin(b),math.cos(b)]]
def multiply(q,r):
    x,y,z,w=q;a,b,c,d=r
    return [w*a+x*d+y*c-z*b,w*b-x*c+y*d+z*a,w*c+x*b-y*a+z*d,w*d-x*a-y*b-z*c]
def conjugate(q):return [-q[0],-q[1],-q[2],q[3]]
def frameQuat(v):
    a=math.atan2(v[1],v[0])/2;b=math.asin(unit(v)[2])/2
    return multiply([0,0,math.sin(a),math.cos(a)],[0,-math.sin(b),0,math.cos(b)])
def bind(k,m,s):
    if s.get('kind')!='loft' or s.get('bone') not in ('wing-n','wing-f') or not s.get('weights') or 'primaries' not in s['id']:return False
    side=s['bone'][-1];sign=-1 if side=='n' else 1
    elbow,wrist,tip=([x,y,z*sign] for x,y,z in (f['shoulderFrameElbow'],f['shoulderFrameWrist'],f['shoulderFrameManusTip']))
    fore=sub(wrist,elbow);ff=frame(fore)
    hand=[dot(axis,sub(tip,wrist)) for axis in ff]
    inv=conjugate(multiply(frameQuat(fore),frameQuat(hand)))
    for row in s['sections']:
        p=multiply(multiply(inv,[*sub(row['at'],wrist),0]),conjugate(inv))
        row['at']=[round(x,10) for x in p[:3]]
        if 'rotation' in row:row['rotation']=[round(x,10) for x in multiply(inv,row['rotation'])]
    s['bone']='hand-'+side;del s['weights'];return True
for p in (next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/mallard-duck/*.json')),ticket/'volumes/animals/mallard-duck.json'):edit(p,bind)
