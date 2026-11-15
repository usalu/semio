import json,math,os,sys
T=os.path.dirname(os.path.abspath(__file__))
def r5(x):
    if isinstance(x,float): return round(x,5)
    if isinstance(x,list): return [r5(i) for i in x]
    if isinstance(x,dict): return {k:r5(v) for k,v in x.items()}
    return x
def base(id,bone,mat,tone,role,kw):
    d={"id":id,"bone":bone,"material":mat,"role":role,"tone":tone}
    d.update(kw); return r5(d)
def put(d,center=None,rot=None,scale=None):
    t={}
    if center is not None and any(center): t["translation"]=list(center)
    if rot is not None: t["rotation"]=list(rot)
    if scale is not None: t["scale"]=list(scale)
    if t: d["transform"]=t
    return d
def box(id,bone,mat,tone,size,c=(0,0,0),r=None,role="body",rot=None,**kw):
    d={"kind":"box","size":list(size)}
    if r: d["cornerRadius"]=r
    d.update(kw); return put(base(id,bone,mat,tone,role,d),c,rot)
def ell(id,bone,mat,tone,radii,c=(0,0,0),role="body",rot=None,**kw):
    d={"kind":"ellipsoid","radii":list(radii)}; d.update(kw); return put(base(id,bone,mat,tone,role,d),c,rot)
def cap(id,bone,mat,tone,p0,p1,r0,r1=None,role="body",lat=None,**kw):
    d={"kind":"capsule","p0":list(p0),"p1":list(p1),"r0":r0,"r1":r0 if r1 is None else r1}
    if lat: d["lateralScale"]=lat
    d.update(kw); return base(id,bone,mat,tone,role,d)
def rev(id,bone,mat,tone,profile,axis=(0,1,0),origin=(0,0,0),role="body",lat=None,sweep=None,**kw):
    d={"kind":"revolve","profile":[list(p) for p in profile],"axis":list(axis),"origin":list(origin)}
    if lat: d["lateralScale"]=lat
    if sweep: d["sweepRad"]=sweep
    d.update(kw); return base(id,bone,mat,tone,role,d)
def ext(id,bone,mat,tone,profile,plane,a,b,holes=None,taper=None,c=(0,0,0),role="body",rot=None,**kw):
    d={"kind":"extrude","profile":[list(p) for p in profile],"plane":plane,"from":a,"to":b}
    if holes: d["holes"]=[[list(p) for p in h] for h in holes]
    if taper: d["taperTo"]=list(taper)
    d.update(kw); return put(base(id,bone,mat,tone,role,d),c,rot)
def loft(id,bone,mat,tone,sections,role="body",caps=True,c=(0,0,0),**kw):
    d={"kind":"loft","sections":[dict(at=list(s[0]),rx=s[1],ry=s[2],**({"exponent":s[3]} if len(s)>3 else {}),**({"rotation":list(s[4])} if len(s)>4 else {})) for s in sections]}
    if caps: d["capStart"]=True; d["capEnd"]=True
    d.update(kw); return put(base(id,bone,mat,tone,role,d),c)
def th(solid,region,layer=None,id=None,**kw):
    d=dict(solid); d["id"]=id or "th-"+solid["id"]; d["region"]=region
    if layer: d["layer"]=layer
    d.update(kw); return d
def ref(id,of,region,layer=None):
    d={"id":id,"of":of,"region":region}
    if layer: d["layer"]=layer
    return d
def st(solid,member,part,id=None,**kw):
    d=dict(solid); d.update(kw); d["id"]=id or ("%s-%s"%(part,solid["id"])); d["member"]=member; d["part"]=part; return d
def sref(id,of,member,part):
    return {"id":id,"of":of,"member":member,"part":part}
def circle(r,n=24,a0=0.0):
    return [(r*math.cos(a0+2*math.pi*i/n),r*math.sin(a0+2*math.pi*i/n)) for i in range(n)]
def rrect(w,h,r,n=4,c=(0,0)):
    pts=[]
    for (sx,sy,a0) in ((1,1,0),(-1,1,90),(-1,-1,180),(1,-1,270)):
        cx=sx*(w/2-r); cy=sy*(h/2-r)
        for i in range(n+1):
            a=math.radians(a0+90*i/n); pts.append((c[0]+cx+r*math.cos(a),c[1]+cy+r*math.sin(a)))
    return pts
def shell(outer,t):
    return outer
def vol(denom,body,thermal,structure,en,de):
    return {"status":"authored","denominator":denom,"body":body,"thermal":thermal,"structure":structure,"decisions":[{"en":en,"de":de}] if isinstance(en,str) else [{"en":e,"de":d} for e,d in zip(en,de)]}
def unavail(d): return {"status":"unavailable","denominator":d}
ANCHORS={"pendant-lamp":"ceiling-hook","gas-boiler":"wall-bracket","gas-hob":"base-contact","hot-water-cylinder":"foot-left","kitchen-extractor-hood":"wall-hook","laptop":"feet-contact","mvhr-unit":"wall-hook"}
MULTI={"gas-meter":["wall-bracket-top","wall-bracket-bottom"],"hot-water-cylinder":["foot-left","foot-right"]}
SLENDER={"pendant-lamp":("cord","th-cord","member-cord")}
def post(slug,volumes):
    for v in volumes.values():
        if v.get("status")!="authored": continue
        for l in ("body","thermal","structure"):
            for s in v[l]:
                if l=="structure" and s.get("part")=="support" and "of" not in s and "anchor" not in s: s["anchor"]=ANCHORS[slug]
                if s["id"] in SLENDER.get(slug,()) or (slug,s["id"]) in SLENDER.get("pairs",()): s["slender"]=True
def write(slug,volumes):
    post(slug,volumes)
    p=os.path.join(T,"volumes","appliances",slug+".json")
    json.dump({"volumes":volumes},open(p,"w"),ensure_ascii=False,indent=1)
    print("wrote",p)
def seed(slug,scale="50"):
    return json.load(open(os.path.join(T,"author-12-seed",slug+".json")))["volumes"][scale]
def bez(p0,p1,p2,p3,n=12):
    out=[]
    for i in range(n+1):
        t=i/n; m=1-t
        out.append((m**3*p0[0]+3*m*m*t*p1[0]+3*m*t*t*p2[0]+t**3*p3[0], m**3*p0[1]+3*m*m*t*p1[1]+3*m*t*t*p2[1]+t**3*p3[1]))
    return out
def offset(pts,d,closed=False):
    out=[]
    n=len(pts)
    for i,p in enumerate(pts):
        a=pts[max(i-1,0)]; b=pts[min(i+1,n-1)]
        tx,ty=b[0]-a[0],b[1]-a[1]; L=math.hypot(tx,ty) or 1
        out.append((p[0]+d*ty/L*-1, p[1]+d*tx/L))
    return out
def shellprof(outer,wall):
    inner=offset(outer,wall)
    return list(outer)+list(reversed(inner))
def qaxis(ax,deg):
    a=math.radians(deg)/2; n=math.sqrt(sum(c*c for c in ax)); s=math.sin(a)/n
    return [ax[0]*s,ax[1]*s,ax[2]*s,math.cos(a)]
def qmul(a,b):
    x1,y1,z1,w1=a;x2,y2,z2,w2=b
    return [w1*x2+x1*w2+y1*z2-z1*y2,w1*y2-x1*z2+y1*w2+z1*x2,w1*z2+x1*y2-y1*x2+z1*w2,w1*w2-x1*x2-y1*y2-z1*z2]
def skin(id,bone,mat,tone,prof,c=(0,0,0),rot=None,role="body",exp=2,**kw):
    secs=[((0,a,0),r,r,exp) for (a,r) in prof]
    return loft(id,bone,mat,tone,secs,role=role,caps=False,c=c,**kw) if rot is None else rotated(loft(id,bone,mat,tone,secs,role=role,caps=False,c=c,**kw),rot)
def rotated(d,rot):
    d.setdefault("transform",{})["rotation"]=list(rot); return d
def cyl(id,bone,mat,tone,r,a0,a1,c=(0,0),axis="y",role="body",n=24,**kw):
    if axis=="y":
        return ext(id,bone,mat,tone,[(p[1]+c[1],p[0]+c[0]) for p in circle(r,n)],"zx",a0,a1,role=role,**kw)
    if axis=="x":
        return ext(id,bone,mat,tone,[(p[0]+c[0],p[1]+c[1]) for p in circle(r,n)],"yz",a0,a1,role=role,**kw)
    return ext(id,bone,mat,tone,[(p[0]+c[0],p[1]+c[1]) for p in circle(r,n)],"xy",a0,a1,role=role,**kw)
QX=qaxis((1,0,0),-90)
def skinz(id,bone,mat,tone,prof,c=(0,0,0),**kw):
    return skin(id,bone,mat,tone,prof,c=c,rot=QX,**kw)
def tube(id,bone,mat,tone,rout,rin,a0,a1,c=(0,0),axis="y",role="body",n=24,**kw):
    d=cyl(id,bone,mat,tone,rout,a0,a1,c,axis,role,n,**kw)
    h=[(p[0],p[1]) for p in ([(q[0],q[1]) for q in circle(rin,n)])]
    if axis=="y": d["holes"]=[[[p[1]+c[1],p[0]+c[0]] for p in circle(rin,n)]]
    elif axis=="x": d["holes"]=[[[p[0]+c[0],p[1]+c[1]] for p in circle(rin,n)]]
    else: d["holes"]=[[[p[0]+c[0],p[1]+c[1]] for p in circle(rin,n)]]
    return r5(d)
def rrbox_prof(w,h,r,c=(0,0),n=3):
    return rrect(w,h,r,n,c)
def setrole(solids,role,*ids):
    for s in solids:
        if s["id"] in ids: s["role"]=role
def drop(solids,*ids):
    return [s for s in solids if s["id"] not in ids]
def get(solids,id):
    return json.loads(json.dumps(next(s for s in solids if s["id"]==id)))
def rtube(id,bone,mat,tone,w,d,t,y0,y1,c=(0,0),r=0.002):
    o=[(p[0]+c[0],p[1]+c[1]) for p in rrect(w,d,r,2)]
    i=[(p[0]+c[0],p[1]+c[1]) for p in rrect(w-2*t,d-2*t,max(r-t,0.0005),2)]
    return ext(id,bone,mat,tone,[(q[1],q[0]) for q in o],"zx",y0,y1,holes=[[(q[1],q[0]) for q in i]])
def sec(solid,faces):
    solid["sectionFaces"]=faces; return solid
def inset(poly,d):
    n=len(poly); lines=[]
    area=sum(poly[i][0]*poly[(i+1)%n][1]-poly[(i+1)%n][0]*poly[i][1] for i in range(n))/2
    sgn=1 if area>0 else -1
    for i in range(n):
        a=poly[i]; b=poly[(i+1)%n]
        dx,dy=b[0]-a[0],b[1]-a[1]; L=math.hypot(dx,dy)
        nx,ny=-dy/L*sgn,dx/L*sgn
        lines.append(((a[0]+nx*d,a[1]+ny*d),(dx,dy)))
    out=[]
    for i in range(n):
        (p,u)=lines[i-1]; (q,v)=lines[i]
        det=u[0]*(-v[1])-u[1]*(-v[0])
        t=((q[0]-p[0])*(-v[1])-(q[1]-p[1])*(-v[0]))/det
        out.append((p[0]+u[0]*t,p[1]+u[1]*t))
    return out
