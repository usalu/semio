import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('culprit.get_right() + np.array([0.75, 0.0, 0.0])', 'culprit.get_right() + np.array([0.95, -0.22, 0.0])')
rep('"Count": ("Count", "Anzahl"), ', '')
rep('fill_color=color, fill_opacity=0.55,', 'fill_color=color, fill_opacity=0.32,')
open(p, "w", encoding="utf-8").write(s)
