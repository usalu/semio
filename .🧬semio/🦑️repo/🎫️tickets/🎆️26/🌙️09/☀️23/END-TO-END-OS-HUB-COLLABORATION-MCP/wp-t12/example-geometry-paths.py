#!/usr/bin/env python3
"""📐️ Prepared patch (rule 20, apply after W2's `--packages all`): generation3d `📐️example-geometry-3d-1`'s Python
oracle answered "no oracle registration" for all 8 rows because it registers one handler per example it DISCOVERS,
and it still globbed the retired `<example>/🧪️tests/🧩️example/🔣️.json` location: the expected-geometry fixtures live
at `<example>/🧫️fixtures/🧩️example/🔣️.json`, which the Rust subject (`include_str!`) and the TypeScript lane already
read. The reference, its module docstring and the feature's description follow the fixture to its one location.
Usage: example-geometry-paths.py --dry-run | --write [--root <dir>]"""
import sys
from pathlib import Path

root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
CASE = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️example-geometry-3d-1"
EDITS = {
    f"{CASE}/🐍️.py": [
        ("`📚️examples/<example>/🧪️tests/🧩️example/🔣️.json` fixtures", "`📚️examples/<example>/🧫️fixtures/🧩️example/🔣️.json` fixtures"),
        ('EXAMPLES.glob("*/🧪️tests/🧩️example/🔣️.json")', 'EXAMPLES.glob("*/🧫️fixtures/🧩️example/🔣️.json")'),
        ('(example_dir / "🧪️tests" / "🧩️example" / "🔣️.json")', '(example_dir / "🧫️fixtures" / "🧩️example" / "🔣️.json")'),
    ],
    f"{CASE}/🥒️.feature": [("Each example gains one `🧪️tests/🧩️example/🔣️.json` expected-stats", "Each example gains one `🧫️fixtures/🧩️example/🔣️.json` expected-stats")],
}
write = "--write" in sys.argv
problems, edits = [], {}
for path, pairs in EDITS.items():
    source = (root / path).read_text(encoding="utf-8")
    for old, new in pairs:
        if source.count(old) != 1:
            problems.append(f"{path}: {source.count(old)} × {old[:60]!r}")
            continue
        source = source.replace(old, new)
    if "🧪️tests/🧩️example" in source or '"🧪️tests" / "🧩️example"' in source:
        problems.append(f"{path}: a retired fixture path is left")
    edits[path] = source
print(f"files={len(edits)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, source in edits.items():
        (root / path).write_text(source, encoding="utf-8")
