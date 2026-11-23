"""📌️ One-time edit of the section catalog pins to the sections the assets declare since 2026-11-20/22 (decisions texts of the assets name them): masonry wall plan B-B at 1.55 m, retractable bollard and sectional garage door elevation A-A at all four scales, and no front section in any asset of the elevation catalog (the bollard exception of the test is removed). Each replacement must match exactly once; files are re-read and written atomically."""
import os
import pathlib

STAFFAGE = pathlib.Path(__file__).resolve().parents[8] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage"
EDITS = {
    STAFFAGE / "🧫️fixtures" / "✂️section-views" / "🔣️.json": [
        ('"masonry-wall-with-lintel": { "front": 1.25, "plan": 1.5 }', '"masonry-wall-with-lintel": { "front": 1.25, "plan": 1.55 }'),
        ('"retractable-bollard": { "50": 0, "100": 0 }', '"retractable-bollard": { "50": 0, "100": 0, "200": 0, "500": 0 }'),
        ('"sectional-garage-door": { "50": 0, "100": 0 }', '"sectional-garage-door": { "50": 0, "100": 0, "200": 0, "500": 0 }'),
    ],
    STAFFAGE / "🧪️tests" / "✂️section-views" / "🟦️.ts": [
        ('expect(rep.sections?.front === undefined || id === "retractable-bollard", `1:${rep.denominator} front`).toBe(true);', 'expect(rep.sections?.front, `1:${rep.denominator} front`).toBeUndefined();'),
    ],
}

for file, replacements in EDITS.items():
    text = file.read_text(encoding="utf-8")
    for old, new in replacements:
        assert text.count(old) == 1, f"{file.name}: {old!r} is not there exactly once"
        text = text.replace(old, new)
    temporary = file.with_name(file.name + ".tmp")
    temporary.write_text(text, encoding="utf-8")
    os.replace(temporary, file)
    print("patched", file)
