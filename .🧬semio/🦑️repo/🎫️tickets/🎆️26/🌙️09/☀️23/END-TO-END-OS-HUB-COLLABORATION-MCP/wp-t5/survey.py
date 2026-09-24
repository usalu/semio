"""Classifies every mutation vocabulary's OpBinary codec by the source of its op tags."""
import json, os, re, subprocess, collections
root = "/Users/ueli/Documents/semio/"
protos = subprocess.run(["find", "✏️s", "🧰️framework", "-path", "*🧬️mutations/💾️binary/📡️.protocol.semio", "-not", "-path", "*/target*"], cwd=root, capture_output=True, text=True).stdout.split("\n")
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
out = []
for p in filter(None, protos):
    vocab = os.path.dirname(os.path.dirname(root + p))
    kinds = sorted(KEBAB.search(d).group(0) for d in os.listdir(vocab) if os.path.isdir(f"{vocab}/{d}") and KEBAB.search(d))
    rsfiles = []
    for dp, dn, fn in os.walk(vocab):
        if "🧪️tests" in dp: continue
        rsfiles += [f"{dp}/{f}" for f in fn if f.endswith(".rs")]
    impl = [f for f in rsfiles if re.search(r"impl\s+(?:protocol::|store::)?OpBinary\s+for", open(f, encoding="utf-8").read())]
    txt = "".join(open(f, encoding="utf-8").read() for f in impl)
    shape = []
    if "variants_binary::" in txt: shape.append("variants")
    if "REGISTRY" in txt: shape.append("registry")
    if "OP_KEYWORDS" in txt: shape.append("op-keywords")
    if re.search(r"write_u8\(\s*\d+\s*\)|=>\s*\d+\s*,|\b\d+\s*=>", txt): shape.append("literal")
    if not impl: shape.append("no-impl")
    records = len(re.findall(r"^record\s+[a-z]", open(root + p, encoding="utf-8").read(), re.M))
    out.append({"proto": p, "kinds": kinds, "impl": [os.path.relpath(f, root) for f in impl], "shape": shape, "records": records})
json.dump(out, open(root + ".tmp-ticket/wp-t5/generated/survey.json", "w"), ensure_ascii=False, indent=1)
print(collections.Counter(tuple(o["shape"]) for o in out))
for o in out:
    if o["shape"] in (["no-impl"],) or len(o["impl"]) != 1: print(o["shape"], len(o["impl"]), o["proto"].split("🗿️artifacts/")[-1][:100])
