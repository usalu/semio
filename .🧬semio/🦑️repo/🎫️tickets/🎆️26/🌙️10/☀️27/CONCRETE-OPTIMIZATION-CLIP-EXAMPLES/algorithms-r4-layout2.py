"""📐 Round 4 layout edits, second pass (scratch copy, then copied into place)."""
import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
for old, new in (
    ('''group.add(_room.text([self.tr("zoom", "Zoom"), "×", self._n(LENS_ZOOM)], lens + mn.UP * .85, size=14, color=INK))''', '''group.add(self._chip([self.tr("zoom", "Zoom"), "×", self._n(LENS_ZOOM)], lens + mn.UP * .85, size=14))'''),
    ('''spot + mn.LEFT * 1.5 + mn.DOWN * .45, color=PATTERN''', '''spot + mn.LEFT * 1.5, color=PATTERN'''),
    ('''                            tail=[self.tr("(what-if, not simulated)", "(nicht simuliert)")])''', '''                            note=[self.tr("what-if, not simulated", "nicht simuliert")])'''),
    ('''(3.9, y + .15, 0), color=colour, size=14, width=3.9),''', '''(4.05, y + .15, 0), color=colour, size=14, width=3.6),'''),
    ('''self.heat["star"].add(self._chip([self.tr("map minimum", "Kartenminimum")], self.heat["star"].get_center() + mn.UP * .45 + mn.LEFT * .3, size=14, color=GOOD, width=1.5))''',
     '''self.heat["star"].add(mn.VGroup(mn.Star(n=5, outer_radius=.11, inner_radius=.048, color=INK, stroke_width=1, fill_color=GOOD, fill_opacity=1).move_to((-2.35, 1.82, 0)),
                                        self._chip([self.tr("map minimum", "Kartenminimum")], (-1.52, 1.82, 0), size=14, color=GOOD, width=1.5)))'''),
    ('''*self._p(minimum, "", 2)[:1], "× 1.01 =" if not self._german() else "× 1,01 =", *self._p(limit, "kWh/a", 1)]''', '''*self._p(minimum, "", 2)[:1], "× 1.01 =" if not self._german() else "× 1,01 =", *self._p(limit, "", 1)[:1]]'''),
):
    assert src.count(old) == 1, old[:70]
    src = src.replace(old, new)
Path(sys.argv[2]).write_text(src, encoding="utf-8")
