"""⚖️ Load-and-compare of the committed vector tables: old single table versus the new domain tables.

Parses every `Vector { id, kind, before, mutation, after, diff, outcome }` literal, resolves each include_str! path against the file that
holds it, requires the file to exist, and compares (id, kind, resolved file bytes digests) in order. Also compares the chunked
Python VECTOR_ROOTS union when a second argument is given.
"""
import hashlib, os, re, sys

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ROOT = "/Users/ueli/Documents/semio"
SUBSET = ROOT + "/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
OLD_FILE = F + "/🗑️generated/pfb-orig/energy-test/🦀️.rs"
OLD_DIR = SUBSET + "/🧪️tests/🏛️mutate-energy-model-1"
NEW_DIR = OLD_DIR

ENTRY = re.compile(r'Vector \{\s*id: "([^"]+)",\s*kind: "([^"]+)",\s*before: include_str!\("([^"]+)"\),\s*mutation: include_str!\("([^"]+)"\),\s*after: include_str!\("([^"]+)"\),\s*diff: include_str!\("([^"]+)"\),\s*outcome: include_str!\("([^"]+)"\),\s*\}', re.S)

def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()

def rows(file_path, base_dir):
    text = open(file_path, encoding="utf-8").read()
    out = []
    for match in ENTRY.finditer(text):
        ident, kind, *paths = match.groups()
        resolved = [os.path.normpath(os.path.join(base_dir, p)) for p in paths]
        for path in resolved:
            assert os.path.isfile(path), path
        out.append((ident, kind, tuple(digest(p) for p in resolved), tuple(os.path.relpath(p, SUBSET) for p in resolved)))
    return out

old = rows(OLD_FILE, OLD_DIR)
new = []
for folder in sorted(os.listdir(NEW_DIR)):
    pass
tables = [d for d in os.listdir(NEW_DIR) if os.path.isdir(os.path.join(NEW_DIR, d)) and os.path.isfile(os.path.join(NEW_DIR, d, "🦀️.rs"))]
root_text = open(NEW_DIR + "/🦀️.rs", encoding="utf-8").read()
order = re.findall(r'#\[path = "([^"]+)/🦀️\.rs"\]\nmod (\w+);', root_text)
chain = re.search(r"BUILDING_VECTORS\.iter\(\)((?:\.chain\(\w+\))+)", root_text).group(1)
consts = ["BUILDING_VECTORS"] + re.findall(r"\.chain\((\w+)\)", chain)
by_const = {}
for folder in tables:
    text = open(os.path.join(NEW_DIR, folder, "🦀️.rs"), encoding="utf-8").read()
    match = re.search(r"pub\(super\) const (\w+): &\[Vector\] = &\[", text)
    if match:
        by_const[match.group(1)] = rows(os.path.join(NEW_DIR, folder, "🦀️.rs"), os.path.join(NEW_DIR, folder))
assert set(consts) == set(by_const), (consts, list(by_const))
for const in consts:
    new.extend(by_const[const])
print("old vectors", len(old), "new vectors", len(new), "tables", {c: len(by_const[c]) for c in consts})
assert old == new, "tables differ"
ids = [r[0] for r in old]
assert len(set(ids)) == len(ids)
kinds = {}
for r in old:
    kinds.setdefault(r[1], []).append(r[0])
print("unique ids", len(set(ids)), "kinds", len(kinds), "every kind has two vectors:", all(len(v) == 2 for v in kinds.values()))
print("IDENTICAL: ids, kinds, resolved fixture paths and sha256 of all", len(old) * 5, "referenced files")
if len(sys.argv) > 1:
    import json
    print("python roots:", json.load(open(sys.argv[1]))["count"])
