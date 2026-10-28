#!/usr/bin/env python3
"""👓 Staged batch `walk-views` (lane peeredit): a project holds the command walk it reads through the view of its closure recipe, so a walk change that only moves scheduling files keeps product owners verified.
usage: python3 -I stage.py check|apply|revert|make|rebase <root>
make/rebase : cut files/ + base/ + manifest.json from anchors.json on the CURRENT bytes of <root> (every anchor must occur exactly once) plus the new files under new/.
check       : exit 0 when every file is `base` or `applied`, 1 otherwise (prints each state).
apply       : every file must hold its base hash (absent for a new file) or already the new one, else exit 2 (drift: run `rebase <root>` first); atomic replace that keeps the file's mode (new files 0644).
revert      : every file must hold the new hash (or already base), else exit 2; restores base bytes, removes new files and the directories they emptied."""
import hashlib, json, os, shutil, stat, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
INFERENCE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/"
def sha(data): return None if data is None else hashlib.sha256(data).hexdigest()
def read(path):
    try:
        with open(path, "rb") as f: return f.read()
    except FileNotFoundError: return None
def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    mode = stat.S_IMODE(os.stat(target).st_mode) if os.path.exists(target) else 0o644
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.chmod(tmp, mode); os.replace(tmp, target)
def make(root):
    anchors = json.load(open(os.path.join(HERE, "anchors.json"), encoding="utf8"))
    for sub in ("files", "base"): shutil.rmtree(os.path.join(HERE, sub), ignore_errors=True); os.makedirs(os.path.join(HERE, sub))
    manifest, n = {}, 0
    for rel, edits in anchors.items():
        text = read(os.path.join(root, rel)).decode("utf8")
        for edit in edits:
            count = text.count(edit["old"])
            if count != 1 and text.count(edit["new"]) == 1: continue
            if count != 1: sys.exit(f"anchor {edit['name']} occurs {count} times in {rel[-60:]}")
            text = text.replace(edit["old"], edit["new"])
        n += 1; name = f"f{n:02d}"
        open(os.path.join(HERE, "base", name), "wb").write(read(os.path.join(root, rel)))
        open(os.path.join(HERE, "files", name), "wb").write(text.encode("utf8"))
        manifest[rel] = {"file": name, "base": sha(read(os.path.join(root, rel))), "new": sha(text.encode("utf8"))}
    for top, _dirs, names in os.walk(os.path.join(HERE, "new")):
        for leaf in sorted(names):
            source = os.path.join(top, leaf); rel = INFERENCE + os.path.relpath(source, os.path.join(HERE, "new")).replace(os.sep, "/")
            n += 1; name = f"f{n:02d}"; data = read(source)
            open(os.path.join(HERE, "files", name), "wb").write(data)
            manifest[rel] = {"file": name, "base": sha(read(os.path.join(root, rel))), "new": sha(data)}
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(f"made {len(manifest)} files from {root}")
def main():
    verb = sys.argv[1] if len(sys.argv) > 1 else "check"
    if len(sys.argv) < 3: sys.exit("usage: stage.py check|apply|revert|make|rebase <root>")
    root = os.path.abspath(sys.argv[2])
    if verb in ("make", "rebase"): return make(root)
    rows = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
    state = [(rel, row, "applied" if sha(read(os.path.join(root, rel))) == row["new"] else "base" if sha(read(os.path.join(root, rel))) == row["base"] else "drifted") for rel, row in rows.items()]
    drifted = [rel for rel, _row, kind in state if kind == "drifted"]
    if verb == "check":
        for rel, _row, kind in state: print(kind, rel[-90:])
        sys.exit(1 if drifted else 0)
    if drifted and verb in ("apply", "revert"):
        for rel in drifted: print("DRIFTED", rel[-90:])
        sys.exit(2)
    if verb == "apply":
        for rel, row, kind in state:
            if kind != "applied": atomic(os.path.join(root, rel), read(os.path.join(HERE, "files", row["file"]))); print("applied", rel[-90:])
        return
    if verb == "revert":
        for rel, row, kind in state:
            if kind != "applied": continue
            target = os.path.join(root, rel)
            if row["base"] is None:
                os.remove(target); parent = os.path.dirname(target)
                while parent != root and os.path.isdir(parent) and not os.listdir(parent): os.rmdir(parent); parent = os.path.dirname(parent)
            else: atomic(target, read(os.path.join(HERE, "base", row["file"])))
            print("reverted", rel[-90:])
        return
    sys.exit("verb: check|apply|revert|make|rebase")
main()
