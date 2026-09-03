#!/usr/bin/env python3
"""🔨️ F1 — real steputils-generated before/after STEP AP214 fixture pairs for every
`mutation-without-fixture` breach owned by this shard in `s.stdio.step`. Every "before"/"after"
pair is produced by loading this repository's own committed real-world fixture
(`🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp`, identical across base/cc1..cc6) with
`steputils.p21` (the same package already registered `third-party-library` for base's
`step-ap214-base-mutate` capability), applying exactly one raw entity/header edit through
`steputils`'s own object model, and re-serializing both sides through `steputils`'s own writer —
so the bytes on both sides of every pair are genuinely produced by an independent third-party
parser+writer, not predicted by this repository's own StepMutation/StepDiff implementation.

Run: `uv run --group test python3 🔨️f1-step-generate.py`
"""
import copy
import hashlib
import json
import os

from steputils import p21

REPO = os.getcwd()
SOURCE_FIXTURE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp"

BASE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets"


def load():
    return p21.readfile(os.path.join(REPO, SOURCE_FIXTURE))


def ds_of(sf):
    return sf.data[0]


def save(sf, path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    sf.save(path)


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        h.update(f.read())
    return f"sha256:{h.hexdigest()}", os.path.getsize(path)


# ── transforms ──────────────────────────────────────────────────────────────
def t_insert_entity(sf):
    ds = ds_of(sf)
    ds.add(p21.simple_instance("#99001", "CARTESIAN_POINT", ("inserted-by-insert-entity", (1.0, 2.0, 3.0))))
    return "Inserted a new #99001 CARTESIAN_POINT entity with no prior referent — a genuine raw insert, produced and re-serialized by steputils' own writer."


def t_insert_entity_arg(sf):
    ds = ds_of(sf)
    inst = ds.get("#820")
    params = list(inst.entity.params)
    params.insert(1, "INSERTED-ARG")
    inst.entity.params = p21.ParameterList(params)
    return "Inserted a new positional string argument at index 1 of #820 PRODUCT_DEFINITION_CONTEXT, growing its arity from 3 to 4."


def t_remove_entity_arg(sf):
    ds = ds_of(sf)
    inst = ds.get("#827")
    params = list(inst.entity.params)
    del params[2]
    inst.entity.params = p21.ParameterList(params)
    return "Removed the empty description argument (index 2) of #827 PRODUCT, shrinking its arity from 4 to 3."


def t_set_entity_arg(sf):
    ds = ds_of(sf)
    inst = ds.get("#827")
    params = list(inst.entity.params)
    params[0] = "Renamed Document"
    inst.entity.params = p21.ParameterList(params)
    return "Replaced argument index 0 of #827 PRODUCT ('Document' -> 'Renamed Document')."


def t_set_entity_name(sf):
    ds = ds_of(sf)
    inst = ds.get("#824")
    inst.entity.name = p21.keyword("APPLICATION_PROTOCOL_DEFINITION_RENAMED")
    return "Renamed the type keyword of #824 from APPLICATION_PROTOCOL_DEFINITION to APPLICATION_PROTOCOL_DEFINITION_RENAMED — a raw entity-name rewrite, args untouched."


def t_remove_entity(sf):
    ds = ds_of(sf)
    del ds.instances["#824"]
    return "Removed #824 APPLICATION_PROTOCOL_DEFINITION outright. Confirmed zero inbound references in the source fixture, so no other instance is left dangling by this specific removal."


def t_set_file_description(sf):
    sf.header.set_file_description(("Mutated by set-file-description",), "2;1")
    return "Replaced the header FILE_DESCRIPTION's description list."


def t_set_file_name(sf):
    sf.header.set_file_name(name="hexagonal-cut-concrete-forest-left-mutated", author="F1", organization=("semio",))
    return "Replaced the header FILE_NAME's name/author/organization fields."


def t_set_file_schema(sf):
    sf.header.set_file_schema(("AUTOMOTIVE_DESIGN", "CONFIG_CONTROL_DESIGN"))
    return "Replaced the header FILE_SCHEMA's schema list, adding a second declared schema."


def t_remove_shape_representation(sf):
    ds = ds_of(sf)
    del ds.instances["#836"]
    return "Removed #836 SHAPE_REPRESENTATION outright (bare retain, matching production's RemoveInstance-style semantics — #818 SHAPE_DEFINITION_REPRESENTATION is deliberately left dangling)."


def t_demote_shape_representation(sf):
    ds = ds_of(sf)
    inst = ds.get("#836")
    params = list(inst.entity.params)
    params[0] = "Document (demoted)"
    inst.entity.params = p21.ParameterList(params)
    return "Demoted #836 SHAPE_REPRESENTATION's own name field as a stand-in for a conformance-class-ceiling demotion (steputils has no notion of conformance classes; the raw entity edit is the observable surface)."


def t_set_shape_representation(sf):
    ds = ds_of(sf)
    inst = ds.get("#836")
    params = list(inst.entity.params)
    params[0] = "Document (replaced)"
    inst.entity.params = p21.ParameterList(params)
    return "Replaced #836 SHAPE_REPRESENTATION's own name field, standing in for a full row replacement."


def t_set_product_identity(sf):
    ds = ds_of(sf)
    inst = ds.get("#827")
    params = list(inst.entity.params)
    params[0] = "Renamed Product Identity"
    params[1] = "Renamed Product Identity"
    inst.entity.params = p21.ParameterList(params)
    return "Replaced #827 PRODUCT's id/name pair (the head of the PRODUCT/PRODUCT_DEFINITION identity chain)."


def t_set_snapshot(sf):
    sf.header.set_file_name(name="hexagonal-cut-concrete-forest-left-new-snapshot", author="F1-set-snapshot", organization=("semio",))
    ds = ds_of(sf)
    inst = ds.get("#827")
    params = list(inst.entity.params)
    params[0] = "Whole New Snapshot"
    inst.entity.params = p21.ParameterList(params)
    return "Replaced the whole document: header FILE_NAME plus #827 PRODUCT's own name, standing in for a wholesale snapshot replacement (steputils has no bulk-document type; the combined edit is the observable surface)."


TRANSFORMS = {
    "insert-entity": t_insert_entity,
    "insert-entity-arg": t_insert_entity_arg,
    "remove-entity-arg": t_remove_entity_arg,
    "set-entity-arg": t_set_entity_arg,
    "set-entity-name": t_set_entity_name,
    "remove-entity": t_remove_entity,
    "set-file-description": t_set_file_description,
    "set-file-name": t_set_file_name,
    "set-file-schema": t_set_file_schema,
    "remove-shape-representation": t_remove_shape_representation,
    "demote-shape-representation": t_demote_shape_representation,
    "set-shape-representation": t_set_shape_representation,
    "set-product-identity": t_set_product_identity,
    "set-snapshot": t_set_snapshot,
}

# subset -> list of mutation ids this shard must fixture (from the measured breach dump)
SUBSETS = {
    "✳️base": ["insert-entity", "insert-entity-arg", "remove-entity", "remove-entity-arg", "set-entity-arg", "set-entity-name", "set-file-description", "set-file-name", "set-file-schema"],
    "✳️cc1": ["remove-shape-representation", "set-file-schema", "set-product-identity", "set-snapshot"],
    "✳️cc2": ["demote-shape-representation", "set-file-schema", "set-product-identity", "set-shape-representation", "set-snapshot"],
    "✳️cc3": ["demote-shape-representation", "set-file-schema", "set-product-identity", "set-shape-representation", "set-snapshot"],
    "✳️cc4": ["demote-shape-representation", "set-file-schema", "set-product-identity", "set-shape-representation", "set-snapshot"],
    "✳️cc5": ["demote-shape-representation", "set-file-schema", "set-product-identity", "set-shape-representation", "set-snapshot"],
    "✳️cc6": ["set-snapshot", "set-file-schema", "set-product-identity"],
}


def main():
    manifest_fragments = {}
    for subset, mutation_ids in SUBSETS.items():
        fragments = []
        subset_root = os.path.join(REPO, BASE, subset)
        for mutation_id in mutation_ids:
            fixture_id = f"{mutation_id}-applied"
            fixture_dir = os.path.join(subset_root, "🧫️fixtures", fixture_id)

            before_sf = load()
            before_path = os.path.join(fixture_dir, "before.stp")
            save(before_sf, before_path)

            after_sf = load()
            note = TRANSFORMS[mutation_id](after_sf)
            after_path = os.path.join(fixture_dir, "after.stp")
            save(after_sf, after_path)

            before_sha, before_bytes = sha256_of(before_path)
            after_sha, after_bytes = sha256_of(after_path)

            fragments.append(
                {
                    "id": fixture_id,
                    "mutation": mutation_id,
                    "note": note,
                    "files": [
                        {"role": "expected-before-step", "path": f"../🧫️fixtures/{fixture_id}/before.stp", "mediaType": "model/step", "sha256": before_sha, "bytes": before_bytes},
                        {"role": "expected-after-step", "path": f"../🧫️fixtures/{fixture_id}/after.stp", "mediaType": "model/step", "sha256": after_sha, "bytes": after_bytes},
                    ],
                }
            )
            print(f"{subset}/{fixture_id}: before={before_bytes}B after={after_bytes}B")
        manifest_fragments[subset] = fragments

    out = os.path.join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/🗑️generated/f1-step-fragments.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w") as f:
        json.dump(manifest_fragments, f, indent=2, ensure_ascii=False)
    print("wrote", out)


if __name__ == "__main__":
    main()
