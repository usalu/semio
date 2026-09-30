"""📏️ W3-CODES: states payload-intrinsic refusals in the leaf schemas they belong to (negative-witness rule): hard value
bounds as standard keywords, cross-field invariants as `x-semio-invariant`. Text edits keep the hand formatting;
idempotent."""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
WFC3D = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
FORMS = "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
BOUNDS = [
    (f"{WFC3D}/⚖️change-tile-weight", ["weight"], '"exclusiveMinimum": 0'),
    (f"{WFC3D}/📐️resize-slot", ["width", "height", "depth"], '"exclusiveMinimum": 0'),
]
RASTER = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
INVERTIBLE = ("invertible-transform",
              "Every affine placement the payload carries is invertible: its linear part has a non-zero determinant (a·d − b·c ≠ 0).",
              "Jede affine Platzierung der Nutzlast ist invertierbar: Ihr linearer Anteil hat eine von null verschiedene Determinante (a·d − b·c ≠ 0).")
INVARIANTS = [
    (f"{WFC3D}/🔗️connect-slots", [("no-self-loop",
     "An adjacency edge connects two different slots, so fromSlotId differs from toSlotId.",
     "Eine Nachbarschaftskante verbindet zwei verschiedene Slots, fromSlotId unterscheidet sich also von toSlotId.")]),
    (f"{FORMS}/📨️commit-response", [("unique-answer-questions",
     "A response answers each question at most once: every answer names a distinct, non-empty questionId.",
     "Eine Antwort beantwortet jede Frage höchstens einmal: Jede Antwort nennt eine eigene, nicht leere questionId.")]),
    (f"{RASTER}/📐️change-layer-transform", [INVERTIBLE]),
    (f"{RASTER}/🎨️change-layer-pixels", [INVERTIBLE]),
    (f"{RASTER}/🎭️change-layer-mask", [INVERTIBLE, ("bounded-mask-area",
     "A mask covers at most 16,777,216 pixels: width × height ≤ 16,777,216.",
     "Eine Maske umfasst höchstens 16.777.216 Pixel: Breite × Höhe ≤ 16.777.216.")]),
]

failures = []
for leaf, properties, keyword in BOUNDS:
    path = ROOT / leaf / "🧬️schema" / "🔣️.json"
    text = path.read_text(encoding="utf-8")
    for name in properties:
        pattern = re.compile(r'("' + name + r'": \{\n(\s+)"type": "number",)(?!\n\s+' + re.escape(keyword) + ')')
        text, count = pattern.subn(lambda m: m.group(1) + "\n" + m.group(2) + keyword + ",", text)
        if count == 0 and keyword not in text.split('"' + name + '": {', 1)[1].split("}", 1)[0]:
            failures.append(f"{path}: {name}")
    path.write_text(text, encoding="utf-8")
    print("[w3-codes] bounded", leaf, properties)
for leaf, invariants in INVARIANTS:
    path = ROOT / leaf / "🧬️schema" / "🔣️.json"
    text = path.read_text(encoding="utf-8")
    if '"x-semio-invariant"' in text:
        continue
    entries = ",\n".join(f'    {{\n      "id": "{identifier}",\n      "description": {{\n        "en": "{en}",\n        "de": "{de}"\n      }}\n    }}' for identifier, en, de in invariants)
    block = f'  "x-semio-invariant": [\n{entries}\n  ],\n'
    lines = text.splitlines(keepends=True)
    index = next((i for i, line in enumerate(lines) if re.match(r'  "title": ?"', line) and line.rstrip().endswith(',')), None)
    index = index if index is not None else next((i for i, line in enumerate(lines) if re.match(r'  "\$id": ?"', line)), None)
    if index is None:
        failures.append(str(path))
        continue
    lines.insert(index + 1, block)
    path.write_text("".join(lines), encoding="utf-8")
    print("[w3-codes] declared", [identifier for identifier, _, _ in invariants], "in", leaf)
if failures:
    sys.exit("unmatched: " + ", ".join(failures))
