import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
a = s.index('        radiator = _radiator("physics-fundamentals-radiator", np.array([5.3')
b = s.index('        flows = {"air": False, "water": False}')
new = '''        radiator = _staffage("physics-fundamentals-radiator-front", unit=1.2, anchor=np.array([5.5, 0.0, 0.0]))
        feed = [np.array(p, dtype=float) for p in ((3.45, 0.75, 0.0), (4.75, 0.75, 0.0), (4.75, 0.03, 0.0), (4.9, 0.03, 0.0))]
        back = [np.array(p, dtype=float) for p in ((6.1, 0.03, 0.0), (6.25, 0.03, 0.0), (6.25, -0.3, 0.0), (3.9, -0.3, 0.0),
                                                    (3.9, 0.0, 0.0), (3.45, 0.0, 0.0))]
        hot_rays = VGroup(VMobject(stroke_width=3, color=P_RED).set_points_as_corners(feed),
                          VMobject(stroke_width=3, color=P_ORANGE).set_points_as_corners(back))
        water_path = VMobject().set_points_as_corners(feed + back + feed[:1])
'''
s = s[:a] + new + s[b:]
s = s.replace('np.array([5.0, -0.5, 0.0]), size=LABEL_FONT_SIZE,\n                     color=P_RED)', 'np.array([5.2, -0.7, 0.0]), size=LABEL_FONT_SIZE,\n                     color=P_RED)', 1)
open(p, "w", encoding="utf-8", newline="").write(s)
