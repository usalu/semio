"""Writes the decided `sweep` into every extruded hint without one of the given catalog assets, by exact text insertion (never re-serialising the file), re-reading the file immediately before the write.
Usage: catalog-sweep-apply.py <category-folder/slug>... ; prints per asset the number of insertions. Value is "w" (decided per asset by catalog-sweep-probe.ts and catalog-sweep-asset.ts)."""
import hashlib, re, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
for spec in sys.argv[1:]:
    path = S + spec + "/🔣️.json"
    raw = open(path, encoding="utf-8").read()
    new, n = re.subn(r'(?m)^([ \t]*)"kind": "extruded",[ \t]*\n(?![ \t]*"sweep")', r'\1"kind": "extruded",\n\1"sweep": "w",\n', raw)
    if n == 0:
        print(spec, 0); continue
    again = open(path, encoding="utf-8").read()
    if hashlib.sha256(again.encode()).digest() != hashlib.sha256(raw.encode()).digest():
        raw = again
        new, n = re.subn(r'(?m)^([ \t]*)"kind": "extruded",[ \t]*\n(?![ \t]*"sweep")', r'\1"kind": "extruded",\n\1"sweep": "w",\n', raw)
    open(path, "w", encoding="utf-8").write(new)
    print(spec, n)
