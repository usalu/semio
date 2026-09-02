#!/usr/bin/env python3
"""🧭️ Realigns stale `#[path = "…"]` module mounts with what is actually on disk.

A repo-wide taxonomy sweep is renaming a directory's own Rust file from `🦀️component.rs` to bare
`🦀️.rs`. It moves the files and the mounts in separate passes, so in between, a crate's glue points
at a name that no longer exists and the whole crate fails with `couldn't read …: No such file or
directory` — which masks every real diagnostic behind it.

Only rewrites a mount when the target is missing AND the renamed sibling exists, so it can never
invent a path or fight a mount that is already correct. Idempotent; safe to run before any build.
"""
import io, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)

SWAPS = [("🦀️component.rs", "🦀️.rs"), ("🦀️.rs", "🦀️component.rs")]
PATH_RE = re.compile(r'#\[path = "([^"]+)"\]')

files = subprocess.run(
    ["grep", "-rlF", "--include=*.rs", "#[path = ", "🧰️framework", "✏️s"],
    capture_output=True).stdout.decode("utf-8").split("\n")

total, touched = 0, []
for path in [f for f in files if f and "/target/" not in f]:
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        continue
    base, fixes = os.path.dirname(path), 0

    def repair(match):
        global fixes
        rel = match.group(1)
        if os.path.exists(os.path.normpath(os.path.join(base, rel))):
            return match.group(0)
        for old, new in SWAPS:
            if rel.endswith(old):
                candidate = rel[: -len(old)] + new
                if os.path.exists(os.path.normpath(os.path.join(base, candidate))):
                    fixes += 1
                    return '#[path = "%s"]' % candidate
        # 🗂️ Third shape: a named leaf `🦀️<name>.rs` becomes a directory `<emoji><name>/🦀️.rs`.
        leaf = os.path.basename(rel)
        m2 = re.match(r'^🦀️(.+)\.rs$', leaf)
        if m2:
            stem = m2.group(1).replace("_", "-")
            parent = os.path.normpath(os.path.join(base, os.path.dirname(rel)))
            if os.path.isdir(parent):
                for entry in os.listdir(parent):
                    if not os.path.isdir(os.path.join(parent, entry)):
                        continue
                    # directory names carry a leading emoji; compare the ASCII tail
                    tail = re.sub(r'^[^A-Za-z0-9]+', '', entry)
                    if tail == stem and os.path.exists(os.path.join(parent, entry, "🦀️.rs")):
                        candidate = os.path.join(os.path.dirname(rel), entry, "🦀️.rs")
                        fixes += 1
                        return '#[path = "%s"]' % candidate
        return match.group(0)

    repaired = PATH_RE.sub(repair, text)
    if fixes:
        io.open(path, "w", encoding="utf-8").write(repaired)
        total += fixes
        touched.append((path, fixes))

print("module paths realigned: %d across %d files" % (total, len(touched)))
for path, count in touched[:12]:
    print("  %3d  %s" % (count, path))
if len(touched) > 12:
    print("  … and %d more files" % (len(touched) - 12))
