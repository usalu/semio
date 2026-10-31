"""Hand-entered profile tables of the human body kit (reference: jogger, thigh 0.4447 m). Numbers are authored; code only
converts station tables into smooth closed paths and scales them by the asset's bone-length factor."""
import math
from lib import *

def f(v): return f"{v:.4f}".rstrip('0').rstrip('.') if abs(v)>1e-5 else "0"
def cr(pts,closed=False,t=1/6):
    """Catmull-Rom through pts -> list of 'C' segments strings (open)."""
    P=pts; n=len(P); out=[]
    for i in range(n-1):
        p0=P[i-1] if i>0 else P[i]; p1=P[i]; p2=P[i+1]; p3=P[i+2] if i+2<n else P[i+1]
        c1=(p1[0]+(p2[0]-p0[0])*t,p1[1]+(p2[1]-p0[1])*t); c2=(p2[0]-(p3[0]-p1[0])*t,p2[1]-(p3[1]-p1[1])*t)
        out.append(f"C {f(c1[0])} {f(c1[1])} {f(c2[0])} {f(c2[1])} {f(p2[0])} {f(p2[1])}")
    return out
def profile(st,s=1.0,cap0=None,cap1=None):
    """st: list of (x, ylow, yhigh) stations along the bone; smooth outline; caps: None=flat, 'round'=semicircle-ish"""
    up=[(x*s,yh*s) for x,yl,yh in st]; lo=[(x*s,yl*s) for x,yl,yh in reversed(st)]
    d=f"M {f(up[0][0])} {f(up[0][1])} "+" ".join(cr(up))
    if cap1=='round':
        x,yl,yh=st[-1]; r=(yh-yl)/2*s; xm=x*s+r*0.9
        d+=f" C {f(up[-1][0]+r*0.55)} {f(up[-1][1])} {f(xm)} {f((yh+yl)/2*s+r*0.55)} {f(xm)} {f((yh+yl)/2*s)} C {f(xm)} {f((yh+yl)/2*s-r*0.55)} {f(lo[0][0]+r*0.55)} {f(lo[0][1])} {f(lo[0][0])} {f(lo[0][1])}"
    else: d+=f" L {f(lo[0][0])} {f(lo[0][1])}"
    d+=" "+" ".join(cr(lo))
    if cap0=='round':
        x,yl,yh=st[0]; r=(yh-yl)/2*s; xm=x*s-r*0.9
        d+=f" C {f(lo[-1][0]-r*0.55)} {f(lo[-1][1])} {f(xm)} {f((yh+yl)/2*s-r*0.55)} {f(xm)} {f((yh+yl)/2*s)} C {f(xm)} {f((yh+yl)/2*s+r*0.55)} {f(up[0][0]-r*0.55)} {f(up[0][1])} {f(up[0][0])} {f(up[0][1])}"
    d+=" Z"
    return d
def sub(st,x0,x1,grow=0.0):
    """stations of a sub-length x0..x1 interpolated linearly, with outline grown by `grow` (garment over skin)."""
    def at(x):
        for i in range(len(st)-1):
            a,b=st[i],st[i+1]
            if a[0]<=x<=b[0]:
                u=(x-a[0])/(b[0]-a[0]) if b[0]>a[0] else 0
                return (x,a[1]+(b[1]-a[1])*u,a[2]+(b[2]-a[2])*u)
        return (x,st[-1][1],st[-1][2]) if x>=st[-1][0] else (x,st[0][1],st[0][2])
    xs=[x0]+[q[0] for q in st if x0<q[0]<x1]+[x1]
    return [(x,at(x)[1]-grow,at(x)[2]+grow) for x in xs]
def sc(st,s): return [(x*s,l*s,h*s) for x,l,h in st]
def spath(pts,s=1.0,close=True):
    d=f"M {f(pts[0][0]*s)} {f(pts[0][1]*s)} "+" ".join(f"L {f(x*s)} {f(y*s)}" for x,y in pts[1:])
    return d+(" Z" if close else "")
def cpath(pts,s=1.0):
    """closed smooth path through pts (Catmull-Rom, wrap-around)."""
    P=[(x*s,y*s) for x,y in pts]; n=len(P); out=[f"M {f(P[0][0])} {f(P[0][1])}"]; t=1/6
    for i in range(n):
        p0=P[i-1]; p1=P[i]; p2=P[(i+1)%n]; p3=P[(i+2)%n]
        c1=(p1[0]+(p2[0]-p0[0])*t,p1[1]+(p2[1]-p0[1])*t); c2=(p2[0]-(p3[0]-p1[0])*t,p2[1]-(p3[1]-p1[1])*t)
        out.append(f"C {f(c1[0])} {f(c1[1])} {f(c2[0])} {f(c2[1])} {f(p2[0])} {f(p2[1])}")
    return " ".join(out)+" Z"

# ---- station tables, reference scale (x along bone, y low, y high), frames as documented in the asset rig ----------------------
# torso/pelvis/head: x up, front = -y.  thigh/shank/arm: x down, front = +y.  foot: x forward, y up.
TORSO=[(-0.07,-0.105,0.098),(0.0,-0.112,0.102),(0.10,-0.116,0.104),(0.20,-0.126,0.114),(0.30,-0.134,0.124),(0.38,-0.130,0.122),(0.4447,-0.095,0.100),(0.4757,-0.050,0.060)]
PELVIS=[(-0.12,-0.075,0.070),(-0.05,-0.112,0.118),(0.03,-0.120,0.128),(0.1034,-0.112,0.105)]
THIGH=[(0.0,-0.112,0.105),(0.08,-0.108,0.100),(0.20,-0.088,0.086),(0.32,-0.068,0.070),(0.4447,-0.054,0.056)]
SHANK=[(0.0,-0.060,0.056),(0.10,-0.074,0.052),(0.20,-0.068,0.046),(0.32,-0.044,0.040),(0.4343,-0.034,0.034)]
UARM=[(0.0,-0.050,0.052),(0.10,-0.045,0.048),(0.22,-0.038,0.040),(0.3206,-0.034,0.034)]
FARM=[(0.0,-0.036,0.037),(0.10,-0.034,0.038),(0.20,-0.028,0.032),(0.2792,-0.024,0.025)]
HAND=[(-0.012,-0.030,0.030),(0.04,-0.034,0.030),(0.10,-0.030,0.032),(0.15,-0.024,0.022),(0.1861,-0.012,0.010)]
NECK=[(-0.04,-0.050,0.052),(0.02,-0.047,0.050),(0.09,-0.044,0.046)]
HEAD_V1="M 0.0 0.0447 L 0.0776 0.0397 C 0.1034 0.0655 0.1344 0.0993 0.1965 0.0973 C 0.2534 0.0943 0.2854 0.0397 0.2813 -0.0149 C 0.2771 -0.0645 0.2461 -0.0874 0.212 -0.0884 L 0.1944 -0.0913 C 0.1841 -0.0953 0.1675 -0.1072 0.1541 -0.1171 C 0.1458 -0.1112 0.1427 -0.0993 0.1386 -0.0933 C 0.1282 -0.0973 0.12 -0.0933 0.1138 -0.0874 C 0.1055 -0.0913 0.0951 -0.0874 0.091 -0.0794 C 0.0827 -0.0615 0.0786 -0.0447 0.0745 -0.0338 C 0.0517 -0.0417 0.0227 -0.0496 0.0 -0.0516 Z"
HEAD_100=[(0.0,0.045),(0.075,0.045),(0.125,0.085),(0.20,0.098),(0.265,0.075),(0.2813,0.0),(0.262,-0.058),(0.212,-0.088),(0.185,-0.095),(0.1541,-0.1171),(0.1386,-0.0933),(0.1138,-0.0874),(0.091,-0.0794),(0.0745,-0.0338),(0.0,-0.0516)]
HEAD_200=[(0.0,0.045),(0.08,0.048),(0.15,0.090),(0.22,0.098),(0.275,0.05),(0.275,-0.05),(0.21,-0.09),(0.1541,-0.1171),(0.1386,-0.0933),(0.10,-0.085),(0.075,-0.034),(0.0,-0.0516)]
SHOE_FOOT=[(-0.068,-0.0827),(-0.072,-0.04),(-0.058,0.036),(0.031,0.036),(0.09,0.012),(0.1344,-0.0153),(0.1344,-0.0827)]

# ---- transformation of path data between bone frames ----------------------------------------------------------------------------
def xform_d(d,M):
    import re
    toks=re.findall(r'[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?',d); out=[];i=0;cmd=None
    while i<len(toks):
        t=toks[i]
        if t in 'MLCQZ':
            cmd=t;i+=1
            if t=='Z': out.append('Z'); continue
        n={'M':2,'L':2,'C':6,'Q':4}[cmd]; v=[float(q) for q in toks[i:i+n]]; i+=n
        pts=[pm(M,v[j],v[j+1]) for j in range(0,n,2)]
        out.append(cmd+" "+" ".join(f"{f(x)} {f(y)}" for x,y in pts))
        if cmd=='M': cmd='L'
    return " ".join(out)
def inv(M):
    a,b,c,d_,e,f_=M; det=a*d_-b*c
    ia,ib,ic,id_=d_/det,-b/det,-c/det,a/det
    return (ia,ib,ic,id_,-(ia*e+ib*f_),-(ic*e+id_*f_))
def mul(A,B):
    return (A[0]*B[0]+A[1]*B[2],A[0]*B[1]+A[1]*B[3],A[2]*B[0]+A[3]*B[2],A[2]*B[1]+A[3]*B[3],A[0]*B[4]+A[1]*B[5]+A[4],A[2]*B[4]+A[3]*B[5]+A[5])
def reframe(d,frm,to,W):
    return xform_d(d,mul(inv(bone_matrix(W[to])),bone_matrix(W[frm])))

def shp(id,bone,role,z,filled,tone,geom,region=None,regions=None,material=None,hatch=None,penMm=None):
    o={"id":id,"bone":bone,"role":role,"z":round(z,3),"filled":filled,"tone":tone}
    o.update(geom)
    if region: o["region"]=region
    if regions: o["regions"]=regions
    if material: o["material"]=material
    if hatch: o["hatch"]=hatch
    if penMm: o["penMm"]=penMm
    return o
def P_(d): return {"kind":"path","d":d}
def L_(pts,closed=False): return {"kind":"poly","points":[[round(x,4),round(y,4)] for x,y in pts],"closed":closed}
def E_(cx,cy,rx,ry,rot=0):
    g={"kind":"ellipse","cx":round(cx,4),"cy":round(cy,4),"rx":round(rx,4),"ry":round(ry,4)}
    if rot: g["rotationDeg"]=rot
    return g
def R_(x,y,w,h,r=0):
    g={"kind":"rect","x":round(x,4),"y":round(y,4),"w":round(w,4),"h":round(h,4)}
    if r: g["r"]=r
    return g
def C_(x0,y0,x1,y1,r0,r1): return {"kind":"capsule","x0":round(x0,4),"y0":round(y0,4),"x1":round(x1,4),"y1":round(y1,4),"r0":round(r0,4),"r1":round(r1,4)}
def PENS(body,detail,texture,glazing,ground): return {"body":body,"detail":detail,"texture":texture,"glazing":glazing,"ground":ground}
def DEC(**k): return {n:T(*k[n]) for n in("silhouette","retained","merged","omitted","thinElements","hatching","styles","overlays")}
