import os, re, json, collections, sys
ROOT = "/Users/ueli/Documents/semio"
F = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split")
G = os.path.join(F, "🗑️generated")
files = json.load(open(os.path.join(G, "frd-files.json"), encoding="utf-8"))
cat = collections.Counter(); ex = collections.defaultdict(list)
def classify(l):
    s = l.strip()
    if "install_default_schema_leaf_reader" in s: return "leaf-reader-install"
    if re.search(r'let path = std::path::Path::new\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\((relative|path_from_manifest)\);', s): return "helper-std"
    if "register_schema_leaf_documents" in s: return "schema-leaf-doc"
    if re.search(r'(include_str|include_bytes)!\(concat!\(env!\("CARGO_MANIFEST_DIR"\)', s): return "include-concat"
    if re.search(r'fs::read(_to_string)?\(concat!\(env!\("CARGO_MANIFEST_DIR"\), *"[^"]*"\)', s): return "inline-read-concat"
    if re.search(r'fs::read(_to_string)?\(\s*(std::path::|::std::path::)?Path(Buf)?::(new|from)\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\("[^"]*"\)\)', s): return "inline-read-join"
    if "nx.json" in s: return "repo-root-nx"
    if "concat!(env!(\"CARGO_MANIFEST_DIR\")" in s: return "concat-other"
    if re.search(r'Path(Buf)?::(new|from)\(env!\("CARGO_MANIFEST_DIR"\)\)', s): return "path-new-other"
    return "other"
for rel in files:
    t = open(os.path.join(ROOT, rel), encoding="utf-8").read()
    for i, l in enumerate(t.split("\n"), 1):
        if "CARGO_MANIFEST_DIR" in l:
            c = classify(l); cat[c] += 1
            if len(ex[c]) < 400: ex[c].append((rel, i, l.strip()[:300]))
for c, n in cat.most_common(): print(c, n)
json.dump(ex, open(os.path.join(G, "frd-line-cats.json"), "w"), ensure_ascii=False, indent=1)
