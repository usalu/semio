"""own quick renderer: all named poses of an asset at one scale side by side (no hidden-line, fills + outlines)"""
import sys, json, os, subprocess
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-H3")
import lib
from io_ import *
def sheet(name, scale, px=60, out=None, poses=None):
    A = load(name)['asset']; rep = A['representations'][str(scale)]
    ps = [p for p in A['poses'] if not p.get('key')] if poses is None else [p for p in A['poses'] if p['id'] in poses]
    mm = scale/1000.0
    parts = []; x = 0
    W = A['dimensions']['widthM']*1.25; H = A['dimensions']['heightM']+0.1
    for p in ps:
        root = lib.pose_root(A, p) if p['contacts'] else dict(p.get('root') or {})
        Wd = lib.fk(A, p['bones'], root)
        els = []
        for s in sorted(rep['geometry']['elevation'], key=lambda s: s['z']):
            els.append(lib.shape_svg(s, lib.bone_matrix(Wd[s['bone']]), rep['pens'], {}, scale, True))
        parts.append(f'<g transform="translate({x},0)">{"".join(els)}<line x1="-0.5" y1="0" x2="{W}" y2="0" stroke="#888" stroke-width="{0.1*mm}"/></g>')
        x += W + 0.2
    vb = f"-0.6 {-H} {x} {H+0.05}"
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="{x/mm*px:.0f}" height="{(H+0.05)/mm*px:.0f}" viewBox="{vb}"><rect x="-0.6" y="{-H}" width="{x}" height="{H+0.05}" fill="white"/>{"".join(parts)}</svg>'
    out = out or f"{T}/🗑️generated/CVF-H2/pose-{name}-{scale}.png"
    open(out.replace('.png', '.svg'), 'w').write(svg)
    subprocess.run(["rsvg-convert", "-o", out, out.replace('.png', '.svg')], check=True); os.remove(out.replace('.png', '.svg'))
    return out
if __name__ == "__main__":
    print(sheet(sys.argv[1], int(sys.argv[2]), float(sys.argv[3]) if len(sys.argv) > 3 else 60))
