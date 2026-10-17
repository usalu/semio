"""📦 rii manifest editor: inserts `name = { path = "<rel>" }` lines into a dependency table without reordering the lines that exist (position = before the first line of the same group whose key sorts after).
usage: addep.py <manifest> <table> <key>=<path-from-repo-root-or-abs> ... ; groups: first-party renderer crates first, then the rest, as the tables are ordered."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ed import *
FP = "semio-framework-os-renderer-wgpu-"
def key_of(line):
    return line.split("=", 1)[0].strip()
def grp(k):
    return 0 if k.startswith(FP) else 1
def add(manifest, table, items, dry=False):
    text = read(manifest)
    m = re.search(r"(?m)^\[" + re.escape(table) + r"\]\n((?:[^\[\n][^\n]*\n)*)", text)
    assert m, ("table missing", manifest[-60:], table)
    lines = m.group(1).split("\n")
    assert lines[-1] == ""
    lines = lines[:-1]
    have = {key_of(l) for l in lines if l.strip() and not l.lstrip().startswith("#")}
    mdir = os.path.dirname(manifest)
    for key, target in items:
        if key in have:
            print("have", key)
            continue
        rel = os.path.relpath(target if target.startswith("/") else REPO + "/" + target, mdir)
        new = f'{key} = {{ path = "{rel}" }}'
        pos = len(lines)
        for i, l in enumerate(lines):
            if not l.strip() or l.lstrip().startswith("#"):
                continue
            k = key_of(l)
            if (grp(k), k) > (grp(key), key):
                pos = i
                break
        lines.insert(pos, new)
        have.add(key)
    out = text[:m.start(1)] + "\n".join(lines) + "\n" + text[m.end(1):]
    if dry or out == text:
        print("dry" if dry else "no change", manifest[-70:])
        return
    backup(manifest)
    assert read(manifest) == text
    with open(manifest, "w", encoding="utf8") as h:
        h.write(out)
    note(manifest)
    print("edited", manifest[-80:])
