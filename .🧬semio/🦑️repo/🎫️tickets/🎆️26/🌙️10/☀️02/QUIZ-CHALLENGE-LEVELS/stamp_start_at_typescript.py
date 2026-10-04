"""🕰️ Puts the `at` of `start-run` first in the `start(...)` helper calls of the TypeScript run-lifecycle suite: each call takes
the decision time of the context it is decided with on the same line (`context(n)`, `capped(n)`, `brokenContext(n)`, a bare
`n]` pair), or 0 where it is never decided. Prints what it stamped per line; writes through the existing file handle so
a process that maps the file cannot block it.
"""
import pathlib
import re

PATH = pathlib.Path(__file__).resolve().parents[7] / "🧰️framework" / "🛍️products" / "❓️quiz" / "🧪️tests" / "🔁️run-lifecycle" / "🟦️.ts"
CALL = re.compile(r"(?<!\w)(?<!\w\.)start\(")
STAMPED = re.compile(r"^\d[\d_]*(?:,|$)")
DECIDED = re.compile(r"(?:context|capped|brokenContext)\((\d[\d_]*)|^\), (\d[\d_]*)\]")


def closing(line, opening):
    """🔚️ The index just past the parenthesis that closes the one at ``opening``."""
    depth = 0
    for index in range(opening, len(line)):
        depth += {"(": 1, ")": -1}.get(line[index], 0)
        if depth == 0:
            return index + 1
    raise ValueError(line)


lines = PATH.read_bytes().decode("utf-8").split("\n")
for number, line in enumerate(lines):
    if "const start = " in line:
        continue
    position = 0
    while (match := CALL.search(line, position)) is not None:
        end = closing(line, match.end() - 1)
        rest = line[end - 1 :]
        decided = DECIDED.search(rest)
        at = next(group for group in decided.groups() if group) if decided else ("20" if "context(20 + index)" in rest else "0")
        inner = line[match.end() : end - 1]
        if STAMPED.match(inner):
            position = end
            continue
        stamped ="start(%s%s)" % (at, ", " + inner if inner else "")
        line = line[: match.start()] + stamped + line[end:]
        position = match.start() + len(stamped)
        print("[DEBUG] %d: start(%s) at %s" % (number + 1, inner, at))
    lines[number] = line
payload = "\n".join(lines).encode("utf-8")
with open(PATH, "r+b") as handle:
    handle.write(payload)
    handle.truncate(len(payload))
