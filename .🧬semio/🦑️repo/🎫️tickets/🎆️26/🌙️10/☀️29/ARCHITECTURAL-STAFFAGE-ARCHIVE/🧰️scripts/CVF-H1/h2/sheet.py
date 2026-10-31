"""compose print-size previews of all scales side by side (shared ground), at dpi, as one svg->png"""
import sys; sys.path.insert(0,'.')
from lib import *
def sheet(name, items, dpi=1000, gap=0.03):
    """items: list of (shapes, world, denom, pens, bounds, mode)"""
    parts=[]; x=0; Hmax=0
    for shapes,w,den,pens,bounds,mode in items:
        svg=render_svg(shapes,w,den,pens,bounds=bounds,mode=mode)
        import re
        m=re.search(r'viewBox="0 0 ([\d.]+) ([\d.]+)"',svg); W,H=float(m.group(1)),float(m.group(2))
        inner=svg.split('>',1)[1].rsplit('</svg>',1)[0]
        parts.append((x,W,H,inner)); x+=W+3; Hmax=max(Hmax,H)
    o=[f'<svg xmlns="http://www.w3.org/2000/svg" width="{x:.2f}mm" height="{Hmax:.2f}mm" viewBox="0 0 {x:.3f} {Hmax:.3f}"><rect width="100%" height="100%" fill="#fff"/>']
    for px,W,H,inner in parts: o.append(f'<g transform="translate({px:.3f},{Hmax-H:.3f})">{inner}</g>')
    o.append('</svg>')
    return to_png("\n".join(o),name,dpi)
