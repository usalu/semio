from pathlib import Path
a = Path("🎯️optimization/🎬️clip/🎨️visuals/🐍️.py")
s = a.read_text(encoding="utf-8")
old = 'Path(_art.__file__).parent / "🫟️silhouettes/🔣️.json", *_theme.source_files())'
assert old in s
a.write_text(s.replace(old, 'Path(_art.__file__).parent / "🫟️silhouettes/🔣️.json", Path(_art.__file__).parent / "🔣️icons/🔣️.json",\n            Path(_art.__file__).parent / "🧍️staffage/🔣️.json", *_theme.source_files())'), encoding="utf-8", newline="")
b = Path("⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py")
s = b.read_text(encoding="utf-8")
old = 'visuals / "🔣️.json", visuals / "🫟️silhouettes/🔣️.json"}'
assert old in s
b.write_text(s.replace(old, 'visuals / "🔣️.json", visuals / "🫟️silhouettes/🔣️.json", visuals / "🔣️icons/🔣️.json", visuals / "🧍️staffage/🔣️.json"}'), encoding="utf-8", newline="")
