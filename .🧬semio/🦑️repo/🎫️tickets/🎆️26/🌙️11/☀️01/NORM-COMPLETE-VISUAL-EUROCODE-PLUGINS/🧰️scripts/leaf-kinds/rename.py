"""Renames generated kinds to their short names across one artifact tree (dirs, text, taxonomy, oracle), then enforces the 256-byte path guard. Usage: python -I rename.py en1999"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import model as M


def forms(old, new):
    ko, kn = old, new
    po, pn = pascal(ko), pascal(kn)
    out = [(ko, kn), (ko.replace("-", "_"), kn.replace("-", "_")), (po, pn), (po[:1].lower() + po[1:], pn[:1].lower() + pn[1:]),
           ("Changed" + po[6:], "Changed" + pn[6:]), ("parse" + po, "parse" + pn), (ko[len("change-"):], kn[len("change-"):])]
    return out


def apply(pairs, roots, extra_files):
    rules = []
    for old, new in sorted(pairs, key=lambda p: -len(p[0])):
        for a, b in forms(old, new):
            rules.append((re.compile(r"(?<![A-Za-z0-9_-])%s(?![A-Za-z0-9_-])" % re.escape(a)) if "-" in a or "_" in a else re.compile(r"(?<![A-Za-z0-9])%s(?![a-z0-9])" % re.escape(a)) if not a.startswith("parse") else re.compile(r"%s(?![A-Za-z0-9])" % re.escape(a)), b))
    files = list(extra_files)
    for r in roots:
        for d, _, fs in os.walk(r):
            files += [os.path.join(d, f) for f in fs if f.endswith((".rs", ".json", ".ts", ".graphql", ".proto", ".py", ".feature", ".semio", ".toml"))]
    n = 0
    for f in files:
        try:
            t = rd(f)
        except Exception:
            continue
        u = t
        for rx, b in rules:
            u = rx.sub(b, u)
        if u != t:
            open(f, "w", encoding="utf-8").write(u)
            n += 1
    return n


def rename_dirs(roots, pairs):
    m = {o: n for o, n in pairs}
    done = 0
    for r in roots:
        for d, dirs, _ in os.walk(r, topdown=False):
            for x in dirs:
                k = re.sub(r"^[^a-z]+", "", x)
                if k in m:
                    pre = x[: len(x) - len(k)]
                    os.rename(os.path.join(d, x), os.path.join(d, pre + m[k]))
                    done += 1
    return done


def guard(root):
    bad = []
    for d, _, fs in os.walk(root):
        for f in fs:
            p = os.path.join(d, f)
            if len(p[len(SEMIO) + 1:].encode()) >= 256:
                bad.append(p)
    return bad
