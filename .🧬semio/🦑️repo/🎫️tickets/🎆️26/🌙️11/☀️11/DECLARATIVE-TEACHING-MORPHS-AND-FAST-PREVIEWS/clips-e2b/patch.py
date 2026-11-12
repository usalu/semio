"""Scratch helper for batch E2b: apply exact, unique string replacements to a clip script (re-reads the file right before writing)."""

import sys
from pathlib import Path

ANCHOR = 'set_vo_language("de")\n'
HEADER = ANCHOR + '_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'
STAGE = "        apply_scene_style(self)\n"


def apply(path, edits, scenes=(), name="stage"):
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    if "_morph = importlib" not in text:
        assert text.count(ANCHOR) == 1, "anchor"
        text = text.replace(ANCHOR, HEADER)
    for scene in scenes:
        head = f"class {scene}("
        start = text.index(head)
        marker = text.index(STAGE, start)
        after = text[marker + len(STAGE):marker + len(STAGE) + 60]
        if f"{name} = _morph.stage" not in after:
            text = text[:marker] + STAGE + f"        {name} = _morph.stage(self, TRANSITIONS)\n" + text[marker + len(STAGE):]
    for old, new in edits:
        if text.count(old) == 0 and text.count(new) == 1:
            continue
        assert text.count(old) == 1, f"not unique ({text.count(old)}): {old[:80]}"
        text = text.replace(old, new)
    file.write_text(text, encoding="utf-8", newline="\n")
    print("patched", ascii(str(file)), len(edits), "edits")
