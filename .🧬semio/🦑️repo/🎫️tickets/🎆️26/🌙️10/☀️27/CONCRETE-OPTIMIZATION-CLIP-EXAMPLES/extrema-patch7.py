"""🩹 Patch the extrema scene: smaller plan, taller section corner, two-line east reference label."""
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:70])
    text = text.replace(old, new)


swap("PLAN_AT, PLAN_SCALE = np.array([-5.0, 1.3, 0.]), .18", "PLAN_AT, PLAN_SCALE = np.array([-4.8, 1.55, 0.]), .15")
swap("SECTION_BOX = (-7.0, -2.55, -2.95, -.85)", "SECTION_BOX = (-7.0, -2.55, -2.95, -.15)")
a = text.index("        line_tag = self._t(f\"{self.tr('east reference'")
b = text.index("\n", a) + 1
text = text[:a] + '''        line_tag = _mn.VGroup(self._l("east reference", "Ostfassade als Referenz", size=14, color=WARN, width=1.9, edge="left"),
                              self._t(self._kwh(self.reference_energy), size=14, color=WARN, width=1.9, edge="left")).arrange(_mn.DOWN, aligned_edge=_mn.LEFT, buff=.04)
        line_tag.move_to(axes.c2p(self.low, self.reference_energy) + _mn.UP * .42 + _mn.RIGHT * (.1 + line_tag.width / 2))
''' + text[b:]
path.write_text(text, encoding="utf-8")
