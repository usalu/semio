#!/usr/bin/env python3
"""🏷️ W2-R energy follow-up: recodes the energy leaf refusals that depend on the BASE document (not on the payload alone)
from `mutation.invariant` to the state codes, in the Rust diffs and in the Python second implementation of
`🏛️mutate-energy-model-1`, and rewrites the committed outcomes of the fixtures that witness them.

- a delete refused while another entity still names the target → `mutation.target-referenced` (fem precedent);
- an index/position past the end of the current list → `mutation.target-missing`;
- a payload inconsistent with the target's current state (a reorder that is not a permutation of the held ids, an aperture
  polygon off its host surface's plane) → `mutation.target-mismatch`.

Payload-intrinsic invariants the leaf schema cannot state name their `x-semio-invariant` id in the Python refusal and the
committed outcome. `mutation.invariant` and `mutation.duplicate-id` refusals are raised at `Fatal`, the level the frozen
vocabulary (`🏪️store` `expected_mutation_message_level`) persists them at. Idempotent: a second run changes nothing."""
import glob
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SUBSET = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = f"{SUBSET}/🧬️schema/🧬️mutations"
FIXTURES = f"{SUBSET}/🧫️fixtures/🧬️mutations"
PYTHON = f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🐍️.py"
REFERENCED, MISSING, MISMATCH = "mutation.target-referenced", "mutation.target-missing", "mutation.target-mismatch"


def rust_code(condition):
    if "polygon_lies_on_plane" in condition:
        return MISMATCH
    if "wanted != held" in condition:
        return MISMATCH
    if re.search(r"payload\.(index|from) as usize|\.get\(payload\.index as usize\)", condition):
        return MISSING
    if re.search(r"base\.model\.", condition):
        return REFERENCED
    return None


def python_code(condition):
    if "_on_plane(" in condition:
        return MISMATCH
    if "sorted(wanted) != sorted(" in condition:
        return MISMATCH
    if re.search(r'payload\["(index|from)"\] >=? len\(', condition):
        return MISSING
    if re.search(r"^if (in_use|any\(.*before\[\"model\"\])", condition) and not re.search(r"payload\[\"(zoneIds|terminalZoneIds|equipmentIds)", condition):
        return REFERENCED
    return None


PYTHON_INVARIANTS = {
    "create_plant_loop": ('>= payload["equipmentIds"][index + 1]', "equipment-ids-ascending"),
    "create_air_loop": ('>= payload["terminalZoneIds"][index + 1]', "terminal-zone-ids-ascending"),
    "connect_surfaces": ("if first == second:", "distinct-surfaces"),
    "replace_airflow_network": ("len(zone_ids) != len(node_ids)", "zone-node-pairs"),
    "create_setpoint_manager": ("<=", "outdoor-reset-range"),
    "replace_setpoint_manager_kind": ("<=", "outdoor-reset-range"),
    "create_daily_schedule": (" > ", "limits-ordered"),
    "change_daily_schedule_limits": (" > ", "limits-ordered"),
}
OUTCOME_CODES = {
    **{case: REFERENCED for case in ("♒️delete-battery/⛔️refuses", "🌒️delete-pv-system/⛔️refuses", "🌓️delete-daily-schedule/⛔️refuses", "🎞️delete-time-series-schedule/⛔️refuses", "📍️delete-constant-schedule/⛔️refuses", "📕️delete-annual-schedule/⛔️refuses", "🕖️delete-weekly-schedule/⛔️refuses", "🏚️delete-zone/⛔️refuses-a-used-zone", "🥀️delete-air-loop/⛔️refuses-a-served-loop", "🧨️delete-construction/⛔️refuses", "🪨️delete-material/⛔️refuses", "🪚️delete-surface/⛔️refuses-a-partner")},
    "➖️remove-construction-layer/⛔️refuses": MISSING,
    "🗂️reorder-annual-schedule-rules/⛔️refuses": MISSING,
    "🔀️reorder-construction-layers/⛔️refuses": MISMATCH,
}
OUTCOME_INVARIANTS = {
    "⚗️create-plant-loop/⛔️refuses-a-jumbled-list": "equipment-ids-ascending",
    "🛞️create-air-loop/⛔️refuses-a-jumbled-list": "terminal-zone-ids-ascending",
    "🤝️connect-surfaces/⛔️refuses-a-self-pair": "distinct-surfaces",
    "🫧️replace-airflow-network/⛔️refuses-unpaired-nodes": "zone-node-pairs",
}


def recode_rust(check):
    changed = []
    for path in sorted(glob.glob(f"{MUTATIONS}/*/🔺️diff/🦀️.rs", root_dir=REPO)):
        lines = open(os.path.join(REPO, path), encoding="utf-8").read().split("\n")
        edited = False
        for at, line in enumerate(lines):
            if '"mutation.invariant"' not in line:
                continue
            start = at
            while start > 0 and not re.match(r"\s*(if|let)\b", lines[start]):
                start -= 1
            code = rust_code(" ".join(lines[start:at + 1]))
            if code is not None:
                lines[at] = line.replace('"mutation.invariant"', f'"{code}"', 1)
                edited = True
        if edited:
            changed.append(path)
            if not check:
                open(os.path.join(REPO, path), "w", encoding="utf-8").write("\n".join(lines))
    return changed


FATAL_CODES = ("mutation.invariant", "mutation.duplicate-id")


def level_rust(check):
    changed = []
    for path in sorted(glob.glob(f"{MUTATIONS}/**/🦀️.rs", root_dir=REPO, recursive=True)):
        source = open(os.path.join(REPO, path), encoding="utf-8").read()
        result = source
        for code in FATAL_CODES:
            result = result.replace(f'MutationOutcome::error("{code}"', f'MutationOutcome::fatal("{code}"')
        if result != source:
            changed.append(path)
            if not check:
                open(os.path.join(REPO, path), "w", encoding="utf-8").write(result)
    return changed


def recode_python(check):
    lines = open(os.path.join(REPO, PYTHON), encoding="utf-8").read().split("\n")
    function = None
    edits = 0
    for at, line in enumerate(lines):
        header = re.match(r"def (\w+)\(", line)
        if header:
            function = header.group(1)
        if 'rejected("mutation.invariant"' not in line:
            continue
        condition = lines[at - 1].strip()
        code = python_code(condition)
        if code is not None:
            lines[at] = line.replace('rejected("mutation.invariant"', f'rejected("{code}"', 1)
            edits += 1
            continue
        if function in PYTHON_INVARIANTS:
            marker, identifier = PYTHON_INVARIANTS[function]
            if marker in condition and ", invariant=" not in line:
                lines[at] = re.sub(r"\)\s*$", f', invariant="{identifier}")', line, count=1)
                edits += 1
    source = "\n".join(lines)
    signature = 'def rejected(code, path):\n    """⛔️ A refusal: one fault code and the offending address."""\n    return {"status": "rejected", "code": code, "path": list(path)}'
    if signature in source:
        source = source.replace(signature, 'def rejected(code, path, invariant=None):\n    """⛔️ A refusal: one fault code, the offending address, and — for a payload-intrinsic rule the leaf schema declares in\n    `x-semio-invariant` — the id of that rule."""\n    return {"status": "rejected", "code": code, "path": list(path), **({"invariant": invariant} if invariant is not None else {})}')
        edits += 1
    if not check:
        open(os.path.join(REPO, PYTHON), "w", encoding="utf-8").write(source)
    return edits


def recode_outcomes(check):
    changed = []
    for path in sorted(glob.glob(f"{FIXTURES}/*/*/🎯️outcome/🔣️.json", root_dir=REPO)):
        case = "/".join(path.split("/")[-4:-2])
        outcome = json.load(open(os.path.join(REPO, path), encoding="utf-8"))
        wanted = dict(outcome)
        if case in OUTCOME_CODES:
            wanted["code"] = OUTCOME_CODES[case]
        if case in OUTCOME_INVARIANTS:
            wanted["invariant"] = OUTCOME_INVARIANTS[case]
        if wanted != outcome:
            changed.append(path)
            if not check:
                open(os.path.join(REPO, path), "w", encoding="utf-8").write(json.dumps(wanted, indent=2, ensure_ascii=False) + "\n")
    return changed


def main():
    check = "--check" in sys.argv
    rust = recode_rust(check)
    levels = level_rust(check)
    python = recode_python(check)
    outcomes = recode_outcomes(check)
    for path in rust:
        print("rust", path.split("/🧬️mutations/")[1])
    for path in outcomes:
        print("outcome", path.split("/🧬️mutations/")[1])
    print(f"rust={len(rust)} fatal-levels={len(levels)} python-edits={python} outcomes={len(outcomes)}{' (check only)' if check else ''}")


if __name__ == "__main__":
    main()
