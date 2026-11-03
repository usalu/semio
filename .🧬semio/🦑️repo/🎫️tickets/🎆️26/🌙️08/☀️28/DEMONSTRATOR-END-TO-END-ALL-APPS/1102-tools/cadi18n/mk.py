import sys, difflib, pathlib, subprocess
REPO = pathlib.Path("/Users/ueli/Documents/semio")
HERE = pathlib.Path(__file__).parent
STACK = pathlib.Path("/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/stack")
def read(path):
    stacked = STACK / path
    return (stacked if stacked.exists() else REPO / path).read_text(encoding="utf8")
def diff(path, src, new, out):
    d = "".join(difflib.unified_diff(src.splitlines(True), new.splitlines(True), "a/" + path, "b/" + path))
    out.write_text(d, encoding="utf8")
def edit(path, edits, name):
    src = read(path)
    new = src
    for old, rep, *n in edits:
        want = n[0] if n else 1
        if new.count(old) != want:
            sys.exit(f"FAIL-CLOSED: {old[:70]!r} occurs {new.count(old)}x (want {want}) in {path[-60:]}")
        new = new.replace(old, rep)
    diff(path, src, new, HERE / "patches" / name)
    print("wrote", name)
def create(path, content, name):
    d = "".join(difflib.unified_diff([], content.splitlines(True), "/dev/null", "b/" + path))
    (HERE / "patches" / name).write_text(d, encoding="utf8")
    print("wrote", name)
