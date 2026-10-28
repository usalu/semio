"""🔎 Render bundled body and mathematical glyphs for native visual inspection."""

import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import Scene, VGroup, DOWN, tempconfig

fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
out = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "glyph"
with tempconfig({"media_dir": str(out), "text_dir": str(out / "t"), "pixel_width": 1280, "pixel_height": 720, "write_to_movie": False, "verbosity": "ERROR"}):
    scene = Scene()
    fonts.apply_scene_style(scene)
    rows = VGroup(*[fonts.body_text(text, font_size=30, color="#f7f3e3") for text in ("Anta · semio", "ÄÖÜ äöü ß : ; . „ “ − × · =", "Δ Σ Φ Ψ η θ λ μ π ρ τ φ χ ψ ϑ", "← ↑ → ↓ ⇒ √ ∞ ∮ ≠ ≤ ≥ ₂")]).arrange(DOWN, buff=0.5)
    scene.add(rows)
    scene.camera.capture_mobjects(scene.mobjects)
    out.mkdir(parents=True, exist_ok=True)
    scene.camera.get_image().save(out / "glyphs.png")
print(f"[DEBUG] Glyph proof: {out / 'glyphs.png'}", flush=True)
