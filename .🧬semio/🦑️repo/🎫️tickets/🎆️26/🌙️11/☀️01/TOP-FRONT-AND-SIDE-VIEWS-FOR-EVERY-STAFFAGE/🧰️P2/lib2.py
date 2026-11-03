from lib import *
K=0.5523
def rr(x0,y0,x1,y1,tl=0,tr=0,br=0,bl=0):
    """rounded rectangle path (y up): radii per corner top-left, top-right, bottom-right, bottom-left"""
    f=lambda v:("%.5f"%v).rstrip("0").rstrip(".")
    p=[]
    def pt(x,y): return f(x)+" "+f(y)
    p.append("M "+pt(x0+bl,y0))
    p.append("L "+pt(x1-br,y0))
    if br: p.append("C "+pt(x1-br+br*K,y0)+" "+pt(x1,y0+br-br*K)+" "+pt(x1,y0+br))
    p.append("L "+pt(x1,y1-tr))
    if tr: p.append("C "+pt(x1,y1-tr+tr*K)+" "+pt(x1-tr+tr*K,y1)+" "+pt(x1-tr,y1))
    p.append("L "+pt(x0+tl,y1))
    if tl: p.append("C "+pt(x0+tl-tl*K,y1)+" "+pt(x0,y1-tl+tl*K)+" "+pt(x0,y1-tl))
    p.append("L "+pt(x0,y0+bl))
    if bl: p.append("C "+pt(x0,y0+bl-bl*K)+" "+pt(x0+bl-bl*K,y0)+" "+pt(x0+bl,y0))
    p.append("Z")
    return " ".join(p)
def pth(id,bone,d,**kw): return shp(id,bone,"path",d=d,**kw)
def add_bone(asset,id,parent,x,y,angle,length,en,de):
    bones=asset["rig"]["bones"]
    if any(b["id"]==id for b in bones): return
    b={"id":id,"parent":parent,"title":{"en":en,"de":de},"x":x,"y":y,"angleDeg":angle,"lengthM":length,"joint":{"type":"fixed"}}
    i=max(k for k,bb in enumerate(bones) if bb["id"]==parent)
    bones.insert(len(bones),b)
def rise(asset,bone,eps,en_name,de_name):
    """helper bone 'bone-rise': a fixed child at the origin of `bone`, inclined by eps degrees at rest, so a front rect bound to it with followTip stretches with the tip height of `bone`"""
    b=[bb for bb in asset["rig"]["bones"] if bb["id"]==bone][0]
    add_bone(asset,bone+"-rise",bone,0,0,eps,b["lengthM"],"Front-view height reference of the "+en_name,"Höhenbezug der Vorderansicht für "+de_name)
    return math.sin(math.radians(eps))*b["lengthM"]
def apply2(a,builder,hook=None):
    doc=read_doc(a); asset=doc["asset"]
    if hook: hook(asset)
    asset["presentation"].pop("viewpoints",None)
    for sc,rep in asset["representations"].items():
        if rep["status"]!="authored": continue
        front,plan,dec=builder(int(sc),asset)
        g=rep["geometry"]
        rep["geometry"]={"elevation":g["elevation"],"plan":plan,"front":front}
        for k,(en,de) in dec.items():
            t=rep["decisions"][k]
            if en not in t["en"]: t["en"]=t["en"].rstrip()+" "+en
            if de not in t["de"]: t["de"]=t["de"].rstrip()+" "+de
    write_doc(a,doc)
    print("applied",a)
def hose_samples(n):
    """samples (x,y) in the elbow frame of the shower hose along its three cubic segments"""
    seg=[((0.01,-0.004),(0.07,-0.01),(0.22,-0.14),(0.2,-0.26)),((0.2,-0.26),(0.18,-0.38),(0.07,-0.36),(0.09,-0.22)),((0.09,-0.22),(0.11,-0.06),(0.14,0.26),(0.02196,0.51843))]
    out=[]
    for si,(p0,p1,p2,p3) in enumerate(seg):
        for i in range(n+ (1 if si==2 else 0)):
            t=i/n; m=1-t
            out.append((m**3*p0[0]+3*m*m*t*p1[0]+3*m*t*t*p2[0]+t**3*p3[0], m**3*p0[1]+3*m*m*t*p1[1]+3*m*t*t*p2[1]+t**3*p3[1]))
    return out
def drop_rise(asset):
    asset["rig"]["bones"]=[b for b in asset["rig"]["bones"] if not b["id"].endswith("-rise")]
def ext(asset,sc,eid,z0,z1,id=None,**over):
    """a front or plan shape that extrudes the elevation profile `eid` (bone-local, full bone transform) between the lateral planes z0..z1"""
    el={s["id"]:s for s in asset["representations"][str(sc)]["geometry"]["elevation"]}
    s=copy.deepcopy(el[eid])
    if id: s["id"]=id
    s["solid"]={"kind":"extruded","z0":rnd(z0),"z1":rnd(z1)}
    s.update(over)
    return s
