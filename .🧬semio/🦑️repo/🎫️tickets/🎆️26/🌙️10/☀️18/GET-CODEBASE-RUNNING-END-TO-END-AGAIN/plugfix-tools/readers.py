#!/usr/bin/env python3
"""📖 plugfix static-reader check: Rust tests that read Rust source text by literal path (`include_str!`, `fixture_text!`, `fixture_path!` + `read_to_string`) and then assert marker strings.
Reports (a) read targets that do not exist and (b) marker literals (`contains`/`split_once`/`find`) of the test file that occur in none of its read targets nor their `#[path]` module trees.
usage: readers.py <directory>...   (every `.rs` file below is scanned; targets resolve against the crate manifest directory for `fixture_*` and against the file for `include_str!`)"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
RX_READ = re.compile(r'(include_str!|fixture_text!|fixture_path!|repo_text!)\(\s*(?:format!\(\s*)?"([^"{}]+(?:\{[^}]*\}[^"{}]*)*)"')
RX_MARK = re.compile(r'\.(?:contains|split_once|find|rfind|matches)\(\s*"((?:[^"\\]|\\.)+)"')
RX_PATH = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
CACHE = {}
MANIFESTS = {}


def read(path):
    if path not in CACHE:
        try:
            CACHE[path] = open(path, encoding="utf-8", errors="ignore").read()
        except OSError:
            CACHE[path] = None
    return CACHE[path]


def manifest_dir(path):
    if "roots" not in MANIFESTS:
        sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
        import sweep
        MANIFESTS["roots"] = sweep.libroots()
    d = os.path.dirname(path)
    while d.startswith(REPO) and d != REPO:
        hit = MANIFESTS["roots"].get(os.path.relpath(d, REPO))
        if hit:
            return os.path.join(REPO, hit[0])
        d = os.path.dirname(d)
    return None


def tree(path, seen=None):
    seen = seen if seen is not None else set()
    if path in seen or not path.endswith(".rs"):
        return ""
    seen.add(path)
    text = read(path)
    if text is None:
        return ""
    out = text
    for m in RX_PATH.finditer(text):
        out += tree(os.path.normpath(os.path.join(os.path.dirname(path), m.group(1))), seen)
    return out


def scan(file):
    text = read(file)
    if not text or not RX_READ.search(text):
        return
    targets, missing = {}, []
    for m in RX_READ.finditer(text):
        macro, lit = m.group(1), m.group(2)
        if "{" in lit or not lit.endswith(".rs"):
            continue
        base = os.path.dirname(file) if macro == "include_str!" else manifest_dir(file)
        if base is None:
            continue
        if macro == "repo_text!":
            continue
        target = os.path.normpath(os.path.join(base, lit))
        if os.path.isfile(target):
            targets[target] = tree(target)
        else:
            missing.append(lit)
    rel = os.path.relpath(file, REPO)
    for lit in missing:
        print(f"MISSING-TARGET {rel[-120:]} -> {lit[-100:]}")
    if not targets:
        return
    union = "".join(targets.values())
    seen = set()
    for m in RX_MARK.finditer(text):
        lit = m.group(1).encode().decode("unicode_escape", errors="ignore") if "\\" in m.group(1) else m.group(1)
        if lit in seen or len(lit) < 3:
            continue
        seen.add(lit)
        if lit not in union:
            print(f"MARKER-ABSENT {rel[-120:]} :: {lit[:90]!r}  (targets: {', '.join(os.path.basename(os.path.dirname(t)) for t in targets)})")


def main():
    for root in sys.argv[1:]:
        for dp, dn, fn in os.walk(root):
            dn[:] = [d for d in dn if d not in ("target", "node_modules", ".git", "🗑️generated")]
            for f in fn:
                if f.endswith(".rs"):
                    scan(os.path.join(dp, f))


if __name__ == "__main__":
    main()
