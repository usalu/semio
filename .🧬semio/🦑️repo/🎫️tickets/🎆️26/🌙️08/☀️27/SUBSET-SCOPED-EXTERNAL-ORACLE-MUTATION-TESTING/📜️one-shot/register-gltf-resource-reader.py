#!/usr/bin/env python3
# 🧵️ One-shot: registers the `gltf` 1.4 document-level RESOURCE reader for gltf@2.0/any, and repoints
# the 24 resource kinds (create/delete/move/reorder × accessor, buffer, bufferView, image, sampler,
# texture) off `-uncarried` onto it. Not idempotent — appends oracle/probes/profile/pipeline once.
import hashlib, json, os

ORACLE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
FIX = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧫️fixtures"
PROBES = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔬️probes/🦀️resource-reader/target/release/reader"
CAP = "gltf-2-0-mutate-resource"
OID = "gltf-rs-2-0-mutate-reader"

SINGULAR = ["accessor", "buffer", "buffer-view", "image", "sampler", "texture"]
PLURAL = ["accessors", "buffers", "buffer-views", "images", "samplers", "textures"]
KINDS = [f"{op}-{s}" for op in ("create", "delete", "move") for s in SINGULAR] + [f"reorder-{p}" for p in PLURAL]

d = json.load(open(ORACLE))

d["oracles"].append({
    "id": OID, "kind": "third-party-library", "ecosystem": "rust", "package": "gltf", "version": "1.4",
    "source": {"repository": "https://github.com/gltf-rs/gltf", "license": "MIT OR Apache-2.0"},
    "engine": {"family": "gltf-rs", "implementation": "gltf 1.4 document resource arrays", "version": "1.4.1"},
    "capabilities": [CAP], "comparisonProfiles": ["semantic-gltf-resource-v1"],
    "license": "MIT OR Apache-2.0", "testOnly": True, "productionReachable": False, "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "homepage": "https://github.com/gltf-rs/gltf",
    "rationale": (
        "📖️ A READER, and a THIRD one, because the other two provably cannot answer this question.\n\n"
        "`three`'s GLTFLoader builds a SCENE GRAPH: a resource that nothing references never becomes an "
        "object, so an unreferenced accessor, buffer, bufferView, image, sampler or texture is carried "
        "through as opaque JSON and interpreted by nothing. That reader's own projection says so and omits "
        "these six deliberately — which is why all 24 kinds over them were recorded `-uncarried`.\n\n"
        "`@gltf-transform/core` was evaluated next and rejected on measurement, not on principle: its Root "
        "exposes `listAccessors`, `listBuffers` and `listTextures` but NO `listBufferViews`, `listImages` or "
        "`listSamplers` — it folds glTF's separate `images`, `samplers` and `textures` arrays into a single "
        "`Texture`. It therefore cannot tell `create-image` from `create-texture`, and cannot see a "
        "bufferView at all. It would have covered 6 of the 24 while appearing to cover more.\n\n"
        "`gltf` 1.4 exposes `accessors()`, `buffers()`, `views()`, `images()`, `samplers()` and `textures()` "
        "as separate typed iterators matching the specification's own structure, so each of the 24 kinds "
        "lands on exactly one observable list. Verified: a document carrying an image with no texture entry "
        "reads back as images=1, textures=0 — the distinction gltf-transform structurally cannot make.\n\n"
        "The reader parses the DOCUMENT only (`Gltf::open`, not `gltf::import`): the resource arrays are "
        "fully determined by it, and decoding pixel data would only add ways for a fixture to fail to read. "
        "Nothing here applies a mutation or predicts one — the expected state is the committed `after` half "
        "of each fixture, and this reader judges both sides."
    ),
})

d["probes"].extend([
    {"id": "gltf-resource-project", "kind": "external-process", "ecosystem": "rust", "package": "gltf", "version": "1.4.1",
     "engine": {"family": "gltf-rs", "implementation": "gltf 1.4 document resource arrays", "version": "1.4.1"},
     "capabilities": ["gltf.resource.project"], "outputSchema": "semio.repository-test.probe-report/v2",
     "deterministic": True, "license": "MIT OR Apache-2.0", "testOnly": True, "productionReachable": False,
     "networkDuringExecution": False, "command": [PROBES, "project"]},
    {"id": "gltf-resource-compare", "kind": "external-process", "ecosystem": "rust", "package": "gltf", "version": "1.4.1",
     "engine": {"family": "gltf-rs", "implementation": "gltf 1.4 document resource arrays", "version": "1.4.1"},
     "capabilities": ["gltf.resource.compare"], "outputSchema": "semio.repository-test.probe-report/v2",
     "deterministic": True, "license": "MIT OR Apache-2.0", "testOnly": True, "productionReachable": False,
     "networkDuringExecution": False, "command": [PROBES, "compare"]},
])

d.setdefault("comparisonProfiles", []).append({
    "id": "semantic-gltf-resource-v1",
    "description": "The six glTF RESOURCE arrays as ordered lists — accessors, buffers, bufferViews, images, samplers, textures — each entry carrying the fields a create/delete/move/reorder of it changes. Ordered, because `move` and `reorder` are observable only as order.",
})

d["comparisonPipelines"].append({
    "id": "gltf-2-0-resource-compare-v1",
    "description": "Document-level resource equality through `gltf` 1.4's own public API. GATING for the 24 resource kinds `three` cannot witness.",
    "stages": [
        {"probe": "gltf-resource-project", "description": "An independent document-level reader accepts both files.",
         "inputs": ["expected-gltf", "actual-gltf"], "assertions": {"bothImport": True}},
        {"probe": "gltf-resource-compare", "description": "Ordered equality over the six resource arrays — the operative equality.",
         "inputs": ["expected-gltf", "actual-gltf"], "assertions": {"equal": True}},
    ],
})

by_id = {m["id"]: m for m in d["mutationManifests"][0]["mutations"]}
repointed = 0
for kind in KINDS:
    m = by_id.get(kind)
    if m is None:
        raise SystemExit(f"unknown mutation kind {kind}")
    m["oracleRequirements"] = [{"capability": CAP, "qualifyingKind": "third-party-library", "oracle": OID}]
    repointed += 1

def digest(path):
    b = open(path, "rb").read()
    return f"sha256:{hashlib.sha256(b).hexdigest()}", len(b)

added = 0
for kind in KINDS:
    fid = f"{kind}-applied"
    base = os.path.join(FIX, fid)
    files = []
    for role, name in (("expected-before-gltf", "before.gltf"), ("expected-after-gltf", "after.gltf")):
        sha, n = digest(os.path.join(base, name))
        files.append({"role": role, "path": f"../🧫️fixtures/{fid}/{name}", "mediaType": "model/gltf+json", "sha256": sha, "bytes": n})
    d["fixtureManifests"].append({
        "schema": "semio.repository-test.fixture/v2", "id": fid, "class": "third-party-generated",
        "target": {"artifact": "s.stdio.gltf", "standard": "2.0", "subset": "any"},
        "mutation": kind, "outcome": "applied", "units": {"length": "unitless", "angle": "radian"},
        "files": files,
        "provenance": {"source": "generated", "license": "public-domain (synthetic, no third-party content embedded)"},
        "generator": {"oracle": "three-gltf-2-0-mutate-reader", "packageVersion": "0.182.0", "engineFamily": "threejs", "engineVersion": "0.182.0",
                      "command": "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts"},
        "comparisonPipeline": "gltf-2-0-resource-compare-v1",
    })
    added += 1

json.dump(d, open(ORACLE, "w"), ensure_ascii=False, indent=2)
open(ORACLE, "a").write("\n")
print(f"repointed {repointed} mutation(s), added {added} fixtureManifest(s)")
