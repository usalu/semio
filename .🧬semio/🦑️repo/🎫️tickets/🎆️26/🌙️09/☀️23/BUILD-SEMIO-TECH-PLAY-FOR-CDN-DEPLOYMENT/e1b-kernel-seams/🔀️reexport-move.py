#!/usr/bin/env python3
"""🔀️ Moves names between the os store root's interim `pub use <module>::{..}` / `pub(crate) use <module>::{..}` lines (wave E1b).

Usage: reexport-move.py --from-module M [--to-module N | --drop] --names A,B [--write]
Each name keeps its visibility (`pub` or `pub(crate)`) and cfg line; with --to-module the names join (or open) N's line
of the same visibility, mounted after M's line; with --drop they vanish (the item now lives in the root itself).
"""
import argparse, re

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--from-module", required=True)
    ap.add_argument("--to-module")
    ap.add_argument("--drop", action="store_true")
    ap.add_argument("--names", required=True)
    ap.add_argument("--write", action="store_true")
    a = ap.parse_args()
    names = {n.strip() for n in a.names.split(",") if n.strip()}
    lines = open(ROOT, encoding="utf-8").read().split("\n")
    pat = re.compile(r"^(pub(?:\(crate\))?) use " + re.escape(a.from_module) + r"::(?:\{(.*)\}|(\w+));$")
    moved, last = {}, {}
    for i, l in enumerate(lines):
        m = pat.match(l)
        if not m: continue
        cfg = lines[i - 1] if lines[i - 1].startswith("#[cfg") else None
        items = [x.strip() for x in (m.group(2) if m.group(2) is not None else m.group(3)).split(",") if x.strip()]
        keep = [x for x in items if x not in names]
        for x in items:
            if x in names: moved.setdefault((m.group(1), cfg), []).append(x)
        last[(m.group(1), cfg)] = i
        lines[i] = f"{m.group(1)} use {a.from_module}::{{{', '.join(keep)}}};" if keep else "\0"
        if not keep and cfg: lines[i - 1] = "\0"
    missing = names - {x for v in moved.values() for x in v}
    if missing: raise SystemExit(f"names not re-exported from {a.from_module}: {sorted(missing)}")
    if a.to_module:
        for (vis, cfg), xs in sorted(moved.items(), key=lambda kv: -last[kv[0]]):
            text = (cfg + "\n" if cfg else "") + f"{vis} use {a.to_module}::{{{', '.join(sorted(xs))}}};"
            lines.insert(last[(vis, cfg)] + 1, text)
    out = "\n".join(l for l in lines if l != "\0")
    for k, v in moved.items(): print(f"[reexport] {k[0]} {k[1] or ''} {a.from_module} -> {a.to_module or 'dropped'}: {', '.join(sorted(v))}")
    if a.write: open(ROOT, "w", encoding="utf-8").write(out)


if __name__ == "__main__":
    main()
