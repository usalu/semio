"""🧪 Show repaired real source labels and all declared mathematical symbols."""
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2]).resolve()
OUT.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, str(ROOT))
from manim import Scene, UP, WHITE, tempconfig
from manim_fonts import BODY_FONT, apply_body_font, body_text
from manim_visuals import math_text

CASES = json.loads((ROOT / "glyph_fallback.tests.json").read_text(encoding="utf-8"))
records = []


class GlyphAfter(Scene):
    """✅ Render supported Georgia text alongside symbol-only fallback."""

    def construct(self):
        """📐 Reproduce the three affected source label categories without animation."""
        apply_body_font()
        title = body_text("Portable mathematical glyphs", font_size=30, color=WHITE).move_to([0, 3.1, 0])
        german = body_text(CASES["plain_samples"][0], font_size=24, color=WHITE).move_to([0, 2.25, 0])
        required = CASES["required_symbols"]
        split = len(required) // 2
        rows = [body_text(" ".join(part), font_size=25, color=WHITE).move_to([0, y, 0]) for part, y in ((required[:split], 1.45), (required[split:], 0.65))]
        physical = math_text(r"\vartheta_{\mathrm{e}}=-5\,\mathrm{°C}\quad\vartheta_{\mathrm{i}}=20\,\mathrm{°C}\quad\oint", font_size=30).move_to([0, -0.4, 0])
        cooling = body_text("← Lüftung · Kapitel 3:", font_size=26, color="#6999DA").move_to([0, -1.5, 0])
        balance = body_text("berechnet  →", font_size=26, color=WHITE).move_to([0, -2.5, 0])
        self.add(title, german, *rows, physical, cooling, balance)
        for label, item in (("german", german), ("symbols_a", rows[0]), ("symbols_b", rows[1]), ("cooling", cooling), ("balance", balance)):
            records.append({"label": label, "font": item.font, "map": item.t2f, "cache_name": Path(item.file_name).name})


settings = {"media_dir": str(OUT), "text_dir": str(OUT / "t"), "images_dir": str(OUT / "i"), "tex_dir": str(OUT / "x"), "save_last_frame": True, "write_to_movie": False, "pixel_width": 1600, "pixel_height": 900, "frame_rate": 15, "background_color": "#080b0f", "progress_bar": "none", "verbosity": "WARNING"}
with tempconfig(settings):
    GlyphAfter().render()
result = {"font_helper_sha256": hashlib.sha256((ROOT / "manim_fonts.py").read_bytes()).hexdigest(), "body_font": BODY_FONT, "required_symbols": CASES["required_symbols"], "records": records}
(OUT / "result.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("[DEBUG] Repaired glyph labels:", json.dumps(result, ensure_ascii=False))
