# Records the library paths a tax4/tax5-style staged apply.py touches (check mode, writes nothing) and writes the declared inventory.json of its set. usage: mk-inventory.py <set dir> <staged/apply.py>
import hashlib, json, os, sys
ROOT = "/Users/ueli/Documents/semio"
setdir, script = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
source = open(script, encoding="utf-8").read()
head, loop = source.split("for pid,fn in PATCHES.items():")
os.chdir(ROOT)
sys.argv = [script, "--check"]
env = {"__file__": script, "__name__": "x"}
exec(compile(head, script, "exec"), env)
LIVE = env["LIVE"]
rows = {}
def wrap(name, kind, index):
    original = env.get(name)
    if original is None: return
    def call(*args):
        rel = args[index]
        path = os.path.join(LIVE, rel)
        rows.setdefault(path, {"kind": kind, "staged": None})
        if name == "add":
            rows[path]["staged"] = os.path.relpath(os.path.join(os.path.dirname(script), "new", args[0]), setdir)
        return original(*args)
    env[name] = call
wrap("edit", "patch", 0); wrap("retarget", "patch", 0); wrap("add", "create", 1)
exec(compile("for pid,fn in PATCHES.items():" + loop, script, "exec"), env)
for path, row in rows.items():
    row["kind"] = "create" if row["staged"] else row["kind"]
targets = []
for path in sorted(rows):
    row = rows[path]
    staged = row["staged"]
    sha = hashlib.sha256(open(os.path.join(setdir, staged), "rb").read()).hexdigest() if staged else None
    targets.append({"path": path, "kind": row["kind"], "staged": staged, "stagedSha": sha})
json.dump({"targets": targets}, open(os.path.join(setdir, "inventory.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(targets), "targets ->", os.path.join(setdir, "inventory.json"))
