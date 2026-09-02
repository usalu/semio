#!/usr/bin/env python3
"""🧭️ Repoints relative TypeScript imports at files the taxonomy sweep has renamed.

Same failure as the Rust `#[path]`/`include_*` case, on the TS side: the sweep renames a leaf
(`🟦️interactive-job.ts` -> `🟦️.ts`, `📦️index.tsx` -> `🟦️.tsx`) and updates importers in a later pass,
so in between Vite/Rollup dies with `Could not resolve "…"` and the Storybook build never completes.

Driven by git's rename records first, then by the sweep's naming shape when git has degraded the
rename into an add+delete pair (which it does depending on staging). Only rewrites a specifier that
currently resolves to nothing, so it can never redirect a working import.
"""
import io, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
SCAN = sys.argv[1:] or [".storybook", "🧰️framework", "✏️s", "♻️mit-bestand", "🌎️hub", "🧪️tests"]
EXTS = ("", ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts")

renames = {}
for line in subprocess.run(["git", "status", "--porcelain"], capture_output=True).stdout.decode("utf-8").split("\n"):
    if "R" in line[:2] and " -> " in line:
        old, new = line[3:].split(" -> ", 1)
        renames[old] = new

def resolves(base, spec):
    target = os.path.normpath(os.path.join(base, spec))
    for ext in EXTS:
        if os.path.isfile(target + ext):
            return True
    return os.path.isfile(os.path.join(target, "index.ts")) or os.path.isfile(os.path.join(target, "index.tsx"))

def resolve_new(base, spec):
    absolute = os.path.normpath(os.path.join(base, spec))
    for ext in (".ts", ".tsx", ""):
        moved = renames.get(os.path.relpath(absolute + ext, ROOT))
        if moved and os.path.isfile(os.path.join(ROOT, moved)):
            return os.path.join(ROOT, moved)
    # 🗂️ shape fallback: the leaf collapsed to a bare `🟦️.<ext>` in the same directory
    parent, leaf = os.path.dirname(absolute), os.path.basename(absolute)
    if not os.path.isdir(parent):
        parent = base   # 🧭️ the target directory itself was moved away; search from the importer
    stem = re.sub(r"^[^A-Za-z0-9]+", "", leaf)
    # 🧭️ Honour the specifier's own extension (`.json`, `.css`, …), not just TS, and also try the
    #    sweep's leaf-to-directory shape: `🔣️schema.json` -> `🧬️schema/🔣️.json`.
    own_ext = os.path.splitext(leaf)[1]
    if own_ext and own_ext not in (".ts", ".tsx"):
        bare = re.sub(r"^[^A-Za-z0-9]+", "", os.path.splitext(leaf)[0])
        for entry in sorted(os.listdir(parent)) if os.path.isdir(parent) else []:
            directory = os.path.join(parent, entry)
            if os.path.isdir(directory) and re.sub(r"^[^A-Za-z0-9]+", "", entry) == bare:
                for candidate in sorted(os.listdir(directory)):
                    if candidate.endswith(own_ext):
                        return os.path.join(directory, candidate)
    for ext in ((own_ext,) if own_ext and own_ext not in (".ts", ".tsx") else (".ts", ".tsx")):
        for candidate in sorted(os.listdir(parent)):
            if not candidate.endswith(ext):
                continue
            tail = re.sub(r"^[^A-Za-z0-9]+", "", candidate[: -len(ext)])
            if tail in ("", "index", stem):
                return os.path.join(parent, candidate)
    # 🔎️ Last resort: the sweep sometimes moves a leaf into a different directory entirely. Search a
    #    BOUNDED subtree (the nearest ancestor holding `📦️packages`, else four levels up) for the same
    #    basename and accept it only when the match is unique — an ambiguous match is left alone.
    root, hops = parent, 0
    while hops < 4 and os.path.basename(root) and not os.path.isdir(os.path.join(root, "📦️packages")):
        root, hops = os.path.dirname(root) or ".", hops + 1
    matches = []
    for current, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "dist")]
        if leaf in files:
            matches.append(os.path.join(current, leaf))
        if len(matches) > 1:
            return None
    return matches[0] if len(matches) == 1 else None

EXPORTS = re.compile(r'export\s+(?:async\s+)?(?:declare\s+)?(?:default\s+)?(?:class|function|const|let|var|type|interface|enum)\s+([A-Za-z0-9_$]+)|export\s*\{([^}]*)\}')

def exports_named(path, wanted):
    """🛡️ A collapsed module often becomes a package BARREL that re-exports only part of the old
       surface, so a repoint that matches by path shape can still break the build with
       `"X" is not exported by …`. Only accept a candidate that actually exports what is imported."""
    if not wanted:
        return True
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        return False
    if "export *" in text:
        return True   # re-export barrel: cannot decide cheaply, do not block
    names = set()
    for direct, block in EXPORTS.findall(text):
        if direct:
            names.add(direct)
        for piece in (block or "").split(","):
            piece = piece.strip().split(" as ")[-1].strip().replace("type ", "")
            if piece:
                names.add(piece)
    return wanted <= names


SPEC = re.compile(r'((?:from|import)\s*\(?\s*["\'])(\.[^"\']+)(["\'])')
files = subprocess.run(["grep", "-rlE", r'from\s*["\']\.|import\s*\(\s*["\']\.', "--include=*.ts", "--include=*.tsx"] + SCAN,
                       capture_output=True).stdout.decode("utf-8").split("\n")

total, touched = 0, []
for path in [f for f in files if f and "node_modules" not in f]:
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        continue
    base, fixes = os.path.dirname(path) or ".", 0

    named = {}
    for block, spec_text in re.findall(r'import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*["\'](\.[^"\']+)["\']', text):
        named[spec_text] = {n.strip().split(" as ")[0].replace("type ", "").strip() for n in block.split(",") if n.strip()}

    def repair(match):
        global fixes
        spec = match.group(2)
        if resolves(base, spec):
            return match.group(0)
        moved = resolve_new(base, spec)
        if not moved:
            return match.group(0)
        if os.path.abspath(moved) == os.path.abspath(path):
            return match.group(0)   # 🛡️ never repoint a module at itself
        if not exports_named(moved, named.get(spec, set())):
            return match.group(0)
        rel = os.path.relpath(moved, base)
        rel = re.sub(r"\.(ts|tsx)$", "", rel)
        if not rel.startswith("."):
            rel = "./" + rel
        fixes += 1
        return match.group(1) + rel + match.group(3)

    repaired = SPEC.sub(repair, text)
    if fixes:
        io.open(path, "w", encoding="utf-8").write(repaired)
        total += fixes
        touched.append((path, fixes))

print("ts imports repointed: %d across %d files" % (total, len(touched)))
for path, count in touched[:12]:
    print("  %3d  %s" % (count, path))
