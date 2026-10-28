import json, os
ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
G = B + "/🕸️graph/🕰️generations"
P = "🧰️framework/🔨️modules/🏃️process"
spec = {"edits": [], "newFiles": []}

def read(rel): return open(os.path.join(ROOT, rel), encoding="utf-8").read()

def edit(file, old, new, base=None):
    """Anchored edit: `old` occurs exactly once in the live file (or in `base`, a text standing in for the post-state of an earlier staged batch)."""
    text = base if base is not None else read(file)
    assert text.count(old) == 1, (file.split("/")[-1], text.count(old), old[:80])
    assert new != old
    spec["edits"].append({"file": file, "old": old, "new": new})

def newfile(path, source):
    spec["newFiles"].append({"path": path, "from": "files/" + source})
    return None

def write(directory):
    out = os.path.join(HERE, "staged", directory)
    os.makedirs(out, exist_ok=True)
    os.makedirs(os.path.join(out, "files"), exist_ok=True)
    for item in spec["newFiles"]:
        open(os.path.join(out, item["from"]), "wb").write(open(os.path.join(HERE, item["from"]), "rb").read())
    json.dump(spec, open(os.path.join(out, "edits.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
