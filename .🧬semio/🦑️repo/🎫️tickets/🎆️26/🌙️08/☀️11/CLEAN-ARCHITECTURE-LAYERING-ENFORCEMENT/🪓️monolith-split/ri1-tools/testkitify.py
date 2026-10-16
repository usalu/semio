#!/usr/bin/env python3
# 🧪️ Makes the `test` predicates of cfg attributes on code items testkit-aware: `test` -> `any(test, feature = "testkit")`, so a dependent's test build can enable the
# mock / per-thread variants of a carved crate (the pattern of `semio-framework-job`'s `testkit` feature). Test-module mounts keep their plain `cfg(test)`.
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ed import *
from rs import blank

ATTR = re.compile(r"#\[\s*cfg\s*\(")
TESTKIT = 'any(test, feature = "testkit")'

def balanced_end(clean, i):
    depth = 0
    j = i
    while j < len(clean):
        c = clean[j]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return j
        j += 1
    raise ValueError("unbalanced")

def next_item_is_mod(clean, pos):
    m = re.compile(r"\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*[;{]").match(clean, pos)
    return m is not None

def pred_replace(pred):
    out, i = [], 0
    in_str = False
    tok = re.compile(r"(?<![\w\"])test(?![\w\"])")
    parts = re.split(r'("(?:[^"\\]|\\.)*")', pred)
    return "".join(p if p.startswith('"') else tok.sub(TESTKIT, p) for p in parts)

def testkitify(text):
    clean = blank(text)
    out, last = [], 0
    for m in ATTR.finditer(clean):
        start = m.end() - 1
        end = balanced_end(clean, start)
        pred = text[start + 1:end]
        if not re.search(r"(?<![\w\"])test(?![\w\"])", blank(pred)) :
            continue
        close = clean.index("]", end)
        if next_item_is_mod(clean, close + 1):
            continue
        out.append(text[last:start + 1])
        out.append(pred_replace(pred))
        last = end
    out.append(text[last:])
    new = "".join(out)
    new = re.sub(r"cfg!\(\s*test\s*\)", f"cfg!({TESTKIT})", new)
    return new

if __name__ == "__main__":
    dry = "--dry" in sys.argv
    for p in [a for a in sys.argv[1:] if not a.startswith("--")]:
        t = read(p)
        n = testkitify(t)
        if dry:
            import difflib
            for l in difflib.unified_diff(t.split("\n"), n.split("\n"), lineterm="", n=0):
                if l.startswith(("+", "-")) and not l.startswith(("+++", "---")):
                    print(l[:200])
        elif n != t:
            write(p, n)
            print(p.split("/")[-2], "changed")
