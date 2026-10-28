#!/usr/bin/env python3
"""zt-g staging: anchored edits for the zero-touch batch.
  stage.py stage   -> writes the edited files into staged/overlay/<repo-relative path> (real tree untouched)
  stage.py apply   -> applies the same edits to the real tree (each anchor must occur exactly once)
  (an edit with old=None creates a new file; apply refuses an existing one)
  stage.py verify  -> prints, per edit, whether the anchor is still present exactly once
Edits live in edits/*.py as EDITS = [(path, old, new), ...]; `rel(path, target)` builds a posix relative import."""
import os, sys, posixpath, importlib.util, glob
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 9)))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
def rel(path, target):
    r = posixpath.relpath(target, posixpath.dirname(path))
    return r if r.startswith(".") else "./" + r
def load():
    edits = []
    for f in sorted(glob.glob(os.path.join(HERE, "edits", "*.py"))):
        spec = importlib.util.spec_from_file_location(os.path.basename(f)[:-3], f); m = importlib.util.module_from_spec(spec)
        m.rel, m.L = rel, L; spec.loader.exec_module(m); edits += m.EDITS
    return edits
def main(mode):
    files = {}
    for path, old, new in load():
        if old is None:
            if os.path.exists(os.path.join(ROOT, path)): sys.exit(f"new file already exists: {path}")
            files[path] = new; continue
        text = files.get(path)
        if text is None:
            with open(os.path.join(ROOT, path), encoding="utf8") as h: text = h.read()
        if callable(old):
            changed = old(text)
            if mode == "verify": print(("OK  " if changed != text else "BAD(0) ") + path.replace(L, "L") + " :: <function>"); files[path] = text; continue
            if changed == text: sys.exit(f"function edit changed nothing in {path}")
            files[path] = changed; continue
        n = text.count(old)
        if mode == "verify": print(("OK  " if n == 1 else f"BAD({n}) ") + path.replace(L, "L") + " :: " + old[:70].replace("\n", " ")); files[path] = text; continue
        if n != 1: sys.exit(f"anchor occurs {n}x in {path}: {old[:80]!r}")
        files[path] = text.replace(old, new)
    if mode == "verify": return
    for path, text in files.items():
        dest = os.path.join(ROOT, path) if mode == "apply" else os.path.join(HERE, "overlay", path)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        with open(dest, "w", encoding="utf8", newline="") as h: h.write(text)
        print(mode, path.replace(L, "L"))
if __name__ == "__main__": main(sys.argv[1] if len(sys.argv) > 1 else "verify")
