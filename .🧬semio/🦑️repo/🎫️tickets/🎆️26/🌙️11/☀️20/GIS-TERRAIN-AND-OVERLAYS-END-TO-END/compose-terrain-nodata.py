from difflib import unified_diff
from pathlib import Path
import re
import runpy
import sys

ticket = Path(__file__).resolve().parent
apply_patch = runpy.run_path(str(ticket / "compose-terrain-resolution.py"))["apply_patch"]
base = {}
changes = {}
for file in map(Path, sys.argv[1:]):
    for block in filter(None, re.split(r"(?m)(?=^--- (?:a/|/dev/null))", file.read_text())):
        source, destination, *_ = block.splitlines()
        assert destination.startswith("+++ b/")
        path = Path(destination.removeprefix("+++ b/"))
        assert not path.is_absolute() and len(str(path)) <= 256
        if path not in base:
            original = path.read_text() if path.is_file() else None
            assert (original is None) == (source == "--- /dev/null"), path
            base[path] = original
            changes[path] = original or ""
        try:
            changes[path] = apply_patch(changes[path], block)
        except ValueError as error:
            raise ValueError(f"{path}: {error}") from error
output = ticket / "🗑️generated/terrain-nodata-combined.patch"
output.write_text("".join(
    line for path, before in base.items()
    for line in unified_diff((before or "").splitlines(True), changes[path].splitlines(True),
                             "/dev/null" if before is None else f"a/{path}", f"b/{path}")
))
print(f"[DEBUG] Composed {len(changes)} Terrain NoData source paths.")
