from pathlib import Path

V = Path("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals")
path = V / "🐍️.py"
text = path.read_text(encoding="utf8")

start = text.index("#region Respiration split")
end = text.index("#endregion", start) + len("#endregion")
text = text[:start] + text[end:].lstrip("\n")

start = text.index("def radiator(center")
end = text.index("#endregion", start)
text = text[:start].rstrip("\n") + "\n" + text[end:]
open(str(path), "w", encoding="utf8", newline="").write(text)
print("ok")
