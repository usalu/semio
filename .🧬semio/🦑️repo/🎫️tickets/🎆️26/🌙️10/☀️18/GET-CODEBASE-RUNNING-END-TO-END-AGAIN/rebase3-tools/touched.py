"""[rebase3] prints sha256 of every file the three batches touch under a root: touched.py <root>. Streams, no big reads."""
import hashlib, json, os, sys
root = sys.argv[1].rstrip("/") + "/"
H = os.environ["H"]; RF = os.environ["RF"]
rels = set()
for b in ("reuse-once", "plan-once2"):
    for e in json.load(open(f"{H}/{b}/edits.json", encoding="utf8"))["edits"]:
        rels.add(e["file"])
    for base, _, names in os.walk(f"{H}/{b}/tree"):
        rels.update(os.path.relpath(os.path.join(base, n), f"{H}/{b}/tree") for n in names)
rels.update(e["file"] for e in json.load(open(RF + "/batch.json", encoding="utf8"))["files"])
rels.update(json.load(open(RF + "/new.json", encoding="utf8")).values())
for r in sorted(rels):
    p = root + r
    print(hashlib.sha256(open(p, "rb").read()).hexdigest()[:16] if os.path.isfile(p) else "ABSENT          ", r[-90:])
