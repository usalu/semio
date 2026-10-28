"""Lands the setup-g batch for gate-g. usage: python3 stage.py check | apply | revert | status
check  : every edited file still equals the base this batch was cut from (no peer edited it since), every new file is absent, every anchor is unique.
apply  : check, then copy overlay/ into the checkout (edited files replaced, new files created).
revert : restore base/ copies of the edited files and remove the new files (empty directories left behind are removed too).
status : per file: base | applied | drift | absent."""
import hashlib, json, os, shutil, subprocess, sys
ROOT = os.environ.get("SEMIO_STAGE_ROOT", "/Users/ueli/Documents/semio") .rstrip("/") + "/"
HERE = os.path.dirname(os.path.abspath(__file__)) + "/"
index = json.load(open(HERE + "index-edited.json", encoding="utf8"))
new_files = sorted(os.path.relpath(os.path.join(b, n), HERE + "overlay") for b, _d, ns in os.walk(HERE + "overlay") for n in ns if os.path.relpath(os.path.join(b, n), HERE + "overlay") not in index)
sha = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest()
def state(path):
    full = ROOT + path
    if path in index:
        if not os.path.exists(full): return "absent"
        current = sha(full)
        return "base" if current == index[path]["base"] else "applied" if current == index[path]["new"] else "drift"
    if not os.path.exists(full): return "absent"
    return "applied" if sha(full) == sha(HERE + "overlay/" + path) else "drift"
def rows(): return [(p, state(p)) for p in sorted(index)] + [(p, state(p)) for p in new_files]
cmd = sys.argv[1] if len(sys.argv) > 1 else "status"
if cmd == "status":
    for p, s in rows(): print(s, p)
elif cmd == "check":
    bad = [(p, s) for p, s in rows() if s not in ("base", "absent") or (s == "absent" and p in index)]
    for p, s in bad: print("NOT CLEAN", s, p)
    anchors = subprocess.run([sys.executable, "-I", HERE + "build-overlay.py", "--check"], capture_output=True, text=True)
    print(anchors.stdout.strip() or anchors.stderr.strip())
    sys.exit(1 if bad or anchors.returncode else 0)
elif cmd == "apply":
    if subprocess.run([sys.executable, "-I", HERE + "stage.py", "check"]).returncode: raise SystemExit("check failed; nothing written")
    for p in list(index) + new_files:
        os.makedirs(os.path.dirname(ROOT + p), exist_ok=True); shutil.copy2(HERE + "overlay/" + p, ROOT + p)
    print("applied", len(index) + len(new_files), "files")
elif cmd == "revert":
    for p in index:
        if state(p) in ("applied", "drift"): shutil.copy2(HERE + "base/" + p, ROOT + p)
    for p in new_files:
        if os.path.exists(ROOT + p): os.remove(ROOT + p)
        d = os.path.dirname(ROOT + p)
        while d.startswith(ROOT) and os.path.isdir(d) and not os.listdir(d): os.rmdir(d); d = os.path.dirname(d)
    print("reverted")
else: raise SystemExit("usage: stage.py check|apply|revert|status")
