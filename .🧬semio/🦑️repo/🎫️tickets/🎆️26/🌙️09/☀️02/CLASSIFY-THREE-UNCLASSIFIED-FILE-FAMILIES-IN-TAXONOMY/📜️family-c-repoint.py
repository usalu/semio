import json, os

ROOT = "/Users/ueli/Documents/semio"
mapping = json.load(open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_c_basename_map.json", encoding="utf-8"))
files = [l.strip() for l in open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_c_ref_files_all.txt", encoding="utf-8") if l.strip()]

old_sorted = sorted(mapping.keys(), key=len, reverse=True)

changed = []
for rel in files:
    path = os.path.join(ROOT, rel)
    if not os.path.isfile(path):
        print("MISSING", rel); continue
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    original = content
    for old_b in old_sorted:
        if old_b in content:
            content = content.replace(old_b, mapping[old_b])
    if content != original:
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)
        changed.append(rel)

print("changed:", len(changed))
with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_c_changed_files.txt", "w", encoding="utf-8") as f:
    for rel in changed:
        f.write(rel + "\n")
