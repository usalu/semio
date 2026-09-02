import json, os

ROOT = "/Users/ueli/Documents/semio"
DATA_PATH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_a_files.json"
data = json.load(open(DATA_PATH, encoding="utf-8"))

moves = []

def add_move(old_rel, new_name):
    old_abs = os.path.join(ROOT, old_rel[2:] if old_rel.startswith("./") else old_rel)
    d = os.path.dirname(old_abs)
    new_abs = os.path.join(d, new_name)
    moves.append((old_abs, new_abs))

for p in data["pack"]:
    add_move(p, "🎒️.pack.semio")
for p in data["bin"]:
    add_move(p, "💾️.bin")
for p in data["las"]:
    add_move(p, "🧊️.las")
for p in data["ply"]:
    add_move(p, "🧊️.ply")
for p in data["zip"]:
    add_move(p, "🗜️.zip")

# check for post-rename collisions (two old files mapping to the same new path)
targets = {}
for old, new in moves:
    targets.setdefault(new, []).append(old)
collisions = {k: v for k, v in targets.items() if len(v) > 1}
print("post-rename collisions:", len(collisions))
for k, v in collisions.items():
    print(" ", k, v)

if not collisions:
    executed = []
    for old, new in moves:
        if not os.path.exists(old):
            print("MISSING SOURCE, skip:", old)
            continue
        if os.path.exists(new):
            print("TARGET ALREADY EXISTS, skip:", new)
            continue
        os.rename(old, new)
        executed.append((old, new))
    print("executed moves:", len(executed))
    with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/family_a_moves.json", "w") as f:
        json.dump([[os.path.relpath(o, ROOT), os.path.relpath(n, ROOT)] for o, n in executed], f, ensure_ascii=False, indent=2)
