import json,math,os,glob
S='C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage'
def path(a): return glob.glob(S+'/🗂️catalog/*/'+a+'/*.json')[0]
def load(a): return json.load(open(path(a),encoding='utf-8'))
def save(a,doc):
    import sys,importlib.util
    f=glob.glob('C:/git/semio/.🧬semio/*/*/🎆️26/🌙️11/*/TOP-FRONT*/🧰️FW/atomic_json.py')[0]
    s=importlib.util.spec_from_file_location('atomic_json',f); m=importlib.util.module_from_spec(s); s.loader.exec_module(m)
    m.write_json_atomic(path(a),doc)
def rest(asset):
    """rest world: id -> (ox,oy,angleDeg,length)"""
    w={}
    for b in asset['rig']['bones']:
        if b['parent'] is None: px,py,pa=0,0,0
        else: px,py,pa,_=w[b['parent']]
        c,s=math.cos(math.radians(pa)),math.sin(math.radians(pa))
        w[b['id']]=(px+c*b['x']-s*b['y'],py+s*b['x']+c*b['y'],pa+b['angleDeg'],b['lengthM'])
    return w
def tip(w,i):
    x,y,a,l=w[i]; return (x+l*math.cos(math.radians(a)),y+l*math.sin(math.radians(a)))

G=None
def probe():
    global G
    if G is None:
        import glob as g2
        f=g2.glob('C:/git/semio/.🧬semio/*/*/🎆️26/🌙️11/*/TOP-FRONT*/🗑️generated/H2/probe.json')[0]
        G=json.load(open(f,encoding='utf-8'))
    return G

def dump_rep(asset,sc):
    pass

# ---------- shape constructors ----------
def _base(id,bone,role,z,tone,region,regions,material,hatch,pen,filled=True):
    d={'id':id,'bone':bone,'role':role,'z':z,'filled':filled,'tone':tone}
    if region: d['region']=region
    if regions: d['regions']=regions
    if material: d['material']=material
    if hatch: d['hatch']=hatch
    if pen is not None: d['penMm']=pen
    return d
def _r(v): return round(v,4)
def mk(geom,id,bone,z=0,tone=0.5,region=None,regions=None,material=None,role='body',hatch=None,pen=None,filled=True):
    d=dict(geom); d.update(_base(id,bone,role,round(z,3),tone,region,regions,material,hatch,pen,filled)); return d
def ell(id,bone,cx,cy,rx,ry,rot=None,**k):
    g={'kind':'ellipse','cx':_r(cx),'cy':_r(cy),'rx':_r(rx),'ry':_r(ry)}
    if rot: g['rotationDeg']=rot
    return mk(g,id,bone,**k)
def cap(id,bone,x0,y0,x1,y1,r0,r1,follow=True,**k):
    g={'kind':'capsule','x0':_r(x0),'y0':_r(y0),'x1':_r(x1),'y1':_r(y1),'r0':_r(r0),'r1':_r(r1)}
    if follow: g['followTip']=True
    return mk(g,id,bone,**k)
def rect(id,bone,x,y,w,h,r=None,follow=False,**k):
    g={'kind':'rect','x':_r(x),'y':_r(y),'w':_r(w),'h':_r(h)}
    if r: g['r']=_r(r)
    if follow: g['followTip']=True
    return mk(g,id,bone,**k)
def poly(id,bone,pts,closed=True,follow=False,**k):
    g={'kind':'poly','points':[[_r(a),_r(b)] for a,b in pts],'closed':closed}
    if follow: g['followTip']=True
    return mk(g,id,bone,**k)
def line(id,bone,pts,follow=False,**k):
    k.setdefault('role','detail'); k.setdefault('filled',False)
    return poly(id,bone,pts,closed=False,follow=follow,**k)

def limb(xc,ya,yb,ra,rb,et=0.0,eb=0.0,bulge=1.04):
    ym=(ya+yb)/2; rm=(ra+rb)/2*bulge
    return [(xc-ra*0.82,ya+et),(xc+ra*0.82,ya+et),(xc+ra,ya),(xc+rm,ym),(xc+rb,yb),(xc+rb*0.82,yb-eb),(xc-rb*0.82,yb-eb),(xc-rb,yb),(xc-rm,ym),(xc-ra,ya)]

# ---------- conversion default-pose world -> rest-bound coordinates ----------
class Conv:
    def __init__(s,asset_id):
        s.asset=load(asset_id)['asset']; s.w=rest(s.asset); s.d=probe()[asset_id]['bones']
    def o_rest(s,b): return (s.w[b][0],s.w[b][1])
    def t_rest(s,b): return tip(s.w,b)
    def dxo(s,b): return s.d[b]['o'][0]-s.w[b][0]
    def dyo(s,b): return s.d[b]['o'][1]-s.w[b][1]
    def dxt(s,b): return s.d[b]['t'][0]-s.t_rest(b)[0]
    def dyt(s,b): return s.d[b]['t'][1]-s.t_rest(b)[1]
    def px(s,b,x): return x-s.dxo(b)
    def pcap(s,b,x0,x1): return (x0-s.dxo(b),x1-s.dxt(b))
    def fy(s,b,y):
        return y-s.dyo(b)
    def fyf(s,b,y):
        """default-world y -> rest y for a followTip shape (inverse of the stretch)"""
        oyr=s.w[b][1]; tyr=s.t_rest(b)[1]; oyd=s.d[b]['o'][1]; tyd=s.d[b]['t'][1]
        if abs(tyr-oyr)<1e-9: return y-s.dyo(b)
        if abs(tyd-oyd)<1e-4:
            raise ValueError('degenerate follow '+b)
        return oyr+(y-oyd)*(tyr-oyr)/(tyd-oyd)

def solid(shape,z0,z1,kind='extruded'):
    s=dict(shape); s['solid']={'kind':kind,'z0':round(z0,4),'z1':round(z1,4)}; return s
