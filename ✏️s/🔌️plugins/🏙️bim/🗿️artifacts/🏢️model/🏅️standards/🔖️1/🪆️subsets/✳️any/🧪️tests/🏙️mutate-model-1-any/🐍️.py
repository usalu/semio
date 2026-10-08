"""🔮️ Third-party oracle of the BIM mutation case: `jsonpatch` (RFC 6902), `deepdiff` and `jsonschema`, none of which has seen this repository.

The Rust subject applies each mutation through `apply_model_mutation`. This oracle never applies a mutation. It takes the committed
quintet of the row (before, mutation payload, after, typed diff, outcome) and lets the libraries state the facts:

* `jsonpatch.make_patch(before, after)` derives the RFC 6902 patch; applying it to `before` must reproduce `after` exactly, and the
  members it touches must be exactly the members the typed `ModelDiff` declares, classified `Created` (a record added),
  `Deleted` (a record removed) or `Patched` (a field moved), with the typed diff carrying exactly the changed fields at their new values.
* `deepdiff.DeepDiff` is a structurally unrelated algorithm; it must agree with `jsonpatch` on whether the document moved at all.
* `jsonschema` (draft 7) validates the committed payload against the leaf's own `🧬️schema/🔣️.json` and rejects the same payload once a
  member the schema does not declare is added, which proves the validator ran.
* The inverse sum law: the patch from `after` back to `before` restores `before`, and patch plus inverse patch leave the document unchanged.

The projection each handler returns is the committed after-snapshot (`mutate-*`) or the committed before-snapshot (`inverse-*`), the
document the subject must land on. Run standalone: `python 🐍️.py check <S>/🧫️fixtures/🧬️mutations` (exit 1 on any disagreement).
"""

import json
import os
import re
import sys

import deepdiff
import jsonpatch
import jsonschema
from referencing import Registry, Resource

BEFORE = ("📸️snapshot", "⬅️before", "🔣️.json")
AFTER = ("📸️snapshot", "➡️after", "🔣️.json")
MUTATION = ("🦠️mutation", "🔣️.json")
DIFF = ("🔺️diff", "🔣️.json")
OUTCOME = ("🎯️outcome", "🔣️.json")
LEAF_SCHEMA_ROOT = ("..", "..", "🧬️schema", "🧬️mutations")
LEAF_SCHEMA = ("🧬️schema", "🔣️.json")
SCALAR_SECTIONS = ("project",)
UNDECLARED = "x-undeclared-member"
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"
SNAPSHOT_ASSET = "asset://🎬️demo/📸️snapshot.json"


# region 🔖️Reading
def read_json(*parts):
    """📜️ One committed JSON leaf."""
    with open(os.path.join(*parts), "r", encoding="utf-8") as handle:
        return json.load(handle)


def normal(value):
    """⚖️ The comparison form of a JSON value: integral floats become ints and `null` members vanish, because the typed diff writes an
    absent optional as `null` while the snapshot omits it."""
    if isinstance(value, dict):
        return {key: normal(member) for key, member in value.items() if member is not None}
    if isinstance(value, list):
        return [normal(member) for member in value]
    if isinstance(value, float) and value == int(value) and abs(value) < 1e15:
        return int(value)
    return value


def pointer(path):
    """🧭️ The unescaped segments of an RFC 6901 pointer."""
    return [segment.replace("~1", "/").replace("~0", "~") for segment in path.split("/")[1:]]


ARTIFACT_SCHEMA = ("..", "🧬️schema", "🔣️.json")


def registry_of(fixtures):
    """🗂️ The registry that resolves the leaves' `$ref` into the artifact schema the snapshot JSON Schema is built on."""
    document = read_json(fixtures, *ARTIFACT_SCHEMA)
    return Registry().with_resource(document["$id"], Resource.from_contents(document))


class Row:
    """🧫️ One committed quintet and the directories around it."""

    def __init__(self, directory, leaf_schema, registry):
        self.directory = directory
        self.before = read_json(directory, *BEFORE)
        self.after = read_json(directory, *AFTER)
        self.mutation = read_json(directory, *MUTATION)
        self.diff = read_json(directory, *DIFF)
        self.outcome = read_json(directory, *OUTCOME)
        self.leaf_schema = leaf_schema
        self.registry = registry


def row_at(before_path, params=None):
    """🧫️ The row whose before-snapshot sits at `before_path`; the leaf schema is the sibling tree `🧬️schema/🧬️mutations/<leaf>`."""
    directory = os.path.dirname(os.path.dirname(os.path.dirname(before_path)))
    leaf = os.path.basename(os.path.dirname(directory))
    fixtures = os.path.dirname(os.path.dirname(directory))
    schema_path = os.path.join(fixtures, *LEAF_SCHEMA_ROOT, leaf, *LEAF_SCHEMA)
    return Row(directory, read_json(schema_path), registry_of(os.path.join(fixtures, "..")))


# endregion 🔖️Reading


# region 🔖️Laws
def touched(operations):
    """🪢️ What an RFC 6902 patch reaches, keyed `(section, id)`: `created`, `deleted`, and the changed `fields`; `project` is keyed `(project, None)`."""
    found = {}

    def entry(section, identity):
        return found.setdefault((section, identity), {"created": False, "deleted": False, "fields": set()})

    def reach(path, role):
        segments = pointer(path)
        section = segments[0]
        if section in SCALAR_SECTIONS:
            if len(segments) > 1:
                entry(section, None)["fields"].add(segments[1])
            else:
                raise AssertionError("an operation replaces the whole %s section" % section)
            return
        if len(segments) < 2:
            raise AssertionError("an operation addresses the whole %s collection" % section)
        record = entry(section, segments[1])
        if len(segments) == 2:
            if role == "add":
                record["created"] = True
            elif role == "remove":
                record["deleted"] = True
            else:
                raise AssertionError("an operation replaces the whole record %s/%s" % (section, segments[1]))
        else:
            record["fields"].add(segments[2])

    for operation in operations:
        name = operation["op"]
        if name == "move":
            reach(operation["from"], "remove")
            reach(operation["path"], "add")
        elif name == "copy":
            reach(operation["path"], "add")
        else:
            reach(operation["path"], name)
    return found


def assigned(record, patch):
    """🩹️ The property sets a record holds once an `assigned` patch is applied: `null` removes a property, or a whole set."""
    merged = {name: dict(members) for name, members in (record or {}).items()}
    for name, members in patch.items():
        if members is None:
            merged.pop(name, None)
            continue
        target = merged.setdefault(name, {})
        for key, value in members.items():
            if value is None:
                target.pop(key, None)
            else:
                target[key] = value
        if not target:
            merged.pop(name, None)
    return merged


def claimed(written, record):
    """🩹️ What a Patched typed diff says its fields now hold: an optional-field patch `{"value": x}` (`null` clears) is unwrapped when the
    snapshot holds x, not the wrapper."""
    found = {}
    for name, member in written.items():
        if name == "entry":
            continue
        wrapped = isinstance(member, dict) and list(member) == ["value"] and normal(record.get(name)) != normal(member)
        found[name] = member["value"] if wrapped else member
    return found


def declared(diff):
    """📋️ The typed diff in the same key space as `touched`."""
    found = {}
    for section, members in diff.items():
        if section in SCALAR_SECTIONS:
            found[(section, None)] = {"entry": "Patched", "fields": set(members)}
            continue
        for identity, record in members.items():
            found[(section, identity)] = {"entry": record["entry"], "fields": {key for key in record if key != "entry"}}
    return found


def problems_of(row):
    """🔬️ Every disagreement between the libraries and the committed quintet; empty when they agree."""
    problems = []
    before, after = normal(row.before), normal(row.after)
    patch = jsonpatch.make_patch(before, after)
    operations = patch.patch
    if normal(patch.apply(before)) != after:
        problems.append("applying the derived RFC 6902 patch to the before-snapshot does not reproduce the after-snapshot")
    moved = bool(deepdiff.DeepDiff(before, after))
    if moved != bool(operations):
        problems.append("deepdiff says moved=%s while jsonpatch derives %d operation(s)" % (moved, len(operations)))
    inverse = jsonpatch.make_patch(after, before)
    if normal(inverse.apply(after)) != before:
        problems.append("the inverse RFC 6902 patch does not restore the before-snapshot")
    if normal(inverse.apply(patch.apply(before))) != before:
        problems.append("patch plus inverse patch do not sum to the identity")
    status = row.outcome.get("status")
    if status != "applied":
        if operations or row.diff:
            problems.append("a %s row moves the document or carries a diff" % status)
        return problems
    if not operations:
        problems.append("an applied row leaves the document unchanged")
    reached, listed = touched(operations), declared(row.diff)
    for key in sorted(set(reached) - set(listed), key=str):
        problems.append("RFC 6902 reaches %s but the typed diff does not declare it" % (key,))
    for key in sorted(set(listed) - set(reached), key=str):
        problems.append("the typed diff declares %s but RFC 6902 needs no operation there" % (key,))
    for key in sorted(set(reached) & set(listed), key=str):
        section, identity = key
        fact, claim = reached[key], listed[key]
        expect = "Created" if fact["created"] else "Deleted" if fact["deleted"] else "Patched"
        if claim["entry"] != expect:
            problems.append("%s: RFC 6902 classifies the record %s, the typed diff says %s" % (key, expect, claim["entry"]))
        elif expect == "Patched" and section == "properties":
            written = row.diff[section][identity]["assigned"]
            if set(written) != fact["fields"]:
                problems.append("%s: RFC 6902 changes property sets %s, the typed diff assigns %s" % (key, sorted(fact["fields"]), sorted(written)))
            if normal(assigned(row.before[section].get(identity), written)) != normal(row.after[section].get(identity)):
                problems.append("%s: applying the assigned patch to the before-record does not give the after-record" % (key,))
            continue
        elif expect == "Patched" and claim["fields"] != fact["fields"]:
            problems.append("%s: RFC 6902 changes fields %s, the typed diff carries %s" % (key, sorted(fact["fields"]), sorted(claim["fields"])))
        if expect == "Deleted":
            if claim["fields"]:
                problems.append("%s: a Deleted entry carries fields %s" % (key, sorted(claim["fields"])))
            continue
        moved_record = row.after[section] if identity is None else row.after[section][identity]
        written = row.diff[section] if identity is None else row.diff[section][identity]
        wanted = moved_record if expect == "Created" else {name: moved_record.get(name) for name in claim["fields"]}
        got = {name: member for name, member in written.items() if name != "entry"} if expect == "Created" or identity is None else claimed(written, moved_record)
        if normal(got) != normal(wanted):
            problems.append("%s: the typed diff values differ from the after-snapshot" % (key,))
    return problems


def payload_problems(row, kind):
    """🧬️ The committed payload passes its own leaf schema, carries the kind's discriminator, and the validator rejects an undeclared member."""
    problems = []
    validator = jsonschema.Draft7Validator(row.leaf_schema, registry=row.registry)
    if row.outcome.get("status") == "applied":
        errors = sorted(validator.iter_errors(row.mutation), key=lambda error: list(error.path))
        problems += ["payload: %s" % error.message for error in errors]
    head, *rest = kind.split("-")
    if row.mutation.get("mutation") != head + "".join(word[:1].upper() + word[1:] for word in rest):
        problems.append("payload discriminator %r does not name %s" % (row.mutation.get("mutation"), kind))
    if not list(validator.iter_errors({**row.mutation, UNDECLARED: 1})):
        problems.append("the validator accepted a payload with an undeclared member")
    return problems


# endregion 🔖️Laws


# region 🔖️Handlers
def uris_of(ctx):
    """🔗️ The three fixture URIs a row names, by their leaf."""
    found = {}
    for uri in ctx.step_input_uris():
        for name, marker in (("before", "⬅️before"), ("mutation", "🦠️mutation"), ("after", "➡️after")):
            if marker in uri:
                found[name] = uri
    if len(found) != 3:
        raise AssertionError("the scenario names %s of before, mutation and after" % sorted(found))
    return found


def verdict(ctx, closing):
    """🎯️ Runs every law on the row of a scenario and returns the committed document the subject must land on."""
    spec = ctx.doc_json()
    kind = spec["kind"]
    uris = uris_of(ctx)
    row = row_at(ctx.input(uris["before"]))
    problems = problems_of(row) + payload_problems(row, kind)
    if normal(spec["params"]) != normal(row.mutation):
        problems.append("the feature params differ from the committed payload")
    if problems:
        raise AssertionError("%s: %s" % (kind, "; ".join(problems)))
    from semio_repo_test import Outcome

    document = row.after if closing == "after" else row.before
    return Outcome(document, raw=json.dumps(document, sort_keys=True).encode("utf-8"))


def mutate_handler(ctx):
    """👁️ The oracle of `mutate-<kind>`: the committed after-snapshot, vouched for by the libraries."""
    return verdict(ctx, "after")


def inverse_handler(ctx):
    """↩️ The oracle of `inverse-<kind>`: the committed before-snapshot, which the inverse patch is proven to restore."""
    return verdict(ctx, "before")


def round_trip_handler(ctx):
    """🔁️ The oracle of `identity-round-trip`: the committed DSL bytes are the fixed point and the committed JSON snapshot of the same
    demo is the projection both must reach; the JSON is read by the parser that has never seen the DSL."""
    from semio_repo_test import Outcome

    dsl = ctx.input_bytes(DSL_ASSET)
    snapshot = json.loads(ctx.input_bytes(SNAPSHOT_ASSET).decode("utf-8"))
    dsl.decode("utf-8")
    return Outcome(snapshot, raw=dsl)


def adapter():
    """🧭️ Registration in the ORACLE role only: one handler per Scenario Outline base id serves every kind row; the subject half lives in `🦀️.rs`."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("mutate", mutate_handler).oracle("inverse", inverse_handler).oracle("identity-round-trip", round_trip_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check <fixtures root>` runs every law on every committed quintet beneath a `🧬️mutations` fixtures directory."""
    sys.stdout.reconfigure(encoding="utf-8")
    root = arguments[1]
    failures, checked = [], 0
    for leaf in sorted(os.listdir(root)):
        if not os.path.isdir(os.path.join(root, leaf)):
            continue
        for scenario in sorted(os.listdir(os.path.join(root, leaf))):
            before = os.path.join(root, leaf, scenario, *BEFORE)
            if not os.path.exists(before):
                continue
            row = row_at(before)
            checked += 1
            kind = re.sub(r"^[^a-z0-9]+", "", leaf)
            failures += ["%s/%s: %s" % (leaf, scenario, problem) for problem in problems_of(row) + payload_problems(row, kind)]
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("jsonpatch %s, deepdiff %s, jsonschema: %d quintet(s), %s" % (jsonpatch.__version__, deepdiff.__version__, checked, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
