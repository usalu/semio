"""🧪️ G3 — exercises the generated Python second implementation on every G3 vector without Rust.

The committed `(before, mutation, after, diff, outcome)` quintets are still `{}` stubs because the
energy crate has never compiled, so `SEMIO_ENERGY_WRITE_FIXTURES=1` has never run. This harness
rebuilds each vector's `before` document and its payload by parsing the SAME typed scenario the
generator emits into the Rust fixture case, converts the Rust literals to their JSON forms, and then
runs `🐍️.py`'s own vocabulary and `invert` over them: forward must reach an applied outcome, the
refusal vector must really be refused, and applying the kind's own inverse must land back on `before`.
It judges only the Python side — the Rust side is judged by `cargo test` — but it catches every wire
name, camelCase and key-shape mistake the differential comparison would otherwise only surface after a
full crate build.
"""

from __future__ import annotations

import copy
import importlib.util
import json
import os
import re
import sys
import types

ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END")
ORACLE = os.path.join(ROOT, "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py")

COLLECTIONS = [
    "zones", "spaces", "surfaces", "fenestrations", "materials", "constructions", "people", "lighting",
    "equipment", "thermostats", "humidistats", "setpoint_managers", "ideal_loads", "zone_equipment",
    "air_loops", "plant_loops", "outdoor_air_systems", "infiltrations", "mechanical_ventilations",
    "shading_surfaces", "space_lists", "thermal_enclosures", "adjacency_pairs",
    "electrical_load_centers", "pv_systems", "battery_storage", "shw_systems", "solar_thermal_systems",
    "refrigeration_systems", "water_systems", "faults", "output_variables", "sizing_objects",
    "daylight_zones", "room_air_models",
]


def stub_semio_repo_test() -> None:
    """🧩️ The generated oracle imports its harness types; only their names are needed here."""
    module = types.ModuleType("semio_repo_test")

    class Adapter:
        def __init__(self, name):
            self.name = name

        def oracle(self, *_args, **_kwargs):
            return self

    module.Adapter = Adapter
    module.Context = object
    module.Outcome = object
    sys.modules["semio_repo_test"] = module


def load(path: str, name: str):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def split_args(text: str) -> list[str]:
    """✂️ Splits a Rust argument list on top-level commas."""
    parts, depth, current = [], 0, ""
    for character in text:
        if character in "([{":
            depth += 1
        elif character in ")]}":
            depth -= 1
        if character == "," and depth == 0:
            parts.append(current.strip())
            current = ""
            continue
        current += character
    if current.strip():
        parts.append(current.strip())
    return parts


def literal(text: str):
    """🔤 One Rust literal as the JSON value the snapshot and payload schemas carry."""
    text = text.strip()
    if text in ("true", "false"):
        return text == "true"
    if text == "None":
        return None
    if text in ("Vec::new()", "vec![]"):
        return []
    match = re.fullmatch(r"(?:crate::model::)?(?:EntityId|ScheduleId)\((\d+)\)", text)
    if match:
        return int(match.group(1))
    match = re.fullmatch(r"Some\((.*)\)", text)
    if match:
        return literal(match.group(1))
    match = re.fullmatch(r'"([^"]*)"\.(?:to_string|into)\(\)', text)
    if match:
        return match.group(1)
    match = re.fullmatch(r'"([^"]*)"', text)
    if match:
        return match.group(1)
    match = re.fullmatch(r"vec!\[(.*)\]", text, re.S)
    if match:
        return [literal(part) for part in split_args(match.group(1))]
    match = re.fullmatch(r"crate::model::\w+::(\w+)", text)
    if match:
        return match.group(1)
    match = re.fullmatch(r"-?\d+\.\d+(?:e-?\d+)?", text)
    if match:
        return float(text)
    match = re.fullmatch(r"-?\d+", text)
    if match:
        return int(text)
    raise AssertionError(f"unconverted Rust literal {text!r}")


def struct_fields(body: str) -> dict:
    values = {}
    for part in split_args(body):
        name, _, value = part.partition(":")
        values[name.strip()] = literal(value)
    return values


def build_before(scenario: str) -> dict:
    """📸️ The `before` document the typed scenario builds, in its persisted JSON shape."""
    model = {name: [] for name in COLLECTIONS}
    model["schedules"] = {"constants": [], "daily": [], "weekly": [], "annual": [], "time_series": []}
    for number, name in re.findall(r'zone\((\d+),\s*"([^"]*)"\)', scenario):
        model["zones"].append({"id": int(number), "name": name, "volume_m3": 129.6, "multiplier": 1, "conditioned": True, "part_of_total_floor_area": True})
    for family, body in re.findall(r"model\.schedules\.(\w+)\.push\(crate::schedule::\w+\s*\{(.*?)\}\s*\);", scenario, re.S):
        model["schedules"][family].append(struct_fields(body))
    for collection, body in re.findall(r"model\.(\w+)\.push\(crate::model::\w+\s*\{(.*?)\}\s*\);", scenario, re.S):
        model[collection].append(struct_fields(body))
    return {"model": model}


def build_payload(kind, scenario: str) -> dict:
    call = re.search(r"super::(\w+)\((.*)\)\)\s*$", scenario, re.S)
    assert call, scenario
    arguments = split_args(call.group(2))
    assert len(arguments) == len(kind.fields), (kind.slug, arguments, kind.fields)
    return {field_camel(name): literal(value) for (name, _ty), value in zip(kind.fields, arguments)}


def main() -> int:
    os.chdir(ROOT)
    stub_semio_repo_test()
    generator = load(os.path.join(TICKET, "🐍️generate-mutation-leaves.py"), "generator")
    globals()["field_camel"] = generator.field_camel
    oracle = load(ORACLE, "energy_oracle")
    kinds = [k for k in generator.KINDS if 500 <= k.number <= 699]
    failures, applied_count, rejected_count = [], 0, 0
    for kind in kinds:
        assert kind.slug in oracle.VOCABULARY, f"{kind.slug} is missing from VOCABULARY"
        for emoji, name, _description, scenario in kind.cases:
            label = f"{kind.slug}/{emoji}{name}"
            try:
                before = build_before(scenario)
                payload = build_payload(kind, scenario)
                after, outcome = oracle.VOCABULARY[kind.slug](before, payload)
            except Exception as error:
                failures.append(f"{label}: raised {type(error).__name__}: {error}")
                continue
            if emoji.startswith("⛔"):
                rejected_count += 1
                if outcome["status"] != "rejected":
                    failures.append(f"{label}: expected a refusal, got {json.dumps(outcome)}")
                elif after != before:
                    failures.append(f"{label}: a refused step moved the document")
                continue
            applied_count += 1
            if outcome["status"] != "applied":
                failures.append(f"{label}: expected an application, got {json.dumps(outcome)}")
                continue
            if after == before:
                failures.append(f"{label}: the happy vector did not move the document")
                continue
            restored = copy.deepcopy(after)
            try:
                steps = oracle.invert(kind.slug, before, payload)
            except Exception as error:
                failures.append(f"{label}: invert raised {type(error).__name__}: {error}")
                continue
            if not steps:
                failures.append(f"{label}: an applied step owes an undo but offered none")
                continue
            for step_kind, step_payload in steps:
                restored, step_outcome = oracle.VOCABULARY[step_kind](restored, step_payload)
                if step_outcome["status"] == "rejected":
                    failures.append(f"{label}: the undo step {step_kind} was itself refused ({step_outcome['code']})")
                    break
            else:
                if restored != before:
                    failures.append(f"{label}: undoing did not land back on before")
    print(f"{len(kinds)} G3 kinds, {applied_count} happy vectors, {rejected_count} refusal vectors")
    for failure in failures:
        print("FAIL", failure)
    print("PASS" if not failures else f"{len(failures)} FAILURES")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
