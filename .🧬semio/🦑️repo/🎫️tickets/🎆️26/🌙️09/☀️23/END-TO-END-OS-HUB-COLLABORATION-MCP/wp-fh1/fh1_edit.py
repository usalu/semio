"""✂️ FH1 exact-substring edits in the overlay: `apply(root, [(relative path, old, new[, count])…])` — each `old` must
occur exactly `count` times (default 1, 0 = every occurrence ≥ 1), else nothing is written and the edit is reported."""
import sys
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"


def apply(root, edits, dry=False):
    texts, bad = {}, []
    for edit in edits:
        path, old, new = edit[0], edit[1], edit[2]
        count = edit[3] if len(edit) > 3 else 1
        full = O + root + path
        text = texts.get(full) or open(full).read()
        found = text.count(old)
        if (count and found != count) or (not count and found == 0):
            bad.append(f"{path}: expected {count or '≥1'} × {old[:90]!r}, found {found}")
            continue
        texts[full] = text.replace(old, new)
    for line in bad:
        print("FAIL", line)
    if bad:
        sys.exit(1)
    if not dry:
        for full, text in texts.items():
            open(full, "w").write(text)
    print(f"applied {len(edits)} edits in {len(texts)} files")


def regex(root, path, pattern, replacement, expected):
    import re
    full = O + root + path
    text = open(full).read()
    new, found = re.subn(pattern, replacement, text)
    if found != expected:
        print(f"FAIL {path}: regex {pattern!r} matched {found}, expected {expected}")
        sys.exit(1)
    open(full, "w").write(new)
    print(f"regex {found}× {pattern[:60]!r}")
