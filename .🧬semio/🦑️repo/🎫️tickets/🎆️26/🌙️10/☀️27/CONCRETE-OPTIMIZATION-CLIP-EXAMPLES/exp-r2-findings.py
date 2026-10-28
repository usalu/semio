"""List the layout findings of the experiments scene once per distinct finding with the keys where it occurs (usage: findings.py <en|de>)."""
import sys
from pathlib import Path

folder = Path(__file__).resolve().parent / "🗑️generated/frames/Beat3_FactorialExperiments"
text = (folder / f"Beat3_FactorialExperiments-{sys.argv[1]}-layout.md").read_text(encoding="utf-8")
key, seen = None, {}
for line in text.splitlines():
    if line.startswith("## "):
        key = line[3:]
    elif line.startswith("- ") and line != "- none":
        seen.setdefault(line[2:], []).append(key)
out = []
for finding, keys in seen.items():
    out.append(f"{keys[0]}..{keys[-1]} x{len(keys)}: {finding[:200]}")
sys.stdout.buffer.write(("\n".join(out) + "\nTOTAL distinct " + str(len(seen)) + "\n").encode("utf-8"))
