"""🦵 Measures effective left and right joint-centre spans in the live rig and compares them with the authored segment lengths."""
import json,math
from pathlib import Path
t=Path(__file__).resolve().parent
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
print('| Figure | Upper arm | Forearm | Thigh | Shank | Largest difference between joint-centre distance and authored length |')
print('|---|---:|---:|---:|---:|---:|')
total=0;largest=0
for p in sorted(root.glob('*/*')):
    asset=json.loads(p.read_text())['asset'];bones={b['id']:b for b in asset['rig']['bones']};lengths=[];deviations=[]
    for primary,other,successor in [('upper-arm','arm','forearm'),('forearm','forearm','hand'),('thigh','thigh','shank'),('shank','shank','foot')]:
        pair=[]
        for s in ['l','r']:
            b=bones.get(primary+'-'+s,bones.get(other+'-'+s));nextJoint=bones.get(successor+'-'+s)
            if not b or not nextJoint or nextJoint['parent']!=b['id']:continue
            if asset['id']=='runner-with-blade-prosthesis' and b['id']=='shank-r':continue
            n=math.hypot(nextJoint['x'],nextJoint['y'],nextJoint.get('z',0));total+=1
            pair.append(round(n*1000));deviations.append(abs(n-b['lengthM'])*1000)
        assert pair and min(pair)>100
        lengths.append('/'.join(str(k) for k in pair) if len(set(pair))>1 else str(pair[0]))
    assert max(deviations)<5.1
    largest=max(largest,max(deviations))
    print('| '+p.parent.name+' | '+' | '.join(lengths)+f' | {max(deviations):.1f} |')
print(f'\nAcross {total} measured natural-limb segments the largest difference was {largest:.1f} mm. Longer three-dimensional spans reflect lateral placement of the distal joint. The right lower-leg prosthesis is excluded from the natural shank comparison.')
