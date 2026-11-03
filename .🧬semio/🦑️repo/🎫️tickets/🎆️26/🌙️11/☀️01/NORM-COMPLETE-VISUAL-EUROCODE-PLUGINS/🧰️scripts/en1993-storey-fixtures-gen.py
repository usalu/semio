"""Adds the empty `storeys` collection (and empty delta) next to `craneRunways` in every EN 1993 snapshot/diff fixture, asset and DSL text (idempotent). Usage: python3 -I en1993-storey-fixtures-gen.py."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, pathlib, re

A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
n = 0


def patch(d):
    if isinstance(d, dict):
        out = {}
        for k, v in d.items():
            out[k] = patch(v)
            if k == "craneRunways" and "storeys" not in d:
                out["storeys"] = [] if isinstance(v, list) else {key: [] for key in v}
        d.clear()
        d.update(out)
    elif isinstance(d, list):
        for x in d:
            patch(x)
    return d


for p in sorted(A.rglob("*.json")):
    if "🧬️schema" in p.parts and "🧫️fixtures" not in p.parts:
        continue
    raw = p.read_text(encoding="utf8")
    if "craneRunways" not in raw:
        continue
    d = json.loads(raw)
    patch(d)
    out = json.dumps(d, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "")
    if out != raw:
        p.write_text(out, encoding="utf8")
        n += 1
print("json files patched:", n)
