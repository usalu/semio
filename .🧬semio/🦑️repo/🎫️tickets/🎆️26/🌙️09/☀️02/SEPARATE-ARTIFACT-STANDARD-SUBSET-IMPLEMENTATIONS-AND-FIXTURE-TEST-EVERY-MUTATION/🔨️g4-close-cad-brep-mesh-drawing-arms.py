#!/usr/bin/env python3
"""🔨️ G4 — closes s.stdio.semio@v1/base's remaining apply-cad/apply-brep/apply-mesh/apply-drawing
mutation-without-fixture breaches, extending F1's/G2's own established technique: a genuinely
independent, standalone Rust crate (own [workspace], DEPENDS ON serde_json AND NOTHING ELSE) edits
each arm's own JSON carrier directly (never through this repository's own mutation engine) to
produce a real, non-vacuous before/after pair for one kind whose full forward semantics is a pure
structural array append with zero computed fields (cad's add-layer, brep's create-vertex, mesh's
create-material, drawing's create-layer — each verified against that leaf's own diff body before
being chosen, see the ticket report). Registers one new `serde-json-semio-<arm>-carrier-reader`
oracle per arm (kind: third-party-library, mirroring ✳️document's own
`serde-json-semio-document-carrier-reader` exactly) at that arm's own 🧪️oracle/🔣️.json, then wraps
the real pair into ✳️base's own envelope shape and registers the fixtureManifest there — the
identical two-step pattern g2-close-document-arm.py already established for `document`.
"""
import json, hashlib, os

ROOT = "/Users/ueli/Documents/semio"
SUBSETS_ROOT = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
BASE_DIR = f"{SUBSETS_ROOT}/✳️base"
TICKET_SCRIPT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/🔨️g4-close-cad-brep-mesh-drawing-arms.py"

ARMS = {
    "cad": {
        "kind": "add-layer",
        "subset_dir": "✳️cad",
        "capability": "semio-v1-cad-mutate",
        "engine_dir": "🧬️schema/📸️snapshot",
        "inline_field": "layers",
        "inline_type": "CadLayer",
        "leaf_note": "add-layer's production diff (✳️cad/🧬️schema/🧬️mutations/🗂add-layer/🦀️.rs delegating into agg_diff) is NamedTripleDiff{added: vec![layer.clone()], removed: [], modified: []} on SemioCadSnapshot::layers, validated only for a non-duplicate `name` (validate_named_triple, ✳️base/🧬️schema/🧰️triples/🦀️.rs). No computed field, no cross-reference.",
    },
    "brep": {
        "kind": "create-vertex",
        "subset_dir": "✳️brep",
        "capability": "semio-v1-brep-mutate",
        "engine_dir": "🧬️schema/📸️snapshot",
        "inline_field": "vertices",
        "inline_type": "BrepVertex",
        "leaf_note": "create-vertex's production diff appends a caller-supplied {id, point} to SemioBrepSnapshot::vertices (an id-keyed append); create-vertex/🦀️.rs's own doc comment states a duplicate id already present in base is a no-op, never a duplicate. No computed field, no cross-reference.",
    },
    "mesh": {
        "kind": "create-material",
        "subset_dir": "✳️mesh",
        "capability": "semio-v1-mesh-mutate",
        "engine_dir": "🧬️schema/📸️snapshot",
        "inline_field": "materials",
        "inline_type": "SemioMaterial",
        "leaf_note": "create-material's production diff appends a caller-supplied SemioMaterial to SemioMeshSnapshot::materials (an id-keyed append); create-material/🦀️.rs's own doc comment states a duplicate id already present in base is a no-op. No computed field, no cross-reference.",
    },
    "drawing": {
        "kind": "create-layer",
        "subset_dir": "✳️drawing",
        "capability": "semio-v1-drawing-mutate",
        "engine_dir": "🧬️schema/📸️snapshot",
        "inline_field": "layers",
        "inline_type": "DrawLayer",
        "leaf_note": "create-layer's production diff (create-layer/🔺️diff/🦀️.rs) inserts a caller-supplied DrawLayer at index.min(base.layers.len()); an index at or past the end is a plain append, and a duplicate id already present in base is rejected outright (fatal). No computed field, no cross-reference for an end-of-list index.",
    },
}


def load(p):
    with open(p) as f:
        return json.load(f)


def dump(p, obj):
    with open(p, "w") as f:
        json.dump(obj, f, indent=2, ensure_ascii=False)
        f.write("\n")


def sha(b):
    return "sha256:" + hashlib.sha256(b).hexdigest()


def write_json(path, obj):
    text = json.dumps(obj, indent=2, ensure_ascii=False, sort_keys=False) + "\n"
    with open(path, "w") as f:
        f.write(text)
    return text.encode("utf-8")


def wrap(doc, arm):
    subset = dict(doc)
    subset["subset"] = arm
    return {"schema": "stdio.semio", "subset": subset}


for arm, spec in ARMS.items():
    subset_dir = f"{SUBSETS_ROOT}/{spec['subset_dir']}"
    oracle_json_path = f"{subset_dir}/🧪️oracle/🔣️.json"
    fixture_dir = f"{subset_dir}/🧫️fixtures/{spec['kind']}-applied"

    before_doc = load(f"{fixture_dir}/before.json")
    after_doc = load(f"{fixture_dir}/after.json")
    assert before_doc != after_doc, f"{arm}: must be non-vacuous"

    # 1. Register the new carrier oracle at the ARM's own oracle.json, mirroring
    #    serde-json-semio-document-carrier-reader exactly (kind, ecosystem, package, structure).
    oracle_id = f"serde-json-semio-{arm}-carrier-reader"
    reg = load(oracle_json_path)
    existing_oracle_ids = {o["id"] for o in reg["oracles"]}
    if oracle_id not in existing_oracle_ids:
        rationale = (
            f"\U0001F4D6️ A READER over this subset's own JSON export, covering exactly the one kind whose full "
            f"forward semantics reduces to a pure structural edit.\n\n"
            f"{spec['leaf_note']}\n\n"
            f"`Semio{arm.capitalize()}Snapshot::{spec['inline_field']}` is an INLINE `Vec<{spec['inline_type']}>` "
            f"carrying its full content directly in this subset's own JSON export (`{spec['subset_dir']}/{spec['engine_dir']}/\U0001F980️.rs`), "
            f"so a genuinely fresh, unique-keyed `{spec['kind']}` payload is a carrier-level fact a third-party JSON "
            f"implementation witnesses in full — the identical inline-versus-composed discriminator "
            f"`serde-json-semio-document-carrier-reader`'s own rationale already applies to `document`'s "
            f"`insert-image`/`remove-image`/`set-image-bytes` (`✳️document/\U0001F9EA️oracle/\U0001F523️.json`).\n\n"
            f"The reading tool is `{spec['subset_dir']}/\U0001F3EDgenerator/\U0001F980️json-engine` — its own standalone crate "
            f"(own `[workspace]`, `[dependencies] serde_json = \"1\"` and nothing else), which builds a deterministic "
            f"seed document, applies `{spec['kind']}` as a domain-blind edit to the JSON tree (never through this "
            f"repository's own mutation engine), and refuses to write a pair whose projection does not move. Scoped "
            f"HONESTLY to this one kind: the other kinds this subset's own `{spec['capability']}` capability names are "
            f"NOT claimed by this oracle — most carry computed fields, cross-references or binary payloads this "
            f"generic reader does not attempt (mirroring `dxf-crate-cad-r12-read`'s own partial-coverage precedent in "
            f"this same file, which likewise reuses the shared capability id rather than declaring a new one for a "
            f"subset of kinds)."
        )
        oracle_entry = {
            "id": oracle_id,
            "kind": "third-party-library",
            "ecosystem": "rust",
            "package": "serde_json",
            "version": "1",
            "source": {"repository": "https://github.com/serde-rs/json", "license": "MIT OR Apache-2.0"},
            "engine": {"family": "serde-json", "implementation": "serde_json 1 value tree", "version": "1"},
            "capabilities": [spec["capability"]],
            "comparisonProfiles": ["ordered-json-v1"],
            "license": "MIT OR Apache-2.0",
            "testOnly": True,
            "productionReachable": False,
            "networkDuringExecution": False,
            "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
            "homepage": "https://docs.rs/serde_json",
            "rationale": rationale,
        }
        reg["oracles"].append(oracle_entry)
        dump(oracle_json_path, reg)
        print(f"[{arm}] registered oracle {oracle_id} in {oracle_json_path}")
    else:
        print(f"[{arm}] oracle {oracle_id} already registered — skipped")

    # 2. Wrap the real before/after pair into base's envelope shape and register the fixtureManifest
    #    there, citing the oracle just registered — the identical g2-close-document-arm.py pattern.
    before_env = wrap(before_doc, arm)
    after_env = wrap(after_doc, arm)
    assert before_env != after_env, f"{arm}: envelope wrap must be non-vacuous"

    base_fixture_out = f"{BASE_DIR}/🧫️fixtures/apply-{arm}-applied"
    os.makedirs(base_fixture_out, exist_ok=True)
    before_bytes = write_json(f"{base_fixture_out}/before.json", before_env)
    after_bytes = write_json(f"{base_fixture_out}/after.json", after_env)

    base_oracle_json = f"{BASE_DIR}/🧪️oracle/🔣️.json"
    base_reg = load(base_oracle_json)
    fm_id = f"apply-{arm}-applied"
    existing_fm_ids = {fm["id"] for fm in base_reg["fixtureManifests"]}
    if fm_id in existing_fm_ids:
        print(f"[{arm}] fixtureManifest {fm_id} already registered at base — skipped")
        continue

    fixture_manifest = {
        "schema": "semio.repository-test.fixture/v2",
        "id": fm_id,
        "class": "third-party-generated",
        "target": {"artifact": "s.stdio.semio", "standard": "v1", "subset": "base"},
        "mutation": arm,
        "outcome": "applied",
        "units": {"length": "unitless", "angle": "radian"},
        "files": [
            {
                "role": "expected-before-json",
                "path": f"../🧫️fixtures/apply-{arm}-applied/before.json",
                "mediaType": "application/json",
                "sha256": sha(before_bytes),
                "bytes": len(before_bytes),
            },
            {
                "role": "expected-after-json",
                "path": f"../🧫️fixtures/apply-{arm}-applied/after.json",
                "mediaType": "application/json",
                "sha256": sha(after_bytes),
                "bytes": len(after_bytes),
            },
        ],
        "generator": {
            "oracle": oracle_id,
            "packageVersion": "1",
            "engineFamily": "serde-json",
            "engineVersion": "1",
            "command": f"cargo run --release --manifest-path {spec['subset_dir']}/🏭️generator/🦀️json-engine/Cargo.toml --bin generate -- ../../🧫️fixtures ; python3 {TICKET_SCRIPT}",
            "platform": "darwin-arm64",
        },
        "provenance": {
            "source": "generated",
            "license": "public-domain (synthetic, no third-party content embedded)",
            "attribution": (
                f"The envelope threading is this file's own; the wrapped `{arm}` content and its real mutation "
                f"effect ({spec['kind']}) were produced by {oracle_id} — that ARM subset's own newly-registered "
                f"third-party-library oracle (registered in ../../../../🪆️subsets/{spec['subset_dir']}/🧪️oracle/🔣️.json, "
                f"visible here because the oracle registry is repository-wide), a genuine independent JSON editor "
                f"over Semio{arm.capitalize()}Snapshot's own inline {spec['inline_field']} carrier, not a third-party "
                f"package producing THIS file directly; class: third-party-generated is used only because it is the "
                f"schema's own closest-fitting enum value for a generator-produced fixture."
            ),
            "security": "scanned-clean",
            "privacy": "no-personal-data",
        },
        "comparisonProfile": "ordered-json-v1",
        "reproducible": True,
        "family": "structural",
        "notes": (
            f"Applies {arm}'s own {spec['kind']} mutation (a genuinely fresh, unique-keyed "
            f"{spec['inline_type']} appended to {spec['inline_field']}) inside the envelope's own "
            f"SemioSnapshot{{schema, subset: SemioSubsetSnapshot::{arm.capitalize()}(...)}} shape. Wraps "
            f"{spec['subset_dir']}/🧫️fixtures/{spec['kind']}-applied's own real before/after pair (produced by "
            f"{oracle_id} editing the JSON carrier directly, never through this repository's own mutation engine) "
            f"verbatim. Demonstrates the envelope's own routing law: a wrapped mutation whose arm ('{arm}') matches "
            f"the base snapshot's own arm threads through to a real, observable content change."
        ),
    }
    base_reg["fixtureManifests"].append(fixture_manifest)
    dump(base_oracle_json, base_reg)
    print(f"[{arm}] registered fixtureManifest {fm_id} at {base_oracle_json}")
    print(f"[{arm}] before bytes {len(before_bytes)} after bytes {len(after_bytes)}")

print("done")
