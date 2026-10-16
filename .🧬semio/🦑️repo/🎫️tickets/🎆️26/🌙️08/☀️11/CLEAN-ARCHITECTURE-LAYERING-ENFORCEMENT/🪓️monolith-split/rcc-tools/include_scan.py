"""🔎 Resolves Rust include_str!, include_bytes! and #[path] literals of grep output and keeps those that cross technology roots."""
import os, re, sys

ROOT = "/Users/ueli/Documents/semio"
TARGET_ROOTS = ("✏️s", "🏢️semio-tech", "♻️mit-bestand")
manifest_cache = {}

def manifest_dir(file_dir):
    d = file_dir
    while True:
        if d in manifest_cache:
            return manifest_cache[d]
        if os.path.exists(os.path.join(d, "Cargo.toml")):
            manifest_cache[d] = d
            return d
        parent = os.path.dirname(d)
        if parent == d or not d.startswith(ROOT):
            return None
        d = parent

LIT = re.compile(r'(include_str!|include_bytes!|#\[path\s*=)\s*\(?\s*"([^"]+)"')
CAT = re.compile(r'(include_str!|include_bytes!)\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"')

def main(raw, out):
    rows = []
    for line in open(raw, encoding="utf-8"):
        m = re.match(r'^(.*?):(\d+):(.*)$', line.rstrip("\n"))
        if not m:
            continue
        rel, no, text = m.group(1), m.group(2), m.group(3)
        if text.lstrip().startswith("//"):
            continue
        fdir = os.path.dirname(os.path.join(ROOT, rel))
        for cm in CAT.finditer(text):
            md = manifest_dir(fdir)
            if md is None:
                continue
            target = os.path.normpath(md + cm.group(2))
            rows.append((rel, no, cm.group(1), cm.group(2), target))
        for lm in LIT.finditer(text):
            target = os.path.normpath(os.path.join(fdir, lm.group(2)))
            rows.append((rel, no, lm.group(1), lm.group(2), target))
    cross = []
    for rel, no, kind, lit, target in rows:
        if not target.startswith(ROOT + "/"):
            continue
        top = target[len(ROOT) + 1:].split("/", 1)[0]
        if top in TARGET_ROOTS:
            cross.append((rel, no, kind, top, target[len(ROOT) + 1:]))
    with open(out, "w", encoding="utf-8") as fh:
        for rel, no, kind, top, tgt in cross:
            fh.write(f"{rel}:{no}\t{kind}\t{top}\t{tgt}\n")
    print(f"directives={len(rows)} cross={len(cross)}")

main(sys.argv[1], sys.argv[2])
