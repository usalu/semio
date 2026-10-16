# 🗂️ World model of the integration pass: file ownership (longest-prefix map of the plan tooling), crate file lists, module trees.
import os, re, json, collections, sys
from common import *
sys.path.insert(0, SPLIT + "/pwp5-tools")
import importlib.util
_spec = importlib.util.spec_from_file_location("pt5", SPLIT + "/pwp5-tools/plan_tools.py")
os.environ.setdefault("CLOSURE", GEN + "/pwp0-closure.json")
pt = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(pt)
SKIP_DIRS = {"node_modules", "dist", "target", "📦️packages", ".cache"}

_ROOT_PREFIX = None


def file_owner(f):
    """crate of a plugin-relative file: the plan map, refined by the crate root directories of the frozen table (longest prefix wins)"""
    global _ROOT_PREFIX
    if _ROOT_PREFIX is None:
        _ROOT_PREFIX = {c["root"] + "/": k for k, c in CRATES.items() if c["kind"] in ("lower",)}
    best = pt.file_crate(f)
    bl = -1
    if not best.startswith("UNMAPPED"):
        bl = max((len(k) for k in pt.MAP if f.startswith(k) and pt.MAP[k] == best), default=-1)
    for p, k in _ROOT_PREFIX.items():
        if f.startswith(p) and len(p) > bl:
            best, bl = k, len(p)
    return best


def all_rs():
    out = []
    for base, dirs, files in os.walk(PLUG):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            if f.endswith(".rs"):
                out.append(os.path.relpath(os.path.join(base, f), PLUG))
    return out

def owners(files=None):
    res = collections.defaultdict(list)
    for f in files or all_rs():
        res[file_owner(f)].append(f)
    return res

if __name__ == "__main__":
    o = owners()
    for k in sorted(o):
        if k.startswith("UNMAPPED") or k.startswith("TOP"):
            continue
        print(k, len(o[k]))
