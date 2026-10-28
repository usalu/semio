"""🧨 Prove the independent glyph oracle rejects the former Anta registration collision."""

import importlib
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
theme = importlib.import_module(MODULE + ".🎨️theme.🐍️")
sources = theme.font_sources()


def collision_sources():
    """💥 Restore the conflicting same-family Anta subsets only in this child process."""
    return {**sources, theme.FONT_SANS: tuple(sorted((theme.ASSET_ROOT / "🔤️fonts/🚀️anta").rglob("*.ttf")))}


theme.font_sources = collision_sources
theme.font_files.cache_clear()
suite = unittest.defaultTestLoader.loadTestsFromName(MODULE + ".🔤️fonts.🧪️tests.🐍️.GlyphFallbackTest.test_native_rendered_glyphs_match_exact_source_outlines")
result = unittest.TextTestRunner(verbosity=2).run(suite)
if result.wasSuccessful():
    raise SystemExit("The former font collision was not rejected")
print("[DEBUG] The independent glyph oracle rejected the former same-family font collision")
