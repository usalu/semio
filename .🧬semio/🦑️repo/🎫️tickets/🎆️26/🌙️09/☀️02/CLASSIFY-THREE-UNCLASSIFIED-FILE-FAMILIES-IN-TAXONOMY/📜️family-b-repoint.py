import os

ROOT = "/Users/ueli/Documents/semio"
files = set()
for fn in ["family_b_fixture_ref_files.txt", "family_b_schema_ref_files.txt"]:
    for l in open(f"/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/{fn}", encoding="utf-8"):
        l = l.strip()
        if l:
            files.add(l)

print("total files to process:", len(files))

changed = []
for rel in sorted(files):
    path = os.path.join(ROOT, rel)
    if not os.path.isfile(path):
        print("MISSING", rel); continue
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    original = content
    content = content.replace("🧪️fixture.json", "🧪️fixture/🔣️.json")
    content = content.replace("🧪️schema.json", "🧪️schema/🔣️.json")
    if content != original:
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)
        changed.append(rel)

print("changed:", len(changed))
with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_b_changed_files.txt", "w", encoding="utf-8") as f:
    for rel in changed:
        f.write(rel + "\n")
