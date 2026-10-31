import sys, json; sys.path.insert(0,'.')
from lib import *
def pts_of(segs):
    out=[]; cur=None
    for c,ps in segs:
        if c in 'ML': out.append(ps[0]); cur=ps[0]
        elif c=='C':
            p0=cur
            for i in range(1,21):
                t=i/20; a=(1-t)**3; b=3*(1-t)**2*t; cc=3*(1-t)*t*t; dd=t**3
                out.append((a*p0[0]+b*ps[0][0]+cc*ps[1][0]+dd*ps[2][0], a*p0[1]+b*ps[0][1]+cc*ps[1][1]+dd*ps[2][1]))
            cur=ps[2]
    return out
def bbox(shapes,world,skip=()):
    xs=[];ys=[]
    for s in shapes:
        if s['role']=='ground' or s['id'] in skip: continue
        m=world[s['bone']]; pw=s.get('penMm')
        for x,y in pts_of(shape_path_local(s)):
            X,Y=ap(m,x,y); xs.append(X); ys.append(Y)
    return min(xs),min(ys),max(xs),max(ys)
if __name__=='__main__':
    p="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"+sys.argv[1]+"/🔣️.json"
    d=json.load(open(p))['asset']; w=pose_world(d); dm=d['dimensions']
    print('dims',dm)
    for k,r in d['representations'].items():
        if r['status']!='authored': print(k,'unavailable'); continue
        x0,y0,x1,y1=bbox(r['geometry']['elevation'],w)
        print(k,f"w={x1-x0:.3f} h={y1-y0:.3f} x[{x0:.3f},{x1:.3f}] y[{y0:.3f},{y1:.3f}]  dW={x1-x0-dm['widthM']:+.3f} dH={y1-y0-dm['heightM']:+.3f}")
