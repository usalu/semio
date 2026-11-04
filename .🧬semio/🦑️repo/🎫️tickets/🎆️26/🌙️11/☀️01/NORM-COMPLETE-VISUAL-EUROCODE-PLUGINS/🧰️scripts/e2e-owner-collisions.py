#!/usr/bin/env python3
"""🔎 Report duplicate build-owner names, roots, and package names."""
import json
from collections import defaultdict
from pathlib import Path

registry = json.loads(Path("/home/deck/semio/📋️build-owners.json").read_text())
owners = registry["owners"]
by_name = defaultdict(list)
by_root = defaultdict(list)
by_package = defaultdict(list)
bad_manifest = []
for owner in owners:
    by_name[owner["name"]].append(owner["root"])
    roots = [owner["root"], *(owner.get("nativeRoots") or [])]
    for root in roots:
        by_root[root].append(owner["name"])
    for package in owner.get("packages") or []:
        by_package[package].append(owner["name"])
    manifest = owner["manifest"]
    root = owner["root"]
    expected = manifest if root == "." else root + "/" + ("📋️project.json" if manifest.endswith("project.json") else "Cargo.toml")
    if manifest not in {expected} and not (
        manifest == (root if root != "." else "") + ("/Cargo.toml" if root != "." else "Cargo.toml")
        or manifest.endswith("Cargo.toml")
        or manifest.endswith("project.json")
    ):
        bad_manifest.append((owner["name"], root, manifest))

def dups(mapping):
    return {key: values for key, values in mapping.items() if len(values) > 1}

out = {
    "owners": len(owners),
    "duplicate_names": dups(by_name),
    "duplicate_roots": {k: v for k, v in dups(by_root).items() if "norm" in k or "ndp" in k or True},
    "duplicate_packages": dups(by_package),
}
# Keep the report bounded: full root collisions can be large.
roots = dups(by_root)
norm_roots = {k: v for k, v in roots.items() if "norm" in k or "ndp" in k}
print(json.dumps({
    "owners": len(owners),
    "duplicate_name_count": len(dups(by_name)),
    "duplicate_root_count": len(roots),
    "duplicate_package_count": len(dups(by_package)),
    "duplicate_names": dups(by_name),
    "norm_or_ndp_duplicate_roots": norm_roots,
    "duplicate_packages_sample": dict(list(dups(by_package).items())[:40]),
}, indent=2, ensure_ascii=False))
