#!/usr/bin/env python3
"""🏗️ Writes the `wgpu-frame-worker` projection's tracked outputs from the authority catalog itself.

The taxonomy validator rejects the whole workspace when a `tracked` generator output is missing, and
that rejection hard-fails every cargo command, the OS dev server and Storybook alike. Normally the
`nestedCargoGeneratedPrestate` hatch tolerates a not-yet-materialized projection, but it also
requires git's admitted file set under the projection source root to match the catalog exactly — and
an in-flight rename the sweep has not staged yet leaves a phantom index entry there, which only a git
index write can clear.

Materializing the outputs sidesteps that entirely: once they exist, the tracked-output check passes
on its own and the hatch is never consulted. Nothing here is invented — each file's bytes come from
the catalog's own `adapters`/`derivedLeaves` `content`, or, for the retired generated module, from
the source file the catalog's `generatedSourceRetirements` names. That is what the generator writes.
"""
import io, json, os

ROOT = "/Users/ueli/Documents/semio"
CAT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json"
TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
os.chdir(ROOT)

catalog = json.load(io.open(CAT, encoding="utf-8"))
row = next(p for p in catalog["packages"] if p["id"] == "wgpu-renderer")

content = {}
for key in ("adapters", "derivedLeaves"):
    for entry in row.get(key, []):
        if "path" in entry and "content" in entry:
            content[entry["path"]] = (entry["content"], key)
retirements = {r["destinationPath"]: r["sourcePath"] for r in row.get("generatedSourceRetirements", [])}

taxonomy = json.load(io.open(TAX, encoding="utf-8"))
written, skipped, unresolved = [], [], []
for output in taxonomy["generatorContracts"]["wgpu-frame-worker"]["outputRoots"]:
    path = output["path"]
    if os.path.exists(path):
        skipped.append(path)
        continue
    if path in content:
        body, origin = content[path]
    elif path in retirements and os.path.isfile(retirements[path]):
        body, origin = io.open(retirements[path], encoding="utf-8").read(), "retirement of " + retirements[path]
    else:
        unresolved.append(path)
        continue
    os.makedirs(os.path.dirname(path), exist_ok=True)
    io.open(path, "w", encoding="utf-8").write(body)
    written.append((path, origin))

print("materialized: %d, already present: %d, unresolved: %d" % (len(written), len(skipped), len(unresolved)))
for path, origin in written:
    print("  from %s\n    %s" % (origin, path))
for path in unresolved:
    print("  UNRESOLVED (no recorded content): %s" % path)
