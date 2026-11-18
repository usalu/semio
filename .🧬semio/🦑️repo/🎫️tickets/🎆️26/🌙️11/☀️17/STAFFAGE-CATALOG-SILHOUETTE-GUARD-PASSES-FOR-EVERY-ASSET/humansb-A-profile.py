"""📏 Radius profile of a revolved front/plan shape against the volume hull. python3 humansb-A-profile.py <hull-json-file> <shape-points-json> <origin_y> (u->y = u+origin)"""
import json,sys
hull=json.load(open(sys.argv[1])); pts=json.load(open(sys.argv[2])); o0=float(sys.argv[3])
def r_at(y,side):
    best=None
    n=len(hull)
    for i in range(n):
        a,b=hull[i],hull[(i+1)%n]
        if (a[1]-y)*(b[1]-y)<=0 and a[1]!=b[1]:
            x=a[0]+(b[0]-a[0])*(y-a[1])/(b[1]-a[1])
            if side*x>0: best=x if best is None else (max(best,x) if side>0 else min(best,x))
    return best
for u,r in pts:
    y=u+o0
    ra,rb=r_at(y,1),r_at(y,-1)
    print(f"u {u:.4f} y {y:.4f} r {r:.4f} vol {ra} {rb}")
