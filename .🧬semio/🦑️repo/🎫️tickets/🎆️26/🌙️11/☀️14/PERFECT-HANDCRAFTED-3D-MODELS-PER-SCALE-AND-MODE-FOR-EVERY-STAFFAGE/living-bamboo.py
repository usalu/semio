"""🎋 Preserve the individual geometry of bamboo blades and petioles at architectural scale."""
import json
import math
from pathlib import Path

def sub(a,b): return [x-y for x,y in zip(a,b)]
def add(a,b): return [x+y for x,y in zip(a,b)]
def mul(a,f): return [x*f for x in a]
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def unit(a): return mul(a,1/math.hypot(*a))
def cross(a,b):
    x,y,z=a
    u,v,w=b
    return [y*w-z*v,z*u-x*w,x*v-y*u]

def quaternion(x,y,z):
    m=[[x[i],y[i],z[i]] for i in range(3)]
    t=sum(m[i][i] for i in range(3))
    if t>0:
        v=math.sqrt(t+1)*2
        q=[(m[2][1]-m[1][2])/v,(m[0][2]-m[2][0])/v,(m[1][0]-m[0][1])/v,v/4]
    else:
        i=max(range(3),key=lambda i:m[i][i])
        j=(i+1)%3
        k=(i+2)%3
        v=math.sqrt(1+m[i][i]-m[j][j]-m[k][k])*2
        q=[0,0,0,(m[k][j]-m[j][k])/v]
        q[i]=v/4
        q[j]=(m[j][i]+m[i][j])/v
        q[k]=(m[k][i]+m[i][k])/v
    return [round(a,8) for a in unit(q)]

def transverse(q):
    x,y,z,w=q
    return [1-2*(y*y+z*z),2*(x*y+w*z),2*(x*z-w*y)]

def blade(s):
    rings=s['sections']
    origin=rings[0]['at']
    along=unit(sub(rings[-1]['at'],origin))
    peak=max(rings,key=lambda i:i['rx'])
    width=transverse(peak['rotation'])
    side=unit(sub(width,mul(along,dot(width,along))))
    normal=cross(along,side)
    halves=[[],[]]
    for r in rings:
        middle=sub(r['at'],origin)
        delta=mul(transverse(r['rotation']),r['rx'])
        halves[0].append(sub(middle,delta))
        halves[1].append(add(middle,delta))
    xz=[(dot(p,along),dot(p,normal)) for h in halves for p in h]
    def error(k):
        return max(abs(z-k*x) for x,z in xz)/math.hypot(1,k)
    lo,hi=-.25,.25
    for _ in range(24):
        a,b=(2*lo+hi)/3,(lo+2*hi)/3
        if error(a)<error(b): hi=b
        else: lo=a
    k=(lo+hi)/2
    x=unit(add(along,mul(normal,k)))
    z=cross(x,side)
    sections=[0,1,2,4,6,7]
    edge=[halves[0][i] for i in sections]+[halves[1][i] for i in reversed(sections)]
    profile=[[round(dot(p,x),7),round(dot(p,side),7)] for p in edge]
    keys={a:b for a,b in s.items() if a not in ('kind','sections','capStart','capEnd')}
    return error(k),{**keys,'kind':'extrude','plane':'xy','from':-0.0004,'to':0.0004,'profile':profile,'transform':{'translation':origin,'rotation':quaternion(x,side,z)}}

p=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*vegetation/bamboo-clump/*.json'))
raw=p.read_text()
a=json.loads(raw)['asset']
volume=a['volumes']['50']
body=volume['body']
laminae=[s for s in body if s['material']=='bamboo-leaf' and s['role']=='texture' and len(s.get('sections',[]))==8]
if laminae:
    candidates={s['id']:blade(s) for s in laminae}
    keep=set(sorted(candidates,key=lambda k:candidates[k][0],reverse=True)[:int(len(candidates)*.21)])
    projected=[]
    for i,s in enumerate(body):
        if s['id'] in candidates:
            dev,sheet=candidates[s['id']]
            if s['id'] in keep:
                s['sections']=[s['sections'][j] for j in (0,1,2,4,6,7)]
            else:
                body[i]=sheet
                projected.append(dev)
        elif s['kind']=='capsule' and s['material']=='bamboo-culm' and s['r0']==.0009 and s['r1']==.0007:
            common={k:v for k,v in s.items() if k not in ('p0','p1','r0','r1','kind')}
            body[i]={**common,'kind':'loft','sections':[{'at':s['p0'],'rx':s['r0'],'ry':s['r0']},{'at':s['p1'],'rx':s['r1'],'ry':s['r1']}],'capStart':True,'capEnd':True}
    loc=raw.index('"volumes":')
    start=raw.index('{',raw.index('"50":',loc))
    _,n=json.JSONDecoder().raw_decode(raw[start:])
    if p.read_text()!=raw:raise SystemExit('Bamboo changed concurrently')
    p.write_text(raw[:start]+json.dumps(volume,ensure_ascii=False,indent=2).replace('\n','\n      ')+raw[start+n:])
    print('[DEBUG] planar',len(projected),'max edge residual mm',round(max(projected)*1000,3),'spatial',len(keep))

