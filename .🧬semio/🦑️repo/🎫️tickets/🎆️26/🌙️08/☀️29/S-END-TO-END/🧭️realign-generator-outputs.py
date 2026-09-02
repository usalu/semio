#!/usr/bin/env python3
"""🧭️ Repoints `generatorContracts[*].outputRoots` at files the taxonomy sweep has renamed.

The validator inside `semio-framework-graph`'s build script (and Storybook's `main.ts`) rejects the
whole taxonomy when a `tracked` output path does not exist, which hard-fails every cargo command and
every Storybook build repo-wide. The sweep moves the files and updates the contracts in separate
passes; in between, the contract still names the old path.

Driven by git's own rename records, so it cannot invent a destination: a path is rewritten only when
it is currently missing AND git says that exact path was renamed to something that exists. Rewrites
the catalog digest afterwards, since taxonomy.json is itself part of the pinned chain.
"""
import hashlib, io, json, os, subprocess

ROOT = "/Users/ueli/Documents/semio"
TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
os.chdir(ROOT)

renames, fields = {}, subprocess.run(["git", "status", "--porcelain", "-z"], capture_output=True).stdout.decode("utf-8").split("\0")
i = 0
while i < len(fields):
    entry = fields[i]
    if not entry:
        i += 1
        continue
    if "R" in entry[:2] and i + 1 < len(fields):
        renames[fields[i + 1]] = entry[3:]
        i += 2
    else:
        i += 1


def moved_by_shape(path):
    """🗂️ Git's rename detection flips to add+delete depending on staging, so fall back to the sweep's
    own naming shape: a leaf `<emoji><stem>.<ext>` becomes `<emoji><stem>/<emoji>.<ext>`."""
    import re as _re
    parent, leaf = os.path.dirname(path), os.path.basename(path)
    match = _re.match(r"^([^A-Za-z0-9]*)([A-Za-z0-9._-]+)\.([A-Za-z0-9]+)$", leaf)
    if not match or not os.path.isdir(parent):
        return None
    stem, ext = match.group(2), match.group(3)
    for entry in os.listdir(parent):
        directory = os.path.join(parent, entry)
        if os.path.isdir(directory) and _re.sub(r"^[^A-Za-z0-9]+", "", entry) == stem:
            for candidate in os.listdir(directory):
                if candidate.endswith("." + ext):
                    return os.path.join(directory, candidate)
    return None


taxonomy = json.load(io.open(TAX, encoding="utf-8"))
moved, missing = [], []
for contract_id, contract in taxonomy.get("generatorContracts", {}).items():
    for output in contract.get("outputRoots", []) or []:
        path = output.get("path")
        if not path or os.path.exists(path):
            continue
        destination = renames.get(path) or moved_by_shape(path)
        if destination and os.path.exists(destination):
            output["path"] = destination
            moved.append((contract_id, path, destination))
        else:
            missing.append((contract_id, path))

for contract_id, contract in taxonomy.get("generatorContracts", {}).items():
    patterns = contract.get("inputPatterns")
    if not isinstance(patterns, list):
        continue
    for index, pattern in enumerate(patterns):
        if not isinstance(pattern, str) or not pattern or os.path.exists(pattern):
            continue
        destination = renames.get(pattern) or moved_by_shape(pattern)
        if destination and os.path.exists(destination):
            patterns[index] = destination
            moved.append((contract_id + " inputPatterns", pattern, destination))
    # 🧭️ The validator compares against JS `.sort()`, i.e. UTF-16 code-unit order. Python sorts by
    # code point, which disagrees for astral-plane emoji — encode to UTF-16-BE to match.
    contract["inputPatterns"] = sorted(set(patterns), key=lambda value: value.encode("utf-16-be"))

if moved:
    io.open(TAX, "w", encoding="utf-8").write(json.dumps(taxonomy, ensure_ascii=False, indent=2) + "\n")
print("generator outputs repointed: %d" % len(moved))
for contract_id, old, new in moved:
    print("  %s\n     %s\n  -> %s" % (contract_id, old, new))
if missing:
    print("still missing (no git rename record — needs its generator run):")
    for contract_id, path in missing:
        print("  %s  %s" % (contract_id, path))
