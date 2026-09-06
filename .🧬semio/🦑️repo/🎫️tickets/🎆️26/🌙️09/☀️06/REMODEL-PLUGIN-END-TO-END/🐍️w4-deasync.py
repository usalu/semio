#!/usr/bin/env python3
"""🔧 W4's widening of 🐍️w1-deasync.py onto the editor/viewer/examples surface of the remodel
✳️any subset. Same rule: strip `async` from every prod `fn` (the framework traits and every
`app_commands!`-generated dispatch arm are sync), keep it on anything carrying a test attribute.
Skips 🧪️tests dirs and the lanes owned by other agents (⚙️engine is shared engine code with no
trait impls, 🚪️io is W6's, 🎚️config/👥️presence were W1's and are already clean)."""
import os, re, sys, collections

ROOT = "/Users/ueli/Documents/semio"
SUB = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
BASE = os.path.join(ROOT, SUB)

TARGET_DIRS = [
    os.path.join(BASE, "✏️editor/🎮️commands"),
    os.path.join(BASE, "✏️editor/🎭️modes"),
    os.path.join(BASE, "✏️editor/📌️panels"),
    os.path.join(BASE, "✏️editor/📚️examples"),
    os.path.join(BASE, "✏️editor/🗣️terminology"),
    os.path.join(BASE, "👁️viewer"),
    os.path.join(BASE, "📚️examples"),
]
TARGET_FILES = [os.path.join(BASE, "✏️editor/🦀️.rs")]

APPLY = "--apply" in sys.argv

files = list(TARGET_FILES)
for d in TARGET_DIRS:
    for dp, dns, fns in os.walk(d):
        dns[:] = [x for x in dns if x != "🧪️tests"]
        for f in fns:
            if f.endswith(".rs"):
                files.append(os.path.join(dp, f))
files = sorted(set(files))

afn = re.compile(r'^(\s*)((?:pub(?:\([^)]*\))?\s+)?)async\s+fn\s+([A-Za-z0-9_]+)')
test_attr = re.compile(r'^\s*#\[[^\]]*test[^\]]*\]')

changed_files, edits, skipped = [], [], []
for path in files:
    src = open(path, encoding="utf-8").read().splitlines(keepends=True)
    out, n = list(src), 0
    for i, line in enumerate(src):
        m = afn.match(line)
        if not m:
            continue
        has_test_attr = False
        j = i - 1
        while j >= 0:
            s = src[j]
            if test_attr.match(s):
                has_test_attr = True
                break
            if s.strip().startswith("#[") or s.strip().startswith("//") or not s.strip():
                j -= 1
                continue
            break
        if has_test_attr:
            skipped.append((os.path.relpath(path, ROOT), i + 1, m.group(3)))
        else:
            out[i] = re.sub(r'\basync\s+fn\b', 'fn', line, count=1)
            n += 1
            edits.append((os.path.relpath(path, ROOT), i + 1, m.group(3)))
    if n:
        changed_files.append((os.path.relpath(path, ROOT), n))
        if APPLY:
            open(path, "w", encoding="utf-8").write("".join(out))

for rel, n in changed_files:
    print(f"{n:4d}  {rel.replace(SUB + '/', '')}")
print(f"# files: {len(changed_files)}  edits: {len(edits)}  skipped(test): {len(skipped)}  applied={APPLY}")
print("# by fn name:", dict(sorted(collections.Counter(e[2] for e in edits).items(), key=lambda kv: (-kv[1], kv[0]))))
