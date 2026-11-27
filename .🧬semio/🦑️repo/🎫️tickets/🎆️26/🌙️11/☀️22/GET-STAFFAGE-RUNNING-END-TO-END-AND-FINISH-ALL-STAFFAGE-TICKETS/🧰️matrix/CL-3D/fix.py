"""🩹️ The asset repairs of lane CL-3D, exact text edits of the catalog files (formatting untouched), idempotent: `fix.py <id> --out <file>` writes the repaired copy, `fix.py <id> --write` reads the catalog file right before it writes it; a second run changes nothing.

- `running-child`: `presentation.defaultPose` is `toe-off`, which was marked `key: true` (an in-between key pose of a clip, not offered as a named pose), so no GLB of the presentation pose existed and `🧊️model-catalog` compared `dimensions` (0.671 m ahead, the toe-off stride) with the first named pose `standing-ready` (-51 %). `toe-off` becomes a named pose."""
import glob, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.join(HERE, *[".."] * 9, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")


def between(text, start, end, old, new):
    a = text.index(start)
    b = text.index(end, a)
    block = text[a:b]
    if old not in block:
        return text
    if block.count(old) != 1:
        raise SystemExit(f"expected once between {start!r} and {end!r}: {old!r}")
    return text[:a] + block.replace(old, new) + text[b:]


REPAIRS = {
    "running-child": lambda text: between(text, '"id": "toe-off"', '"id": "landing"', ',\n        "key": true\n', "\n"),
}

asset = sys.argv[1]
path = glob.glob(os.path.join(CATALOG, "*", asset, "🔣️.json"))[0]
before = open(path, encoding="utf-8").read()
after = REPAIRS[asset](before)
if "--out" in sys.argv:
    open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8").write(after)
elif "--write" in sys.argv and after != before:
    open(path, "w", encoding="utf-8").write(after)
print(f"{asset}: {'changed' if after != before else 'unchanged'} ({len(before)} -> {len(after)} bytes)")
