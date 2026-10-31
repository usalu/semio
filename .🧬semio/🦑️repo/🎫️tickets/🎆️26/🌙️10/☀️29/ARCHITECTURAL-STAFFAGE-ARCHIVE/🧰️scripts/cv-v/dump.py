import sys; sys.path.insert(0,'.')
from lib import *
a=sys.argv[1]; d=load_orig(a)["asset"]; r=Rig(d)
print("BONES id parent start end ang")
for b in d["rig"]["bones"]:
    s=r.to_world(b["id"],0,0); e=r.to_world(b["id"],b["lengthM"],0)
    print(f'{b["id"]:12s} {str(b["parent"]):12s} ({s[0]:.2f},{s[1]:.2f})->({e[0]:.2f},{e[1]:.2f}) a={r.angle(b["id"]):.0f} L={b["lengthM"]}')
print("BODY SHAPES bbox")
for s in d["geometry"]["elevation"]:
    if s["role"]!="body": continue
    w=shape_world_d(r,s); n=[float(x) for x in re.findall(r"-?\d+\.?\d*",w)]
    X=n[0::2];Y=n[1::2]
    print(f'{s["id"]:16s} {s["bone"]:12s} x[{min(X):.2f},{max(X):.2f}] y[{min(Y):.2f},{max(Y):.2f}] reg={s.get("region")}')
