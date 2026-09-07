"""🐍️ G1 dry-run of the second implementation, independent of the (currently un-buildable) Rust.

Exercises every G1 kind's forward step and its own `invert` on a synthetic snapshot document shaped
like the committed vectors, asserting: the happy vector moves the document, undo restores it exactly,
and the refusal vector leaves it untouched with a frozen code. Proves the Python is OBSERVABLE.
"""

import copy
import importlib.util
import os
import sys
import types

ROOT = "/Users/ueli/Documents/semio"
ORACLE = os.path.join(ROOT, "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py")
GEN = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/🐍️generate-mutation-leaves.py")

stub = types.ModuleType("semio_repo_test")
stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a, **k: self})
stub.Context = object
stub.Outcome = object
sys.modules["semio_repo_test"] = stub

spec = importlib.util.spec_from_file_location("energy_oracle", ORACLE)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

spec2 = importlib.util.spec_from_file_location("gen", GEN)
gen = importlib.util.module_from_spec(spec2)
spec2.loader.exec_module(gen)

FROZEN = {"mutation.target-missing", "mutation.no-op", "mutation.partial", "mutation.clamped", "mutation.duplicate-id", "mutation.invariant", "mutation.cascade"}

EMPTY_COLLECTIONS = ["zones", "spaces", "surfaces", "fenestrations", "materials", "constructions", "people", "lighting", "equipment", "thermostats", "humidistats", "setpoint_managers", "ideal_loads", "zone_equipment", "air_loops", "plant_loops", "outdoor_air_systems", "infiltrations", "mechanical_ventilations", "shading_surfaces", "space_lists", "thermal_enclosures", "adjacency_pairs", "electrical_load_centers", "pv_systems", "battery_storage", "shw_systems", "solar_thermal_systems", "refrigeration_systems", "water_systems", "faults", "output_variables", "sizing_objects", "daylight_zones", "room_air_models"]


def zone(entity_id, name):
    return {"id": entity_id, "name": name, "volume_m3": 129.6, "multiplier": 1, "conditioned": True, "part_of_total_floor_area": True}


def surface(entity_id, name, zone_id, construction_id, boundary="OutdoorAir"):
    return {"id": entity_id, "name": name, "zone_id": zone_id, "class": "ExteriorWall", "vertices_m": [[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]], "construction_id": construction_id, "outside_boundary_condition": boundary, "sun_exposed": True, "wind_exposed": True, "multiplier": 1}


def window(entity_id, name, surface_id, glazing=None):
    return {"id": entity_id, "name": name, "surface_id": surface_id, "u_value_w_m2k": 3.0, "shgc": 0.787, "vlt": 0.86, "area_m2": 6.0, "height_m": 2.0, "sill_height_m": 0.5, "frame_conductance_w_k": 0.0, "divider_conductance_w_k": 0.0, "overhang_depth_m": 0.0, "overhang_offset_m": 0.0, "fin_depth_m": 0.0, "fin_offset_m": 0.0, "glazing_construction_id": glazing}


def document():
    model = {name: [] for name in EMPTY_COLLECTIONS}
    model.update({"name": "BESTEST 600", "version": "1", "site": {}, "airflow_network": None, "ground_temperature": {}, "run_period": {}, "schedules": {"constants": [{"id": 1, "value": 0.2}], "daily": [], "weekly": [], "annual": [], "time_series": []}})
    model["zones"] = [zone(1, "ZONE ONE"), zone(7, "ZONE TWO")]
    model["spaces"] = [{"id": 2, "name": "SPACE ONE", "zone_id": 1, "floor_area_m2": 48.0}]
    model["materials"] = [{"id": 4, "name": "PLASTERBOARD"}]
    model["constructions"] = [{"id": 2, "name": "LIGHTWEIGHT WALL"}, {"id": 8, "name": "HEAVYWEIGHT WALL"}]
    model["surfaces"] = [surface(3, "WALL SOUTH", 1, 2), surface(6, "WALL NORTH", 1, 2)]
    model["fenestrations"] = [window(5, "WINDOW SOUTH", 3)]
    model["shading_surfaces"] = [{"id": 11, "name": "SITE AWNING", "vertices_m": [[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]], "transmittance_schedule_id": None}]
    return {"schema": "s.energy.model", "model": model, "structure": {}, "zones": {}, "referencedModel": None, "weatherLink": None}


VERTICES = [[0.0, 0.0, 0.0], [6.0, 0.0, 0.0], [6.0, 0.0, 2.7]]

#: 🎯 (slug, happy payload, refusal payload, expected refusal code)
CASES = [
    ("create-zone", {"id": 9, "name": "ZONE NINE", "volumeM3": 100.0, "multiplier": 1, "conditioned": True, "partOfTotalFloorArea": True}, {"id": 1, "name": "ZONE NINE", "volumeM3": 100.0, "multiplier": 1, "conditioned": True, "partOfTotalFloorArea": True}, "mutation.duplicate-id"),
    ("delete-zone", {"id": 7}, {"id": 1}, "mutation.invariant"),
    ("create-space", {"id": 12, "name": "SPACE TWO", "zoneId": 1, "floorAreaM2": 20.0}, {"id": 12, "name": "SPACE TWO", "zoneId": 99, "floorAreaM2": 20.0}, "mutation.target-missing"),
    ("delete-space", {"id": 2}, {"id": 99}, "mutation.target-missing"),
    ("rename-space", {"id": 2, "newName": "SPACE 1"}, {"id": 2, "newName": "  "}, "mutation.invariant"),
    ("change-space-floor-area", {"id": 2, "newFloorAreaM2": 96.0}, {"id": 2, "newFloorAreaM2": -1.0}, "mutation.invariant"),
    ("change-space-zone", {"id": 2, "newZoneId": 7}, {"id": 2, "newZoneId": 99}, "mutation.target-missing"),
    ("create-surface", {"id": 13, "name": "WALL EAST", "zoneId": 1, "class": "ExteriorWall", "verticesM": VERTICES, "constructionId": 2, "boundary": "OutdoorAir", "interzoneSurfaceId": None, "sunExposed": True, "windExposed": True, "multiplier": 1}, {"id": 13, "name": "WALL EAST", "zoneId": 1, "class": "ExteriorWall", "verticesM": VERTICES[:2], "constructionId": 2, "boundary": "OutdoorAir", "interzoneSurfaceId": None, "sunExposed": True, "windExposed": True, "multiplier": 1}, "mutation.invariant"),
    ("delete-surface", {"id": 3}, {"id": 99}, "mutation.target-missing"),
    ("rename-surface", {"id": 3, "newName": "WALL S"}, {"id": 3, "newName": "WALL NORTH"}, "mutation.duplicate-id"),
    ("change-surface-zone", {"id": 3, "newZoneId": 7}, {"id": 3, "newZoneId": 99}, "mutation.target-missing"),
    ("change-surface-class", {"id": 3, "newClass": "Roof"}, {"id": 99, "newClass": "Roof"}, "mutation.target-missing"),
    ("replace-surface-vertices", {"id": 3, "newVerticesM": VERTICES}, {"id": 3, "newVerticesM": VERTICES[:2]}, "mutation.invariant"),
    ("change-surface-construction", {"id": 3, "newConstructionId": 8}, {"id": 3, "newConstructionId": 99}, "mutation.target-missing"),
    ("change-surface-boundary-condition", {"id": 3, "newBoundary": "Ground", "newInterzoneSurfaceId": None}, {"id": 3, "newBoundary": "Interzone", "newInterzoneSurfaceId": None}, "mutation.invariant"),
    ("change-surface-sun-exposed", {"id": 3, "newSunExposed": False}, {"id": 99, "newSunExposed": False}, "mutation.target-missing"),
    ("change-surface-wind-exposed", {"id": 3, "newWindExposed": False}, {"id": 99, "newWindExposed": False}, "mutation.target-missing"),
    ("change-surface-multiplier", {"id": 3, "newMultiplier": 4}, {"id": 3, "newMultiplier": 0}, "mutation.invariant"),
    ("create-fenestration", {"id": 14, "name": "WINDOW NORTH", "surfaceId": 6, "uValueWM2k": 3.0, "shgc": 0.787, "vlt": 0.86, "areaM2": 6.0, "heightM": 2.0, "sillHeightM": 0.5, "frameConductanceWK": 0.0, "dividerConductanceWK": 0.0, "overhangDepthM": 0.0, "overhangOffsetM": 0.0, "finDepthM": 0.0, "finOffsetM": 0.0, "glazingConstructionId": None}, {"id": 14, "name": "WINDOW NORTH", "surfaceId": 99, "uValueWM2k": 3.0, "shgc": 0.787, "vlt": 0.86, "areaM2": 6.0, "heightM": 2.0, "sillHeightM": 0.5, "frameConductanceWK": 0.0, "dividerConductanceWK": 0.0, "overhangDepthM": 0.0, "overhangOffsetM": 0.0, "finDepthM": 0.0, "finOffsetM": 0.0, "glazingConstructionId": None}, "mutation.target-missing"),
    ("delete-fenestration", {"id": 5}, {"id": 99}, "mutation.target-missing"),
    ("rename-fenestration", {"id": 5, "newName": "WINDOW S"}, {"id": 5, "newName": " "}, "mutation.invariant"),
    ("change-fenestration-surface", {"id": 5, "newSurfaceId": 6}, {"id": 5, "newSurfaceId": 99}, "mutation.target-missing"),
    ("change-fenestration-u-value", {"id": 5, "newUValueWM2k": 2.74}, {"id": 5, "newUValueWM2k": 0.0}, "mutation.invariant"),
    ("change-fenestration-shgc", {"id": 5, "newShgc": 0.76}, {"id": 5, "newShgc": 1.4}, "mutation.invariant"),
    ("change-fenestration-vlt", {"id": 5, "newVlt": 0.84}, {"id": 5, "newVlt": -0.1}, "mutation.invariant"),
    ("change-fenestration-area", {"id": 5, "newAreaM2": 12.0}, {"id": 5, "newAreaM2": 0.0}, "mutation.invariant"),
    ("change-fenestration-frame-conductance", {"id": 5, "newFrameConductanceWK": 1.2}, {"id": 5, "newFrameConductanceWK": -1.0}, "mutation.invariant"),
    ("change-fenestration-divider-conductance", {"id": 5, "newDividerConductanceWK": 0.4}, {"id": 5, "newDividerConductanceWK": -1.0}, "mutation.invariant"),
    ("create-shading-surface", {"id": 15, "name": "TREE", "verticesM": VERTICES, "transmittanceScheduleId": None}, {"id": 15, "name": "TREE", "verticesM": VERTICES, "transmittanceScheduleId": 99}, "mutation.target-missing"),
    ("delete-shading-surface", {"id": 11}, {"id": 99}, "mutation.target-missing"),
    ("rename-shading-surface", {"id": 11, "newName": "AWNING"}, {"id": 11, "newName": " "}, "mutation.invariant"),
    ("replace-shading-surface-vertices", {"id": 11, "newVerticesM": VERTICES}, {"id": 11, "newVerticesM": VERTICES[:2]}, "mutation.invariant"),
    ("change-shading-surface-transmittance-schedule", {"id": 11, "newTransmittanceScheduleId": 1}, {"id": 11, "newTransmittanceScheduleId": 99}, "mutation.target-missing"),
    ("connect-surfaces", {"surfaceAId": 3, "surfaceBId": 6}, {"surfaceAId": 3, "surfaceBId": 3}, "mutation.invariant"),
    ("disconnect-surfaces", {"surfaceAId": 3, "surfaceBId": 6}, {"surfaceAId": 3, "surfaceBId": 99}, "mutation.target-missing"),
    ("bind-fenestration-glazing-construction", {"id": 5, "constructionId": 8}, {"id": 5, "constructionId": 99}, "mutation.target-missing"),
    ("clear-fenestration-glazing-construction", {"id": 5}, {"id": 5}, "mutation.target-missing"),
    ("change-fenestration-height", {"id": 5, "newHeightM": 2.4}, {"id": 5, "newHeightM": 0.0}, "mutation.invariant"),
    ("change-fenestration-sill-height", {"id": 5, "newSillHeightM": 0.9}, {"id": 5, "newSillHeightM": -0.2}, "mutation.invariant"),
    ("change-fenestration-overhang-depth", {"id": 5, "newOverhangDepthM": 1.0}, {"id": 5, "newOverhangDepthM": -1.0}, "mutation.invariant"),
    ("change-fenestration-overhang-offset", {"id": 5, "newOverhangOffsetM": 0.5}, {"id": 5, "newOverhangOffsetM": -0.5}, "mutation.invariant"),
    ("change-fenestration-fin-depth", {"id": 5, "newFinDepthM": 1.0}, {"id": 5, "newFinDepthM": -1.0}, "mutation.invariant"),
    ("change-fenestration-fin-offset", {"id": 5, "newFinOffsetM": 0.2}, {"id": 5, "newFinOffsetM": -0.2}, "mutation.invariant"),
]

#: 🎬️ Documents a kind needs beyond the shared one (a bound glazing slot, an existing adjacency pair).
def seeded(slug):
    before = document()
    if slug in ("disconnect-surfaces",):
        before["model"]["adjacency_pairs"] = [{"surface_a_id": 3, "surface_b_id": 6}]
    if slug == "clear-fenestration-glazing-construction":
        before["model"]["fenestrations"] = [window(5, "WINDOW SOUTH", 3, glazing=8)]
    return before


def main():
    g1 = {k.slug for k in gen.KINDS if 105 <= k.number < 300}
    covered = {slug for slug, *_ in CASES}
    missing = g1 - covered
    assert not missing, f"G1 kinds with no dry-run vector: {sorted(missing)}"
    moved = 0
    for slug, happy, refusal, code in CASES:
        handler = oracle.VOCABULARY[slug]
        before = seeded(slug)
        after, outcome = handler(before, happy)
        assert outcome["status"] == "applied", f"{slug}: happy vector was refused: {outcome}"
        assert after != before, f"{slug}: happy vector did not move the document (a stub would look like this)"
        assert before == seeded(slug), f"{slug}: the handler mutated its input document"
        restored = after
        for step_slug, step_payload in oracle.invert(slug, before, happy):
            restored, step_outcome = oracle.VOCABULARY[step_slug](restored, step_payload)
            assert step_outcome["status"] != "rejected", f"{slug}: undo step {step_slug} was itself refused: {step_outcome}"
        assert restored == before, f"{slug}: undo did not restore the document\n  before={before['model']}\n  after ={restored['model']}"
        moved += 1

        refused_before = seeded(slug)
        if slug == "clear-fenestration-glazing-construction":
            refused_before["model"]["fenestrations"] = [window(5, "WINDOW SOUTH", 3, glazing=None)]
        refused_after, refused_outcome = handler(refused_before, refusal)
        assert refused_outcome["status"] == "rejected", f"{slug}: refusal vector was accepted: {refused_outcome}"
        assert refused_outcome["code"] == code, f"{slug}: refusal reported {refused_outcome['code']!r}, expected {code!r}"
        assert refused_outcome["code"] in FROZEN, f"{slug}: {refused_outcome['code']!r} is not a frozen code"
        assert refused_after == refused_before, f"{slug}: a refused step moved the document"
        assert oracle.invert(slug, refused_before, refusal) == [], f"{slug}: a refused step owes undo steps"

    print(f"G1 python second implementation: {moved}/{len(g1)} kinds move the document, undo restores it exactly, and every refusal reports a frozen code and leaves the document untouched")


main()
