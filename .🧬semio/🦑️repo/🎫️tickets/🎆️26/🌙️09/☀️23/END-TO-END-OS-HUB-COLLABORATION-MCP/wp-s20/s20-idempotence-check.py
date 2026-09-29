"""🔁️ S20: proves a prepared set script is idempotent WITHOUT touching the tree — applies its hunks in memory, then
re-classifies every hunk against the result: each must read `applied` (and a second in-memory pass changes nothing).
Works for scripts exposing `hunks()` → (file, before, after) [+ optional WHOLE]; usage: python3 s20-idempotence-check.py <script.py>
"""
import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("setscript", sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
root = module.ROOT
texts: dict[str, str] = {}
for rel, before, after in module.hunks():
    text = texts.setdefault(rel, (root / rel).read_text())
    if (after != "" and text.count(after) == 1) or (after == "" and before not in text):
        continue
    assert text.count(before) == 1, f"{rel}: anchor {text.count(before)}×"
    texts[rel] = text.replace(before, after)
bad = []
for rel, before, after in module.hunks():
    text = texts[rel]
    if not ((after != "" and text.count(after) == 1) or (after == "" and before not in text)):
        bad.append(f"{rel.split('/')[-2]}: {before.strip().splitlines()[0][:70] if before.strip() else '<empty>'}")
print("idempotent" if not bad else "NOT idempotent:\n" + "\n".join(bad))
