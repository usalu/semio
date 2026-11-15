from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = ROOT.read_text(encoding="utf-8")
old = r"\text{ in }\mathrm"
new = r"\;\text{in}\;\;\mathrm"
print(text.count(old))
text = text.replace(old, new)
ROOT.write_text(text, encoding="utf-8")
