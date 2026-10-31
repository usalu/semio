import sys
from pathlib import Path
p = Path(sys.argv[1])
s = p.read_text(encoding="utf-8")

def rep(old, new):
    global s
    assert s.count(old) == 1, old[:60]
    s = s.replace(old, new)

rep('''            dots.append(_engine.Dot(((left + 0.1), (top + bottom) / 2, 0), radius=0.07, color=self.tcol(self.ktype[self.keys[i]])))''',
    '''            dots.append(_morph.token("", kind="item", color=self.tcol(self.ktype[self.keys[i]]), size=0.8).move_to((left + 0.16, (top + bottom) / 2, 0)))''')
rep('''self.later(_morph.scatter(self.silhouettes(self.keys[:shown]), dots, lag=0.12, path_arc=-0.7, run_time=1.0, limit=2.4))''',
    '''self.later(_morph.scatter([self._axon.capsules[k].copy() for k in self.keys[:shown]], dots, lag=0.12, path_arc=-0.7, run_time=1.0, limit=2.4, pairs_fn=_morph.capsule_pairs, dip=0.15))''')
rep('''        pill = _morph.token(self.vals[("Program", f"M{self.swap_row}")], kind="cell", color=self.tcol(SWAP_FROM)).move_to(lst.point(f"M{self.swap_row}"))
        lit = self.silhouettes([key])[0]
        self.later(_engine.FadeIn(pill, run_time=0.3), when="post")
        self.later(_morph.morph(pill, lit, path_arc=-0.6, run_time=1.3), when="post", clean=[lit])''',
    '''        pill = _morph.token(self.vals[("Program", f"M{self.swap_row}")], kind="item", color=self.tcol(SWAP_FROM)).move_to(lst.point(f"M{self.swap_row}"))
        lit = self._axon.capsules[key].copy()
        self.later(_engine.FadeIn(pill, run_time=0.3), when="post")
        self.later(_morph.morph(pill, lit, pairs=[(b, a) for a, b in _morph.capsule_pairs(lit, pill)], path_arc=-0.6, run_time=1.3, dip=0.15), when="post", clean=[lit])''')
rep('''        self.step("area", *self.stage(win.select("G12"), win.recalculate(self.type_book("A", keep=(f"G12:G{last}", "G16:G16"))),
                  drop=[legend], add=[*hatches, *dims, card, *frames, handle, legend2]), seconds=2.8)''',
    '''        plays = self.stage(win.select("G12"), drop=[legend], add=[*hatches, *dims, card, *frames, handle, legend2])
        areas = [_morph.token(self.cell_text(self.wb, self.vals, "Type A", f"G{12 + i}"), kind="cell", color=FITTING_COLORS[f["key"]], size=0.8).move_to(win.point(f"G{12 + i}")) for i, f in enumerate(t.fittings)]
        rects = [plan.fits[i][0][0].copy().set_fill(FITTING_COLORS[f["key"]], opacity=0.5) for i, f in enumerate(t.fittings)]
        self.later(self.fly(rects, areas, run=0.9, total=1.6, arc=0.4), when="post")
        self.later(win.recalculate(self.type_book("A", keep=(f"G12:G{last}", "G16:G16"))), when="post", clean=areas)
        self.step("area", *plays, seconds=2.8)''')
rep('''        self.step("volume", *self.stage(vol_win.select("B10"), left=vol_win, right=[axon, pop, *vol_frames]), seconds=2.4)''',
    '''        plays = self.stage(vol_win.select("B10"), left=vol_win, right=[axon, pop, *vol_frames])
        volume = _morph.token(self.cell_text(self.wb, self.vals, "Type A", "B10"), kind="cell", color=DATA, size=0.8).move_to(vol_win.point("B10"))
        self.later(_morph.morph(axon.copy(), volume, path_arc=0.5, run_time=1.2), when="post", clean=[volume])
        self.step("volume", *plays, seconds=2.4)''')
rep('''        self.step("xlookup", *self.stage(look.select("C27"), look.recalculate(self.program_book(self.wb, keep=(*kept, "B21:C24", "B27:C27"))), left=look,
                  right=[plan, floor, fills, plan.people, legend, tag, val, count, *frames]), seconds=2.8)''',
    '''        plays = self.stage(look.select("C27"), look.recalculate(self.program_book(self.wb, keep=(*kept, "B21:C24", "B27:C27"))), left=look,
                           right=[plan, floor, fills, plan.people, legend, tag, val, count, *frames])
        found = _morph.token(self.cell_text(self.wb, self.vals, "Program", "C27"), kind="cell", color=GOOD, size=0.8).move_to(look.point("C27"))
        landing = floor.copy().set_fill(GOOD, opacity=0.45)
        self.later(_engine.FadeIn(found, run_time=0.3), when="post")
        self.later(_morph.morph(found, landing, path_arc=-0.5, run_time=1.2), when="post", clean=[landing])
        self.step("xlookup", *plays, seconds=2.8)''')
rep('''        self.step("update", *self.stage(win.recalculate(self.program_book(self.swapped, keep=KEEP_ALL)), *move, left=win, right=[chart, *delta, total_chip], add=trace), seconds=3.0)''',
    '''        plays = self.stage(win.recalculate(self.program_book(self.swapped, keep=KEEP_ALL)), *move, left=win, right=[chart, total_chip], add=trace)
        starts = [d.copy().move_to(win.point(c)) for d, c in zip(delta, ("B21", "B23"))]
        self.later(self.fly(starts, delta, run=1.0, total=1.2, arc=-0.4), when="post")
        self.step("update", *plays, seconds=3.0)
        self.keep(*delta)''')
Path(sys.argv[2]).write_text(s, encoding="utf-8")
