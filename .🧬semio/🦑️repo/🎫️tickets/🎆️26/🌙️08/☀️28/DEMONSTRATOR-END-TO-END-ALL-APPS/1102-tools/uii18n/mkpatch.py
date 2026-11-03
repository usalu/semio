import sys, difflib, pathlib
def patch(path, edits, out):
    src = pathlib.Path(path).read_text()
    new = src
    for old, rep in edits:
        if new.count(old) != 1:
            sys.exit(f"FAIL-CLOSED: {old[:60]!r} occurs {new.count(old)}x in {path}")
        new = new.replace(old, rep)
    d = difflib.unified_diff(src.splitlines(True), new.splitlines(True), "a/" + path, "b/" + path)
    pathlib.Path(out).write_text("".join(d))
    print("wrote", out)
