#!/usr/bin/env python3
"""🗂️ Moves the flat mutation leaves of one `🧬️mutations` collection into domain-operation groups (lane P4, decision D3).

Each `🧬️mutations/<leaf>` becomes `🧬️mutations/<group>/<leaf>`. Every relative literal path in a moved file (`#[path]`,
`include_str!`, `include_bytes!`) that leaves the moved leaf is recomputed from the new location; the leaf descriptor's
`owner` follows; every `#[path]`/include literal elsewhere in `--rewrite` files that pointed into a moved leaf is retargeted.
A directory without a leaf descriptor (an editor command) moves the same way. Prints the `mutationDomainOwners` rows (`group → {leaf → semantic kind}`) for the taxonomy.

usage: 🗂️group-leaves.py <mutations dir> <groups.json> [--rewrite file ...] [--apply]
groups.json: {"<emoji+group>": ["<leaf dir>", ...], ...}
"""
import json, os, re, shutil, sys

REPO = "/Users/ueli/Documents/semio"
root = os.path.abspath(sys.argv[1])
groups = json.load(open(sys.argv[2], encoding="utf-8"))
apply = "--apply" in sys.argv
rewrite = sys.argv[sys.argv.index("--rewrite") + 1:] if "--rewrite" in sys.argv else []
rewrite = [os.path.abspath(p) for p in rewrite if p != "--apply"]
LITERAL = re.compile(r'(#\[path\s*=\s*"|include_str!\(\s*"|include_bytes!\(\s*")([^"]+)(")')
moves = {}
for group, leaves in groups.items():
    for leaf in leaves:
        old = os.path.join(root, leaf)
        if not os.path.isdir(old):
            raise SystemExit(f"missing leaf {leaf}")
        moves[old] = os.path.join(root, group, leaf)


def moved_path(path):
    for old, new in moves.items():
        if path == old or path.startswith(old + os.sep):
            return new + path[len(old):]
    return path


def fix_text(text, old_file, new_file):
    def rep(m):
        target = os.path.normpath(os.path.join(os.path.dirname(old_file), m.group(2)))
        new_target = moved_path(target)
        rel = os.path.relpath(new_target, os.path.dirname(new_file))
        return m.group(1) + rel + m.group(3) if rel != m.group(2) else m.group(0)
    return LITERAL.sub(rep, text)


rows = {}
changed = []
for old, new in moves.items():
    leaf = os.path.basename(old)
    group = os.path.basename(os.path.dirname(new))
    if os.path.exists(os.path.join(old, "🔣️.json")):
        rows.setdefault(group, {})[leaf] = json.load(open(os.path.join(old, "🔣️.json"), encoding="utf-8"))["semanticKind"]
    for dirpath, _, files in os.walk(old):
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            text = open(path, encoding="utf-8").read()
            fixed = fix_text(text, path, moved_path(path))
            if fixed != text:
                changed.append((path, fixed))
for path in rewrite:
    text = open(path, encoding="utf-8").read()
    fixed = fix_text(text, path, path)
    if fixed != text:
        changed.append((path, fixed))
print(f"leaves={len(moves)} files-with-literal-changes={len(changed)}")
print(json.dumps(rows, ensure_ascii=False, indent=1))
if not apply:
    sys.exit(0)
for path, fixed in changed:
    open(path, "w", encoding="utf-8").write(fixed)
for old, new in moves.items():
    os.makedirs(os.path.dirname(new), exist_ok=True)
    shutil.move(old, new)
    descriptor_path = os.path.join(new, "🔣️.json")
    if not os.path.exists(descriptor_path):
        continue
    text = open(descriptor_path, encoding="utf-8").read()
    old_owner = os.path.relpath(old, REPO)
    new_owner = os.path.relpath(new, REPO)
    open(descriptor_path, "w", encoding="utf-8").write(text.replace(f'"owner": "{old_owner}"', f'"owner": "{new_owner}"'))
json.dump({os.path.relpath(root, REPO): rows}, open(os.path.join(os.path.dirname(os.path.abspath(sys.argv[2])), os.path.basename(sys.argv[2]).replace(".json", ".domain-owners.json")), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("moved")
