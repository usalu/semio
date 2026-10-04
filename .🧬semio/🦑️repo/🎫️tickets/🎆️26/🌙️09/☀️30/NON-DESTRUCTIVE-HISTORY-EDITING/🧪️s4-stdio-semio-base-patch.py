#!/usr/bin/env python3
"""✉️ D4 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING for `✉️mutate-semio-base`: the envelope's `patch-snapshot`
specification vector — the committed value-subset envelope of `📸️set-snapshot/✉️replaces`, one pointer operation into
its value arm, and the after-envelope the host's independent `patched_snapshot` yields (its exact inverse checked to
restore the before-envelope) — registered as a catalog vector with its fixture manifest and exercised by a
`mutate-patch-snapshot`/`inverse-patch-snapshot` scenario pair. Idempotent; `--check` exits 1 while anything is pending.

@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base"
SOURCE = SUBSET / "🧫️fixtures/🧬️mutations/📸️set-snapshot/✉️replaces/📸️snapshot/⬅️before/🔣️.json"
LEAF, SCENARIO, SCENARIO_ID = "🩹️patch-snapshot", "✏️edits", "edits-the-value-arm-of-the-envelope"
VECTOR = SUBSET / "🧫️fixtures/🧬️mutations" / LEAF / SCENARIO
FEATURE = SUBSET / "🧪️tests/✉️mutate-semio-base/🥒️.feature"
CATALOG = SUBSET / "🔮️oracles/🔣️.json"
PATCH = {"operation": "set", "path": "/subset/root/entries/0/value/value", "value": "Envelope Fixture, patched"}
ROOT_URI = f"shared://🧬️mutations/{LEAF}/{SCENARIO}"
SCENARIOS = f"""  @id-mutate-patch-snapshot
  @level-exhaustive
  @mode-differential
  Scenario: patch-snapshot edits one value of the committed value-subset envelope through its pointer
    Given the committed before-envelope {ROOT_URI}/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation {ROOT_URI}/🦠️mutation/🔣️.json
    And the committed after-envelope {ROOT_URI}/📸️snapshot/➡️after/🔣️.json
    When patch-snapshot is applied through apply_semio_mutation
    Then the envelope equals the committed after-envelope, still carries the value subset and raises no diagnostic

  @id-inverse-patch-snapshot
  @level-exhaustive
  @mode-property
  Scenario: Undoing patch-snapshot restores the committed before-envelope
    Given the committed before-envelope {ROOT_URI}/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation {ROOT_URI}/🦠️mutation/🔣️.json
    When patch-snapshot is applied through apply_semio_mutation
    And the mutation's own computed inverse is applied through apply_semio_mutation
    Then the envelope equals the committed before-envelope, still carries the value subset and raises no diagnostic

"""


def host():
    spec = importlib.util.spec_from_file_location("semio_repo_test", ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules["semio_repo_test"] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def text(value) -> str:
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def files() -> dict[pathlib.Path, str]:
    library = host()
    before = json.loads(SOURCE.read_text(encoding="utf-8"))
    after = library.patched_snapshot(before, PATCH)
    assert after != before, "the patch must move the envelope"
    assert library.patched_snapshot(after, library.snapshot_patch_inverse(before, PATCH)) == before, "the exact inverse must restore the envelope"
    return {
        VECTOR / "🦠️mutation/🔣️.json": text({"mutation": "patchSnapshot", "payload": {"patch": PATCH}}),
        VECTOR / "📸️snapshot/⬅️before/🔣️.json": text(before),
        VECTOR / "📸️snapshot/➡️after/🔣️.json": text(after),
        VECTOR / "🎯️outcome/🔣️.json": text({"status": "applied"}),
    }


def with_catalog(current: str, written: dict[pathlib.Path, str]) -> str:
    catalog = json.loads(current)
    for mutation_catalog in catalog["mutationCatalogs"]:
        vectors = mutation_catalog.setdefault("vectors", [])
        vector = next((entry for entry in vectors if entry["mutationId"] == "patch-snapshot"), None)
        if vector is None:
            vector = {"mutationId": "patch-snapshot", "sourceMutationDirectoryName": LEAF, "mutationDirectoryName": LEAF, "scenarios": []}
            vectors.insert(1, vector)
        if not any(scenario["directoryName"] == SCENARIO for scenario in vector["scenarios"]):
            vector["scenarios"].append({"id": SCENARIO_ID, "directoryName": SCENARIO})
    template = next(entry for entry in catalog["fixtureManifests"] if entry["id"] == "set-snapshot-replaces-the-envelope-wrapping-a-value-subset")
    roles = [("expected-before-json", "📸️snapshot/⬅️before/🔣️.json"), ("mutation-json", "🦠️mutation/🔣️.json"), ("expected-after-json", "📸️snapshot/➡️after/🔣️.json"), ("outcome-json", "🎯️outcome/🔣️.json")]
    entry = json.loads(json.dumps(template))
    entry.update({"id": f"patch-snapshot-{SCENARIO_ID}", "mutation": "patch-snapshot", "notes": "One RFC 6901 set into the value arm: the title entry's string moves from `Envelope Fixture` to `Envelope Fixture, patched`. The after-envelope is the host's independent `patched_snapshot` of the before-envelope; the committed carrier records no diff, because the patch's diff is the subject's own derivation."})
    entry["files"] = [{"role": role, "path": f"../🧫️fixtures/🧬️mutations/{LEAF}/{SCENARIO}/{relative}", "mediaType": "application/json", "sha256": "sha256:" + hashlib.sha256(written[VECTOR / relative].encode()).hexdigest(), "bytes": len(written[VECTOR / relative].encode())} for role, relative in roles]
    catalog["fixtureManifests"] = [existing for existing in catalog["fixtureManifests"] if existing["id"] != entry["id"]] + [entry]
    indent = 2 if current.startswith('{\n  "') else 1
    return json.dumps(catalog, indent=indent, ensure_ascii=False) + "\n"


def with_scenarios(current: str) -> str:
    if "@id-mutate-patch-snapshot" in current:
        return current
    anchor = "  @id-identity-round-trip\n"
    assert current.count(anchor) == 1, "the identity round trip anchor is missing"
    return current.replace(anchor, SCENARIOS + anchor)


def main() -> int:
    check = "--check" in sys.argv
    written = files()
    wanted = dict(written)
    wanted[CATALOG] = with_catalog(CATALOG.read_text(encoding="utf-8"), written)
    wanted[FEATURE] = with_scenarios(FEATURE.read_text(encoding="utf-8"))
    pending = 0
    for path, content in wanted.items():
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if current != content:
            pending += 1
            print(f"{'pending' if check else 'written'}: {path.relative_to(SUBSET)}")
            if not check:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
    print(f"patched_snapshot vector verified (moves, exact inverse restores); {pending} file(s) {'pending' if check else 'written'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
