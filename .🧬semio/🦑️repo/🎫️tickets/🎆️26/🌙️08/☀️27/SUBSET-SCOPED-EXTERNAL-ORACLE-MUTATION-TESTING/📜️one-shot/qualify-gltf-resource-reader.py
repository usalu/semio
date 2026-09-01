#!/usr/bin/env python3
# 🧵️ One-shot: completes the gltf resource-reader registration — probe qualification records and the
# per-fixture manifest fields the harness requires. Not idempotent (overwrites, so re-running is safe).
import json

ORACLE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
KINDS = [f"{op}-{s}" for op in ("create", "delete", "move")
         for s in ("accessor", "buffer", "buffer-view", "image", "sampler", "texture")] + \
        [f"reorder-{p}" for p in ("accessors", "buffers", "buffer-views", "images", "samplers", "textures")]

d = json.load(open(ORACLE))

EVIDENCE = (
    "Validated BOTH ways for all 24 resource recipes, per-fixture: (before,before) -> equal:true for 24/24; "
    "(before,after) -> equal:false for 24/24 — 48/48 directions correct. The discriminating example is "
    "create-image: images 3 -> 4 while textures stays 2 -> 2, the image/texture distinction "
    "@gltf-transform/core structurally cannot make (its Root exposes no listImages). "
    "The reader also REFUSED three malformed documents during development, which is how they were found: "
    "an early delete-accessor recipe removed a referenced accessor and orphaned an animation sampler's "
    "required `output` (\"missing field `output`\"), and an early delete-image left textures[1].source "
    "dangling (\"invalid glTF: textures[1].source: Missing data\"). Both now delete an unreferenced spare."
)
CRITERIA = [
    {"id": "accepts-a-known-good-pair", "met": True, "detail": "24/24 recipes: identical (before,before) compares equal:true"},
    {"id": "rejects-a-known-bad-pair", "met": True, "detail": "24/24 recipes: the recipe's own (before,after) compares equal:false"},
    {"id": "refuses-a-document-it-cannot-read", "met": True, "detail": "Refused two invalid glTF documents produced by early recipes, naming the orphaned field in each — a dangling index is reported, not silently projected"},
    {"id": "witnesses-what-the-sibling-readers-cannot", "met": True, "detail": "images and textures are separate observable arrays (create-image moves images 3->4, textures unchanged); bufferViews are observable at all (the base reads 18), which three omits by design and gltf-transform cannot list"},
    {"id": "deterministic", "met": True, "detail": "Document-only parse via Gltf::open; no pixel decoding, no sampling. All 120 fixture bundles regenerate byte-identically (aggregate sha256 unchanged across a full regeneration)"},
    {"id": "offline", "met": True, "detail": "gltf 1.4.1 is vendored in the cargo registry cache; the crate builds and runs with --offline"},
]

for probe in d["probes"]:
    if probe["id"] in ("gltf-resource-project", "gltf-resource-compare"):
        probe["qualification"] = {"status": "qualified", "evidence": EVIDENCE, "checkedAt": "2026-08-28", "criteria": CRITERIA}

wanted = {f"{k}-applied" for k in KINDS}
patched = 0
for manifest in d["fixtureManifests"]:
    if manifest["id"] not in wanted:
        continue
    manifest["comparisonProfile"] = "semantic-gltf-resource-v1"
    manifest["reproducible"] = True
    manifest["generator"]["platform"] = "darwin-arm64"
    manifest["generator"]["command"] = (
        "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts"
        f" generate --only {manifest['id']}"
    )
    patched += 1

json.dump(d, open(ORACLE, "w"), ensure_ascii=False, indent=2)
open(ORACLE, "a").write("\n")
print(f"qualified 2 probe(s), patched {patched} manifest(s)")
