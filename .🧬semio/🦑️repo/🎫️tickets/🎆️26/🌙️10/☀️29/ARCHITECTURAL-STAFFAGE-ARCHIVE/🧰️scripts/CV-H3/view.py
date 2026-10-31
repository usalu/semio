import sys,json; from lib import *
# usage: view.py asset pose scale pxmm [x0 x1 y0 y1] ; pose may be clip@t
a=sys.argv[1]; pose=sys.argv[2]; sc=sys.argv[3]; px=float(sys.argv[4])
A=json.load(open(out_path(a)))['asset']
crop=tuple(float(v) for v in sys.argv[5:9]) if len(sys.argv)>8 else None
p=next(p for p in A['poses'] if p['id']==pose)
tag=f"{a}-{pose}-{sc}-{int(px)}"
render(A,sc,p['bones'],pose_root(A,p),f"{GEN}/{tag}.png",pxmm=px,crop=crop)
print(f"{GEN}/{tag}.png")
