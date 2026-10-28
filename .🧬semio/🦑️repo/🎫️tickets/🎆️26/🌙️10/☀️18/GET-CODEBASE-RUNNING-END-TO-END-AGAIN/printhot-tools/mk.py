"""🏭 Builds a staged batch of lane printhot from the LIVE base files and an anchored edit script: `python3 -I mk.py <batch>` (reads `<batch>/edit.py`, writes `staged/<batch>/{manifest.json,files/,base/,stage.py,verify.sh}`); rerun after a gate landing touched a file (rebase)."""
import hashlib, importlib.util, json, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and ROOT != "/":
    ROOT = os.path.dirname(ROOT)
batch = sys.argv[1]
edit = os.path.join(HERE, batch, "edit.py")
source = open(edit, encoding="utf8").read()
scope = {"__name__": "mk", "__file__": edit}
exec(compile(source.replace("\nrun(os.path.abspath(sys.argv[1]), sys.argv[2])", ""), edit, "exec"), scope)
EDITS, NEW = scope["EDITS"], scope["NEW"]
out = os.path.join(HERE, "staged", batch)
shutil.rmtree(out, ignore_errors=True)
os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
work = tempfile.mkdtemp(prefix="ph-mk-")
digest = lambda data: hashlib.sha256(data).hexdigest()
for rel in EDITS:
    os.makedirs(os.path.dirname(os.path.join(work, rel)), exist_ok=True)
    shutil.copy2(os.path.join(ROOT, rel), os.path.join(work, rel))
subprocess.run([sys.executable, "-I", edit, work, "apply"], check=True)
manifest = {}
for index, rel in enumerate(list(EDITS) + list(NEW)):
    name = "%02d%s" % (index, os.path.splitext(rel)[1])
    new = open(os.path.join(work, rel), "rb").read()
    base = None
    if rel in EDITS:
        data = open(os.path.join(ROOT, rel), "rb").read(); base = digest(data)
        open(os.path.join(out, "base", name), "wb").write(data)
    elif os.path.exists(os.path.join(ROOT, rel)):
        sys.exit("new file already live: " + rel)
    open(os.path.join(out, "files", name), "wb").write(new)
    manifest[rel] = {"file": name, "base": base, "new": digest(new)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copy2(os.path.join(HERE, "stage.py"), os.path.join(out, "stage.py"))
shutil.copy2(os.path.join(HERE, batch, "verify.sh"), os.path.join(out, "verify.sh"))
shutil.rmtree(work)
print("staged", out, len(manifest), "files")
