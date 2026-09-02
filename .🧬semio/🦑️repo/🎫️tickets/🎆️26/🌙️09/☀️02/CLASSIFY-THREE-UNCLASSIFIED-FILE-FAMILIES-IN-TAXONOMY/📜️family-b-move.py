import os, json

ROOT = "/Users/ueli/Documents/semio"
files = [l.strip() for l in open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_b_files.txt", encoding="utf-8") if l.strip()]

moves = []
for rel in files:
    old_abs = os.path.join(ROOT, rel[2:] if rel.startswith("./") else rel)
    d = os.path.dirname(old_abs)
    base = os.path.basename(old_abs)
    subdir_name = "🧪️fixture" if base == "🧪️fixture.json" else "🧪️schema"
    new_dir = os.path.join(d, subdir_name)
    new_abs = os.path.join(new_dir, "🔣️.json")
    moves.append((old_abs, new_abs, new_dir))

executed = []
for old, new, newdir in moves:
    if not os.path.exists(old):
        print("MISSING SOURCE", old); continue
    if os.path.exists(new):
        print("TARGET EXISTS", new); continue
    os.makedirs(newdir, exist_ok=True)
    os.rename(old, new)
    executed.append((old, new))

print("executed:", len(executed))
with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_b_moves.json", "w") as f:
    json.dump([[os.path.relpath(o, ROOT), os.path.relpath(n, ROOT)] for o, n in executed], f, ensure_ascii=False, indent=2)
