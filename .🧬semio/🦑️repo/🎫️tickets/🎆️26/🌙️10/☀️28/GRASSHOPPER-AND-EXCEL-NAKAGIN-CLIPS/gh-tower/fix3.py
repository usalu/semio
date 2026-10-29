import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)


rep("LC, LW, LH = (-2.45, -0.03), 9.15, 5.05\nVX, VY, VW, VH = 4.6, -0.03, 4.6, 5.05", "LC, LW, LH = (-2.35, -0.03), 9.3, 5.05\nVX, VY, VW, VH = 4.7, -0.03, 4.4, 5.05")
rep("width=width or 3.9, selected=selected)", "width=width or 3.6, selected=selected)")
rep("max_lines=width and 4 or 14,", "max_lines=14 if oid == \"gp\" or not width else 4,")
rep("box.set_fill(COLORS[key], opacity=0.28)", "box.set_fill(COLORS[key], opacity=0.15)")
rep('spots = {"core": (-5.3, 1.25), "planes": (-2.6, 1.25), "capsules": (0.2, 1.25), "capital": (-3.9, -1.5), "podium": (-1.0, -1.5)}',
    'spots = {"core": (-5.2, 1.45), "planes": (-2.5, 1.45), "capsules": (0.3, 1.45), "capital": (-3.6, -1.1), "podium": (-0.8, -1.1)}')
rep('layout_core = {"side": (-4.95, 1.45), "sh": (-4.95, 0.5), "n": (-4.95, -0.45), "ps": (-4.95, -1.4), "tn": (-2.3, -1.0), "tot": (-0.8, -0.1), "core": (1.1, 0.8)}',
    'layout_core = {"side": (-4.7, 1.45), "sh": (-4.7, 0.5), "n": (-4.7, -0.45), "ps": (-4.7, -1.4), "tn": (-1.9, -1.0), "tot": (-0.5, -0.1), "core": (1.2, 0.8)}')
rep('{"ps": (-4.95, 1.55), "sh": (-4.95, 0.5), "n": (-4.95, -0.9), "z0": (-2.4, 1.3), "ser": (-1.0, 0.15), "gp": (1.3, 0.0)}', '{"ps": (-4.7, 1.55), "sh": (-4.7, 0.5), "n": (-4.7, -0.9), "z0": (-1.8, 1.3), "ser": (-0.5, 0.15), "gp": (1.5, 0.0)}')
rep('count_layout = {"n": (-4.95, 1.4), "ser": (-2.3, 1.4), "ori": (0.2, 1.4), "st": (-2.4, -0.7), "stc": (0.4, -0.3), "stl": (0.4, -1.3)}',
    'count_layout = {"n": (-4.7, 1.4), "ser": (-1.9, 1.4), "ori": (0.3, 1.4), "st": (-2.0, -0.6), "stc": (1.2, -0.3), "stl": (1.2, -1.3)}')
rep('(-4.4, -1.3), size=30, color=TXT)', '(-4.5, -1.6), size=30, color=TXT)')
rep('{"pitch": (-4.95, 1.4), "hp": (-2.2, 1.4), "nhp": (-2.2, -0.3), "sx": (-0.2, 0.7)}', '{"pitch": (-4.7, 1.4), "hp": (-1.9, 1.4), "nhp": (-1.9, -0.3), "sx": (0.0, 0.7)}')
rep('sx_panel = self.gh_panel(self._lines(self._solve().tree("sx")), at=(1.5, -0.8), width=1.4)', 'sx_panel = self.gh_panel(self._lines(self._solve().tree("sx")), at=(1.4, -0.8), width=1.4)')
rep('{"capp": (-5.0, 1.2), "xy": (-5.5, -0.8), "slots": (-2.4, -1.0), "ori": (0.5, 0.2)}', '{"capp": (-5.0, 1.2), "xy": (-5.4, -0.8), "slots": (-2.3, -1.0), "ori": (0.6, 0.2)}')
rep('at=(-5.2, 1.0), name="Types")\n        item_part', 'at=(-5.0, 1.0), name="Types")\n        item_part')
rep('{"ow": (-4.95, 1.3), "ch": (-4.95, 0.2), "cw": (-2.2, 1.0), "capital": (0.7, 0.6)}', '{"ow": (-4.7, 1.3), "ch": (-4.7, 0.2), "cw": (-1.9, 1.0), "capital": (1.0, 0.6)}')
rep('{"n": (-4.95, 1.5), "tot": (-2.6, 1.5), "chh": (-2.6, 0.0), "czz": (-1.0, 0.75), "capital": (0.95, 0.75)}', '{"n": (-4.7, 1.5), "tot": (-2.3, 1.5), "chh": (-2.3, 0.0), "czz": (-0.7, 0.75), "capital": (1.2, 0.75)}')
rep('{"pw": (-4.95, 1.5), "pd": (-4.95, 0.4), "rect": (-2.0, 1.0), "uzp": (-2.0, -0.9), "ext": (0.6, 0.0)}', '{"pw": (-4.7, 1.5), "pd": (-4.7, 0.4), "rect": (-1.8, 1.0), "uzp": (-1.8, -0.9), "ext": (0.7, 0.0)}')
rep("at=(-4.95, 1.0 - 1.2 * i), decimals=0, width=3.9)", "at=(-4.7, 1.0 - 1.2 * i), decimals=0, width=3.6)")
rep("self.quantity(side, \"m\", 1), offset=-0.3, color=DATA)", "self.quantity(side, \"m\", 1), offset=-0.45, color=DATA)")
rep("self.quantity(height, \"m\"), offset=-0.3, color=WARN)", "self.quantity(height, \"m\"), offset=0.6, color=WARN)")
open(p, "w", encoding="utf-8").write(s)
