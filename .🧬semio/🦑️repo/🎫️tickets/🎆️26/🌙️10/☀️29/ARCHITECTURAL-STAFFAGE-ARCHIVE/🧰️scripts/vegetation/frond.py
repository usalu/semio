import json,math,re,subprocess,sys
# usage: frond.py <id> <fr-boneprefix> <region> <material> <z> : emits blade, rachis and leaflet shapes for every bone whose id starts with the prefix (rest-pose positions from world.ts)
i,prefix,region,material,z=sys.argv[1:6]; z=float(z)
out=subprocess.check_output(["repo","tools","exec","bun","--","world.ts",f"design/{i}-a.json"],text=True)
rows={m[1]:(float(m[2]),float(m[3]),float(m[4]),float(m[5])) for m in re.finditer(r"^(\S+)\s+o=\(([-\d.]+),([-\d.]+)\) tip=\(([-\d.]+),([-\d.]+)\)",out,re.M)}
shapes=[]
for b,(ox,oy,tx,ty) in rows.items():
    if not b.startswith(prefix): continue
    dist=b.endswith("b"); L=math.hypot(tx-ox,ty-oy); ux,uy=(tx-ox)/L,(ty-oy)/L; nx,ny=-uy,ux; ang=math.degrees(math.atan2(uy,ux))
    shapes.append({"id":f"{b}-blade","bone":b,"role":"body","lod":0,"z":z,"filled":True,"tone":0.5,"kind":"taper","w":1,"p0":[ox,oy],"p1":[tx,ty],"w0":0.45 if dist else 0.1,"w1":0.03 if dist else 0.45,"wob":[1.5,1.3] if dist else [1.1,1.3],"round1":True,"region":region,"material":material})
    shapes.append({"id":f"{b}-rachis","bone":b,"role":"detail","lod":1,"z":z+0.05,"filled":False,"tone":0.2,"kind":"poly","w":1,"points":[[round(ox,3),round(oy,3)],[round(tx,3),round(ty,3)]],"closed":False})
    for lod,step,scale,tag in ((2,0.26,1.0,"leaflets"),(3,0.13,0.55,"leaflets-fine")):
        marks=[]
        s=0.3 if dist else 0.5
        while s<L-0.08:
            wmax=0.5 if dist else 0.3
            prof=wmax*min(1,(L-s)/(0.45*L))**0.8*min(1,s/0.5+0.35) if dist else wmax*min(1,s/L+0.4)
            for side in (1,-1):
                a=ang+side*50
                marks.append([round(ox+ux*s,3),round(oy+uy*s,3),round(a,1),round(max(0.1,prof*1.05*scale),2)])
            s+=step
        shapes.append({"id":f"{b}-{tag}","bone":b,"role":"texture","lod":lod,"z":z+0.06,"filled":False,"tone":0.25,"kind":"marks","w":1,"curl":0.04,"marks":marks})
print(json.dumps(shapes,separators=(",",":")))
