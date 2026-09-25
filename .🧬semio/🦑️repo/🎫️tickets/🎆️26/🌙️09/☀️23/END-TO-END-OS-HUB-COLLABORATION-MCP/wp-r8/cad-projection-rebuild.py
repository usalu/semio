"""Rebuilds the CAD projection of 📐️cad-draw-path-projection on the current CAD example tree (ticket input, R8 round 3).

Every current file is bound exactly (`liveBindings`); model and category identities are the current directory names; member
identities are the semantic stems, recovered from git rename history where the 09-05 path-budget pass shortened a carrier
(`a01.json`, `…-8a1d88.json`, `t-cc7756-ffc804.json`). Destinations follow the catalog's category rules.
"""
import collections, hashlib, json, os, re, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
GOLDEN = f"{REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json"
TAXONOMY = f"{REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
SHORTENED = re.compile(r"(-[0-9a-f]{6}(-[0-9a-f]{6})?\.json|^a\d\d\.json|^t\.json)$")
EMOJI = re.compile(r"^((?:[\U0001F000-\U0001FAFF -⯿〰〽]️?(?:‍[\U0001F000-\U0001FAFF☀-➿]️?)*))")

golden = json.load(open(GOLDEN, encoding="utf-8"))
projection = next(p for p in golden["projections"] if p["contractId"] == "artifact-example-model-catalog-v1")
source_root, destination_root = projection["sourceRoot"], projection["destinationRoot"]
taxonomy = json.load(open(TAXONOMY, encoding="utf-8"))
rules = {r["sourceDirectoryName"]: r for r in taxonomy["semanticPathProjectionCatalogContracts"]["cad-model-catalog-v1"]["categoryRules"]}

log = subprocess.run(["git", "-c", "diff.renameLimit=30000", "log", "-M", "--diff-filter=R", "--name-status", "--format=", "--", source_root], cwd=REPO, capture_output=True, text=True, check=True).stdout
previous = {}
for line in log.splitlines():
    parts = line.split("\t")
    if len(parts) == 3 and parts[0].startswith("R"):
        previous.setdefault(parts[2], parts[1])

def semantic_basename(path):
    while SHORTENED.search(os.path.basename(path)) and path in previous:
        path = previous[path]
    name = os.path.basename(path)
    if SHORTENED.search(name):
        sys.exit(f"no semantic ancestor for {path}")
    return name

def stem(name):
    return EMOJI.sub("", name)[: -len(".json")]

live = sorted(os.path.relpath(os.path.join(d, f), f"{REPO}/{source_root}") for d, _, fs in os.walk(f"{REPO}/{source_root}") for f in fs if f.endswith(".json"))
rows = []
for relative in live:
    segments = relative.split("/")
    model = segments[0]
    if len(segments) == 2:
        rows.append((f"{source_root}/{relative}", f"{destination_root}/{model}/🔣️.json", relative))
        continue
    rule = rules[segments[1]]
    if rule["sourceShape"] == "direct-semantic-json" and len(segments) == 3:
        member = stem(semantic_basename(f"{source_root}/{relative}"))
        rows.append((f"{source_root}/{model}/{segments[1]}/🔣️{member}.json", f"{destination_root}/{model}/{segments[1]}/{rule['memberDirectoryEmoji']}{member}/🔣️.json", relative))
    elif rule["sourceShape"] == "nested-fixed-json" and len(segments) == 4:
        rows.append((f"{source_root}/{model}/{segments[1]}/{segments[2]}/{rule['fixedSourceFilename']}", f"{destination_root}/{model}/{segments[1]}/{segments[2]}/🔣️.json", relative))
    else:
        sys.exit(f"unshaped CAD member {relative}")
rows.sort(key=lambda row: row[0].encode())
if len({r[0] for r in rows}) != len(rows) or len({r[1] for r in rows}) != len(rows):
    sys.exit("canonical source or destination collision")

mappings = [{"sourcePath": s, "destinationPath": d} for s, d, _ in rows]
directories = set()
for m in mappings:
    path = os.path.dirname(m["destinationPath"])
    while True:
        directories.add(path)
        if path == destination_root:
            break
        path = os.path.dirname(path)
counts = collections.Counter(r[2].split("/")[1] for r in rows if len(r[2].split("/")) > 2)
models = []
for model in sorted({r[2].split("/")[0] for r in rows}, key=lambda n: n.encode()):
    manifest = json.load(open(f"{REPO}/{source_root}/{model}/🔣️modelDefinition.json", encoding="utf-8"))
    models.append({"directoryName": model, "id": manifest["id"], "schema": manifest["schema"], "version": manifest["version"]})
category_rules = []
for name, rule in rules.items():
    entry = {"sourceDirectoryName": name, "sourceShape": rule["sourceShape"], "manifestSchema": rule["manifestSchema"]}
    entry.update({"memberEmoji": rule["memberDirectoryEmoji"]} if rule["sourceShape"] == "direct-semantic-json" else {"fixedSourceFilename": rule["fixedSourceFilename"]})
    entry["count"] = counts[name]
    category_rules.append(entry)

projection.update({
    "sourceFileCount": len(mappings),
    "destinationDirectoryCount": len(directories),
    "destinationNodeCount": len(directories) + len(mappings),
    "maxPathBytes": max(len(m["destinationPath"].encode()) for m in mappings),
    "mappingDigest": hashlib.sha256("\n".join(f"{m['sourcePath']}\0{m['destinationPath']}" for m in mappings).encode()).hexdigest(),
})
projection["modelCatalog"]["models"] = models
projection["modelCatalog"]["categoryRules"] = category_rules
projection["mappings"] = mappings
projection["liveBindings"] = [{"source": s[len(source_root) + 1:], "live": live_relative} for s, _, live_relative in rows]
if "--write" in sys.argv:
    open(GOLDEN, "w", encoding="utf-8").write(json.dumps(golden, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({k: projection[k] for k in ("sourceFileCount", "destinationDirectoryCount", "destinationNodeCount", "maxPathBytes", "mappingDigest")}, ensure_ascii=False))
print("category counts", dict(counts), "total", sum(counts.values()))
