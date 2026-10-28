"""[gram3-g] writes the texts of every file named by the edits of the given batches (applied in order to the live file) under <out>/<path relative to 🏗️build/>; used to map line numbers of the counting runs, which are taken from the patched text. usage: materialize.py <out> <batch dir>..."""
import json, os, sys
ROOT = "/Users/ueli/Documents/semio/"
out, batches = sys.argv[1], sys.argv[2:]
texts = {}
for batch in batches:
    spec = json.load(open(os.path.join(batch, "edits.json"), encoding="utf8"))
    for e in spec["edits"]:
        file = e.get("file") or spec["file"]
        if not file.endswith((".mjs", ".ts", ".mts")):
            continue
        t = texts.get(file) or open(ROOT + file, encoding="utf8").read()
        if t.count(e["old"]) != 1:
            sys.exit("anchor " + e["name"])
        texts[file] = t.replace(e["old"], e["new"])
for file, t in texts.items():
    rel = file.split("🏗️build/", 1)[-1]
    os.makedirs(os.path.dirname(os.path.join(out, rel)), exist_ok=True)
    open(os.path.join(out, rel), "w", encoding="utf8").write(t)
print(len(texts), "files")
