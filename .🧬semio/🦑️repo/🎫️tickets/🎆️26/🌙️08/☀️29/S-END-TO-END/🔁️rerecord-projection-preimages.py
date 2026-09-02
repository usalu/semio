#!/usr/bin/env python3
"""🔁️ Re-records the nested-cargo projection preimage chain after a source edit.

The chain is tamper-evident in two layers: `🔣️taxonomy.json`'s
`semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogSha256` pins the
catalog fixture, and the catalog pins a sha256+size for every mapped source file. Editing a mapped
source without re-recording both makes `semanticPackageGenerationAuthority` throw
"Nested Cargo source preimage drift" — which `nestedCargoGeneratedPrestate` swallows in a bare
`catch`, so the failure surfaces only as an unrelated-looking "tracked output ... is missing", and
`semio-framework-graph`'s build script then hard-fails EVERY cargo command repo-wide.

Idempotent: prints nothing but a zero count when the chain is already consistent.
"""
import hashlib, io, json, os, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
CAT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json"
TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

os.chdir(ROOT)
catalog = json.load(io.open(CAT, encoding="utf-8"))

# 🧭️ Zeroth layer: the sweep also RENAMES mapped sources (e.g. `📜️world.wit` -> `🧪️world/📜️.wit`).
#    Repoint any catalog path that is missing but that git records as renamed to something that does
#    exist. Driven by git's own records so it cannot invent a destination; un-materialized projection
#    destinations have no rename record and are left alone.
renames = {}
for _line in subprocess.run(["git", "status", "--porcelain"], capture_output=True).stdout.decode("utf-8").split("\n"):
    if "R" in _line[:2] and " -> " in _line:
        _old, _new = _line[3:].split(" -> ", 1)
        renames[_old] = _new

def moved_by_shape(path):
    """🗂️ Git's rename detection flips to plain add+delete depending on staging, so fall back to the
    sweep's own naming shape: a leaf `<emoji><stem>.<ext>` becomes `<emoji><stem>/<emoji>.<ext>`."""
    import re as _re
    parent, leaf = os.path.dirname(path), os.path.basename(path)
    match = _re.match(r"^([^A-Za-z0-9]*)([A-Za-z0-9._-]+)\.([A-Za-z0-9]+)$", leaf)
    if not match or not os.path.isdir(parent):
        return None
    stem, ext = match.group(2), match.group(3)
    for entry in os.listdir(parent):
        directory = os.path.join(parent, entry)
        if not os.path.isdir(directory):
            continue
        if _re.sub(r"^[^A-Za-z0-9]+", "", entry) != stem:
            continue
        for candidate in os.listdir(directory):
            if candidate.endswith("." + ext):
                return os.path.join(directory, candidate)
    return None

def repoint(node):
    count = 0
    if isinstance(node, dict):
        for key, value in node.items():
            if key in ("sourceRelativePath",):  # 🛡️ hardcoded contract shape, matched exactly by the validator — never repoint
                continue
            if isinstance(value, str) and key.endswith(("Path", "path")) and value and not os.path.exists(value):
                moved = renames.get(value) or moved_by_shape(value)
                if moved and os.path.exists(moved):
                    node[key] = moved
                    print("  repointed %s\n     %s\n  -> %s" % (key, value, moved))
                    count += 1
            else:
                count += repoint(value)
    elif isinstance(node, list):
        for item in node:
            count += repoint(item)
    return count

repointed = repoint(catalog)
print("catalog paths repointed: %d" % repointed)
changed = []
for package in catalog["packages"]:
    for mapping in package["mappings"]:
        source = mapping["sourcePath"]
        if not os.path.exists(source):
            continue
        raw = open(source, "rb").read()
        digest, size = hashlib.sha256(raw).hexdigest(), len(raw)
        if mapping.get("sourceHash") != digest or mapping.get("sourceSize") != size:
            changed.append((package["id"], source))
            mapping["sourceHash"], mapping["sourceSize"] = digest, size
# 🔗️ Third layer: a derived registration leaf pins the hash of the source it was derived FROM, and
#    it must agree with that source's mapping entry above.
by_source = {m["sourcePath"]: m["sourceHash"] for p in catalog["packages"] for m in p["mappings"]}
for package in catalog["packages"]:
    for leaf in package.get("derivedLeaves", []):
        origin = leaf.get("originSourcePath")
        if origin in by_source and leaf.get("originSourceHash") != by_source[origin]:
            leaf["originSourceHash"] = by_source[origin]
            changed.append((package["id"], "derivedLeaf:" + leaf.get("id", "?")))

if changed or repointed:
    io.open(CAT, "w", encoding="utf-8").write(json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")

catalog_digest = hashlib.sha256(open(CAT, "rb").read()).hexdigest()
taxonomy = json.load(io.open(TAX, encoding="utf-8"))
contract = taxonomy["semanticPackageProjectionContracts"]["nested-cargo-packages-v1"]
digest_changed = contract.get("authorityCatalogSha256") != catalog_digest
if digest_changed:
    contract["authorityCatalogSha256"] = catalog_digest
    io.open(TAX, "w", encoding="utf-8").write(json.dumps(taxonomy, ensure_ascii=False, indent=2) + "\n")

print("preimages re-recorded: %d, catalog digest updated: %s" % (len(changed), digest_changed))
for package_id, source in changed:
    print("  ", package_id, source)
