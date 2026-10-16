"""🧰️ Edit helpers for the dashboard carve: exact-anchor replacement, region extraction, file IO."""
import os, re

def rd(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()

def wr(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)

def sub(path, old, new, count=1):
    text = rd(path)
    found = text.count(old)
    assert found == count, f"{path}: expected {count} of {old[:90]!r}, found {found}"
    wr(path, text.replace(old, new))

def subs(path, pairs):
    text = rd(path)
    for old, new, *rest in pairs:
        count = rest[0] if rest else 1
        found = text.count(old)
        assert found == count, f"{path}: expected {count} of {old[:90]!r}, found {found}"
        text = text.replace(old, new)
    wr(path, text)

def take(path, start, end, include_end=True, remove=True):
    """Cuts the text from the line starting with `start` through the line starting with `end`."""
    text = rd(path)
    lines = text.split("\n")
    starts = [i for i, line in enumerate(lines) if line.startswith(start)]
    assert len(starts) == 1, f"{path}: start {start!r} found {len(starts)}"
    first = starts[0]
    ends = [i for i in range(first, len(lines)) if lines[i].startswith(end)]
    assert ends, f"{path}: end {end!r} not found after {start!r}"
    last = ends[0]
    cut = lines[first:last + 1] if include_end else lines[first:last]
    if remove:
        del lines[first:last + 1 if include_end else last]
        wr(path, "\n".join(lines))
    return "\n".join(cut) + "\n"

def region(path, name, remove=True):
    return take(path, f"// #region {name}", f"// #endregion {name}", True, remove)

def drop_blank_runs(path):
    text = rd(path)
    text = re.sub(r"\n{3,}", "\n\n", text)
    wr(path, text)
