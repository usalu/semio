"""🔎 Where do the algorithms captions wrap? Flags a line break between a number and its unit or following word."""
import importlib, os, re, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(Path(__file__).parent / "🗑️generated/fonts/wrap")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
visuals._fonts.apply_body_font()
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
art = visuals._art
scene = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧬️algorithms.🐍️").Beat6_EvolutionarySearch
for key, en, de in scene.NARRATION:
    for lang, text in (("en", en), ("de", de)):
        lines = fonts.wrap_body_lines(text, font_size=fonts.CAPTION_FONT_SIZE, max_width=art.CAPTION_MAX_WIDTH)
        flag = len(lines) > 1 and re.search(r"[\d.,]+$", lines[0]) is not None
        if flag or "-v" in sys.argv:
            print(("SPLIT " if flag else "      ") + f"{key} {lang}: {lines}")
