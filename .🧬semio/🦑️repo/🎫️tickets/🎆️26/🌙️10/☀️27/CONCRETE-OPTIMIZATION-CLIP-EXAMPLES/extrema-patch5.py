"""🩹 Patch the extrema scene: define the saving formula before the maximize step uses it."""
import sys
from pathlib import Path

path = Path(sys.argv[1])
lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
formula = next(i for i, line in enumerate(lines) if line.startswith("        formula = _mn.VGroup("))
step = next(i for i, line in enumerate(lines) if line.startswith('        self.step("maximize",'))
assert formula > step
block = lines.pop(formula)
lines.insert(step, block)
path.write_text("".join(lines), encoding="utf-8")
