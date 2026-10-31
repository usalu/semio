import sys,json; from lib import *
for a in sys.argv[1:]:
    A=json.load(open(out_path(a)))['asset']; D=A['dimensions']; dp=next(p for p in A['poses'] if p['id']==A['presentation']['defaultPose'])
    print(a,D)
    for k in ("500","200","100","50"):
        x0,x1,y0,y1=bbox(A,k,dp['bones'],pose_root(A,dp))
        print("  ",k,"w",round(x1-x0,3),"h",round(y1-y0,3),"ymin",round(y0,3))
    # lowest point per pose at each scale
    for k in ("500","100"):
        worst=min(((bbox(A,k,p['bones'],pose_root(A,p))[2],p['id']) for p in A['poses'] if p['contacts']),key=lambda t:t[0])
        print("  ground",k,"worst",worst)
