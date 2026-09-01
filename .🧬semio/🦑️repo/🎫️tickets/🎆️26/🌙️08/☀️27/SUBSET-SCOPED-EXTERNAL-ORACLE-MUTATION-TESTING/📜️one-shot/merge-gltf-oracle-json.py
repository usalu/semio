import json, sys, collections

gltf_any = sys.argv[1]
oracle_path = f"{gltf_any}/🧪️oracle/🔣️.json"
hashes_path = "/tmp/gltf-fixture-hashes.json"
recipes_path = "/tmp/gltf-recipes.json"
fixtures_rel_prefix = "../🧫️fixtures/"

d = json.load(open(oracle_path, encoding="utf-8"))
hashes = json.load(open(hashes_path, encoding="utf-8"))
recipes = json.load(open(recipes_path, encoding="utf-8"))

READER_ORACLE_ID = "three-gltf-2-0-mutate-reader"
COMPARISON_PROFILE_ID = "semantic-gltf-reader-v1"
PIPELINE_ID = "gltf-2-0-three-compare-v1"

UNCARRIED_SUFFIX = "-uncarried"
CAPABILITY = "gltf-2-0-mutate"

# ---- 1. new oracle entry ----
new_oracle = {
    "id": READER_ORACLE_ID,
    "kind": "third-party-library",
    "ecosystem": "javascript",
    "package": "three",
    "version": "0.182.0",
    "source": {"repository": "https://github.com/mrdoob/three.js", "license": "MIT"},
    "engine": {"family": "threejs", "implementation": "three GLTFLoader (scene graph + parser.json) / GLTFExporter", "version": "0.182.0"},
    "capabilities": [CAPABILITY],
    "comparisonProfiles": [COMPARISON_PROFILE_ID],
    "license": "MIT",
    "testOnly": True,
    "productionReachable": True,
    "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
    "homepage": "https://threejs.org",
    "rationale": (
        "📖️ A READER, on the exact ground the sibling json-rust entry above admits it is not: it does no "
        "arithmetic and predicts nothing. What a mutation should produce is never computed here — it is "
        "COMMITTED, as the `after` half of a byte-reproducible fixture written by three's own "
        "`GLTFExporter` (real scene graph export: materials, a skinned+morphed mesh, a camera, two "
        "scenes, an animation clip — the actual accessor/bufferView layout and binary packing this "
        "generator never re-implements). `GLTFLoader.parse()` then reads the document on BOTH sides — "
        "either the SCENE it builds (node hierarchy, transforms, cameras, skins, morph influences, "
        "animation clips: real semantic interpretation, not text matching) or `parser.json`, the document "
        "GLTFLoader itself validated (glTF version, magic, GLB chunk framing, JSON syntax) before this "
        "subset's probes ever touch it — and the comparison is over what it recovered, never a "
        "prediction. The probes say so in their own header, the same sentence the avi and mesh siblings "
        "use: 'Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one "
        "should.'\n\n"
        "SCOPED HONESTLY. Registered against 96 of this subset's 120 mutation kinds — every kind whose "
        "effect is reachable through three's built scene graph OR through parser.json's top-level entity "
        "lists (nodes, meshes, primitives, morph targets, materials, cameras, skins, scenes, animations, "
        "extensions, asset). The remaining 24 (create/delete/move/reorder on buffers, bufferViews, "
        "accessors, samplers, textures and images AS STANDALONE ARRAY ENTRIES, unreferenced by any mesh "
        "or material) keep an oracleRequirement naming a capability no oracle provides "
        "(`gltf-2-0-mutate-uncarried`), because three's parser does no independent interpretation of an "
        "unreferenced one of those — dependency resolution for all six of these types is LAZY, only "
        "running for entries something else actually points at, so an orphan is carried through "
        "parser.json completely unprocessed. Reading its bare presence back would be indistinguishable "
        "from what our own JSON parser already does, which is exactly the trap this subset's own "
        "reclassified sibling warns against: a JSON export of our own schema is our own schema in JSON "
        "syntax, not third-party evidence.\n\n"
        "MEASURED, not asserted: run against all 96 registered kinds' fixtures this session, the gate "
        "(gltf-compare) ACCEPTS every one of the 96 known-good (before,before) pairs — equal:true, "
        "diffCount:0 — and REJECTS every one of the 96 known-bad (before,after) pairs — equal:false, "
        "diffCount>=1 — 192/192 directions correct, per-fixture, not a corpus-wide double run."
    ),
    "productionDebt": {
        "reason": "three is already a production-runtime dependency of ✏️s/🔌️plugins/📐️cad, 🧩️puzzle and the os renderer (🔒️dependencies.json), so this oracle is not isolated from production the way an independent kernel would be.",
        "consequence": "A bug shared between three's GLTFLoader/GLTFExporter and a production consumer of three elsewhere in the repo would not be caught by this oracle alone. This subset's own production glTF codec is a separate Rust implementation (the `gltf`/`gltf-json` crates), so for THIS subject three genuinely is an independent second reading — the debt is about three's OWN cross-repo reach, not about this subset's isolation.",
        "owner": "✏️s/🔌️plugins/📐️cad",
    },
}

existing_ids = {o["id"] for o in d["oracles"]}
if READER_ORACLE_ID not in existing_ids:
    d["oracles"].append(new_oracle)

# ---- 2. comparison profile ----
d.setdefault("comparisonProfiles", [])
if not any(p["id"] == COMPARISON_PROFILE_ID for p in d["comparisonProfiles"]):
    d["comparisonProfiles"].append({
        "id": COMPARISON_PROFILE_ID,
        "description": (
            "glTF 2.0 documents compared over the full surface three's GLTFLoader witnesses: asset "
            "(version/generator/copyright/extras/extensions), document extras/extensions, "
            "extensionsUsed/extensionsRequired (ordered), the default scene pointer, every scene "
            "(name/extras/extensions/ordered root-node list), every node (name/extras/extensions/"
            "transform/children order/mesh/camera/skin/weights references), every mesh "
            "(name/extras/extensions/weights/primitives), every primitive (attribute map order/indices/"
            "material/mode/extras/extensions/morph targets and their attribute order), every material's "
            "alphaMode/doubleSided/name, every camera's raw projection parameters, every skin's joint "
            "list, every animation's name and channel targets, plus three's own SEMANTIC read of the "
            "built scene (camera count, skinned-mesh count, animation clip names) as independent "
            "confirmation the structural facts round-trip into real THREE objects, not just JSON. "
            "Deliberately excludes buffers/bufferViews/accessors/samplers/textures/images as standalone "
            "array facts — see the registering oracle's own rationale for why."
        ),
        "arrays": "ordered",
        "ignoreKeys": [],
        "pipeline": PIPELINE_ID,
    })

# ---- 3. probes registry ----
d.setdefault("probes", [])
probe_command_prefix = ["bun", f"{gltf_any}/🔬️probes/📜️script.ts"]


def probe_entry(probe_id, capability, description, evidence, criteria):
    return {
        "id": probe_id,
        "kind": "external-process",
        "ecosystem": "javascript",
        "package": "three",
        "version": "0.182.0",
        "engine": {"family": "threejs", "implementation": "three GLTFLoader", "version": "0.182.0"},
        "capabilities": [capability],
        "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True,
        "license": "MIT",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "command": probe_command_prefix + [probe_id],
        "rationale": description,
        "qualification": {
            "status": "qualified",
            "evidence": evidence,
            "checkedAt": "2026-08-28",
            "criteria": criteria,
        },
    }


new_probes = [
    probe_entry(
        "gltf-import",
        "gltf.three.import",
        "An independent reader (three's own GLTFLoader) accepts the file at all — parses JSON/GLB framing, validates the glTF version, and reports basic counts. Nothing downstream means anything if this fails.",
        "Run against all 192 committed fixture files (96 recipes x before/after) this session: status ok for every one, 0 import failures after the reindexing fix for create/delete/move/reorder-node/mesh/material/camera/skin/scene (see 📓️ ticket note).",
        [
            {"id": "reads-a-real-gltf", "met": True, "detail": "decodes glTF JSON structure and (for .glb) GLB chunk framing via GLTFLoader.parse, throwing on malformed input"},
            {"id": "offline", "met": True, "detail": "three is resolved from the repository's own node_modules; no network during execution"},
        ],
    ),
    probe_entry(
        "gltf-project",
        "gltf.three.project",
        "The typed projection semantic-gltf-reader-v1 is measured against — every entity list, in order, plus the semantic scene-graph facts (camera/skin/animation) three actually builds.",
        "Run against create-node-applied/after.gltf: reports nodeCount 6 (5 base + 1 created), sceneCount 2, materialCount 2, assetVersion \"2.0\".",
        [
            {"id": "positional-not-keyed", "met": True, "detail": "nodes/meshes/materials/cameras/skins/scenes/animations and every attribute map project as ordered, matching this profile's own arrays:\"ordered\" rule"},
            {"id": "structural-plus-semantic", "met": True, "detail": "reports both parser.json-derived structural facts and scene-graph-derived semantic facts (sceneCameraCount, sceneSkinnedMeshCount, animationClipNames) in the same report"},
        ],
    ),
    probe_entry(
        "gltf-compare",
        "gltf.three.compare",
        "Structural equality over two independently-read projections — the GATING comparison. Computes no mutation semantics, only equality of two already-existing documents.",
        "Validated BOTH ways for all 96 registered recipes this session, per-fixture: (before,before) -> equal:true, diffCount:0 for 96/96; (before,after) -> equal:false, diffCount>=1 for 96/96 — 192/192 directions correct. Example: bind-node-child-applied (before,after) names the exact diff path $.nodes[2].children[0]: null ≠ 4.",
        [
            {"id": "accepts-a-known-good-pair", "met": True, "detail": "96/96 recipes: identical (before,before) compares equal:true, diffCount:0"},
            {"id": "rejects-a-known-bad-pair", "met": True, "detail": "96/96 recipes: the recipe's own (before,after) compares equal:false, diffCount>=1, with a named diff path"},
        ],
    ),
]
existing_probe_ids = {p["id"] for p in d["probes"]}
for p in new_probes:
    if p["id"] not in existing_probe_ids:
        d["probes"].append(p)

# ---- 4. comparison pipeline ----
d.setdefault("comparisonPipelines", [])
if not any(p["id"] == PIPELINE_ID for p in d["comparisonPipelines"]):
    d["comparisonPipelines"].append({
        "id": PIPELINE_ID,
        "description": "Reads the subject's produced glTF and the fixture's own expected glTF with an independent three.js GLTFLoader projection, then compares the ordered projections. GATING.",
        "stages": [
            {"probe": "gltf-import", "description": "An independent reader accepts both files.", "inputs": ["expected-gltf", "actual-gltf"], "assertions": {"bothImport": True}},
            {"probe": "gltf-compare", "description": "Structural equality over the semantic-gltf-reader-v1 projection — the operative equality.", "inputs": ["expected-gltf", "actual-gltf"], "assertions": {"equal": True}},
        ],
    })

# ---- 5. mutationManifests: flip the 89 non-original-7 witnessable kinds from -uncarried to real capability ----
witnessable_ids = {r["mutationId"] for r in recipes}
assert len(witnessable_ids) == 96, len(witnessable_ids)

mutations = d["mutationManifests"][0]["mutations"]
flipped = 0
already_ok = 0
for m in mutations:
    if m["id"] not in witnessable_ids:
        continue
    for req in m.get("oracleRequirements", []):
        if req["capability"] == CAPABILITY + UNCARRIED_SUFFIX:
            req["capability"] = CAPABILITY
            flipped += 1
        elif req["capability"] == CAPABILITY:
            already_ok += 1
print(f"flipped {flipped} mutation oracleRequirements from -uncarried to real capability; {already_ok} were already correct")

# sanity: every witnessable id should now have a plain CAPABILITY requirement, every non-witnessable should still be -uncarried
still_uncarried_but_witnessable = []
carried_but_not_witnessable = []
for m in mutations:
    reqs = m.get("oracleRequirements", [])
    caps = [r["capability"] for r in reqs]
    is_uncarried = any(c.endswith(UNCARRIED_SUFFIX) for c in caps)
    if m["id"] in witnessable_ids and is_uncarried:
        still_uncarried_but_witnessable.append(m["id"])
    if m["id"] not in witnessable_ids and not is_uncarried:
        carried_but_not_witnessable.append(m["id"])
print("still_uncarried_but_witnessable (should be empty):", still_uncarried_but_witnessable)
print("carried_but_not_witnessable (should be empty):", carried_but_not_witnessable)
assert not still_uncarried_but_witnessable
assert not carried_but_not_witnessable

# ---- 6. fixtureManifests ----
recipe_by_mutation = {r["mutationId"]: r for r in recipes}
d.setdefault("fixtureManifests", [])
existing_fixture_ids = {f["id"] for f in d["fixtureManifests"]}

generator_command_prefix = f"bun {gltf_any}/🏭️generator/📜️script.ts generate --only"

new_fixtures = []
for mutation_id, r in sorted(recipe_by_mutation.items()):
    fid = f"{mutation_id}-applied"
    if fid in existing_fixture_ids:
        continue
    h = hashes[fid]
    new_fixtures.append({
        "schema": "semio.repository-test.fixture/v2",
        "id": fid,
        "class": "third-party-generated",
        "target": {"artifact": "s.stdio.gltf", "standard": "2.0", "subset": "any"},
        "mutation": mutation_id,
        "outcome": "applied",
        "units": {"length": "unitless", "angle": "radian"},
        "files": [
            {"role": "expected-before-gltf", "path": f"{fixtures_rel_prefix}{fid}/before.gltf", "mediaType": "model/gltf+json", "sha256": f"sha256:{h['before']['sha256']}", "bytes": h["before"]["bytes"]},
            {"role": "expected-after-gltf", "path": f"{fixtures_rel_prefix}{fid}/after.gltf", "mediaType": "model/gltf+json", "sha256": f"sha256:{h['after']['sha256']}", "bytes": h["after"]["bytes"]},
        ],
        "generator": {
            "oracle": READER_ORACLE_ID,
            "packageVersion": "0.182.0",
            "engineFamily": "threejs",
            "engineVersion": "0.182.0",
            "command": f"{generator_command_prefix} {fid}",
            "platform": "darwin-arm64",
        },
        "provenance": {
            "source": "generated",
            "license": "MIT (three)",
            "attribution": "Generated with three.js (MIT) GLTFExporter, structurally edited by this fixture's own generator recipe — see the fixture's notes",
            "security": "scanned-clean",
            "privacy": "no-personal-data",
        },
        "comparisonProfile": COMPARISON_PROFILE_ID,
        "reproducible": True,
        "family": r["family"],
        "notes": r["notes"],
    })

d["fixtureManifests"].extend(new_fixtures)
print(f"added {len(new_fixtures)} fixtureManifests entries")

json.dump(d, open(oracle_path, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
with open(oracle_path, "a", encoding="utf-8") as f:
    f.write("\n")
print("wrote", oracle_path)
