#!/usr/bin/env python3
"""🔨️ F1 — real before/after IFC fixture pairs for every `mutation-without-fixture` breach owned by
this shard in `s.stdio.ifc`. Two genuine third-party writers are used, matched to the already-
registered oracle each subset actually carries:

- `2x3/base`, `2x3/cobie`, `2x3/cv20`, `2x3/sav` are DOMAIN-level mutations (a building's own Name,
  Elevation, unit prefixes, structural rows). These use `ifcopenshell` 0.8.4.post1's own high-level
  Python object model (`entity.Attribute = value`, `ifcopenshell.api.root.create_entity`,
  `file.remove`) — already registered `third-party-library` for each of these subsets' own
  capability by shard E2.
- `4/any` is a RAW Part-21 entity-graph mutation (insert/remove/set an entity or one positional
  argument, exactly `step/ap214/base`'s own model) — arity-changing edits ifcopenshell's schema-typed
  API cannot perform (IFC4 entities have fixed EXPRESS arity). `steputils` 0.1 is schema-agnostic
  Part-21 syntax and already proven (via `🔨️f1-step-generate.py`) to read+write real IFC/STEP text
  identically; it is registered here as a new subset-scoped oracle for `ifc-4-any-mutate`.

Run: `uv run --group test python3 🔨️f1-ifc-generate.py`
"""
import hashlib
import json
import os

import ifcopenshell
import ifcopenshell.api.root
from steputils import p21

REPO = os.getcwd()
ROOT_2X3 = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets"
ROOT_4 = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets"

FIXTURE_2X3_BASE = f"{ROOT_2X3}/✳️base/🧫️fixtures/🧪️wellness-center-sama-street-level/🏗️.ifc"
FIXTURE_2X3_COBIE = f"{ROOT_2X3}/✳️cobie/🧫️fixtures/🧪️wellness-center-sama-street-level/🏗️.ifc"
FIXTURE_2X3_CV20 = f"{ROOT_2X3}/✳️cv20/🧫️fixtures/🧪️wellness-center-sama-street-level/🏗️.ifc"
FIXTURE_2X3_SAV = f"{ROOT_2X3}/✳️sav/🧫️fixtures/🧪️wellness-center-sama-structural-seed/🏗️.ifc"
FIXTURE_4_ANY = f"{ROOT_4}/✳️any/🧫️fixtures/🧪️nakagin-capsule-tower/🏗️.ifc"


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        h.update(f.read())
    return f"sha256:{h.hexdigest()}", os.path.getsize(path)


def ifc_pair(subset_root, mutation_id, source_fixture, transform, media="model/ifc", ext="ifc"):
    fixture_id = f"{mutation_id}-applied"
    fixture_dir = os.path.join(REPO, subset_root, "🧫️fixtures", fixture_id)
    os.makedirs(fixture_dir, exist_ok=True)

    before = ifcopenshell.open(os.path.join(REPO, source_fixture))
    before_path = os.path.join(fixture_dir, f"before.{ext}")
    before.write(before_path)

    after = ifcopenshell.open(os.path.join(REPO, source_fixture))
    note = transform(after)
    after_path = os.path.join(fixture_dir, f"after.{ext}")
    after.write(after_path)

    before_sha, before_bytes = sha256_of(before_path)
    after_sha, after_bytes = sha256_of(after_path)
    return {
        "id": fixture_id,
        "mutation": mutation_id,
        "note": note,
        "generator_kind": "ifcopenshell",
        "files": [
            {"role": "expected-before-ifc", "path": f"../🧫️fixtures/{fixture_id}/before.{ext}", "mediaType": media, "sha256": before_sha, "bytes": before_bytes},
            {"role": "expected-after-ifc", "path": f"../🧫️fixtures/{fixture_id}/after.{ext}", "mediaType": media, "sha256": after_sha, "bytes": after_bytes},
        ],
    }


def step_pair(subset_root, mutation_id, source_fixture, transform):
    fixture_id = f"{mutation_id}-applied"
    fixture_dir = os.path.join(REPO, subset_root, "🧫️fixtures", fixture_id)
    os.makedirs(fixture_dir, exist_ok=True)

    before = p21.readfile(os.path.join(REPO, source_fixture))
    before_path = os.path.join(fixture_dir, "before.ifc")
    before.save(before_path)

    after = p21.readfile(os.path.join(REPO, source_fixture))
    note = transform(after)
    after_path = os.path.join(fixture_dir, "after.ifc")
    after.save(after_path)

    before_sha, before_bytes = sha256_of(before_path)
    after_sha, after_bytes = sha256_of(after_path)
    return {
        "id": fixture_id,
        "mutation": mutation_id,
        "note": note,
        "generator_kind": "steputils",
        "files": [
            {"role": "expected-before-ifc", "path": f"../🧫️fixtures/{fixture_id}/before.ifc", "mediaType": "model/ifc", "sha256": before_sha, "bytes": before_bytes},
            {"role": "expected-after-ifc", "path": f"../🧫️fixtures/{fixture_id}/after.ifc", "mediaType": "model/ifc", "sha256": after_sha, "bytes": after_bytes},
        ],
    }


# ── 2x3/base (ifcopenshell, domain-level) ───────────────────────────────────
def base_upsert_instance(f):
    b = f.by_type("IfcBuilding")[0]
    b.Name = "Upserted Building Name"
    return "Upserted #IfcBuilding's own Name attribute through ifcopenshell's typed object model, re-serialized by its own writer — standing in for UpsertInstance's raw instance-content replace."


def base_remove_instance(f):
    target = f.by_id(659570)
    assert len(f.get_inverse(target)) == 0, "removal target must have zero inbound references to match production's bare-retain (no reference repair) semantics"
    f.remove(target)
    return "Removed #659570 IfcRelDefinesByProperties outright via ifcopenshell's file.remove. Confirmed zero inbound references in the source fixture (measured with file.get_inverse before removal), so ifcopenshell's reference-repairing removal is observably identical to production's bare-retain RemoveInstance for this specific instance."


def base_set_header(f):
    f.header.file_name.name = "wellness-center-sama-street-level-mutated"
    f.header.file_name.author = ("F1",)
    f.header.file_name.organization = ("semio",)
    return "Replaced the header FILE_NAME's name/author/organization fields via ifcopenshell's own header object."


# ── 2x3/cobie (ifcopenshell) ─────────────────────────────────────────────────
def cobie_set_facility_name(f):
    b = f.by_type("IfcBuilding")[0]
    b.Name = "Renamed Facility"
    return "Set #IfcBuilding.Name (the COBie facility row's own name) via ifcopenshell."


def cobie_set_floor_elevation(f):
    storeys = sorted(f.by_type("IfcBuildingStorey"), key=lambda s: s.id())
    storeys[1].Elevation = 999.0
    return f"Set #{storeys[1].id()} IfcBuildingStorey ({storeys[1].Name!r})'s Elevation to 999.0 via ifcopenshell."


def cobie_set_space(f):
    space = ifcopenshell.api.root.create_entity(f, ifc_class="IfcSpace", name="Room 101")
    space.GlobalId = "2PlvyCHRv1QuRoDETERMNS"
    return "Created a new #IfcSpace ('Room 101') via ifcopenshell.api.root.create_entity — this fixture's own real-world source has no IfcSpace, so COBie's set-space mutation is witnessed by a genuine third-party-constructed entity rather than an edit of a pre-existing one. GlobalId pinned to a fixed literal after creation so the fixture is byte-reproducible across regenerations."


def cobie_set_type_assignment(f):
    rdt = sorted(f.by_type("IfcRelDefinesByType"), key=lambda r: r.id())[0]
    rdt.Name = "Renamed Type Assignment"
    return f"Set #{rdt.id()} IfcRelDefinesByType.Name via ifcopenshell."


def cobie_set_view_definition(f):
    f.header.file_description.description = ("ViewDefinition [COBie_2_4]",)
    return "Replaced the header FILE_DESCRIPTION's view-definition text via ifcopenshell's own header object."


def cobie_set_snapshot(f):
    b = f.by_type("IfcBuilding")[0]
    b.Name = "Whole New Snapshot Facility"
    storeys = sorted(f.by_type("IfcBuildingStorey"), key=lambda s: s.id())
    storeys[0].Elevation = 111.0
    return "Replaced both #IfcBuilding.Name and the first #IfcBuildingStorey.Elevation together, standing in for a wholesale COBie snapshot replacement (ifcopenshell has no single bulk-document type; the combined edit is the observable surface)."


# ── 2x3/cv20 (ifcopenshell) ──────────────────────────────────────────────────
def cv20_set_structural_entity(f):
    col = sorted(f.by_type("IfcColumn"), key=lambda c: c.id())[0]
    col.Name = "Renamed Structural Column"
    return f"Set #{col.id()} IfcColumn.Name via ifcopenshell — a real structural building element already present in this fixture, standing in for CV2.0's structural-entity row."


def cv20_set_project_units(f):
    units = [u for u in f.by_type("IfcSIUnit") if u.UnitType == "LENGTHUNIT" and u.Prefix is None]
    units[0].Prefix = "CENTI"
    return f"Set #{units[0].id()} IfcSIUnit (LENGTHUNIT, previously unprefixed metre)'s Prefix to .CENTI. via ifcopenshell."


def cv20_set_product_placement(f):
    col = sorted(f.by_type("IfcColumn"), key=lambda c: c.id())[0]
    pt = col.ObjectPlacement.RelativePlacement.Location
    pt.Coordinates = (1000.0, 2000.0, 3000.0)
    return f"Set #{pt.id()} IfcCartesianPoint (the coordinate underlying #{col.id()} IfcColumn's own ObjectPlacement chain)'s Coordinates via ifcopenshell."


def cv20_set_view_definition(f):
    f.header.file_description.description = ("ViewDefinition [StructuralAnalysisView]",)
    return "Replaced the header FILE_DESCRIPTION's view-definition text via ifcopenshell."


def cv20_set_snapshot(f):
    col = sorted(f.by_type("IfcColumn"), key=lambda c: c.id())[0]
    col.Name = "Whole New Snapshot Column"
    units = [u for u in f.by_type("IfcSIUnit") if u.UnitType == "LENGTHUNIT" and u.Prefix is None]
    units[0].Prefix = "DECI"
    return "Replaced both an IfcColumn.Name and an IfcSIUnit.Prefix together, standing in for a wholesale CV2.0 snapshot replacement."


# ── 2x3/sav (ifcopenshell) ───────────────────────────────────────────────────
def sav_set_analysis_model(f):
    m = f.by_type("IfcStructuralAnalysisModel")[0]
    m.Name = "Renamed Analysis Model"
    return f"Set #{m.id()} IfcStructuralAnalysisModel.Name via ifcopenshell."


def sav_set_group_assignment(f):
    g = f.by_type("IfcRelAssignsToGroup")[0]
    g.Name = "Renamed Group Assignment"
    return f"Set #{g.id()} IfcRelAssignsToGroup.Name via ifcopenshell."


def sav_set_load_group(f):
    lg = f.by_type("IfcStructuralLoadGroup")[0]
    lg.Name = "Renamed Load Group"
    return f"Set #{lg.id()} IfcStructuralLoadGroup.Name via ifcopenshell."


def sav_set_view_definition(f):
    f.header.file_description.description = ("ViewDefinition [StructuralAnalysisView]",)
    return "Replaced the header FILE_DESCRIPTION's view-definition text via ifcopenshell."


def sav_set_snapshot(f):
    m = f.by_type("IfcStructuralAnalysisModel")[0]
    m.Name = "Whole New Snapshot Model"
    lg = f.by_type("IfcStructuralLoadGroup")[0]
    lg.Name = "Whole New Snapshot Load Group"
    return "Replaced both an IfcStructuralAnalysisModel.Name and an IfcStructuralLoadGroup.Name together, standing in for a wholesale SAV snapshot replacement."


# ── 4/any (steputils, raw Part-21 entity graph) ─────────────────────────────
def ifc4_insert_entity(sf):
    ds = sf.data[0]
    ds.add(p21.simple_instance("#999001", "IFCCARTESIANPOINT", ("inserted-by-insert-entity", (1.0, 2.0, 3.0))))
    return "Inserted a new #999001 IFCCARTESIANPOINT-shaped entity with no prior referent — a genuine raw insert, re-serialized by steputils' own writer."


def ifc4_insert_entity_arg(sf):
    ds = sf.data[0]
    inst = ds.get("#1")  # IfcProject
    params = list(inst.entity.params)
    params.insert(2, "INSERTED-ARG")
    inst.entity.params = p21.ParameterList(params)
    return "Inserted a new positional string argument at index 2 of #1 IFCPROJECT."


def ifc4_remove_entity_arg(sf):
    ds = sf.data[0]
    inst = ds.get("#1")
    params = list(inst.entity.params)
    del params[3]
    inst.entity.params = p21.ParameterList(params)
    return "Removed the positional argument at index 3 of #1 IFCPROJECT."


def ifc4_set_entity_arg(sf):
    ds = sf.data[0]
    inst = ds.get("#1")
    params = list(inst.entity.params)
    params[2] = "Renamed Project"
    inst.entity.params = p21.ParameterList(params)
    return "Replaced argument index 2 (Name) of #1 IFCPROJECT."


def ifc4_set_entity_name(sf):
    ds = sf.data[0]
    inst = ds.get("#1")
    inst.entity.name = p21.keyword("IFCPROJECT_RENAMED")
    return "Renamed the type keyword of #1 from IFCPROJECT to IFCPROJECT_RENAMED — a raw entity-name rewrite, args untouched."


def ifc4_remove_entity(sf):
    ds = sf.data[0]
    refs = ds.references()
    inbound = set()
    for ref in refs:
        inst = ds.get(ref)
        for p in inst.entity.params:
            _collect_refs(p, inbound)
    candidate = next(r for r in refs if r not in inbound and r != "#1")
    del ds.instances[candidate]
    return f"Removed {candidate} outright — confirmed zero inbound references among this fixture's own entity graph before removal, so nothing else is left dangling by this specific removal."


def _collect_refs(value, out):
    if isinstance(value, p21.Reference):
        out.add(str(value))
    elif isinstance(value, (p21.ParameterList, tuple, list)):
        for v in value:
            _collect_refs(v, out)


def ifc4_set_file_description(sf):
    sf.header.set_file_description(("Mutated by set-file-description",), "2;1")
    return "Replaced the header FILE_DESCRIPTION's description list."


def ifc4_set_file_name(sf):
    sf.header.set_file_name(name="nakagin-capsule-tower-mutated", author="F1", organization=("semio",))
    return "Replaced the header FILE_NAME's name/author/organization fields."


def ifc4_set_file_schema(sf):
    sf.header.set_file_schema(("IFC4", "IFC4X1_ADD1"))
    return "Replaced the header FILE_SCHEMA's schema list, adding a second declared schema."


IFC4_TRANSFORMS = {
    "insert-entity": ifc4_insert_entity,
    "insert-entity-arg": ifc4_insert_entity_arg,
    "remove-entity": ifc4_remove_entity,
    "remove-entity-arg": ifc4_remove_entity_arg,
    "set-entity-arg": ifc4_set_entity_arg,
    "set-entity-name": ifc4_set_entity_name,
    "set-file-description": ifc4_set_file_description,
    "set-file-name": ifc4_set_file_name,
    "set-file-schema": ifc4_set_file_schema,
}


def main():
    fragments = {"✳️base": [], "✳️cobie": [], "✳️cv20": [], "✳️sav": [], "4-any": []}

    base_root = f"{ROOT_2X3}/✳️base"
    fragments["✳️base"].append(ifc_pair(base_root, "upsert-instance", FIXTURE_2X3_BASE, base_upsert_instance))
    fragments["✳️base"].append(ifc_pair(base_root, "remove-instance", FIXTURE_2X3_BASE, base_remove_instance))
    fragments["✳️base"].append(ifc_pair(base_root, "set-header", FIXTURE_2X3_BASE, base_set_header))

    cobie_root = f"{ROOT_2X3}/✳️cobie"
    fragments["✳️cobie"].append(ifc_pair(cobie_root, "set-facility-name", FIXTURE_2X3_COBIE, cobie_set_facility_name))
    fragments["✳️cobie"].append(ifc_pair(cobie_root, "set-floor-elevation", FIXTURE_2X3_COBIE, cobie_set_floor_elevation))
    fragments["✳️cobie"].append(ifc_pair(cobie_root, "set-space", FIXTURE_2X3_COBIE, cobie_set_space))
    fragments["✳️cobie"].append(ifc_pair(cobie_root, "set-type-assignment", FIXTURE_2X3_COBIE, cobie_set_type_assignment))
    fragments["✳️cobie"].append(ifc_pair(cobie_root, "set-view-definition", FIXTURE_2X3_COBIE, cobie_set_view_definition))
    fragments["✳️cobie"].append(ifc_pair(cobie_root, "set-snapshot", FIXTURE_2X3_COBIE, cobie_set_snapshot))

    cv20_root = f"{ROOT_2X3}/✳️cv20"
    fragments["✳️cv20"].append(ifc_pair(cv20_root, "set-structural-entity", FIXTURE_2X3_CV20, cv20_set_structural_entity))
    fragments["✳️cv20"].append(ifc_pair(cv20_root, "set-project-units", FIXTURE_2X3_CV20, cv20_set_project_units))
    fragments["✳️cv20"].append(ifc_pair(cv20_root, "set-product-placement", FIXTURE_2X3_CV20, cv20_set_product_placement))
    fragments["✳️cv20"].append(ifc_pair(cv20_root, "set-view-definition", FIXTURE_2X3_CV20, cv20_set_view_definition))
    fragments["✳️cv20"].append(ifc_pair(cv20_root, "set-snapshot", FIXTURE_2X3_CV20, cv20_set_snapshot))

    sav_root = f"{ROOT_2X3}/✳️sav"
    fragments["✳️sav"].append(ifc_pair(sav_root, "set-analysis-model", FIXTURE_2X3_SAV, sav_set_analysis_model))
    fragments["✳️sav"].append(ifc_pair(sav_root, "set-group-assignment", FIXTURE_2X3_SAV, sav_set_group_assignment))
    fragments["✳️sav"].append(ifc_pair(sav_root, "set-load-group", FIXTURE_2X3_SAV, sav_set_load_group))
    fragments["✳️sav"].append(ifc_pair(sav_root, "set-view-definition", FIXTURE_2X3_SAV, sav_set_view_definition))
    fragments["✳️sav"].append(ifc_pair(sav_root, "set-snapshot", FIXTURE_2X3_SAV, sav_set_snapshot))

    any_root = f"{ROOT_4}/✳️any"
    for mutation_id, transform in IFC4_TRANSFORMS.items():
        fragments["4-any"].append(step_pair(any_root, mutation_id, FIXTURE_4_ANY, transform))

    for subset, items in fragments.items():
        for item in items:
            b = item["files"][0]["bytes"]
            a = item["files"][1]["bytes"]
            print(f"{subset}/{item['id']} [{item['generator_kind']}]: before={b}B after={a}B")

    out = os.path.join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/🗑️generated/f1-ifc-fragments.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w") as f:
        json.dump(fragments, f, indent=2, ensure_ascii=False)
    print("wrote", out)


if __name__ == "__main__":
    main()
