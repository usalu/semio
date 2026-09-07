"""🔍️ Third-party validation of every `s.energy.model` mutation leaf's `🧬️.schema.json`.

Two independent checks per committed fixture vector, both run by `jsonschema` (the pinned
third-party Draft-07 implementation in the root `[dependency-groups] test`), never by our own code:

1. every leaf `🧬️.schema.json` is itself a valid Draft-07 schema (metaschema check);
2. every `🧪️tests/<case>/🦠️mutation/🔣️.json` validates against BOTH its own leaf schema (with the
   `mutation` wire discriminator stripped, which the leaf schema does not declare) and the aggregate
   `🧬️mutations/🔣️.json` union (which does declare it, as a `const`), matching exactly one branch.

Run from the repository root: `uv run --group test python3 <this file>`.
"""

from __future__ import annotations

import json
import os

from jsonschema import Draft7Validator
from jsonschema.validators import validator_for

SUBSET = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = f"{SUBSET}/🧬️schema/🧬️mutations"


def read(path: str):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def main() -> int:
    aggregate = read(f"{MUT}/🔣️.json")
    union = validator_for(aggregate)(aggregate)
    branches = {entry["$ref"].rsplit("/", 1)[-1] for entry in aggregate["oneOf"]}
    checked, failures, hit = 0, [], set()
    for entry in sorted(os.listdir(MUT)):
        directory = os.path.join(MUT, entry)
        leaf_path = os.path.join(directory, "🧬️.schema.json")
        if not os.path.isfile(leaf_path):
            continue
        descriptor = read(os.path.join(directory, "🔣️.json"))
        leaf = read(leaf_path)
        Draft7Validator.check_schema(leaf)
        assert leaf["title"] == descriptor["aggregateVariant"], f"{entry}: schema title != aggregateVariant"
        assert leaf["title"] in branches, f"{entry}: {leaf['title']} is not a branch of the aggregate union"
        payload_validator = validator_for(leaf)(leaf)
        tests = os.path.join(directory, "🧪️tests")
        for case in sorted(os.listdir(tests)) if os.path.isdir(tests) else []:
            wire = read(os.path.join(tests, case, "🦠️mutation", "🔣️.json"))
            if not wire:
                continue
            checked += 1
            hit.add(leaf["title"])
            body = {key: value for key, value in wire.items() if key != "mutation"}
            for label, validator, document in (("leaf", payload_validator, body), ("union", union, wire)):
                errors = sorted(validator.iter_errors(document), key=lambda error: list(error.path))
                if errors:
                    failures.append(f"{entry}/{case} [{label}]: " + "; ".join(f"{list(error.path)}: {error.message}" for error in errors[:3]))
    print(f"payloads checked {checked}, leaf schemas {len(branches)}, branches exercised {len(hit)}, failures {len(failures)}")
    for failure in failures:
        print("  ✗", failure)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
