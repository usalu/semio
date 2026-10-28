"""🧱 [td-misc] builds a flat staged batch from an anchored edit script: copies every touched file of the LIVE root as `base/`, applies `edits/<batch>.py` (`EDITS = [(rel, [(old, new), ...]), ...]`, every anchor exactly once; `new` text for a new file is given as `[(None, text)]`) and writes `files/`, `manifest.json` ({rel:{file,base,new}}), the mode-keeping `stage.py` (xplat5 template) and `verify.sh` (from `edits/<batch>.verify.sh`). usage: python3 -I build.py <batch> [--root LIVE]"""
import hashlib, json, os, runpy, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
batch = sys.argv[1]
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(C)))))))
assert os.path.isdir(os.path.join(root, "🧰️framework")), root
spec = runpy.run_path(os.path.join(HERE, "edits", batch + ".py"))
out = os.path.join(HERE, "staged", batch)
if os.path.exists(out): shutil.rmtree(out)
os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
sha = lambda b: hashlib.sha256(b).hexdigest()
manifest = {}
for index, (rel, pairs) in enumerate(spec["EDITS"]):
    path = os.path.join(root, rel); ext = os.path.splitext(rel)[1] or ".txt"; name = "%02d%s" % (index, ext)
    if pairs and pairs[0][0] is None:
        if os.path.exists(path): sys.exit("new file exists on live: " + rel)
        base, text = None, pairs[0][1]
    else:
        base = open(path, "rb").read(); text = base.decode("utf8")
        for old, new in pairs:
            n = text.count(old)
            if n != 1: sys.exit("anchor x%d in %s: %r" % (n, rel, old[:100]))
            text = text.replace(old, new)
        open(os.path.join(out, "base", name), "wb").write(base)
    data = text.encode("utf8")
    open(os.path.join(out, "files", name), "wb").write(data)
    manifest[rel] = {"file": name, "base": None if base is None else sha(base), "new": sha(data)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
doc = spec["DOC"]
start = template.index('"""'); end = template.index('\nusage:', start)
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template[:start] + '"""' + doc + template[end:])
verify = os.path.join(HERE, "edits", batch + ".verify.sh")
if os.path.exists(verify): shutil.copy(verify, os.path.join(out, "verify.sh"))
print("built", batch, len(manifest), "file(s) ->", out)
