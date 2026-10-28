"""🧱 Builds a flat staged batch (manifest.json + files/ + base/ + stage.py + verify.sh) from a new-root: python3 -I lib.py <batch-dir> <live-root> <new-root> <title> <rel>...
base = the live file NOW (absent = new file); new = the file under <new-root>. Re-run after any drift to rebase."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
def build(batch, live, new_root, title, rels, verify_source):
    for sub in ("files", "base"):
        shutil.rmtree(os.path.join(batch, sub), ignore_errors=True); os.makedirs(os.path.join(batch, sub))
    rows = {}
    for index, rel in enumerate(rels):
        name = "%02d%s" % (index, os.path.splitext(rel)[1])
        shutil.copyfile(os.path.join(new_root, rel), os.path.join(batch, "files", name))
        if os.path.exists(os.path.join(live, rel)): shutil.copyfile(os.path.join(live, rel), os.path.join(batch, "base", name))
        rows[rel] = {"file": name, "base": sha(os.path.join(batch, "base", name)), "new": sha(os.path.join(batch, "files", name))}
    json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    text = open(os.path.join(HERE, "stage.template.py"), encoding="utf8").read().replace("🌌 Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern.", title)
    open(os.path.join(batch, "stage.py"), "w", encoding="utf8").write(text)
    shutil.copyfile(verify_source, os.path.join(batch, "verify.sh"))
    changed = [rel for rel, row in rows.items() if row["base"] != row["new"]]
    print("batch %s: %d files (%d new, %d changed vs live)" % (batch[-40:], len(rows), sum(1 for r in rows.values() if r["base"] is None), len(changed)))
if __name__ == "__main__":
    batch, live, new_root, title = sys.argv[1:5]
    build(batch, live, new_root, title, sys.argv[5:], os.path.join(HERE, "verify-" + os.path.basename(batch) + ".sh"))
