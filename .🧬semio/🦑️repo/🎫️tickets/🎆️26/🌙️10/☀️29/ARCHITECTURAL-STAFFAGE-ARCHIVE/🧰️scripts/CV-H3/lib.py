import json, math, re, subprocess, os
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
HERE=os.path.dirname(os.path.abspath(__file__))
GEN=os.path.abspath(os.path.join(HERE,"../../🗑️generated/CV-H3"))
def load_v1(a): return json.load(open(f"{HERE}/v1/{a}.json"))
def out_path(a): return f"{S}/🗂️catalog/👤️humans/{a}/🔣️.json"
def T(en,de): return {"en":en,"de":de}
def rnd(x,n=4): return round(x+0.0,n)

# ---- FK -------------------------------------------------------------
def rot(a): c,s=math.cos(math.radians(a)),math.sin(math.radians(a)); return c,s
def fk(asset,pose_bones,root=None,bone_extra=None):
    bones={b['id']:b for b in asset['rig']['bones']}
    W={}
    root=root or {}
    def go(bid):
        if bid in W: return W[bid]
        b=bones[bid]; off=dict(pose_bones.get(bid,{}))
        if bone_extra and bid in bone_extra: off.update(bone_extra[bid])
        if b['parent'] is None:
            x=b['x']+root.get('x',0)+off.get('dx',0); y=b['y']+root.get('y',0)+off.get('dy',0); a=b['angleDeg']+root.get('angleDeg',0)+off.get('angleDeg',0)
        else:
            px,py,pa=go(b['parent']); c,s=rot(pa)
            lx=b['x']+off.get('dx',0); ly=b['y']+off.get('dy',0)
            x=px+c*lx-s*ly; y=py+s*lx+c*ly; a=pa+b['angleDeg']+off.get('angleDeg',0)
        W[bid]=(x,y,a); return W[bid]
    for k in bones: go(k)
    return W

# ---- paths ----------------------------------------------------------
def tx_path(d,M):
    toks=re.findall(r'[MLCQZHV]|-?\d*\.?\d+(?:e-?\d+)?',d)
    out=[];i=0;nums=lambda n:[float(t) for t in toks[i:i+n]]
    def ap(x,y): X=M[0]*x+M[1]*y+M[4]; Y=M[2]*x+M[3]*y+M[5]; return f"{X:.5f},{-Y:.5f}"
    cmd=None
    while i<len(toks):
        t=toks[i]
        if t in 'MLCQZ':
            cmd=t;i+=1
            if t=='Z': out.append('Z'); continue
        n={'M':2,'L':2,'C':6,'Q':4}[cmd]; v=nums(n); i+=n
        pts=" ".join(ap(v[j],v[j+1]) for j in range(0,n,2)); out.append(cmd+" "+pts)
        if cmd=='M': cmd='L'
    return " ".join(out)
def bone_matrix(w): c,s=rot(w[2]); return (c,-s,s,c,w[0],w[1])
def pm(M,x,y): return (M[0]*x+M[1]*y+M[4], M[2]*x+M[3]*y+M[5])
def shape_svg(sh,M,pens,hatches,scale,tone_fill=True):
    """returns svg element string in model metres (y flipped), stroke widths in metres = mm/scale*..."""
    mm=scale/1000.0  # metres per paper mm = denom/1000
    role=sh['role']; pen=sh.get('penMm') or pens.get(role)
    if pen is None: return ""
    sw=pen*mm
    k=sh['kind']
    if k=='path': d=tx_path(sh['d'],M); geo=f'<path d="{d}"'
    elif k=='poly':
        pts=" ".join(f"{pm(M,x,y)[0]:.5f},{-pm(M,x,y)[1]:.5f}" for x,y in sh['points'])
        d="M "+pts.replace(" "," L ",) if False else None
        geo=f'<{"polygon" if sh["closed"] else "polyline"} points="{pts}"'
    elif k=='ellipse':
        cx,cy=pm(M,sh['cx'],sh['cy']); ang=math.degrees(math.atan2(M[2],M[0]))+sh.get('rotationDeg',0)
        geo=f'<ellipse cx="{cx:.5f}" cy="{-cy:.5f}" rx="{sh["rx"]:.5f}" ry="{sh["ry"]:.5f}" transform="rotate({-ang:.3f} {cx:.5f} {-cy:.5f})"'
    elif k=='rect':
        x,y,w,h=sh['x'],sh['y'],sh['w'],sh['h']; r=sh.get('r',0)
        pts=[(x,y),(x+w,y),(x+w,y+h),(x,y+h)]
        pts=" ".join(f"{pm(M,a,b)[0]:.5f},{-pm(M,a,b)[1]:.5f}" for a,b in pts); geo=f'<polygon points="{pts}"'
    elif k=='capsule':
        x0,y0,x1,y1,r0,r1=[sh[q] for q in('x0','y0','x1','y1','r0','r1')]
        dx,dy=x1-x0,y1-y0; L=math.hypot(dx,dy) or 1e-9; nx,ny=-dy/L,dx/L
        a=math.atan2(dy,dx)
        pts=[(x0+nx*r0,y0+ny*r0),(x1+nx*r1,y1+ny*r1)]
        arc1=[(x1+r1*math.cos(a+math.pi/2-t*math.pi/12),y1+r1*math.sin(a+math.pi/2-t*math.pi/12)) for t in range(0,13)]
        arc0=[(x0+r0*math.cos(a-math.pi/2-t*math.pi/12),y0+r0*math.sin(a-math.pi/2-t*math.pi/12)) for t in range(0,13)]
        P=[(x0+nx*r0,y0+ny*r0)]+arc1+arc0
        pts=" ".join(f"{pm(M,a_,b_)[0]:.5f},{-pm(M,a_,b_)[1]:.5f}" for a_,b_ in P); geo=f'<polygon points="{pts}"'
    else: return ""
    filled=sh['filled'] and k!='polyline'
    fill=f'fill="{tone_hex(sh["tone"])}"' if filled and tone_fill else 'fill="none"'
    if sh['role']=='glazing': fill='fill="#cfe8f5"' if filled else 'fill="none"'
    return f'{geo} {fill} stroke="#000" stroke-width="{sw:.6f}" stroke-linejoin="round" stroke-linecap="round"/>'
def tone_hex(t): v=int(255*t); v=max(0,min(255,v)); return f"#{v:02x}{v:02x}{v:02x}"

def render(asset,rep_key,pose_bones,root,out_png,pxmm=14,zoom=1.0,bone_extra=None,crop=None,view='elevation',margin_mm=3,extra_svg="",tone_fill=True):
    rep=asset['representations'][rep_key]; den=int(rep_key)
    W=fk(asset,pose_bones,root,bone_extra)
    shapes=sorted(rep['geometry'][view],key=lambda s:s['z'])
    pens=rep['pens']; hatches={h['id']:h for h in rep['hatches']}
    els=[]
    for s in shapes:
        els.append(shape_svg(s,bone_matrix(W[s['bone']]),pens,hatches,den,tone_fill))
    h=asset['dimensions']['heightM']; w=asset['dimensions']['widthM']
    mm=den/1000.0
    x0=crop[0] if crop else -w*0.7; x1=crop[1] if crop else w*0.7+0.3; y0=crop[2] if crop else -0.05; y1=crop[3] if crop else h+0.1
    vb=(x0,-y1,x1-x0,y1-y0)
    z=zoom
    pw=(x1-x0)/mm*pxmm*z; ph=(y1-y0)/mm*pxmm*z
    ground=f'<line x1="{x0}" y1="0" x2="{x1}" y2="0" stroke="#888" stroke-width="{0.1*mm:.5f}"/>'
    svg=f'<svg xmlns="http://www.w3.org/2000/svg" width="{pw:.0f}" height="{ph:.0f}" viewBox="{vb[0]:.4f} {vb[1]:.4f} {vb[2]:.4f} {vb[3]:.4f}"><rect x="{vb[0]}" y="{vb[1]}" width="{vb[2]}" height="{vb[3]}" fill="white"/>{ground}{"".join(els)}{extra_svg}</svg>'
    p=out_png.replace(".png",".svg"); open(p,"w").write(svg)
    subprocess.run(["rsvg-convert","-o",out_png,p],check=True); os.remove(p)

def path_points(d,n=12):
    toks=re.findall(r'[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?',d); pts=[]; i=0; cmd=None; cur=(0,0); start=(0,0)
    while i<len(toks):
        t=toks[i]
        if t in 'MLCQZ':
            cmd=t; i+=1
            if t=='Z': cur=start; continue
        k={'M':2,'L':2,'C':6,'Q':4}[cmd]; v=[float(q) for q in toks[i:i+k]]; i+=k
        if cmd in 'ML':
            cur=(v[0],v[1]); pts.append(cur)
            if cmd=='M': start=cur; cmd='L'
        elif cmd=='C':
            p0=cur; p1=(v[0],v[1]); p2=(v[2],v[3]); p3=(v[4],v[5])
            for u in range(1,n+1):
                t_=u/n; a=(1-t_)**3; b=3*(1-t_)**2*t_; c=3*(1-t_)*t_**2; e=t_**3
                pts.append((a*p0[0]+b*p1[0]+c*p2[0]+e*p3[0],a*p0[1]+b*p1[1]+c*p2[1]+e*p3[1]))
            cur=p3
        elif cmd=='Q':
            p0=cur; p1=(v[0],v[1]); p2=(v[2],v[3])
            for u in range(1,n+1):
                t_=u/n; pts.append(((1-t_)**2*p0[0]+2*(1-t_)*t_*p1[0]+t_**2*p2[0],(1-t_)**2*p0[1]+2*(1-t_)*t_*p1[1]+t_**2*p2[1]))
            cur=p2
    return pts
def bbox(asset,rep_key,pose_bones,root,roles=('body','detail','glazing')):
    W=fk(asset,pose_bones,root); xs=[];ys=[]
    for s in asset['representations'][rep_key]['geometry']['elevation']:
        if s['role'] not in roles: continue
        M=bone_matrix(W[s['bone']]); k=s['kind']
        if k=='path': pts=path_points(s['d'])
        elif k=='poly': pts=[tuple(p) for p in s['points']]
        elif k=='ellipse': pts=[(s['cx']+s['rx']*math.cos(t/6),s['cy']+s['ry']*math.sin(t/6)) for t in range(37)]
        elif k=='rect': pts=[(s['x'],s['y']),(s['x']+s['w'],s['y']+s['h'])]
        elif k=='capsule': pts=[(s['x0']+s['r0']*math.cos(t/6),s['y0']+s['r0']*math.sin(t/6)) for t in range(37)]+[(s['x1']+s['r1']*math.cos(t/6),s['y1']+s['r1']*math.sin(t/6)) for t in range(37)]
        for x,y in pts:
            X,Y=pm(M,x,y); xs.append(X); ys.append(Y)
    return min(xs),max(xs),min(ys),max(ys)

def anchors_world(asset,W):
    out={}
    for a in asset['anchors']:
        out[a['id']]=pm(bone_matrix(W[a['bone']]),a['x'],a['y'])
    return out
def pose_root(asset,pose):
    root=dict(pose.get('root') or {})
    if 'y' not in root:
        W=fk(asset,pose['bones'],root); A=anchors_world(asset,W)
        ys=[A[c][1] for c in pose['contacts'] if c in A and next(a for a in asset['anchors'] if a['id']==c).get('support','contact')=='contact' and next(a for a in asset['anchors'] if a['id']==c)['kind']=='contact']
        root['y']=-min(ys) if ys else 0
    return root
