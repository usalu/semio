import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
f = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
print(f.__file__, hasattr(f, "beat_subtitle"), hasattr(f, "scene_title"), [n for n in dir(f) if "title" in n.lower() or "sub" in n.lower()])
