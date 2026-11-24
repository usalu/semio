"""Rewrites the en1990 mutation-vector placeholders of FactorOverride.factor ("normal", "normal-alt") to members of the FactorKind enum, in every role file of every bundle."""
import pathlib, re, sys

ROOT = pathlib.Path(sys.argv[1])
PLACEHOLDERS = {"normal": "gamma", "normal-alt": "psi0"}
PATTERN = re.compile(r'("(?:factor|newFactor)": )"(normal(?:-alt)?)"')
changed = 0
for path in sorted(ROOT.rglob("🔣️.*.json")):
    text = path.read_text(encoding="utf8")
    rewritten = PATTERN.sub(lambda m: f'{m.group(1)}"{PLACEHOLDERS[m.group(2)]}"', text)
    if rewritten != text:
        path.write_text(rewritten, encoding="utf8")
        changed += 1
print(f"rewrote {changed} files")
