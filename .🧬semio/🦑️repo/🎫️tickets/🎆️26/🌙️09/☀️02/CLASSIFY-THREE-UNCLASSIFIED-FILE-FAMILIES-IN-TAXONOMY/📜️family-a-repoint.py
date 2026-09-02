import json, os, re

ROOT = "/Users/ueli/Documents/semio"
moves = json.load(open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_a_moves.json", encoding="utf-8"))

# basename -> new basename (all old basenames of same category map to the same new one)
mapping = {}
for old, new in moves:
    old_b = os.path.basename(old)
    new_b = os.path.basename(new)
    mapping[old_b] = new_b

print("mapping entries:", len(mapping))

files = [l.strip() for l in open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_a_ref_files.txt", encoding="utf-8") if l.strip()]

# sort basenames longest-first to avoid partial-overlap issues (none expected, but safe)
old_basenames_sorted = sorted(mapping.keys(), key=len, reverse=True)

changed_files = []
unchanged_but_matched = []
for rel in files:
    path = os.path.join(ROOT, rel)
    if not os.path.isfile(path):
        continue
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    original = content
    for old_b in old_basenames_sorted:
        if old_b in content:
            content = content.replace(old_b, mapping[old_b])
    if content != original:
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)
        changed_files.append(rel)

print("changed files:", len(changed_files))
with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_a_changed_files.txt", "w", encoding="utf-8") as f:
    for rel in changed_files:
        f.write(rel + "\n")
