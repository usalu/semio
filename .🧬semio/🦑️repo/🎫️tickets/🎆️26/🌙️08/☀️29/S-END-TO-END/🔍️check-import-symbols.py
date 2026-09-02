#!/usr/bin/env python3
"""🔍️ Flags relative imports whose named symbols the target module does not export.

`🧭️realign-ts-imports.py` repoints an import purely by path shape, so when the sweep collapses a
module it can land on a package barrel that re-exports only part of the old surface — the build then
fails late with `"X" is not exported by …`. This checks every relative import's named bindings
against the target's own exports (including `export * from` one level deep) and reports mismatches,
so a bad repoint is caught in seconds instead of at the end of a three-minute Vite build.

Read-only: reports, never edits.
"""
import io, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
SCAN = sys.argv[1:] or [".storybook", "🧰️framework", "✏️s"]
IMPORT = re.compile(r'import\s*(type\s*)?\{([^}]*)\}\s*from\s*["\'](\.[^"\']+)["\']', re.S)
EXPORT = re.compile(r'export\s+(?:async\s+)?(?:declare\s+)?(?:default\s+)?(?:class|function|const|let|var|type|interface|enum)\s+([A-Za-z0-9_$]+)')
EXPORT_LIST = re.compile(r'export\s*\{([^}]*)\}')
STAR = re.compile(r'export\s*\*\s*from\s*["\'](\.[^"\']+)["\']')

def resolve(base, spec):
    target = os.path.normpath(os.path.join(base, spec))
    for ext in ("", ".ts", ".tsx"):
        if os.path.isfile(target + ext):
            return target + ext
    for name in ("index.ts", "index.tsx"):
        if os.path.isfile(os.path.join(target, name)):
            return os.path.join(target, name)
    return None

def exports_of(path, depth=0):
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        return set()
    names = set(EXPORT.findall(text))
    for block in EXPORT_LIST.findall(text):
        for piece in block.split(","):
            piece = piece.strip().split(" as ")[-1].strip()
            if piece:
                names.add(piece.lstrip("type ").strip())
    if depth < 1:
        for spec in STAR.findall(text):
            nested = resolve(os.path.dirname(path), spec)
            if nested:
                names |= exports_of(nested, depth + 1)
    return names

files = subprocess.run(["grep", "-rlE", r'from\s*["\']\.', "--include=*.ts", "--include=*.tsx"] + SCAN,
                       capture_output=True).stdout.decode("utf-8").split("\n")
problems = 0
for path in [f for f in files if f and "node_modules" not in f]:
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        continue
    base = os.path.dirname(path) or "."
    for _, names, spec in IMPORT.findall(text):
        target = resolve(base, spec)
        if not target:
            continue
        available = exports_of(target)
        if not available:
            continue
        wanted = {n.strip().split(" as ")[0].replace("type ", "").strip() for n in names.split(",") if n.strip()}
        missing = sorted(w for w in wanted if w and w not in available)
        if missing:
            problems += 1
            print("%s\n  imports %s\n  from    %s\n  missing %s" % (path, ", ".join(sorted(wanted))[:90], spec, ", ".join(missing)))
print("imports with missing symbols: %d" % problems)
