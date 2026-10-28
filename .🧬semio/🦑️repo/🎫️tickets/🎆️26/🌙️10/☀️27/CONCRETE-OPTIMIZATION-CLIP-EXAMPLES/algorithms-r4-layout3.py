"""📐 Round 4 layout edits, third pass: the map-minimum legend is a sibling of the star, not a child; pattern 16-run tag moves to the right column."""
import re
import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
src = src.replace('''        self.heat["star"].add(mn.VGroup(mn.Star(''', '''        self.star_pair = mn.VGroup(self.heat["star"], mn.VGroup(mn.Star(''', 1)
old = '''self._chip([self.tr("map minimum", "Kartenminimum")], (-1.52, 1.82, 0), size=14, color=GOOD, width=1.5)))'''
assert src.count(old) == 1
src = src.replace(old, old + ")", 1)
src = src.replace('mn.FadeIn(heat["star"])', "mn.FadeIn(self.star_pair)").replace('mn.FadeOut(heat["star"])', "mn.FadeOut(self.star_pair)")
src = src.replace('mn.FadeIn(self.heat["star"])', "mn.FadeIn(self.star_pair)")
src = src.replace('*old_dots, self.heat["star"]', "*old_dots, self.star_pair")
old = '''self._chip([self._n(reached), self.tr("simulations: within 1 %", "Simulationen: innerhalb 1 %")], spot + mn.LEFT * 1.5, color=PATTERN, size=14, width=2.5).set_z_index(8)]'''
assert src.count(old) == 1
src = src.replace(old, '''self._chip([self.tr("ring: within 1 % at", "Ring: innerhalb 1 % bei"), self._n(reached)], (5.75, 1.35, 0), color=PATTERN, size=14, width=2.4)]''')
Path(sys.argv[2]).write_text(src, encoding="utf-8")
