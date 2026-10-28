"""Apply a list of exact text replacements to the pareto scene (run with -I and a path argument)."""
import sys
from pathlib import Path

target = Path(sys.argv[1])
text = target.read_text(encoding="utf-8")
pairs = [
    ('        self.step("grid", mn.FadeIn(total), mn.FadeOut(s.better), mn.FadeOut(s.cursor), mn.FadeIn(s.plain), mn.LaggedStart(',
     '        self.remove(s.cursor)\n        self.add(s.plain)\n        self.step("grid", mn.FadeIn(total), mn.FadeOut(s.better), mn.LaggedStart('),
]
for old, new in pairs:
    if old not in text:
        raise SystemExit("missing: " + old[:70])
    text = text.replace(old, new)
target.write_text(text, encoding="utf-8")
print("ok")
