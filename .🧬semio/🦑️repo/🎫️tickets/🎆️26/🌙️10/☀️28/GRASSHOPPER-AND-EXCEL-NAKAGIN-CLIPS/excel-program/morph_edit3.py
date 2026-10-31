import sys
from pathlib import Path
s = Path(sys.argv[1]).read_text(encoding="utf-8")

def rep(old, new):
    global s
    assert s.count(old) == 1, old[:70]
    s = s.replace(old, new)

rep('''        self.step("program", *self.stage(right=[plan, outline, plan.people, bed_dim, *heads]), seconds=2.0)''',
'''        plays = self.stage(right=[plan, outline, plan.people, bed_dim, *heads])
        rects = [rect.copy().set_fill(colour, opacity=0.5) for fits in plan.fits.values() for rect, _, colour in fits]
        spots = [_morph.token("", kind="item", color=r.get_color(), size=0.8).move_to((PLAN_AT[0] - 0.65 + 0.3 * i, 2.0, 0)) for i, r in enumerate(rects)]
        person = _morph.token(self.number(t.occupants, 0), kind="item", color=INK, size=0.8).move_to((PLAN_AT[0] + 1.75, 2.0, 0))
        self.later(self.fly([*rects, plan.people.copy()], [*spots, person], run=1.0, total=1.8, arc=0.5), when="post")
        self.later(_engine.FadeOut(*spots, person, run_time=0.4), when="post", clean=[*spots, person])
        self.step("program", *plays, seconds=2.0)''')
rep('''        self.step("net", *self.stage(net_win.select("B9"), net_win.recalculate(self.type_book("A", keep=("G12:G16", "B9:B9"))), left=net_win,
                  drop=[*hatches, *dims, card, *frames, handle, legend2], add=[floor_hatch, d1, d2, net_label, card2, *frames2]), seconds=2.2)''',
'''        plays = self.stage(net_win.select("B9"), left=net_win, drop=[*hatches, *dims, card, *frames, handle, legend2], add=[floor_hatch, d1, d2, net_label, card2, *frames2])
        labels = [_morph.token(self.number(v, 2) + " m", kind="cell", color=DATA, size=0.8).move_to(d.get_center()) for d, v in ((d1, iw), (d2, il))]
        landings = [_morph.token(self.cell_text(self.wb, self.vals, "Type A", "B9"), kind="cell", color=GOOD, size=0.8).move_to(net_win.point("B9")) for _ in labels]
        floor_to = plan.floor.copy().set_fill(GOOD, opacity=0.3)
        self.later(self.fly(labels, landings, run=1.0, total=1.2, arc=0.4), when="post")
        self.later(net_win.recalculate(self.type_book("A", keep=("G12:G16", "B9:B9"))), when="post", clean=[landings[1]])
        self.later(_morph.morph(landings[0], floor_to, path_arc=-0.5, run_time=1.2), when="post", clean=[floor_to])
        self.step("net", *plays, seconds=2.2)''')
rep('''        self.step("free", *self.stage(free_win.recalculate(self.type_book("A", keep=("G12:G16", "B9:B10", "B16:B16"))), left=free_win,
                  drop=[floor_hatch, d1, d2, net_label, card2, *frames2], add=[floor_fill, fit_all, free_label, card3, legend3, occupants, *refs]), seconds=2.2)''',
'''        plays = self.stage(left=free_win, drop=[floor_hatch, d1, d2, net_label, card2, *frames2], add=[fit_all, card3, legend3, occupants, *refs])
        pieces = [rect.copy().set_fill(colour, opacity=0.5) for fits in plan.fits.values() for rect, _, colour in fits]
        free_val = _morph.token(self.cell_text(self.wb, self.vals, "Type A", "B16"), kind="cell", color=GOOD, size=0.8).move_to(free_win.point("B16"))
        tint = floor_fill.copy()
        self.later(self.collect([(pieces, free_val)], run=0.8, total=1.6), when="post")
        self.later(free_win.recalculate(self.type_book("A", keep=("G12:G16", "B9:B10", "B16:B16"))), when="post", clean=[])
        self.later(_morph.morph(free_val, tint, path_arc=-0.5, run_time=1.2), when="post")
        self.step("free", *plays, seconds=2.2)
        self.keep(tint)
        self.add(free_label)
        self.keep(free_label)''')
rep('''        starts = [d.copy().move_to(win.point(c)) for d, c in zip(delta, ("B21", "B23"))]
        self.later(self.fly(starts, delta, run=1.0, total=1.2, arc=-0.4), when="post")''',
'''        starts = [d.copy().move_to(self.capsule_center(key)) for d in delta]
        self.later(self.fly(starts, delta, run=1.1, total=1.4, arc=-0.4))''')
rep('''        self.step("check", *self.stage(check.select("B18"), check.recalculate(self.swapped), left=check, right=[tower2, ok, eq]), seconds=2.6)''',
'''        plays = self.stage(check.select("B18"), check.recalculate(self.swapped), left=check, right=[tower2, eq])
        sums = [_morph.token(self.number(self.swapped_vals[("Program", c)], 0), kind="number", color=GOOD, size=0.8).move_to(p) for c, p in (("B8", (TOWER_AT[0] - 0.9, -1.7, 0)), ("B17", (TOWER_AT[0] + 0.9, -1.7, 0)))]
        self.later(self.collect([(sums, ok)], run=0.9, total=1.3), when="post")
        self.step("check", *plays, seconds=2.6)
        self.keep(ok)''')
Path(sys.argv[2]).write_text(s, encoding="utf-8")
