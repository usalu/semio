"""🧮 Recomputes the `new` hashes of a staged batch's manifest from its files/ (rows whose new is null stay null): python3 -I rehash.py <batchDir>"""
import hashlib, json, pathlib, sys
b = pathlib.Path(sys.argv[1]); m = json.loads((b / "manifest.json").read_text())
for rel, e in m.items():
    if e["new"] is not None:
        h = hashlib.sha256((b / "files" / e["file"]).read_bytes()).hexdigest()
        if h != e["new"]: print("rehashed", e["file"], rel[-70:]); e["new"] = h
(b / "manifest.json").write_text(json.dumps(m, ensure_ascii=False, indent=1))
