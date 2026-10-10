#!/usr/bin/env python3
"""🏷️ Third-party ORACLE for the `s.bim.model@1` inference `🏷️effective-properties`.

The subject (Rust, `semio-s-artifact-bim-model`) derives what an element really has: the value it states itself, else the value of its type, else the default of the
property set template that applies to its kind, plus a finding for every own value that breaks its definition and for every required property without any value.
This file reproduces the table from the SAME committed snapshot with a library that has never seen this repository: `jsonschema` (Draft 2020-12). Every property
definition of a template becomes a JSON Schema (`type` string, number, integer or boolean; `minimum`, `maximum`, `enum`) and the validator, not this repository,
decides whether a value is allowed. A value whose kind tag differs from the kind of its definition is a kind mismatch before the schema is asked (the schema of an
integer accepts `50.0`, the property kind does not).

The rules are restated here from the authored records alone: a template applies to the records of the kinds it lists; a holder is every record that states properties
or falls under a template, and every instance of a holder type; an instance inherits the effective properties of its type, a value it states itself wins, a default
of a template fills what is still empty; an inherited value is never re-reported at its instance; a type is also checked against the templates of the kind it types
(reported once when a template lists both kinds).

Audits beyond the table: the schema of every definition is itself a valid schema, every default value satisfies its own definition, an own value has a finding exactly
when the validator rejects it, a required property without a value has exactly one finding, and no value with the source `Type` or `Default` carries a finding.

The committed expectations under `🧫️fixtures/💡️inferences/🏷️effective-properties/<case>/💡️inference/🏷️effective-properties/🔣️.json` are WRITTEN by this file
(`write`), never by hand, and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🏷️effective-properties>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🏷️effective-properties>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see https://json-schema.org/draft/2020-12/json-schema-validation — the vocabulary of the schemas derived from the definitions
"""

# region 🔖️Imports
import importlib.metadata
import importlib.util
import json
import sys
from pathlib import Path

import jsonschema

# endregion 🔖️Imports


# region 🔖️Vocabulary
JSON_TYPE = {"Text": "string", "Real": "number", "Integer": "integer", "Boolean": "boolean", "Length": "number", "Area": "number", "Volume": "number", "Angle": "number"}
SERVED = {"WallType": "Wall", "SlabType": "Slab", "CeilingType": "Ceiling", "RoofType": "Roof", "ColumnType": "Column", "BeamType": "Beam", "WindowType": "Window", "DoorType": "Door"}
ELEMENTS = [("sites", "Site"), ("buildings", "Building"), ("storeys", "Storey"), ("walls", "Wall"), ("curtain_walls", "CurtainWall"), ("columns", "Column"), ("beams", "Beam"), ("slabs", "Slab"), ("ceilings", "Ceiling"), ("roofs", "Roof"), ("stairs", "Stair"), ("ramps", "Ramp"), ("railings", "Railing"), ("spaces", "Space"), ("zones", "Zone")]
TYPES = [("wall_types", "WallType"), ("slab_types", "SlabType"), ("ceiling_types", "CeilingType"), ("roof_types", "RoofType"), ("column_types", "ColumnType"), ("beam_types", "BeamType"), ("window_types", "WindowType"), ("door_types", "DoorType")]
TYPE_FIELD = [("walls", "wall_type"), ("slabs", "slab_type"), ("ceilings", "ceiling_type"), ("roofs", "roof_type"), ("columns", "column_type"), ("beams", "beam_type")]
PRIORITY = {"minimum": 0, "maximum": 1, "enum": 2}
ISSUE = {"minimum": "BelowMinimum", "maximum": "AboveMaximum", "enum": "NotAllowed"}


def untag(tagged):
    """🏷️ `(kind, payload)` of a tagged property value such as `{"Real": {"value": 0.35}}`."""
    (kind, body), = tagged.items()
    return kind, body["value"]


def schema_of(definition):
    """📐️ The JSON Schema of one property definition."""
    schema = {"type": JSON_TYPE[definition["kind"]]}
    if definition.get("minimum") is not None:
        schema["minimum"] = definition["minimum"]
    if definition.get("maximum") is not None:
        schema["maximum"] = definition["maximum"]
    if definition.get("allowed"):
        schema["enum"] = [untag(item)[1] for item in definition["allowed"]]
    return schema


def violation(definition, tagged):
    """🚫️ The issue of a value against its definition, `None` when the value is allowed."""
    kind, payload = untag(tagged)
    if kind != definition["kind"]:
        return "KindMismatch"
    errors = sorted(jsonschema.Draft202012Validator(schema_of(definition)).iter_errors(payload), key=lambda error: PRIORITY.get(error.validator, 3))
    if not errors:
        return None
    return ISSUE.get(errors[0].validator, "KindMismatch")


# endregion 🔖️Vocabulary


# region 🔖️Holders
def target_of(snapshot, record):
    """🎯️ The template target a record id names."""
    for field, target in ELEMENTS + TYPES:
        if record in snapshot.get(field, {}):
            return target
    opening = snapshot.get("openings", {}).get(record)
    if opening:
        return {"Window": "Window", "Door": "Door", "Void": "Void"}[next(iter(opening["kind"]))]
    return None


def type_of(snapshot, record):
    """🏛️ The type record an instance names, when it exists."""
    for field, link in TYPE_FIELD:
        row = snapshot.get(field, {}).get(record)
        if row:
            return row[link] if row[link] in snapshot.get(link + "s", {}) else None
    opening = snapshot.get("openings", {}).get(record)
    if opening:
        kind, body = next(iter(opening["kind"].items()))
        field = {"Window": "window_type", "Door": "door_type"}.get(kind)
        if field and body[field] in snapshot.get(field + "s", {}):
            return body[field]
    return None


def instances_of(snapshot, type_id):
    found = [record for field, link in TYPE_FIELD for record, row in snapshot.get(field, {}).items() if row[link] == type_id]
    for record, opening in snapshot.get("openings", {}).items():
        kind, body = next(iter(opening["kind"].items()))
        if kind in ("Window", "Door") and body[kind.lower() + "_type"] == type_id:
            found.append(record)
    return found


def applying(snapshot, target):
    """🧱️ The templates, by id, whose list names the target."""
    return [(template_id, snapshot["property_templates"][template_id]) for template_id in sorted(snapshot.get("property_templates", {})) if target in snapshot["property_templates"][template_id]["applies_to"]]


def holders_of(snapshot):
    """🧭️ Every holder id: records with properties or under a template, and the instances of holder types."""
    everything = [record for field, _ in ELEMENTS + TYPES for record in snapshot.get(field, {})] + list(snapshot.get("openings", {}))
    ids = {record for record in snapshot.get("properties", {}) if target_of(snapshot, record)}
    if snapshot.get("property_templates"):
        ids |= {record for record in everything if applying(snapshot, target_of(snapshot, record))}
    types = {record for record in ids if target_of(snapshot, record) in SERVED}
    ids |= {instance for type_id in types for instance in instances_of(snapshot, type_id)}
    return sorted(types), sorted(ids - types)


# endregion 🔖️Holders


# region 🔖️Table
def effective(snapshot, record, inherited):
    """🏷️ `(values, findings)` of one holder, the values as `{set: {name: (tagged, source)}}`."""
    values = {}
    for set_name, properties in (inherited or {}).items():
        for name, (tagged, source) in properties.items():
            values.setdefault(set_name, {})[name] = (tagged, "Default" if source == "Default" else "Type")
    for set_name, properties in snapshot.get("properties", {}).get(record, {}).items():
        for name, tagged in properties.items():
            values.setdefault(set_name, {})[name] = (tagged, "Own")
    findings = []
    target = target_of(snapshot, record)
    for _, template in applying(snapshot, target):
        for definition in template["properties"]:
            slot = values.get(template["name"], {}).get(definition["name"])
            if slot is not None:
                issue = violation(definition, slot[0]) if slot[1] == "Own" else None
                if issue:
                    findings.append({"set": template["name"], "property": definition["name"], "issue": issue})
            elif definition.get("default_value") is not None:
                values.setdefault(template["name"], {})[definition["name"]] = (definition["default_value"], "Default")
            elif definition.get("required"):
                findings.append({"set": template["name"], "property": definition["name"], "issue": "Missing"})
    if target in SERVED:
        for _, template in applying(snapshot, SERVED[target]):
            if target in template["applies_to"]:
                continue
            for definition in template["properties"]:
                slot = values.get(template["name"], {}).get(definition["name"])
                if slot is not None and slot[1] == "Own" and violation(definition, slot[0]):
                    findings.append({"set": template["name"], "property": definition["name"], "issue": violation(definition, slot[0])})
    return values, findings


def row_of(values, findings):
    rows = []
    for set_name in sorted(values):
        for name in sorted(values[set_name]):
            tagged, source = values[set_name][name]
            kind, payload = untag(tagged)
            row = {"set": set_name, "name": name, "kind": kind}
            if kind == "Text":
                row["text"] = payload
            elif kind == "Boolean":
                row["flag"] = payload
            else:
                row["number"] = float(payload)
            row["source"] = source
            rows.append(row)
    return {"values": rows, "findings": findings}


def computed(snapshot):
    """🏷️ `({holder: (values, findings)}, ...)` in the order the rules apply: types first."""
    types, instances = holders_of(snapshot)
    done = {}
    for record in types:
        done[record] = effective(snapshot, record, None)
    for record in instances:
        parent = type_of(snapshot, record)
        done[record] = effective(snapshot, record, done[parent][0] if parent in done else None)
    return done


def tables(snapshot):
    """🏷️ The `🏷️effective-properties` table of a snapshot."""
    return {record: row_of(*done) for record, done in sorted(computed(snapshot).items())}


# endregion 🔖️Table


# region 🔖️Audit
def problems_of(snapshot):
    """🩺️ Disagreements between the validator and the findings, plus the invariants of the derivation."""
    problems = []
    for template_id, template in snapshot.get("property_templates", {}).items():
        for definition in template["properties"]:
            try:
                jsonschema.Draft202012Validator.check_schema(schema_of(definition))
            except jsonschema.SchemaError as error:
                problems.append("%s.%s: its schema is invalid (%s)" % (template_id, definition["name"], error.message))
            default = definition.get("default_value")
            if default is not None and violation(definition, default):
                problems.append("%s.%s: the default value breaks its own definition (%s)" % (template_id, definition["name"], violation(definition, default)))
    for record, (values, findings) in computed(snapshot).items():
        flagged = {(row["set"], row["property"]) for row in findings}
        if len(flagged) != len(findings):
            problems.append("%s: a property is reported twice" % record)
        for set_name, properties in values.items():
            for name, (_, source) in properties.items():
                if source != "Own" and (set_name, name) in flagged:
                    problems.append("%s: the %s value of %s.%s carries a finding" % (record, source, set_name, name))
        for row in findings:
            if row["issue"] == "Missing" and values.get(row["set"], {}).get(row["property"]) is not None:
                problems.append("%s: %s.%s is reported missing although it has a value" % (record, row["set"], row["property"]))
    return problems


# endregion 🔖️Audit


# region 🔖️Projection
def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (the levels oracle's comparison)."""
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels").compare(expected, actual, path)


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def psets_handler(ctx):
    """🏷️ Oracle answer for `🏷️effective-properties`, after the audits agreed with it."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = problems_of(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = tables(snapshot)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("psets-walls-columns-spaces", psets_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        failures += ["%s: %s" % (case.name, problem) for problem in problems_of(snapshot)]
        table = tables(snapshot)
        target = case / "💡️inference" / "🏷️effective-properties" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            if target.exists():
                target.unlink()
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), table, "effective-properties")]
        print("%s: jsonschema %s, %d holders, %d findings" % (case.name, importlib.metadata.version("jsonschema"), len(table), sum(len(row["findings"]) for row in table.values())))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
