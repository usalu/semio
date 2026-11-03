import json,re,math
from decimal import Decimal
from dump import S,load,world
ASSETS="hot-water-cylinder kitchen-extractor-hood laptop mvhr-unit pendant-lamp room-thermostat server-rack shower-head thermostatic-radiator-valve toaster vacuum-cleaner split-ac-indoor-unit".split()
def path_of(a): return S+a+"/🔣️.json"
def _fix(m):
    mant,exp=m.group(1),int(m.group(2))
    if exp>6: return m.group(0)
    d=Decimal(mant).scaleb(-exp)
    return format(d,"f")
def dumps(doc,indent=2):
    s=json.dumps(doc,ensure_ascii=False,indent=indent)
    s=re.sub(r'(?<![\w.])(-?\d+(?:\.\d+)?)e-0?(\d+)(?![\w.])',_fix,s)
    return s+"\n"
def read_doc(a): return json.load(open(path_of(a),encoding="utf8"))
def indent_of(a):
    t=open(path_of(a),encoding="utf8").read(200)
    return 1 if t.startswith('{\n "') else 2
def write_doc(a,doc):
    import os,tempfile
    path=path_of(a); folder=os.path.dirname(path)
    fd,tmp=tempfile.mkstemp(dir=folder,suffix=".tmp")
    try:
        with os.fdopen(fd,"w",encoding="utf8",newline="\n") as f: f.write(dumps(doc,indent_of(a)))
        os.replace(tmp,path)
    except BaseException:
        if os.path.exists(tmp): os.remove(tmp)
        raise
def rnd(v):
    return round(v,5)+0.0
# shape helpers (ids, bone, role, z, tone, ... ) -> dict in the file's key order
def shp(id,bone,kind,role="body",z=1,tone=0.5,filled=True,region=None,regions=None,material=None,**k):
    o={"id":id,"bone":bone,"role":role,"z":z,"filled":filled,"tone":tone,"kind":kind}
    if kind=="rect": o.update({"x":rnd(k["x"]),"y":rnd(k["y"]),"w":rnd(k["w"]),"h":rnd(k["h"])}); 
    if kind=="rect" and k.get("r"): o["r"]=rnd(k["r"])
    if kind=="poly":
        o["points"]=[[rnd(x),rnd(y)] for x,y in k["points"]]; o["closed"]=k.get("closed",True)
    if kind=="path": o["d"]=k["d"]
    if kind=="capsule":
        o.update({"x0":rnd(k["x0"]),"y0":rnd(k["y0"]),"x1":rnd(k["x1"]),"y1":rnd(k["y1"]),"r0":rnd(k["r0"]),"r1":rnd(k.get("r1",k["r0"]))})
    if kind=="ellipse":
        o.update({kk:rnd(k[kk]) for kk in ("cx","cy","rx","ry")})
    if k.get("followTip"): o["followTip"]=True
    if region: o["region"]=region
    if regions: o["regions"]=regions
    if material: o["material"]=material
    return o
def circle(cx,cy,r,n=24,a0=0):
    return [(cx+r*math.cos(a0+2*math.pi*i/n),cy+r*math.sin(a0+2*math.pi*i/n)) for i in range(n)]
def ring(cx,cy,r,n=24,a0=0.0): return circle(cx,cy,r,n,a0)
def ellipse_pts(cx,cy,rx,ry,n=24,a0=0.0): return [(cx+rx*math.cos(a0+2*math.pi*i/n),cy+ry*math.sin(a0+2*math.pi*i/n)) for i in range(n)]
def arc_pts(cx,cy,rx,ry,a0,a1,n=12): return [(cx+rx*math.cos(a0+(a1-a0)*i/n),cy+ry*math.sin(a0+(a1-a0)*i/n)) for i in range(n+1)]
def apply(a,builder):
    """builder(scale)->(front,plan,decisions) ; decisions = {key:(en,de)} appended. Replaces front/plan, removes viewpoints."""
    doc=read_doc(a); asset=doc["asset"]
    asset["presentation"].pop("viewpoints",None)
    for sc,rep in asset["representations"].items():
        if rep["status"]!="authored": continue
        front,plan,dec=builder(int(sc),asset)
        g=rep["geometry"]
        rep["geometry"]={"elevation":g["elevation"],"plan":plan,"front":front} if list(g.keys())[:2]==["elevation","plan"] else {**{k:v for k,v in g.items() if k not in("plan","front")},"plan":plan,"front":front}
        for k,(en,de) in dec.items():
            t=rep["decisions"][k]
            if en not in t["en"]: t["en"]=t["en"].rstrip()+" "+en
            if de not in t["de"]: t["de"]=t["de"].rstrip()+" "+de
    write_doc(a,doc)
    print("applied",a)
def rc(id,bone,x0,y0,x1,y1,**kw):
    return shp(id,bone,"rect",x=min(x0,x1),y=min(y0,y1),w=abs(x1-x0),h=abs(y1-y0),**kw)
def pl(id,bone,pts,**kw):
    return shp(id,bone,"poly",points=pts,**kw)
def line(id,bone,pts,**kw):
    kw.setdefault("role","detail"); kw.setdefault("tone",0.4); kw.setdefault("z",3.5)
    return shp(id,bone,"poly",points=pts,filled=False,closed=False,**kw)
def cap(id,bone,x0,y0,x1,y1,r0,r1=None,**kw):
    return shp(id,bone,"capsule",x0=x0,y0=y0,x1=x1,y1=y1,r0=r0,r1=r1 if r1 is not None else r0,**kw)
import copy
def shift_path(d,dx,dy):
    toks=re.findall(r"[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?",d); out=[];i=0;cmd=None;n=0
    for t in toks:
        if t.isalpha(): cmd=t; n=0; out.append(t); continue
        v=float(t)
        if cmd in "MLC": v=v+(dx if n%2==0 else dy)
        elif cmd=="H": v+=dx
        elif cmd=="V": v+=dy
        else: raise ValueError("path command "+str(cmd))
        n+=1; out.append(("%.5f"%v).rstrip("0").rstrip("."))
    return " ".join(out)
def to_world(asset,shape,dx=0.0,dy=0.0,mirror_x=False):
    """copy of an elevation shape moved by its bone's rest translation (bones at angle 0) -> usable as a front/plan shape in model metres"""
    W=world(asset); bx,by,ba=W[shape["bone"]]
    assert abs(ba)<1e-9
    s=copy.deepcopy(shape); ox=bx+dx; oy=by+dy
    k=s["kind"]
    if k=="rect": s["x"]=rnd(s["x"]+ox); s["y"]=rnd(s["y"]+oy)
    elif k=="ellipse": s["cx"]=rnd(s["cx"]+ox); s["cy"]=rnd(s["cy"]+oy)
    elif k=="capsule":
        for a,b in (("x0","y0"),("x1","y1")): s[a]=rnd(s[a]+ox); s[b]=rnd(s[b]+oy)
    elif k=="poly": s["points"]=[[rnd(x+ox),rnd(y+oy)] for x,y in s["points"]]
    elif k=="path": s["d"]=shift_path(s["d"],ox,oy)
    return s
