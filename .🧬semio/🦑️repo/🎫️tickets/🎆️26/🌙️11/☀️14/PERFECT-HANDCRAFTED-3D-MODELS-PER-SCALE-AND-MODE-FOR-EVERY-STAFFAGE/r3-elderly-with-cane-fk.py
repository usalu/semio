"""🦴 Forward kinematics of the elderly-with-cane rig per pose: joint positions in metres."""
import json,glob,math,sys
F=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/elderly-with-cane/🔣️.json")[0]
a=json.load(open(F))['asset']
bones={b['id']:b for b in a['rig']['bones']}
def fk(over):
    out={}
    def w(i):
        if i in out: return out[i]
        b=bones[i]; ang=over.get(i,{}).get('angleDeg',0)
        if b['parent'] is None:
            o=(b['x']+over.get(i,{}).get('dx',0),b['y']+over.get(i,{}).get('dy',0),b.get('z',0) or 0); A=b['angleDeg']+ang
        else:
            px,py,pz,pa=w(b['parent'])[:4]
            c,s=math.cos(math.radians(pa)),math.sin(math.radians(pa))
            o=(px+c*b['x']-s*b['y'],py+s*b['x']+c*b['y'],pz+(b.get('z',0) or 0)); A=pa+b['angleDeg']+ang
        e=(o[0]+math.cos(math.radians(A))*b['lengthM'],o[1]+math.sin(math.radians(A))*b['lengthM'])
        out[i]=(o[0],o[1],o[2],A,e[0],e[1]); return out[i]
    for i in bones: w(i)
    return out
for p in a['poses']:
    if len(sys.argv)>1 and p['id'] not in sys.argv[1:]: continue
    o=fk(p['bones']); miny=min(min(v[1],v[5]) for v in o.values())
    print('==',p['id'],'ground',p.get('ground'),'miny',round(miny,4),{k:v for k,v in p['bones'].items()})
    for k,v in o.items(): print(f"  {k:10s} o=({v[0]:+.3f},{v[1]:+.3f},{v[2]:+.3f}) ang={v[3]%360:7.1f} end=({v[4]:+.3f},{v[5]:+.3f})")
