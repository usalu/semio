import sys,json
from lib import *
A=Asset(sys.argv[1]); views=sys.argv[3].split(',') if len(sys.argv)>3 else ['front','plan']
bones=sys.argv[2].split(',')
def desc(s):
    k=s['kind']
    if k in('poly','path'):
        pts=s.get('points')
        if pts is None:
            import re
            pts=[[float(a),float(b)] for a,b in re.findall(r'(-?[\d.e-]+) (-?[\d.e-]+)',s['d'])]
        xs=[p[0] for p in pts]; ys=[p[1] for p in pts]
        return f"{k} x[{min(xs):.3f},{max(xs):.3f}] y[{min(ys):.3f},{max(ys):.3f}] n{len(pts)}"
    if k=='capsule': return f"capsule ({s['x0']:.3f},{s['y0']:.3f})-({s['x1']:.3f},{s['y1']:.3f}) r{s['r0']:.3f},{s['r1']:.3f}"
    if k=='ellipse': return f"ellipse c({s['cx']:.3f},{s['cy']:.3f}) r({s['rx']:.3f},{s['ry']:.3f}) rot{s.get('rotationDeg',0)}"
    if k=='rect': return f"rect x{s['x']:.3f} y{s['y']:.3f} w{s['w']:.3f} h{s['h']:.3f}"
    return k
g=A.data['representations']['50']['geometry']
for v in views:
    for s in g[v]:
        if bones==['all'] or s.get('bone') in bones: print(v,s['id'],s.get('bone'),s['role'],s.get('region'),desc(s),json.dumps(s['solid']) if 'solid' in s else '')
