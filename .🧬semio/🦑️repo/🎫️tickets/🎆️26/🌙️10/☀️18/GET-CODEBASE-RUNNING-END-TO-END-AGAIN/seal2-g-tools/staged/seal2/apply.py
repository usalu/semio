#!/usr/bin/env python3
"""🧩️ Applies, checks or reverts a staged batch of anchored edits and new files (stdlib only; macOS, Linux, Windows).
usage: python3 -I apply.py check|apply|revert [--root DIR]     (DIR defaults to the repository that holds this batch)
An edit is `old` -> `new` inside one file; `old` must occur exactly once in the file before apply and `new` exactly once before revert. The batch refuses (exit 2, nothing written) when any edit is neither applied nor applicable, or when a new file exists with other content. Idempotent: a second apply says `already applied`. Writes are atomic per file (temporary file + rename).
Exit: 0 consistent state, 2 conflict, 3 usage."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
if not args or args[0] not in ("check", "apply", "revert"):
    print(__doc__); sys.exit(3)
command = args.pop(0)
root = os.path.abspath(args[args.index("--root") + 1]) if "--root" in args else os.path.abspath(os.path.join(HERE, *([".."] * int(open(os.path.join(HERE, "depth.txt")).read()))))
manifest = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf-8"))
read = lambda p: open(p, encoding="utf-8").read()
def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".%d.tmp" % os.getpid()
    with open(temporary, "w", encoding="utf-8", newline="") as f: f.write(text)
    if os.path.exists(path): os.chmod(temporary, os.stat(path).st_mode & 0o7777)
    os.replace(temporary, path)
state = {}; texts = {}
for edit in manifest["edits"]:
    path = os.path.join(root, edit["file"])
    if edit["file"] not in texts:
        texts[edit["file"]] = read(path) if os.path.exists(path) else None
    text = texts[edit["file"]]
    old, new = read(os.path.join(HERE, edit["old"])), read(os.path.join(HERE, edit["new"]))
    if text is None: state[edit["name"]] = "missing-file"
    else:
        a, b = text.count(old), text.count(new)
        state[edit["name"]] = "base" if a == 1 and b == 0 else "applied" if b == 1 and a == 0 else "applied" if new.count(old) and b == 1 else "conflict(old x%d, new x%d)" % (a, b)
for item in manifest["files"]:
    path = os.path.join(root, item["path"]); want = read(os.path.join(HERE, item["source"]))
    state[item["path"]] = "base" if not os.path.exists(path) else "applied" if read(path) == want else "conflict(other content)"
bad = {k: v for k, v in state.items() if v.startswith("conflict") or v == "missing-file"}
kinds = set(state.values()) - {"base", "applied"}
for key, value in state.items(): print("%-9s %s" % (value, key))
if bad:
    print("CONFLICT: nothing written"); sys.exit(2)
if command == "check":
    print("state:", "applied" if all(v == "applied" for v in state.values()) else "base" if all(v == "base" for v in state.values()) else "mixed"); sys.exit(0)
want = "applied" if command == "apply" else "base"
if all(v == want for v in state.values()):
    print("already applied" if command == "apply" else "already base"); sys.exit(0)
if any(v == want for v in state.values()) and command == "apply" and not os.environ.get("SEMIO_STAGE_MIXED"):
    print("MIXED state (part of the batch is applied): refusing; revert first"); sys.exit(2)
for edit in manifest["edits"]:
    old, new = read(os.path.join(HERE, edit["old"])), read(os.path.join(HERE, edit["new"]))
    a, b = (old, new) if command == "apply" else (new, old)
    texts[edit["file"]] = texts[edit["file"]].replace(a, b, 1) if texts[edit["file"]].count(a) == 1 and state[edit["name"]] != want else texts[edit["file"]]
for file, text in texts.items(): write(os.path.join(root, file), text)
for item in manifest["files"]:
    path = os.path.join(root, item["path"])
    if command == "apply": write(path, read(os.path.join(HERE, item["source"])))
    elif os.path.exists(path): os.remove(path)
print({"apply": "applied", "revert": "reverted"}[command] + ":", len(manifest["edits"]), "edits,", len(manifest["files"]), "files")
