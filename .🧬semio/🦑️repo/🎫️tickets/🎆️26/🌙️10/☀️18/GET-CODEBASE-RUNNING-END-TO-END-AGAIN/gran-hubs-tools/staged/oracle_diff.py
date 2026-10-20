#!/usr/bin/env python3
"""🔬 Error-multiset differential of two oracle runs, keyed by (code, normalised message, normalised label). Crate module and file are dropped from the key so that moved code compares equal; privacy and resolution errors are listed in full.

usage: oracle_diff.py <pre.err> <post.err>   (exit 0 when no new error)
"""
import collections, json, re, sys

PATHRX = re.compile(r"\b(?:_*[a-z][a-z0-9_]*::)+(?=[A-Za-z_{(<])")
MODRX = re.compile(r"__m_[A-Za-z0-9_]+")
PRIV = {"E0603", "E0616", "E0624", "E0451", "E0364", "E0365", "E0445", "E0446"}
RESOLVE = {"E0412", "E0425", "E0422", "E0423", "E0432", "E0433", "E0405", "E0407", "E0531", "E0532", "E0574", "E0583", "E0599"}


def nrm(s):
    return re.sub(r"\bTr\d+\b", "Tr", MODRX.sub("@M", PATHRX.sub("", s or "")))


def load(path):
    keys = collections.Counter()
    first = {}
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except ValueError:
            continue
        if d.get("level") != "error":
            continue
        msg = d.get("message", "")
        if msg.startswith("aborting due to") or "previous error" in msg:
            continue
        spans = d.get("spans") or [{}]
        prim = next((s for s in spans if s.get("is_primary")), spans[0])
        if str(prim.get("file_name", "")).endswith("__pos_root.rs"):
            continue
        k = ((d.get("code") or {}).get("code") or "", nrm(msg), nrm(prim.get("label")))
        keys[k] += 1
        first.setdefault(k, (prim.get("file_name", ""), prim.get("line_start", 0)))
    return keys, first


def main():
    a, fa = load(sys.argv[1])
    b, fb = load(sys.argv[2])
    new, lost = b - a, a - b
    print(f"pre {sum(a.values())} errors, post {sum(b.values())} errors; new {sum(new.values())}, lost {sum(lost.values())}")
    for tag, side, where in (("NEW", new, fb), ("LOST", lost, fa)):
        for k, n in sorted(side.items(), key=str):
            kind = "privacy" if k[0] in PRIV else "resolve" if k[0] in RESOLVE else "other"
            print(f"  {tag} x{n} [{kind}] {k[0]} {k[1][:140]}  @ {str(where[k][0])[-60:]}:{where[k][1]}")
    return 0 if not new else 1


if __name__ == "__main__":
    sys.exit(main())
