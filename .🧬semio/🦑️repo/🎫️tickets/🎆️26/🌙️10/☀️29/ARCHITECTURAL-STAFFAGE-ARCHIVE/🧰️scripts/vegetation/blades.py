import json,math,re,subprocess,sys
# usage: blades.py <id> : emits a fan of narrow blade pieces for every bl-?a / bl-?b bone pair (rest-pose positions from world.ts); lower pieces on the 'a' bone, arching pieces on the 'b' bone
i=sys.argv[1]
out=subprocess.check_output(["repo","tools","exec","bun","--","world.ts",f"design/{i}-a.json"],text=True)
W={m[1]:tuple(float(m[k]) for k in range(2,7)) for m in re.finditer(r"^(\S+)\s+o=\(([-\d.]+),([-\d.]+)\) tip=\(([-\d.]+),([-\d.]+)\) ang=([-\d.]+)",out,re.M)}
fan=[(-11,1.0),(-4,1.06),(4,0.95),(11,0.88)]; shapes=[]
for g in ("L1","L2","L3","R1","R2","R3"):
    a,b=W[f"bl-{g}a"],W[f"bl-{g}b"]; La=math.hypot(a[2]-a[0],a[3]-a[1]); Lb=math.hypot(b[2]-b[0],b[3]-b[1])
    for n,(off,f) in enumerate(fan):
        ang=math.radians(a[4]+off); p0=(a[0],a[1]); p1=(p0[0]+La*f*math.cos(ang),p0[1]+La*f*math.sin(ang))
        ang2=math.radians(b[4]+off*1.4); q1=(p1[0]+Lb*f*math.cos(ang2),p1[1]+Lb*f*math.sin(ang2))
        z=2.0+0.01*n
        shapes.append({"id":f"blade-{g}-{n+1}a","bone":f"bl-{g}a","role":"body","lod":0,"z":z,"filled":True,"tone":0.55,"kind":"taper","w":1,"p0":[round(c,3) for c in p0],"p1":[round(c,3) for c in p1],"w0":0.034,"w1":0.03,"bow":0.01,"region":"blades","material":"miscanthus-blade"})
        shapes.append({"id":f"blade-{g}-{n+1}b","bone":f"bl-{g}b","role":"body","lod":0,"z":z+0.001,"filled":True,"tone":0.55,"kind":"taper","w":1,"p0":[round(c,3) for c in p1],"p1":[round(c,3) for c in q1],"w0":0.03,"w1":0.004,"bow":-0.02,"round1":True,"region":"blades","material":"miscanthus-blade"})
    for lod,tag,step in ((2,"midribs",1),):
        pass
    shapes.append({"id":f"midrib-{g}","bone":f"bl-{g}b","role":"texture","lod":2,"z":2.1,"filled":False,"tone":0.2,"kind":"poly","w":1,"points":[[round(b[0],3),round(b[1],3)],[round(b[2],3),round(b[3],3)]],"closed":False})
print(json.dumps(shapes,separators=(",",":")))
