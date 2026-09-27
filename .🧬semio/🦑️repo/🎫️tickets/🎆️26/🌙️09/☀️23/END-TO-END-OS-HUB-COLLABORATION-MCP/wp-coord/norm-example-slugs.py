#!/usr/bin/env python3
"""🏷️ Coordinator one-off (session 13, chain b3 run 3 failed at `plugin-registry:check`): 29 norm example directories
added by another team's norm Wave C/D are not `emoji+VS16+kebab` slugs. Emoji-led names gain the missing U+FE0F; the
emoji-less VDI 3805 `blatt-N` / `blatt-N-fail` sets gain 📄️ (Blatt = sheet) / ❌️. Example ids (`pub const ID`) are the
kebab part and stay unchanged; `📚️examples/<old>/` path references in norm sources are rewritten.
Usage: norm-example-slugs.py --dry-run | --write"""
import os, re, sys
from pathlib import Path
ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm")
SKIP = {"🧪️tests", "🧫️fixtures"}
SLUG = re.compile(r"^[^\x00-\x7f]+️[a-z0-9]+(-[a-z0-9]+)*$")
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv: sys.exit(__doc__)
def new_name(name):
    if not name[0].isascii():
        head = name[0]
        rest = name[1:]
        if rest.startswith("️"): return None
        return head + "️" + rest
    return ("❌️" if name.endswith("-fail") else "📄️") + name
renames = []
for ex in ROOT.rglob("📚️examples"):
    if any(p in ("dist", "target", "node_modules") for p in ex.parts) or not ex.is_dir(): continue
    for child in sorted(ex.iterdir()):
        if not child.is_dir() or child.name in SKIP or SLUG.match(child.name): continue
        nn = new_name(child.name)
        if nn is None or not SLUG.match(nn): print("UNFIXABLE", child); continue
        renames.append((child, child.with_name(nn)))
print(f"renames={len(renames)}")
for a, b in renames: print(f"  {a.relative_to(ROOT)} -> {b.name}")
texts = [p for p in ROOT.rglob("*") if p.is_file() and p.suffix in (".rs", ".ts", ".json", ".toml", ".md", ".py") and not any(x in ("dist", "target", "node_modules") for x in p.parts)]
edits = 0
for p in texts:
    s = p.read_text(encoding="utf-8", errors="surrogateescape"); t = s
    for a, b in renames:
        for tail in ("/", "\""):
            t = t.replace("📚️examples/" + a.name + tail, "📚️examples/" + b.name + tail)
    if t != s:
        edits += 1; print("  edit", p.relative_to(ROOT))
        if write: p.write_text(t, encoding="utf-8", errors="surrogateescape")
if write:
    for a, b in renames: os.rename(a, b)
print(f"files_edited={edits} write={write}")
