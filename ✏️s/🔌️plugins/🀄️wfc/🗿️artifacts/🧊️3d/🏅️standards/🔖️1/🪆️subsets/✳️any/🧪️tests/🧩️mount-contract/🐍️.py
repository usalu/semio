#!/usr/bin/env python3
"""🧩️ The wfc3d mount contract, asserted from the committed fixture — the Python half of the same
statement the Rust `🦀️.rs` and the TypeScript `🟦️.ts` siblings make. It links nothing: the fixture is
the single source, so a drift in app ids, window kinds, the mutation roster, the inference route or
the example roster fails in three languages at once."""

import json
import os
import re
import sys

CONTRACT = os.path.join(os.path.dirname(__file__), "..", "..", "🧫️fixtures", "🧩️mount-contract", "🔣️.json")
SURFACE = re.compile(r"^(?P<kind>[a-z0-9.]+)@(?P<standard>[^/]+)/(?P<subset>[^#]+)#(?P<role>editor|viewer)$")


def assert_mount_contract(contract):
    problems = []
    for identifier, role in ((contract["editorAppId"], "editor"), (contract["viewerAppId"], "viewer")):
        match = SURFACE.match(identifier)
        if match is None:
            problems.append(f"{identifier} is not a canonical surface id")
            continue
        if match.group("kind") != "s.wfc.wfc3d":
            problems.append(f"{identifier} names {match.group('kind')}")
        if (match.group("standard"), match.group("subset")) != ("1", "*"):
            problems.append(f"{identifier} is not standard 1 subset *")
        if match.group("role") != role:
            problems.append(f"{identifier} is not the {role} surface")
    if contract["editorWindowKinds"] != ["wfc-graph", "wfc-3d-preview"]:
        problems.append("the editor mounts the shared graph window and its own 3d preview, in that order")
    if contract["viewerWindowKinds"] != ["wfc-3d-view"]:
        problems.append("the viewer mounts exactly one read-only window")
    if len(contract["mutations"]) != 15:
        problems.append("fifteen mutation kinds")
    if len(set(contract["mutations"])) != len(contract["mutations"]):
        problems.append("mutation kinds are unique")
    if contract["inferenceToolId"] != "s.wfc.wfc3d.solve" or contract["inferenceJobKind"] != "semio.infer":
        problems.append("the solve answers on the semio.infer cold-job route")
    if len(contract["examples"]) != 3:
        problems.append("three bundled examples")
    for example in contract["examples"]:
        if set(example["label"]) != {"en", "de"}:
            problems.append(f"example {example['id']} must be localized en and de")
        if example["slots"] <= 0:
            problems.append(f"example {example['id']} must declare at least one slot")
    return problems


def main():
    with open(CONTRACT, encoding="utf-8") as handle:
        contract = json.load(handle)
    problems = assert_mount_contract(contract)
    for problem in problems:
        print(f"wfc3d mount contract: {problem}")
    print("ok" if not problems else f"{len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
