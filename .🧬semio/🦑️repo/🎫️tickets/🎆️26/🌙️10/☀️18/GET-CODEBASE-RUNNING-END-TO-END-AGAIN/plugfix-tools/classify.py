#!/usr/bin/env python3
"""🗂️ plugfix classifier: reads the raw sweep results (/Users/ueli/.cache/plugfix/raw/*.json) and sorts every resolve-level error into
DUP (duplicate definitions), UNDECLARED (first path segment that is neither dependency, alias nor std), MISSING (module file, local import, local module member),
IMPORT (name that the crate defines elsewhere but the failing scope does not import), PRIVATE (E0603) and NOISE (externals the sandbox cannot resolve).
usage: classify.py [--noise] [raw.json ...]  (default: every file in the raw directory)"""
import collections, glob, json, os, re, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sweep

RAW = "/Users/ueli/.cache/plugfix/raw"
DUP = {"E0428", "E0252", "E0254", "E0255", "E0259", "E0260", "E0201", "E0592", "E0124"}
NAME_CODES = {"E0412", "E0425", "E0422", "E0423", "E0405", "E0531", "E0532", "E0574", "E0433", "E0432", "E0407", "E0426"}
STD = {"std", "core", "alloc", "proc_macro", "test", "crate", "self", "super", "Self"}


def classify(code, msg, known, defined):
    if code in DUP:
        return "DUP"
    if code == "E0583":
        return "MISSING"
    if code == "E0603":
        return "PRIVATE"
    if code not in NAME_CODES:
        return "NOISE"
    m = re.match(r"cannot find module or crate `(\w+)` in this scope", msg) or re.match(r"failed to resolve: use of (?:unresolved module or unlinked crate|undeclared crate or module) `(\w+)`", msg)
    if m:
        x = m.group(1)
        return "NOISE" if x in known else ("IMPORT" if x in defined else "UNDECLARED")
    m = re.match(r"cannot find `(\w+)` in `(\w+)`$", msg) or re.match(r"cannot find `(\w+)` in the crate root", msg)
    if m:
        a = m.group(1)
        b = m.group(2) if m.lastindex == 2 else "crate"
        if a in known or b in known:
            return "NOISE"
        return "MISSING"
    m = re.match(r"cannot find (?:[\w, ]+?) `(\w+)` in module `([\w:]+)`", msg)
    if m:
        x, mod = m.groups()
        root = mod.split("::")[0]
        if root in known and root not in ("crate", "self", "super"):
            return "NOISE"
        return "MISSING"
    m = re.match(r"cannot find (?:[\w, ]+?) `(\w+)` in the crate root", msg)
    if m:
        return "NOISE" if m.group(1) in known else "MISSING"
    m = re.match(r"cannot find (?:[\w, ]+?) `(\w+)` in this scope", msg)
    if m:
        x = m.group(1)
        return "IMPORT" if x in defined else "NOISE"
    if msg.startswith("unresolved import"):
        kinds = set()
        for path in re.findall(r"`([^`]+)`", msg):
            first = path.split("::")[0]
            if first in known:
                kinds.add("NOISE")
            elif first in ("crate", "self", "super") or first in defined:
                kinds.add("MISSING")
            else:
                kinds.add("UNDECLARED")
        return "UNDECLARED" if "UNDECLARED" in kinds else ("MISSING" if "MISSING" in kinds else "NOISE")
    m = re.match(r"expected (?:[\w, ]+?), found (?:[\w ]+?) `([\w:]+)`", msg)
    if m:
        return "MISSING" if m.group(1).startswith(("crate", "self", "super")) else "NOISE"
    return "OTHER"


def main():
    args = sys.argv[1:]
    noise = False
    if args and args[0] == "--noise":
        noise = True
        args = args[1:]
    files = args or sorted(glob.glob(RAW + "/*.json"))
    libnames = sweep.libnames()
    total = collections.Counter()
    for f in files:
        r = json.load(open(f, encoding="utf-8"))
        known, defined = set(r["known"]), set(r["defined"])
        known |= sweep.manifest_info(r["mdir"])[2]
        known |= {libnames.get(k, k) for k in known}
        buckets = collections.OrderedDict()
        for cfg, errs in r["errors"].items():
            for code, msg, where in errs:
                kind = classify(code, msg, known, defined)
                total[kind] += 1
                if kind == "NOISE" and not noise:
                    continue
                key = (kind, code, msg, where.rsplit(":", 1)[0])
                buckets.setdefault(key, [cfg, where.rsplit(":", 1)[1], 0])
                buckets[key][2] += 1
        if buckets:
            print("##", r["mdir"].replace("✏️s/🔌️plugins/", "").replace("/📦️packages/🦀️rust", ""))
            for (kind, code, msg, where), (cfg, line, n) in buckets.items():
                print(f"  {kind:10} {code} {msg[:120]} x{n} [{cfg}] @ {where[-110:]}:{line}")
    print("TOTAL", dict(total))


if __name__ == "__main__":
    main()
