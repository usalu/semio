import importlib, sys
from pathlib import Path
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
root = Path("🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip")
for doc in sorted(root.glob("*/🔀️transitions/🔣️.json")):
    d = spec.load(doc)
    problems = spec.validate(d)
    print(doc.parent.parent.name, len(d.transitions), [str(p) for p in problems][:5])
