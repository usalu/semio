import importlib, os, sys
from pathlib import Path
T = Path(__file__).resolve().parent
ROOT = next(p for p in T.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(T / "🗑️generated/fonts/probe")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
missing = [c for c in map(chr, range(33, 127)) if not fonts._font_supports("Share Tech Mono", c)]
print("[DEBUG] mono missing", missing)
missing = [c for c in "ØΩ²³±−×·→≈≤≥°Ä ÖÜäöüß–—" if not fonts._font_supports("Anta", c)]
print("[DEBUG] sans missing", missing)
import manim
t = manim.Text("a b c", font="Share Tech Mono", font_size=20)
print("[DEBUG] glyphs of 'a b c'", len(t.submobjects))
