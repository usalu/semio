import os, json, re

ROOT = "/Users/ueli/Documents/semio"
files = [l.strip() for l in open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_c_files.txt", encoding="utf-8") if l.strip()]

SLUG_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")

moves = []
for rel in files:
    old_abs = os.path.join(ROOT, rel[2:] if rel.startswith("./") else rel)
    d = os.path.dirname(old_abs)
    base = os.path.basename(old_abs)
    assert base.startswith("🎯️")
    stem = base[len("🎯️"):]
    if stem.endswith(".schema.json"):
        case = stem[:-len(".schema.json")]
        leaf = "🔣️.schema.json"
    elif stem.endswith(".json"):
        case = stem[:-len(".json")]
        leaf = "🔣️.json"
    else:
        raise ValueError(f"unexpected suffix: {base}")
    if not SLUG_RE.match(case):
        print("SLUG MISMATCH:", base, case)
        continue
    new_dir = os.path.join(d, f"🧫️{case}")
    new_abs = os.path.join(new_dir, leaf)
    moves.append((old_abs, new_abs, new_dir))

# check collisions: same new_abs twice, or new_dir already exists with different content
targets = {}
for old, new, nd in moves:
    targets.setdefault(new, []).append(old)
collisions = {k: v for k, v in targets.items() if len(v) > 1}
print("post-move collisions:", len(collisions))
for k, v in collisions.items():
    print(" ", k, v)

if not collisions:
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
    with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_c_moves.json", "w") as f:
        json.dump([[os.path.relpath(o, ROOT), os.path.relpath(n, ROOT)] for o, n in executed], f, ensure_ascii=False, indent=2)
