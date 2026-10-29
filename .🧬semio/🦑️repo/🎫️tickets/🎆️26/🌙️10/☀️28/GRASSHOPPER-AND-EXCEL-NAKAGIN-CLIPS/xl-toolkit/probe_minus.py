import importlib, sys, copy
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
f = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
print(f.BODY_FONT if hasattr(f, "BODY_FONT") else "?", f.resolve_body_font())
for ch in ("-", "−", "–", "_"):
    t = f.body_text("a" + ch + "b", font_size=40, color="#ffffff")
    print(repr(ch), round(t.width, 3), round(t.height, 3), len(t.submobjects))
print(f._font_supports(f.resolve_body_font(), "-"), f._font_supports(f.resolve_body_font(), "−"))
