"""🧭️ Surveys the editor-layer mutation vocabularies the contract gate reports as `unregistered-mutation-vocabulary`:
kinds (leaf descriptors), aggregate and snapshot types, whether a language-neutral vector file exists, and the owner's
crate. Writes JSON to stdout.

Usage: editor-survey.py <breaches.json>"""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
rows = json.load(open(sys.argv[1]))
out = []
for row in rows:
    if row["id"] != "unregistered-mutation-vocabulary": continue
    vocab = row["scope"]
    owner = os.path.dirname(os.path.dirname(vocab))
    kinds = []
    for entry in sorted(os.listdir(root + vocab)):
        leaf = os.path.join(root + vocab, entry, "🔣️.json")
        if os.path.isfile(leaf):
            try: kinds.append(json.load(open(leaf)).get("semanticKind"))
            except ValueError: pass
    source = root + vocab + "/🦀️.rs"
    text = open(source).read() if os.path.exists(source) else ""
    agg = re.search(r"#\[mutations\(snapshot = ([\w:]+)[^\]]*\]\s*(?:#\[[^\n]*\n\s*|//[^\n]*\n\s*)*pub enum (\w+)", text)
    vectors = [f for f in ("🧫️fixtures/🔁️mutations.json", "🧫️fixtures/🔁️mutation-contracts.json") if os.path.exists(root + owner + "/" + f)]
    crate = None
    d = root + owner
    while d.startswith(root) and crate is None:
        m = os.path.join(d, "📦️packages/🦀️rust/Cargo.toml")
        if os.path.exists(m): crate = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(m).read()).group(1)
        d = os.path.dirname(d)
    out.append({"owner": owner, "kinds": kinds, "aggregate": agg.group(2) if agg else None, "snapshot": agg.group(1) if agg else None, "vectors": vectors, "crate": crate})
json.dump(out, sys.stdout, indent=1, ensure_ascii=False)
