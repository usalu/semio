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
import hashlib, io, json, os, sys

ROOT = "/Users/ueli/Documents/semio"
CAT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json"
TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"

os.chdir(ROOT)
catalog = json.load(io.open(CAT, encoding="utf-8"))
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

if changed:
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
