"""Splice the audited NARRATION block into the experiments scene file."""
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
scene = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️experiments/🐍️.py"
text = scene.read_text(encoding="utf-8")
start = text.index("    NARRATION = [")
end = text.index("    def construct(self):")
block = (TICKET / "experiments-narration-block.txt").read_text(encoding="utf-8")
with open(scene, "w", encoding="utf-8", newline="\n") as handle:
    handle.write(text[:start] + block + "\n" + text[end:])
print("spliced", block.count('["'), "clauses")
