from difflib import unified_diff
from pathlib import Path
import re
import runpy
import sys

ticket = Path(__file__).resolve().parent
terrain = Path(sys.argv[1])
facets = terrain / "🧬️schema/🗺️imported-map/🏔️elevation"
replacements = {
    "🔗️.graphql": [("  rows: [[Int!]!]!", "  rowsMillimetres: [[Int!]!]!", 2)],
    "🛰️.proto": [
        ("sint32 height_meters", "sint32 height_millimetres", 1),
        ("ElevationRow rows = 2;", "ElevationRow rows_millimetres = 2;", 1),
    ],
}
diff = []
for name, fields in replacements.items():
    path = facets / name
    before = path.read_text(encoding="utf-8")
    after = before
    for old, new, count in fields:
        if after.count(old) != count:
            raise ValueError(f"Unexpected {path} field {old!r}")
        after = after.replace(old, new)
    diff.extend(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{path}", f"b/{path}"))
(ticket / "terrain-rectangular-wire-facets.patch").write_text("".join(diff), encoding="utf-8")

compose = runpy.run_path(str(ticket / "compose-terrain-resolution.py"), run_name="__main__")
destination = ticket / "🗑️generated" / "rectangular-wire"
destination.mkdir(parents=True, exist_ok=True)
facets_by_name = {"🔣️.json": "elevation.json", "🔗️.graphql": "elevation.graphql", "🛰️.proto": "elevation.proto"}
patch = (ticket / "terrain-rectangular-combined.patch").read_text(encoding="utf-8")
for block in re.split(r"(?m)(?=^--- (?:a/|/dev/null))", patch):
    if not block: continue
    path = Path(block.splitlines()[1].removeprefix("+++ b/"))
    if path.parent == facets and path.name in facets_by_name:
        staged = compose["apply_patch"](path.read_text(encoding="utf-8"), block)
        (destination / facets_by_name[path.name]).write_text(staged, encoding="utf-8")
