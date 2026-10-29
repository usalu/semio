import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
old = '''        card = _box(x, bottom - 0.04, x + tip_w, bottom - 0.04 - tip_h, TIP, "#7A7A7A", W_THIN)
        text = place(message, (x + 0.15, bottom - 0.04 - tip_h / 2 - _cap(S_SMALL) / 2, 0), "bl")'''
new = '''        edge = bottom - 0.04 if bottom - 0.08 - tip_h > window.grid_bottom - 0.01 else top + 0.04 + tip_h
        card = _box(x, edge, x + tip_w, edge - tip_h, TIP, "#7A7A7A", W_THIN)
        text = place(message, (x + 0.15, edge - tip_h / 2 - _cap(S_SMALL) / 2, 0), "bl")'''
assert old in src
src = src.replace(old, new)
old2 = src[src.index("            src_rect = window.rect(a1)"):src.index("        delta = start - centre")]
new2 = '''            src_rect = window.rect(a1)
            start = np.array([src_rect[0] + 0.14, (src_rect[1] + src_rect[3]) / 2, 0.0])
            if ":" in a1:
                group.add(polyline(_ring(*src_rect), color, 2.2, closed=True))
            group.add(_engine.Dot(start, radius=0.06, color=color))
            dash = None
'''
src = src.replace(old2, new2)
src = src.replace("centre = np.array([(end_rect[0] + end_rect[2]) / 2, (end_rect[1] + end_rect[3]) / 2, 0.0])", "centre = np.array([end_rect[0] + 0.14, (end_rect[1] + end_rect[3]) / 2, 0.0])")
open(path, "w", encoding="utf-8").write(src)
