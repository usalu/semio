"""✂️ Anchored single-occurrence replace with re-read right before the write (atomic via rename).
usage as module: from ttcedit import replace; replace(path, old, new)"""
import os


def replace(path, old, new, count=1):
    with open(path, encoding="utf-8") as f:
        text = f.read()
    assert text.count(old) == count, f"{path}: anchor occurs {text.count(old)}x (expected {count}): {old[:80]!r}"
    out = text.replace(old, new)
    with open(path, encoding="utf-8") as f:
        assert f.read() == text, "file changed while editing " + path
    tmp = path + ".ttc-tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(out)
    os.replace(tmp, path)
    print("edited", os.path.basename(os.path.dirname(path)) + "/" + os.path.basename(path))
