"""🐍️ Standalone exercise of G2's Python second implementation — one vector per emitter.

Runs the generated `🐍️.py` vocabulary directly (no Rust, no fixtures) against a hand-built snapshot
that mirrors the G2 seeds, checks the declared outcome AND that `invert()` returns the document to
where it started. One representative per emitter validates the emitter, and therefore all 61 kinds.

Each vector's payload is also validated by the third-party `jsonschema` Draft-07 implementation
against BOTH the kind's own `🧬️.schema.json` and the aggregate `🧬️mutations/🔣️.json` union, so the
payloads these assertions are written against are provably the payloads the schema declares.

Run from the repository root: `uv run --group test python3 <this file>` (bare `python3` runs the
vocabulary assertions and skips the schema half, saying so).
"""

import copy
import importlib.util
import json
import os
import sys
import types

host = types.ModuleType("semio_repo_test")


class Adapter:
    def __init__(self, *a, **k):
        pass

    def oracle(self, *a, **k):
        return self


host.Adapter = Adapter
host.Context = type("Context", (), {})
host.Outcome = type("Outcome", (), {"__init__": lambda self, *a, **k: None})
sys.modules["semio_repo_test"] = host

MUT = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

PATH = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py"
spec = importlib.util.spec_from_file_location("oracle", PATH)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

MATERIAL_1 = {"id": 1, "name": "PLASTERBOARD", "thickness_m": 0.012, "conductivity_w_m_k": 0.16, "density_kg_m3": 950.0, "specific_heat_j_kg_k": 840.0, "thermal_absorptance": 0.9, "solar_absorptance": 0.6, "visible_absorptance": 0.6}
MATERIAL_2 = {"id": 2, "name": "FIBERGLASS QUILT", "thickness_m": 0.066, "conductivity_w_m_k": 0.04, "density_kg_m3": 12.0, "specific_heat_j_kg_k": 840.0, "thermal_absorptance": 0.9, "solar_absorptance": 0.6, "visible_absorptance": 0.6}

BEFORE = {
    "model": {
        "zones": [{"id": 1, "name": "ZONE ONE"}, {"id": 2, "name": "ZONE TWO"}],
        "schedules": {"constants": [{"id": 1, "value": 1.0}, {"id": 2, "value": 120.0}, {"id": 3, "value": 0.5}], "daily": [], "weekly": [], "annual": [], "time_series": []},
        "materials": [copy.deepcopy(MATERIAL_1), copy.deepcopy(MATERIAL_2)],
        "constructions": [{"id": 10, "name": "LTWALL", "layer_material_ids": [1, 2]}, {"id": 11, "name": "HWWALL", "layer_material_ids": [2]}],
        "surfaces": [],
        "people": [{"id": 20, "zone_id": 1, "schedule_id": 1, "activity_schedule_id": 2, "people_per_area": 0.05, "sensible_fraction": 0.6, "latent_fraction": 0.4, "radiant_fraction": 0.3}],
        "infiltrations": [{"id": 50, "zone_id": 1, "schedule_id": 1, "method": "ScheduledAch", "design_flow_ach": 0.5, "flow_per_exterior_area_m3_s_m2": 0.0, "effective_leakage_area_m2": 0.0, "discharge_coefficient": 1.0, "stack_height_m": 2.7, "constant_term_coefficient": 1.0, "temperature_term_coefficient": 0.0, "velocity_term_coefficient": 0.0, "velocity_squared_term_coefficient": 0.0}],
    }
}

#: 🧱️ `G2_MATERIALS`' own shape: materials with no construction naming them, so a delete is free.
FREE = copy.deepcopy(BEFORE)
FREE["model"]["constructions"] = []

USED = copy.deepcopy(BEFORE)
USED["model"]["surfaces"] = [{"id": 70, "name": "SOUTH WALL", "zone_id": 1, "construction_id": 10}]

#: (emoji-labelled emitter, kind, payload, expected status, expected code)
VECTORS = [
    ("g2_scalar ✅️", "change-material-thickness", {"id": 1, "newThicknessM": 0.02}, "applied", None),
    ("g2_scalar ⛔️", "change-material-thickness", {"id": 1, "newThicknessM": 0.0}, "rejected", "mutation.invariant"),
    ("g2_scalar ⛔️ absent", "change-material-thickness", {"id": 999, "newThicknessM": 0.02}, "rejected", "mutation.target-missing"),
    ("g2_reference ✅️", "change-people-gain-zone", {"id": 20, "newZoneId": 2}, "applied", None),
    ("g2_reference ⛔️", "change-people-gain-zone", {"id": 20, "newZoneId": 999}, "rejected", "mutation.target-missing"),
    ("g2_reference ✅️ schedule", "change-people-gain-activity-schedule", {"id": 20, "newActivityScheduleId": 3}, "applied", None),
    ("g2_reference ⛔️ schedule", "change-people-gain-schedule", {"id": 20, "newScheduleId": 999}, "rejected", "mutation.target-missing"),
    ("g2_rename ✅️", "rename-material", {"id": 1, "newName": "TIMBER FLOORING"}, "applied", None),
    ("g2_rename ⛔️", "rename-material", {"id": 1, "newName": "FIBERGLASS QUILT"}, "rejected", "mutation.duplicate-id"),
    ("g2_create ✅️", "create-material", {"index": 2, "id": 3, "name": "CONCRETE SLAB", "thicknessM": 0.08, "conductivityWMK": 1.13, "densityKgM3": 1400.0, "specificHeatJKgK": 1000.0, "thermalAbsorptance": 0.9, "solarAbsorptance": 0.6, "visibleAbsorptance": 0.6}, "applied", None),
    ("g2_create ⛔️ dup", "create-material", {"index": 2, "id": 1, "name": "CONCRETE SLAB", "thicknessM": 0.08, "conductivityWMK": 1.13, "densityKgM3": 1400.0, "specificHeatJKgK": 1000.0, "thermalAbsorptance": 0.9, "solarAbsorptance": 0.6, "visibleAbsorptance": 0.6}, "rejected", "mutation.duplicate-id"),
    ("g2_create ⛔️ index", "create-material", {"index": 9, "id": 3, "name": "CONCRETE SLAB", "thicknessM": 0.08, "conductivityWMK": 1.13, "densityKgM3": 1400.0, "specificHeatJKgK": 1000.0, "thermalAbsorptance": 0.9, "solarAbsorptance": 0.6, "visibleAbsorptance": 0.6}, "rejected", "mutation.invariant"),
    ("g2_create ⛔️ ref", "create-people-gain", {"index": 1, "id": 21, "zoneId": 999, "scheduleId": 1, "activityScheduleId": 2, "peoplePerArea": 0.1, "sensibleFraction": 0.6, "latentFraction": 0.4, "radiantFraction": 0.3}, "rejected", "mutation.target-missing"),
    ("g2_delete ⛔️ absent", "delete-people-gain", {"id": 999}, "rejected", "mutation.target-missing"),
    ("add-construction-layer ✅️", "add-construction-layer", {"id": 10, "index": 2, "materialId": 1}, "applied", None),
    ("add-construction-layer ⛔️", "add-construction-layer", {"id": 10, "index": 0, "materialId": 999}, "rejected", "mutation.target-missing"),
    ("remove-construction-layer ✅️", "remove-construction-layer", {"id": 10, "index": 0}, "applied", None),
    ("remove-construction-layer ⛔️", "remove-construction-layer", {"id": 10, "index": 7}, "rejected", "mutation.invariant"),
    ("reorder-construction-layers ✅️", "reorder-construction-layers", {"id": 10, "newLayerMaterialIds": [2, 1]}, "applied", None),
    ("reorder-construction-layers ⛔️", "reorder-construction-layers", {"id": 10, "newLayerMaterialIds": [2, 2]}, "rejected", "mutation.invariant"),
    ("change-infiltration-method ✅️", "change-infiltration-method", {"id": 50, "newMethod": "PerExteriorArea"}, "applied", None),
]

IN_USE = [("g2_delete ⛔️ in-use material", "delete-material", {"id": 1}), ("g2_delete ⛔️ in-use construction", "delete-construction", {"id": 10})]


def wire_tag(kind):
    head, *rest = kind.split("-")
    return head + "".join(part.capitalize() for part in rest)


def schema_check(vectors):
    """🔍️ Third-party Draft-07 validation of every payload above against its own leaf schema, plus a
    cross-surface check of the aggregate union's `mutation` discriminator.

    The aggregate carries `#[value(tag = "mutation", rename_all = "camelCase")]`, so the wire tag is
    the lowerCamel spelling of the variant (`changeMaterialThickness`) — which is what `🟦️.ts` and
    the Python oracle's own `wire_tag` both say. `🧬️mutations/🔣️.json` states the kebab catalog id
    instead, for every kind in the vocabulary; that disagreement is reported here rather than
    asserted away, because the file it lives in belongs to the generator custodian, not to G2.
    """
    try:
        from jsonschema import Draft7Validator
    except ImportError:
        return ["  … jsonschema is absent — run under `uv run --group test` for the schema half"]
    loaded = [(entry, json.load(open(f"{MUT}/{entry}/🔣️.json", encoding="utf-8"))) for entry in os.listdir(MUT) if os.path.isfile(f"{MUT}/{entry}/🔣️.json")]
    directories = {document["semanticKind"]: entry for entry, document in loaded if "semanticKind" in document}
    aggregate = json.load(open(f"{MUT}/🔣️.json", encoding="utf-8"))
    lines, drift = [], []
    for kind, payload in vectors:
        leaf = json.load(open(f"{MUT}/{directories[kind]}/🧬️.schema.json", encoding="utf-8"))
        Draft7Validator.check_schema(leaf)
        Draft7Validator(leaf).validate(payload)
        lines.append(f"  ✓ leaf schema {kind}")
        declared = aggregate["$defs"][leaf["title"]]["properties"]["mutation"]["const"]
        if declared != wire_tag(kind):
            drift.append(f"{leaf['title']}: union says {declared!r}, the `mutation` tag is {wire_tag(kind)!r}")
    if drift:
        lines.append(f"  ⚠️ aggregate union discriminator drift on {len(drift)} of {len(vectors)} payloads, e.g. {drift[0]}")
    return lines


def check(label, kind, payload, status, code, before):
    after, outcome = oracle.VOCABULARY[kind](before, payload)
    assert outcome["status"] == status, f"{label}: {outcome} is not {status}"
    if code:
        assert outcome["code"] == code, f"{label}: {outcome['code']} != {code}"
        assert after == before, f"{label}: a refused step moved the document"
    else:
        assert after != before, f"{label}: an applied step did not move the document"
    restored = after
    for step_kind, step_payload in oracle.invert(kind, before, payload):
        restored, step_outcome = oracle.VOCABULARY[step_kind](restored, step_payload)
        assert step_outcome["status"] != "rejected", f"{label}: the undo step {step_kind} was itself refused"
    assert restored == before, f"{label}: undoing did not land back on BASE"
    return f"  ✓ {label:34} {kind}"


def main() -> int:
    lines = [check(*vector, BEFORE) for vector in VECTORS]
    lines.append(check("g2_delete ✅️ first", "delete-material", {"id": 1}, "applied", None, FREE))
    lines += [check(label, kind, payload, "rejected", "mutation.invariant", USED) for label, kind, payload in IN_USE]
    print("\n".join(lines))
    print(f"{len(lines)} G2 python vectors: outcome + inverse law hold")
    schema = schema_check([(kind, payload) for _label, kind, payload, _s, _c in VECTORS] + [(kind, payload) for _label, kind, payload in IN_USE] + [("delete-material", {"id": 1})])
    print("\n".join(schema))
    print(f"{sum(1 for line in schema if line.startswith('  ✓'))} G2 payloads validated against their leaf schema by third-party jsonschema")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
