"""🧩️ Compose the staged Terrain change against the live shared source tree."""

from difflib import unified_diff
from pathlib import Path
import re


TICKET = Path(__file__).resolve().parent
PATCHES = [
    "terrain-rectangular-storage-current.patch",
    "terrain-rectangular-wire-facets.patch",
    "terrain-renderer-resolution.patch",
    "terrain-rectangular-ascii.patch",
    "terrain-rectangular-export-work.patch",
    "terrain-rectangular-import-work.patch",
    "terrain-rectangular-producer.patch",
    "terrain-rectangular-edit-reducer.patch",
    "terrain-rectangular-create-and-grade.patch",
    "terrain-rectangular-brush-precision.patch",
]


def apply_patch(text, block):
    lines = text.splitlines(keepends=True)
    position = 0
    for hunk in re.split(r"(?m)(?=^@@ )", block)[1:]:
        header, *body = hunk.splitlines(keepends=True)
        body = [line for line in body if line.startswith((" ", "-", "+"))]
        old = [line[1:] for line in body if line[0] != "+"]
        new = [line[1:] for line in body if line[0] != "-"]
        match = re.match(r"@@ -(\d+)(?:,\d+)? \+", header)
        expected = max(0, int(match.group(1)) - 1) if match else 0
        if lines[expected:expected + len(old)] == old and expected >= position:
            where = expected
        elif old:
            choices = [at for at in range(position, len(lines)) if lines[at:at + len(old)] == old]
            if len(choices) != 1:
                raise ValueError(header.strip())
            where = choices[0]
        else:
            where = min(expected, len(lines))
        lines[where:where + len(old)] = new
        position = where + len(new)
    return "".join(lines)


if __name__ == "__main__":
    live = {}
    staged = {}
    errors = []
    for patch in PATCHES:
        contents = (TICKET / patch).read_text()
        for block in filter(None, re.split(r"(?m)(?=^--- (?:a/|/dev/null))", contents)):
            lines = block.splitlines()
            path = Path(lines[1].removeprefix("+++ b/"))
            if patch == "terrain-rectangular-create-and-grade.patch" and path.parent.name == "📐️elevation-edit":
                added = [line[1:] for line in lines[2:] if line.startswith("+")]
                assert all(line in staged[path].splitlines() for line in added)
                continue
            if path not in live:
                live[path] = path.read_text() if path.exists() else ""
                staged[path] = live[path]
            try:
                staged[path] = apply_patch(staged[path], block)
            except ValueError as error:
                errors.append(f"{patch} {path} {error}")
    if errors:
        raise SystemExit("\n".join(errors))
    out = TICKET / "terrain-rectangular-combined.patch"
    out.write_text("".join(
        line
        for path, before in live.items()
        for line in unified_diff(before.splitlines(True), staged[path].splitlines(True),
                                 "a/" + str(path) if path.exists() else "/dev/null", "b/" + str(path))
    ))
    print(f"[DEBUG] Composed {len(live)} Terrain paths into {out.name}")
