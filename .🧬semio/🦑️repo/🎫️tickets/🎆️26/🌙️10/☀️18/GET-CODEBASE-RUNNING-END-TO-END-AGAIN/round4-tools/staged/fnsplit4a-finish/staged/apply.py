"""🚚 Applies the staged fnsplit4a-finish set to the live tree, anchored and all-or-nothing. Default is a dry run. usage: apply.py [--root <repo>] [--write] [--undo] [--verify]
 modify ops need the live file to hash to baseSha256; create ops need the path absent (or already identical). Any drift refuses the whole set. --write stages every new file next to its target first, backs the originals up, then renames; any failure restores everything. --undo restores from the backup; --verify only compares live hashes with the manifest."""
import hashlib, json, os, shutil, sys, time

here = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
root = None
if "--root" in args:
    root = args[args.index("--root") + 1]
if root is None:
    p = here
    while p != os.path.dirname(p) and not os.path.exists(os.path.join(p, ".git")):
        p = os.path.dirname(p)
    root = p
write, undo, verify = "--write" in args, "--undo" in args, "--verify" in args
manifest = json.load(open(os.path.join(here, "manifest.json"), encoding="utf8"))
ops = manifest["ops"]
backup = os.path.join(here, "backup")
sha = lambda b: hashlib.sha256(b).hexdigest()
live = lambda o: os.path.join(root, *o["path"].split("/"))
cur = lambda o: sha(open(live(o), "rb").read()) if os.path.isfile(live(o)) else None
blob = lambda o: open(os.path.join(here, "blobs", o["sha256"][:24]), "rb").read()

def report():
    state = {"pending": [], "applied": [], "drift": []}
    for o in ops:
        c = cur(o)
        if c == o["sha256"]:
            state["applied"].append(o["path"])
        elif c == o["baseSha256"]:
            state["pending"].append(o["path"])
        else:
            state["drift"].append(o["path"])
    return state

if verify:
    s = report()
    print(f"applied={len(s['applied'])} pending={len(s['pending'])} drift={len(s['drift'])}")
    for d in s["drift"]:
        print("DRIFT", d)
    sys.exit(1 if s["drift"] or s["pending"] else 0)

if undo:
    if not os.path.isdir(backup):
        sys.exit("no backup")
    j = json.load(open(os.path.join(backup, "journal.json"), encoding="utf8"))
    for e in j:
        t = os.path.join(root, *e["path"].split("/"))
        if e["base"] is None:
            if os.path.isfile(t) and sha(open(t, "rb").read()) == e["sha256"]:
                os.remove(t)
        elif os.path.isfile(t) and sha(open(t, "rb").read()) == e["sha256"]:
            shutil.copyfile(os.path.join(backup, e["base"][:24]), t)
        else:
            sys.exit("refusing undo: " + e["path"] + " changed after the apply")
    shutil.rmtree(backup)
    print("undone", len(j))
    sys.exit(0)

s = report()
print(f"root={root} ops={len(ops)} pending={len(s['pending'])} already-applied={len(s['applied'])} drift={len(s['drift'])}")
for d in s["drift"]:
    print("DRIFT", d)
for o in ops:
    if len(o["path"]) > 255:
        s["drift"].append(o["path"])
        print("PATH-TOO-LONG", o["path"])
if s["drift"]:
    sys.exit("refused: drift or long path (nothing written)")
if not write:
    print("dry run: nothing written (use --write)")
    sys.exit(0)
todo = [o for o in ops if o["path"] in set(s["pending"])]
if os.path.isdir(backup):
    sys.exit("refused: a backup exists (undo or remove it first)")
os.makedirs(backup)
tmps, made, journal = [], [], []
try:
    for o in todo:
        t = live(o)
        os.makedirs(os.path.dirname(t), exist_ok=True)
        tmp = t + ".fnsplit4a-finish.tmp"
        open(tmp, "wb").write(blob(o))
        tmps.append((tmp, t, o))
    for tmp, t, o in tmps:
        if o["baseSha256"]:
            shutil.copyfile(t, os.path.join(backup, o["baseSha256"][:24]))
    for tmp, t, o in tmps:
        os.replace(tmp, t)
        made.append(o)
        journal.append({"path": o["path"], "base": o["baseSha256"], "sha256": o["sha256"]})
except BaseException as e:
    for tmp, t, o in tmps:
        if os.path.exists(tmp):
            os.remove(tmp)
    for o in made:
        t = live(o)
        if o["baseSha256"]:
            shutil.copyfile(os.path.join(backup, o["baseSha256"][:24]), t)
        elif os.path.exists(t):
            os.remove(t)
    shutil.rmtree(backup, ignore_errors=True)
    sys.exit("failed and rolled back: " + repr(e))
json.dump(journal, open(os.path.join(backup, "journal.json"), "w", encoding="utf8"), ensure_ascii=False)
print("applied", len(journal), "files at", time.strftime("%H:%M:%S"))
