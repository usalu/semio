"""Path utilities on bone-local geometry (metres): parse/serialise path data, convert primitives to paths, local nudges."""
import math, re
def f(v): return f"{v:.4f}".rstrip('0').rstrip('.') if abs(v) > 1e-5 else "0"
TOK = re.compile(r'[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?')
def parse(d):
    toks = TOK.findall(d); out = []; i = 0; cmd = None
    while i < len(toks):
        t = toks[i]
        if t in 'MLCQZ':
            cmd = t; i += 1
            if t == 'Z': out.append(['Z', []]); continue
        n = {'M': 2, 'L': 2, 'C': 6, 'Q': 4}[cmd]
        v = [float(q) for q in toks[i:i+n]]; i += n
        out.append([cmd, [(v[j], v[j+1]) for j in range(0, n, 2)]])
        if cmd == 'M': cmd = 'L'
    return out
def ser(cmds):
    return " ".join(c if not p else c + " " + " ".join(f"{f(x)} {f(y)}" for x, y in p) for c, p in cmds)
def flat(cmds, n=10):
    pts = []; cur = (0, 0); start = (0, 0)
    for c, p in cmds:
        if c == 'M': cur = p[0]; start = cur; pts.append(cur)
        elif c == 'L': cur = p[0]; pts.append(cur)
        elif c == 'C':
            p0 = cur; p1, p2, p3 = p
            for u in range(1, n+1):
                t = u/n; a = (1-t)**3; b = 3*(1-t)**2*t; cc = 3*(1-t)*t*t; e = t**3
                pts.append((a*p0[0]+b*p1[0]+cc*p2[0]+e*p3[0], a*p0[1]+b*p1[1]+cc*p2[1]+e*p3[1]))
            cur = p3
        elif c == 'Q':
            p0 = cur; p1, p2 = p
            for u in range(1, n+1):
                t = u/n; pts.append(((1-t)**2*p0[0]+2*(1-t)*t*p1[0]+t*t*p2[0], (1-t)**2*p0[1]+2*(1-t)*t*p1[1]+t*t*p2[1]))
            cur = p2
        elif c == 'Z': cur = start
    return pts
def ellipse_path(cx, cy, rx, ry, rot=0):
    k = 0.5522847498; c, s = math.cos(math.radians(rot)), math.sin(math.radians(rot))
    def P(x, y): return (cx + c*x - s*y, cy + s*x + c*y)
    pts = [(rx, 0), (rx, ry*k), (rx*k, ry), (0, ry), (-rx*k, ry), (-rx, ry*k), (-rx, 0), (-rx, -ry*k), (-rx*k, -ry), (0, -ry), (rx*k, -ry), (rx, -ry*k), (rx, 0)]
    q = [P(*p) for p in pts]
    cmds = [['M', [q[0]]]]
    for i in range(4): cmds.append(['C', [q[1+3*i], q[2+3*i], q[3+3*i]]])
    cmds.append(['Z', []]); return cmds
def capsule_path(x0, y0, x1, y1, r0, r1):
    dx, dy = x1-x0, y1-y0; L = math.hypot(dx, dy) or 1e-9; a = math.atan2(dy, dx); nx, ny = -dy/L, dx/L
    def arc(cx, cy, r, a0, a1, n=6):
        return [(cx + r*math.cos(a0+(a1-a0)*u/n), cy + r*math.sin(a0+(a1-a0)*u/n)) for u in range(n+1)]
    pts = arc(x1, y1, r1, a-math.pi/2, a+math.pi/2) + arc(x0, y0, r0, a+math.pi/2, a+3*math.pi/2)
    cmds = [['M', [pts[0]]]] + [['L', [p]] for p in pts[1:]] + [['Z', []]]
    return cmds
def to_cmds(sh):
    k = sh['kind']
    if k == 'path': return parse(sh['d']), True
    if k == 'poly':
        pts = [tuple(p) for p in sh['points']]
        cmds = [['M', [pts[0]]]] + [['L', [p]] for p in pts[1:]] + ([['Z', []]] if sh.get('closed') else [])
        return cmds, sh.get('closed', False)
    if k == 'ellipse': return ellipse_path(sh['cx'], sh['cy'], sh['rx'], sh['ry'], sh.get('rotationDeg', 0)), True
    if k == 'rect':
        x, y, w, h = sh['x'], sh['y'], sh['w'], sh['h']
        pts = [(x, y), (x+w, y), (x+w, y+h), (x, y+h)]
        return [['M', [pts[0]]]] + [['L', [p]] for p in pts[1:]] + [['Z', []]], True
    if k == 'capsule': return capsule_path(*[sh[q] for q in ('x0', 'y0', 'x1', 'y1', 'r0', 'r1')]), True
    raise ValueError(k)
def as_path(sh):
    """convert any primitive shape to a `path` shape in place"""
    if sh['kind'] == 'path': return sh
    cmds, closed = to_cmds(sh)
    for q in ('x0','y0','x1','y1','r0','r1','cx','cy','rx','ry','rotationDeg','x','y','w','h','r','points','closed'): sh.pop(q, None)
    sh['kind'] = 'path'; sh['d'] = ser(cmds); return sh
def nudge(sh, at, vec, rho):
    """move all points of the shape near `at` (local metres) by `vec` with a gaussian kernel of radius rho."""
    as_path(sh); cmds = parse(sh['d'])
    for c in cmds:
        c[1] = [(x + vec[0]*math.exp(-(((x-at[0])**2+(y-at[1])**2)/rho**2)), y + vec[1]*math.exp(-(((x-at[0])**2+(y-at[1])**2)/rho**2))) for x, y in c[1]]
    sh['d'] = ser(cmds)
def transform_cmds(cmds, M):
    a, b, c, d, e, g = M
    return [[cc, [(a*x+c*y+e, b*x+d*y+g) for x, y in p]] for cc, p in cmds]

# ---------------------------------------------------------------- dense polygon deformation
def subpaths(cmds):
    out = []; cur = None
    for c, p in cmds:
        if c == 'M':
            cur = {'cmds': [['M', p]], 'closed': False}; out.append(cur)
        elif c == 'Z':
            cur['closed'] = True
        else:
            cur['cmds'].append([c, p])
    return out
def sp_points(sp, n=16):
    return flat(sp['cmds'], n)
def corner_flags(pts, closed, ang=28):
    n = len(pts); fl = []
    for i in range(n):
        if not closed and (i == 0 or i == n-1): fl.append(True); continue
        a = pts[i-1]; b = pts[i]; c = pts[(i+1) % n]
        v1 = (b[0]-a[0], b[1]-a[1]); v2 = (c[0]-b[0], c[1]-b[1])
        l1 = math.hypot(*v1); l2 = math.hypot(*v2)
        if l1 < 1e-9 or l2 < 1e-9: fl.append(False); continue
        cs = (v1[0]*v2[0]+v1[1]*v2[1])/(l1*l2); fl.append(math.degrees(math.acos(max(-1, min(1, cs)))) > ang)
    return fl
def densify(pts, closed, step):
    out = []; n = len(pts); m = n if closed else n-1
    for i in range(m):
        a = pts[i]; b = pts[(i+1) % n]; L = math.hypot(b[0]-a[0], b[1]-a[1]); k = max(1, int(math.ceil(L/step)))
        for u in range(k): out.append((a[0]+(b[0]-a[0])*u/k, a[1]+(b[1]-a[1])*u/k))
    if not closed: out.append(pts[-1])
    return out
def rdp(pts, tol, closed):
    if len(pts) < 4: return pts
    def seg(pp):
        if len(pp) < 3: return pp
        a, b = pp[0], pp[-1]; dx, dy = b[0]-a[0], b[1]-a[1]; L = math.hypot(dx, dy)
        best = -1; bi = 0
        for i in range(1, len(pp)-1):
            d = abs((pp[i][0]-a[0])*dy-(pp[i][1]-a[1])*dx)/L if L > 1e-12 else math.hypot(pp[i][0]-a[0], pp[i][1]-a[1])
            if d > best: best, bi = d, i
        if best > tol: return seg(pp[:bi+1])[:-1] + seg(pp[bi:])
        return [a, b]
    if closed:
        # split at the farthest pair so closed rings simplify symmetrically
        i0 = 0; j0 = max(range(len(pts)), key=lambda j: math.hypot(pts[j][0]-pts[0][0], pts[j][1]-pts[0][1]))
        a = seg(pts[i0:j0+1]); b = seg(pts[j0:]+[pts[0]])
        return a[:-1] + b[:-1]
    return seg(pts)
def kernel(r, rho):
    if r >= rho: return 0.0
    t = 1 - (r/rho)**2; return t*t
def deform(sh, fields, step, tol, max_move):
    """fields: list of (at(x,y), vec(dx,dy), rho). Converts the shape to a dense polygon path, applies the summed displacement, simplifies."""
    cmds, closed_all = to_cmds(sh)
    sps = subpaths(cmds)
    new_cmds = []
    for sp in sps:
        pts = sp_points(sp)
        closed = sp['closed']
        if closed and len(pts) > 1 and math.hypot(pts[0][0]-pts[-1][0], pts[0][1]-pts[-1][1]) < 1e-9: pts = pts[:-1]
        d = densify(pts, closed, step)
        moved = []
        for (x, y) in d:
            dx = dy = 0.0
            for (at, vec, rho) in fields:
                w = kernel(math.hypot(x-at[0], y-at[1]), rho)
                if w > 0: dx += vec[0]*w; dy += vec[1]*w
            m = math.hypot(dx, dy)
            if m > max_move: dx *= max_move/m; dy *= max_move/m
            moved.append((x+dx, y+dy))
        simp = rdp(moved, tol, closed)
        new_cmds.append(['M', [simp[0]]])
        for p in simp[1:]: new_cmds.append(['L', [p]])
        if closed: new_cmds.append(['Z', []])
    for q in ('x0','y0','x1','y1','r0','r1','cx','cy','rx','ry','rotationDeg','x','y','w','h','r','points','closed'): sh.pop(q, None)
    sh['kind'] = 'path'; sh['d'] = ser(new_cmds)

def inflate(sh, delta, step, tol):
    """offset every closed subpath of the shape outward by delta (metres) along vertex normals (dense polygon), open ones are left"""
    cmds, _ = to_cmds(sh); out = []
    for sp in subpaths(cmds):
        pts = sp_points(sp); closed = sp['closed']
        if closed and math.hypot(pts[0][0]-pts[-1][0], pts[0][1]-pts[-1][1]) < 1e-9: pts = pts[:-1]
        d = densify(pts, closed, step)
        if closed and len(d) > 2:
            area = sum(d[i][0]*d[(i+1)%len(d)][1]-d[(i+1)%len(d)][0]*d[i][1] for i in range(len(d)))/2
            sgn = 1 if area > 0 else -1
            n = len(d); res = []
            for i in range(n):
                a = d[i-1]; c = d[(i+1) % n]; tx, ty = c[0]-a[0], c[1]-a[1]; L = math.hypot(tx, ty) or 1
                res.append((d[i][0] + sgn*ty/L*delta, d[i][1] - sgn*tx/L*delta))
            d = res
        s2 = rdp(d, tol, closed)
        out.append(['M', [s2[0]]]); out += [['L', [p]] for p in s2[1:]]
        if closed: out.append(['Z', []])
    for q in ('x0','y0','x1','y1','r0','r1','cx','cy','rx','ry','rotationDeg','x','y','w','h','r','points','closed'): sh.pop(q, None)
    sh['kind'] = 'path'; sh['d'] = ser(out)
