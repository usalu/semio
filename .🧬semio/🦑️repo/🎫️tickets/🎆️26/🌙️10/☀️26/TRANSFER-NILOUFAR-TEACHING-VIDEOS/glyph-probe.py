"""🧪 Diagnose actual Pango glyph output and candidate per-character fallback."""
import ast
import hashlib
import json
import os
from pathlib import Path
import sys
import unicodedata
import xml.etree.ElementTree as ET

ROOT = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2]).resolve()
OUT.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, str(ROOT))
import cairo
import manimpango
from manim import DOWN, LEFT, Scene, Text, UP, WHITE, config, tempconfig
from manim_fonts import BODY_FONT, apply_body_font

module = ast.parse((ROOT / "manim_visuals.py").read_text(encoding="utf-8"))
symbols = {}
for node in module.body:
    if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "_MATH_SYMBOLS" for target in node.targets):
        symbols = ast.literal_eval(node.value)
arrows = set()
for path in [*ROOT.joinpath("energy").rglob("scene_*.py"), ROOT / "manim_visuals.py"]:
    tree = ast.parse(path.read_text(encoding="utf-8"))
    docstrings = {id(node.body[0].value) for node in ast.walk(tree) if isinstance(node, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)) and node.body and isinstance(node.body[0], ast.Expr) and isinstance(node.body[0].value, ast.Constant) and isinstance(node.body[0].value.value, str)}
    for node in ast.walk(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, str) and id(node) not in docstrings:
            arrows.update(char for char in node.value if "ARROW" in unicodedata.name(char, ""))
required = sorted(set("".join(symbols.values())) | arrows | set("−√≠÷τ²³₂"))
case = {"body_font": BODY_FONT, "declared_math": symbols, "arrows": sorted(arrows), "required": required, "german": "Größe Wärme Lüftung ÄÖÜ äöü ß", "representative": "ϑ ← ↑ → ↓ ⇒ ∮"}
(Path(__file__).parent / "glyph-cases.json").write_text(json.dumps(case, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
available = set(manimpango.list_fonts())
candidates = [name for name in ("Georgia", "Cambria Math", "DejaVu Math TeX Gyre", "DejaVu Serif", "STIX Two Math", "STIXGeneral", "Segoe UI Symbol", "Apple Symbols") if name in available]


def coverage(font, text):
    face = cairo.ToyFontFace(font)
    scaled = cairo.ScaledFont(face, cairo.Matrix(xx=60, yy=60), cairo.Matrix(), cairo.FontOptions())
    return {char: all(glyph[0] != 0 for glyph in scaled.text_to_glyphs(0, 0, char, False)) for char in text}


coverage_by_font = {font: coverage(font, required) for font in candidates}
base_missing = [char for char in required if not coverage_by_font[BODY_FONT][char]]
fallbacks = {char: next(font for font in candidates if font != BODY_FONT and coverage_by_font[font][char]) for char in base_missing}
(OUT / "coverage.json").write_text(json.dumps({"fonts": candidates, "coverage": coverage_by_font, "missing": base_missing, "fallbacks": fallbacks}, indent=2, ensure_ascii=False), encoding="utf-8")
rows = []


class GlyphProbe(Scene):
    """🔎 Show real unanimated glyphs before and with an explicit fallback map."""

    def construct(self):
        """🔤 Save actual SVG cache identities and a visual comparison matrix."""
        apply_body_font()
        items = [("Georgia", BODY_FONT, {}), ("Mapped", BODY_FONT, fallbacks)]
        items += [(font, font, {}) for font in candidates if font != BODY_FONT][:3]
        sample = " ".join(required)
        for index, (label, font, mapping) in enumerate(items):
            y = 3.1 - index * 1.15
            title = Text(label, font=BODY_FONT, font_size=18, color=WHITE).move_to([-5.9, y + 0.25, 0])
            glyphs = Text(sample, font=font, font_size=21, color=WHITE, t2f=mapping).move_to([0.5, y + 0.25, 0])
            if glyphs.width > 11.7:
                glyphs.scale_to_fit_width(11.7)
            german = Text(case["german"], font=font, font_size=19, color=WHITE, t2f=mapping).move_to([0.5, y - 0.22, 0])
            self.add(title, glyphs, german)
            svg = Path(glyphs.file_name)
            rows.append({"label": label, "font": font, "map": mapping, "svg": str(svg), "cache_name": svg.name, "sha256": hashlib.sha256(svg.read_bytes()).hexdigest(), "symbol_definitions": len(ET.parse(svg).findall(".//{http://www.w3.org/2000/svg}symbol"))})
        plain = Text(case["german"], font=BODY_FONT, font_size=24)
        unchanged = Text(case["german"], font=BODY_FONT, font_size=24, t2f={})
        rows.append({"plain_cache_name": Path(plain.file_name).name, "empty_map_cache_name": Path(unchanged.file_name).name, "same_plain_cache": plain.file_name == unchanged.file_name})


settings = {"media_dir": str(OUT), "text_dir": str(OUT / "t"), "images_dir": str(OUT / "i"), "tex_dir": str(OUT / "x"), "save_last_frame": True, "write_to_movie": False, "pixel_width": 1600, "pixel_height": 900, "frame_rate": 15, "background_color": "#080b0f", "progress_bar": "none", "verbosity": "WARNING"}
with tempconfig(settings):
    GlyphProbe().render()
result = {"case": case, "fonts": candidates, "coverage": coverage_by_font, "missing": base_missing, "fallbacks": fallbacks, "rows": rows}
(OUT / "result.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("[DEBUG] Glyph probe:", json.dumps({"fonts": candidates, "missing": base_missing, "fallbacks": fallbacks, "rows": rows}, ensure_ascii=False))
