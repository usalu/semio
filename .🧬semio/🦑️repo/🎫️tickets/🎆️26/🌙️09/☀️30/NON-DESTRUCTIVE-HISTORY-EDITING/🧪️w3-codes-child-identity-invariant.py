"""🪪️ W3-CODES: declares the payload-intrinsic `child-identity` invariant (`x-semio-invariant`) on every leaf
schema whose diff refuses a composed child handle with `mutation.invariant` (formerly `mutation.child-identity`).
Inserts text after the top-level `"title"` line so the file keeps its hand formatting. Idempotent."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
CAD = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SEMIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
LEAVES = [
    f"{CAD}/⚡create-energy-model",
    f"{CAD}/🏛️create-structure-classic-model",
    f"{CAD}/🏢create-building-model",
    f"{CAD}/📐️create-drawing",
    f"{CAD}/🧱create-shape-model",
    f"{SEMIO}/📦️object/🧬️schema/🧬️mutations/🏷️create-properties",
    f"{SEMIO}/📦️object/🧬️schema/🧬️mutations/🕸️create-mesh",
    f"{SEMIO}/📦️object/🧬️schema/🧬️mutations/🧱create-brep",
    f"{SEMIO}/🧰️kit/🧬️schema/🧬️mutations/🏗️create-object",
    f"{SEMIO}/🧰️kit/🧬️schema/🧬️mutations/🏛️create-model",
    f"{SEMIO}/🧰️kit/🧬️schema/🧬️mutations/🏷️create-properties",
]
BLOCK = """  "x-semio-invariant": [
    {
      "id": "child-identity",
      "description": {
        "en": "A composed child's id equals its target artifact id, and the target uses the s.stdio.semio@v1 dialect of the expected subset.",
        "de": "Die Id eines eingebetteten Kindes entspricht der Artefakt-Id seines Ziels, und das Ziel verwendet den Dialekt s.stdio.semio@v1 des erwarteten Subsets."
      }
    }
  ],
"""

failures = []
for leaf in LEAVES:
    path = ROOT / leaf / "🧬️schema" / "🔣️.json"
    text = path.read_text(encoding="utf-8")
    if '"x-semio-invariant"' in text:
        continue
    lines = text.splitlines(keepends=True)
    index = next((i for i, line in enumerate(lines) if line.startswith('  "title": ')), None)
    if index is None:
        failures.append(str(path))
        continue
    lines.insert(index + 1, BLOCK)
    path.write_text("".join(lines), encoding="utf-8")
    print("[w3-codes] declared child-identity in", leaf)
if failures:
    sys.exit("no top-level title line: " + ", ".join(failures))
