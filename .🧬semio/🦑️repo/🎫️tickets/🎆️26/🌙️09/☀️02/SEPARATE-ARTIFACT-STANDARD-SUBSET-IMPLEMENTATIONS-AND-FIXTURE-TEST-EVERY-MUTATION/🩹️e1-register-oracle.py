#!/usr/bin/env python3
"""🩹️ E1 helper: registers one `verified-native-second-implementation` oracle entry in a subset's
`🧪️oracle/🔣️.json`, updates every `mutationManifests[].mutations[].oracleRequirements[].qualifyingKind`
for the given capability to match, and appends a dated note to the MATCHING no-oracle decision's
rationale (never deletes it, never clears its rationale — only records that the asset:// blocker it
named is now resolved). Idempotent.
"""
from __future__ import annotations

import json
import sys


def main() -> int:
    (
        oracle_json_path,
        oracle_id,
        format_artifact_id,
        capability,
        rationale,
        no_oracle_decision_id,
        ecosystems_csv,
        candidates_json,
        specification_source,
        vectors,
    ) = sys.argv[1:11]

    ecosystems = [e for e in ecosystems_csv.split(",") if e]
    candidates = json.loads(candidates_json)
    assert ecosystems, "at least one ecosystem required"
    assert candidates, "at least one candidate required"
    for c in candidates:
        assert len(c["reason"]) >= 10, f"candidate reason too short: {c}"
    assert not format_artifact_id.startswith("s.stdio.") or format_artifact_id == "s.stdio.semio", "format must be semio-native"

    with open(oracle_json_path, "r", encoding="utf-8") as handle:
        data = json.load(handle)

    entry = {
        "id": oracle_id,
        "ecosystem": "python",
        "package": "",
        "capabilities": [capability],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "AGPL-3.0-only",
        "testOnly": True,
        "rationale": rationale,
        "kind": "verified-native-second-implementation",
        "engine": {"family": "none", "implementation": "in-repository second implementation", "version": "0"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "nativeSecondImplementation": {
            "format": format_artifact_id,
            "noThirdPartySurvey": {"ecosystemsSearched": ecosystems, "candidatesConsidered": candidates},
            "subjectImplementationLanguage": "rust",
            "secondImplementationLanguage": "python",
            "specificationSource": specification_source,
            "fixtureCoverage": {"vectors": int(vectors), "capabilitiesCovered": [capability]},
        },
    }
    data["oracles"] = [o for o in data.get("oracles", []) if o.get("id") != oracle_id] + [entry]

    note_marker = "[E1 2026-09-02]"
    for decision in data.get("noOracleDecisions", []):
        if decision.get("id") == no_oracle_decision_id and note_marker not in decision.get("rationale", ""):
            decision["rationale"] = decision["rationale"] + (
                f"\n\n{note_marker} The asset:// fixture-declaration blocker named above is now resolved "
                f"(this feature's Examples now carry declared asset:// fixtures) and a Python second "
                f"implementation is registered as oracle {oracle_id!r} "
                f"(kind verified-native-second-implementation). Capabilities stay empty ([]) — this "
                f"entry is kept, not deleted, as the honest record of what was investigated on the "
                f"carrier side; the mutation-semantics gap it used to name is now discharged by the "
                f"registered oracle instead. See $TICKET/📓️e1-remodel-shooting-layout.md."
            )

    changed = 0
    for manifest in data.get("mutationManifests", []):
        for mutation in manifest.get("mutations", []):
            for req in mutation.get("oracleRequirements", []):
                if req.get("capability") == capability:
                    req["qualifyingKind"] = "verified-native-second-implementation"
                    changed += 1

    with open(oracle_json_path, "w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=2, ensure_ascii=False)
        handle.write("\n")

    with open(oracle_json_path, "r", encoding="utf-8") as handle:
        json.load(handle)
    print(f"ok: {oracle_json_path} — oracle {oracle_id!r} registered, {changed} oracleRequirements updated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
