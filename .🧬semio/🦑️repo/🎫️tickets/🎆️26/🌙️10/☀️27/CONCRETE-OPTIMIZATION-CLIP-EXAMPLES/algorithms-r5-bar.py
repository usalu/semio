"""📊 Round 5: the scene uses the toolkit's energy_bars (digits, total, rule) instead of its own bar (scratch copy, then copied into place)."""
import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
a = src.index("    def _bar(self, point=BAR_AT")
b = src.index("    def _seasons(self):", a)
src = src[:a] + src[b:] if src.index("    def _seasons(self):") > a else src
old = "        self.bar = self._bar()"
assert src.count(old) == 1
src = src.replace(old, '        self.bar = self.energy_bars(self.focus, BAR_AT, BAR_H, 900, compact=True, intensity=False, digits=1, size=13, rule=False, total=lambda design, found: _figures(design)["total"])')
Path(sys.argv[2]).write_text(src, encoding="utf-8")
