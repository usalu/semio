import sys,json
from body import *
A=Asset(sys.argv[1]); vo=view_only(A)
print("DIM",A.data['dimensions'])
print("BONES:", " ".join(f"{b['id']}({b['parent']},{b['angleDeg']:.0f},{b['lengthM']:.3f}{',z'+str(b['z']) if 'z' in b else ''})" for b in A.data['rig']['bones']))
print("MAT",sorted(A.materials)); print("REG",sorted(A.regions))
print("POSES",[p['id'] for p in A.data['poses']][:12],"CLIPS",[c['id'] for c in A.data['clips']])
print("DENOMS",A.denoms)
def r(x):
    if isinstance(x,float): return round(x,3)
    if isinstance(x,list): return [r(i) for i in x]
    if isinstance(x,dict): return {k:r(v) for k,v in x.items()}
    return x
mx=int(sys.argv[2]) if len(sys.argv)>2 else 200
for sid,s in A.draft.items():
    hb=is_human_bone(s['bone'])
    tag=("VO " if sid in vo else "")
    if hb and convertible(s) and sid not in vo:
        ring=s['profile']; us=[p[0] for p in ring]; vs=[p[1] for p in ring]
        print(f"  [conv] {sid} {s['bone']} {s['material']} {s['role']} u[{min(us):.2f},{max(us):.2f}] v[{min(vs):.2f},{max(vs):.2f}] w[{s['from']:.3f},{s['to']:.3f}]"); continue
    g={k:r(v) for k,v in s.items() if k not in('id','bone','kind','material','role','tone','region','regions')}
    if s['kind']=='extrude':
        ring=s['profile']; a=[p[0] for p in ring]; b=[p[1] for p in ring]
        g={'plane':s['plane'],'a':[round(min(a),3),round(max(a),3)],'b':[round(min(b),3),round(max(b),3)],'c':[round(s['from'],3),round(s['to'],3)]}|({'tf':r(s['transform'])} if 'transform' in s else {})|({'rep':s['repeat']} if 'repeat' in s else {})|({'holes':len(s['holes'])} if 'holes' in s else {})|{'npts':len(ring)}
    print(f"{tag}{sid} {s['bone']} {s['kind']} {s['material']} {s['role']} {s['tone']} {s.get('region') or s.get('regions')} {json.dumps(g,separators=(',',':'))[:mx]}")
