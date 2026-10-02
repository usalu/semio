#!/usr/bin/env python3
"""🧭️ S2-PROCEDURAL — resolves module/include/import references left dangling by the REPO-PATH-BUDGET rename onto the names
now on disk (never moves a directory). A missing path segment resolves to the ONE sibling with the same leading emoji
whose slug equals it or extends/shortens it at a hyphen boundary. Dry run by default; `--apply` rewrites the literals.
Run: `python3 🧪️s2-procedural-dangling-refs.py [--apply]`.
"""
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
SCOPES = ["✏️s/🔌️plugins/🌀️procedural", "✏️s/🧑‍💻dev/🧩️composition/🧪️tests", "🌎️hub/🧩️compositions/🌀️procedural"]
PATTERNS = [
    re.compile(r'#\[path = "([^"]+)"\]'),
    re.compile(r'include_(?:str|bytes)!\("([^"]+)"\)'),
    re.compile(r'(?:from|import) "(\.{1,2}/[^"]+)"'),
    re.compile(r'import\("(\.{1,2}/[^"]+)"\)'),
]


def split_emoji(name):
    index = 0
    while index < len(name) and ord(name[index]) >= 128:
        index += 1
    return name[:index], name[index:]


def related(missing, candidate):
    left_emoji, left = split_emoji(missing)
    right_emoji, right = split_emoji(candidate)
    if left_emoji != right_emoji or not left or not right:
        return False
    return left == right or left.startswith(right + "-") or right.startswith(left + "-")


def resolve(base, target):
    parts = target.split("/")
    current = base
    resolved = []
    for part in parts:
        if part in ("", ".", ".."):
            current = os.path.normpath(os.path.join(current, part)) if part else current
            resolved.append(part)
            continue
        candidate = os.path.join(current, part)
        if os.path.exists(candidate):
            current = candidate
            resolved.append(part)
            continue
        if not os.path.isdir(current):
            return None
        siblings = [name for name in os.listdir(current) if related(part, name)]
        if len(siblings) != 1:
            return None
        current = os.path.join(current, siblings[0])
        resolved.append(siblings[0])
    return "/".join(resolved)


def main(apply):
    fixed, unresolved = [], []
    for scope in SCOPES:
        for dirpath, dirnames, files in os.walk(os.path.join(ROOT, scope)):
            dirnames[:] = [name for name in dirnames if name not in ("dist", "node_modules", "target") and not name.endswith("sequence-runtime")]
            for name in files:
                if not name.endswith((".rs", ".ts", ".tsx")):
                    continue
                path = os.path.join(dirpath, name)
                text = open(path, encoding="utf-8").read()
                updated = text
                for pattern in PATTERNS:
                    for match in pattern.finditer(text):
                        target = match.group(1)
                        if target == "." or os.path.exists(os.path.normpath(os.path.join(dirpath, target))):
                            continue
                        repaired = resolve(dirpath, target)
                        if repaired is None:
                            unresolved.append((path.replace(ROOT + "/", ""), target))
                            continue
                        updated = updated.replace(match.group(0), match.group(0).replace(target, repaired), 1)
                        fixed.append((path.replace(ROOT + "/", ""), target, repaired))
                if apply and updated != text:
                    open(path, "w", encoding="utf-8").write(updated)
    for row in fixed:
        print("fix", *row)
    for row in unresolved:
        print("unresolved", *row)
    print(f"fixed={len(fixed)} unresolved={len(unresolved)} applied={apply}")


if __name__ == "__main__":
    main("--apply" in sys.argv)
