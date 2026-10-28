"""🦀️ Copies the path-dependency closure of one crate (each crate directory and the source directory its `[lib]`/`[[test]]` paths point into) from the live tree into a MIRROR that lacks it. usage: python3 -I rustclosure.py <mirror-root> <crate-dir-rel>"""
import os, re, shutil, subprocess, sys
LIVE = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
mirror, crate = os.path.abspath(sys.argv[1]), sys.argv[2]
assert "/sm/" in mirror or "/tmp/m/" in mirror
root_manifest = open(os.path.join(LIVE, "Cargo.toml"), encoding="utf8").read()
workspace_paths = dict(re.findall(r'^\s*([A-Za-z0-9_-]+)\s*=\s*\{[^}\n]*path\s*=\s*"([^"]+)"', root_manifest, re.M))
seen, pending, copied = set(), [crate], 0
while pending:
    rel = os.path.normpath(pending.pop())
    if rel in seen: continue
    seen.add(rel)
    text = open(os.path.join(LIVE, rel, "Cargo.toml"), encoding="utf8").read()
    for dep in re.findall(r'path\s*=\s*"([^"]+)"', text):
        target = os.path.normpath(os.path.join(rel, dep))
        if os.path.exists(os.path.join(LIVE, target, "Cargo.toml")): pending.append(target)
    for name in re.findall(r'^\s*([A-Za-z0-9_-]+)\s*=\s*\{[^}\n]*workspace\s*=\s*true', text, re.M):
        if name in workspace_paths: pending.append(workspace_paths[name])
    for name in re.findall(r'^\s*([A-Za-z0-9_-]+)\.workspace\s*=\s*true', text, re.M):
        if name in workspace_paths: pending.append(workspace_paths[name])
source_dirs = set()
for rel in seen:
    source_dirs.add(rel)
    source_dirs.add(os.path.normpath(os.path.join(rel, "..", "..")))
    text = open(os.path.join(LIVE, rel, "Cargo.toml"), encoding="utf8").read()
    for path in re.findall(r'^\s*path\s*=\s*"([^"]+\.rs)"', text, re.M):
        source_dirs.add(os.path.normpath(os.path.join(rel, os.path.dirname(path))))
for rel in sorted(source_dirs):
    for here, dirs, names in os.walk(os.path.join(LIVE, rel)):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target", ".cache", "dist") and not d.startswith(".")]
        for name in names:
            if not name.endswith((".rs", ".toml", ".json", ".md", ".wgsl", ".txt", ".jsonl", ".in")): continue
            file = os.path.relpath(os.path.join(here, name), LIVE)
            target = os.path.join(mirror, file)
            if os.path.exists(target): continue
            os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copy2(os.path.join(LIVE, file), target); copied += 1
print(f"[DEBUG] crates {len(seen)}, source dirs {len(source_dirs)}, files copied {copied}")
