import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("        pill_w = 3.1\n        centre = (self.x0 + self.right) / 2 + 0.6\n")
b = src.index("        buttons = segments([((self.right - 0.92, cy)")
new = '''        hint = _put(DATA["ui"]["search"][self.language], 0, cy, "l", size, PAPER)
        low, high = name.get_right()[0] + 0.3, self.right - 1.15
        pill_w = min(3.1, high - low)
        group.add(name)
        if pill_w >= hint.width + 0.62:
            centre = min(max((self.x0 + self.right) / 2 + 0.6, low + pill_w / 2), high - pill_w / 2)
            glass_x, pill = centre - pill_w / 2 + 0.2, _engine.RoundedRectangle(corner_radius=0.1, width=pill_w, height=TITLE_H - 0.09, stroke_width=0, fill_color="#FFFFFF", fill_opacity=0.22).move_to(P((centre, cy)))
            group.add(pill, place(hint, (centre - pill_w / 2 + 0.42, cy - _cap(size) / 2, 0), "bl"))
        elif high - low >= 0.4:
            glass_x = high - 0.2
        else:
            glass_x = None
        if glass_x is not None:
            group.add(_engine.Circle(radius=0.05, stroke_color=PAPER, stroke_width=W_EDGE, fill_opacity=0).move_to(P((glass_x, cy + 0.015))),
                      segments([((glass_x + 0.035, cy - 0.02), (glass_x + 0.08, cy - 0.065))], PAPER, W_EDGE))
'''
src = src[:a] + new + src[b:]
src = src.replace("        group.add(name, pill, glass, hint, buttons)\n        return group", "        group.add(buttons)\n        return group", 1)
open(path, "w", encoding="utf-8").write(src)
