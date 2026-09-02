#!/usr/bin/env python3
"""🩹️ Repairs `import … from "./🟦️"`-style SELF imports.

An earlier, unguarded pass of `🧭️realign-ts-imports.py` resolved a collapsed module by path shape
alone, and its bounded search sometimes matched the importing file itself — producing an import a
module makes from itself. Rollup reports these late as `"X" is not exported by <the same file>`.

Repair strategy: for each self-import, find the module that actually exports every named binding,
searching the nearest bounded subtree and accepting only a unique match. Anything ambiguous is left
alone and reported, so nothing is guessed.
"""
import io, os, re, subprocess

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
SCAN = [".storybook", "🧰️framework", "✏️s"]
NAMED = re.compile(r'import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*["\'](\.[^"\']+)["\']')
EXPORTS = re.compile(r'export\s+(?:async\s+)?(?:declare\s+)?(?:default\s+)?(?:class|function|const|let|var|type|interface|enum)\s+([A-Za-z0-9_$]+)|export\s*\{([^}]*)\}')

def resolve(base, spec):
    target = os.path.normpath(os.path.join(base, spec))
    for ext in ("", ".ts", ".tsx"):
        if os.path.isfile(target + ext):
            return target + ext
    return None

def exports_of(path):
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        return set(), False
    names, star = set(), "export *" in text
    for direct, block in EXPORTS.findall(text):
        if direct:
            names.add(direct)
        for piece in (block or "").split(","):
            piece = piece.strip().split(" as ")[-1].strip().replace("type ", "")
            if piece:
                names.add(piece)
    return names, star

def bounded_root(path):
    root, hops = os.path.dirname(path), 0
    while hops < 5 and os.path.basename(root):
        if os.path.isdir(os.path.join(root, "📦️packages")):
            return root
        root, hops = os.path.dirname(root) or ".", hops + 1
    return root or "."

files = subprocess.run(["grep", "-rlE", r'from\s*["\']\.', "--include=*.ts", "--include=*.tsx"] + SCAN,
                       capture_output=True).stdout.decode("utf-8").split("\n")
repaired, ambiguous, unresolved = 0, [], []
for path in [f for f in files if f and "node_modules" not in f]:
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        continue
    base, changed = os.path.dirname(path) or ".", False
    for block, spec in NAMED.findall(text):
        target = resolve(base, spec)
        if not target or os.path.abspath(target) != os.path.abspath(path):
            continue
        wanted = {n.strip().split(" as ")[0].replace("type ", "").strip() for n in block.split(",") if n.strip()}
        if not wanted:
            continue
        matches = []
        for current, dirs, names in os.walk(bounded_root(path)):
            dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "dist")]
            for name in names:
                if not name.endswith((".ts", ".tsx")):
                    continue
                candidate = os.path.join(current, name)
                if os.path.abspath(candidate) == os.path.abspath(path):
                    continue
                available, star = exports_of(candidate)
                if wanted <= available or (star and wanted & available):
                    matches.append(candidate)
        matches = sorted(set(matches))
        if len(matches) != 1:
            (ambiguous if matches else unresolved).append((path, spec, len(matches)))
            continue
        rel = re.sub(r"\.(ts|tsx)$", "", os.path.relpath(matches[0], base))
        if not rel.startswith("."):
            rel = "./" + rel
        text = text.replace('from "%s"' % spec, 'from "%s"' % rel)
        changed, repaired = True, repaired + 1
    if changed:
        io.open(path, "w", encoding="utf-8").write(text)

print("self-imports repaired: %d, ambiguous: %d, unresolved: %d" % (repaired, len(ambiguous), len(unresolved)))
for path, spec, n in (ambiguous + unresolved)[:8]:
    print("  %s  %s  (%d candidates)" % (path, spec, n))
