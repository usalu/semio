"""✅️ Third-party validation of the G4 mutation leaves.

Two independent checks, both run with `jsonschema` (the repo's pinned third-party JSON Schema
implementation, root `pyproject.toml` `[dependency-groups] test`) — never with our own code:

1. every `🧬️.schema.json` under a G4 leaf IS a valid draft-07 schema (`Draft7Validator.check_schema`);
2. every committed `🧪️tests/<case>/🦠️mutation/🔣️.json` payload validates against its own leaf schema,
   once the `SEMIO_ENERGY_WRITE_FIXTURES=1` pass has materialized it (an unmaterialized `{}` seed is
   reported as PENDING, not as a pass).

Run from the repository root: `./.venv/bin/python <this file>`.
"""

from __future__ import annotations

import json
import os
import sys

from jsonschema import Draft7Validator

MUT = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
G4_RANGES = ((700, 799), (800, 899), (900, 999))


def g4_directories(root: str) -> list[str]:
    """📂 The leaf directories this lane owns, read out of the aggregate's own `DIRECTORIES` table
    order via the ledger numbers in `🐍️generate-mutation-leaves.py` — not by guessing at emoji."""
    import importlib.util

    spec = importlib.util.spec_from_file_location("gen", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️generate-mutation-leaves.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return [k.dir for k in module.KINDS if any(low <= k.number <= high for low, high in G4_RANGES)]


def main() -> int:
    root = os.getcwd()
    schemas = 0
    payloads = 0
    pending = 0
    failures = []
    for directory in g4_directories(root):
        schema_path = os.path.join(root, MUT, directory, "🧬️.schema.json")
        if not os.path.isfile(schema_path):
            failures.append(f"{directory}: 🧬️.schema.json is missing")
            continue
        with open(schema_path, encoding="utf-8") as handle:
            schema = json.load(handle)
        try:
            Draft7Validator.check_schema(schema)
            schemas += 1
        except Exception as error:
            failures.append(f"{directory}: schema is not valid draft-07: {error}")
            continue
        validator = Draft7Validator(schema)
        cases_root = os.path.join(root, MUT, directory, "🧪️tests")
        for case in sorted(os.listdir(cases_root)) if os.path.isdir(cases_root) else []:
            payload_path = os.path.join(cases_root, case, "🦠️mutation", "🔣️.json")
            if not os.path.isfile(payload_path):
                failures.append(f"{directory}/{case}: 🦠️mutation/🔣️.json is missing")
                continue
            with open(payload_path, encoding="utf-8") as handle:
                document = json.load(handle)
            if document == {}:
                pending += 1
                continue
            body = {key: value for key, value in document.items() if key != "mutation"}
            errors = sorted(validator.iter_errors(body), key=lambda error: list(error.path))
            if errors:
                failures.append(f"{directory}/{case}: {errors[0].message}")
                continue
            payloads += 1
    print(f"{schemas} G4 payload schemas are valid draft-07; {payloads} committed payloads validate; {pending} not yet materialized")
    for failure in failures:
        print("FAIL", failure)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
