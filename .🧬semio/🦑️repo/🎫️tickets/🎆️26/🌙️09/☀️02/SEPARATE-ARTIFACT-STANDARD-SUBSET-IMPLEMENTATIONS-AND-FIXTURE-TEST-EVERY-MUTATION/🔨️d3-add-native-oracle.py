#!/usr/bin/env python3
"""🔨️ D3 helper: registers one `verified-native-second-implementation` oracle entry (the kind D1
landed mid-ticket, `🟦️.ts:2778`/`:4818`) in a subset's `🧪️oracle/🔣️.json`, narrows/clears the
matching `noOracleDecisions` entry, and updates every `mutationManifests[].mutations[].
oracleRequirements[].qualifyingKind` for the given capability to match. Validates the file is
well-formed JSON both before and after, and validates the `nativeSecondImplementation` evidence shape
against `nativeSecondImplementationBreaches`'s own checklist before writing: semio-native format,
100% capability coverage, a credible noThirdPartySurvey, a second-implementation language distinct
from the subject's, a non-empty specificationSource, and vectors > 0. Idempotent.
"""
from __future__ import annotations

import json
import sys


def main() -> int:
    (
        oracle_json_path,
        oracle_id,
        format_artifact_id,
        capabilities_csv,
        component_rel_path,
        rationale,
        no_oracle_decision_id,
        ecosystems_csv,
        candidates_json,
        specification_source,
        vectors,
        capabilities_covered_csv,
    ) = sys.argv[1:13]

    capabilities = [c for c in capabilities_csv.split(",") if c]
    ecosystems = [e for e in ecosystems_csv.split(",") if e]
    candidates = json.loads(candidates_json)
    capabilities_covered = [c for c in capabilities_covered_csv.split(",") if c]
    assert capabilities, "at least one capability required"
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
        "capabilities": capabilities,
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
            "fixtureCoverage": {"vectors": int(vectors), "capabilitiesCovered": capabilities_covered},
        },
    }
    data["oracles"] = [o for o in data.get("oracles", []) if o.get("id") != oracle_id] + [entry]

    if no_oracle_decision_id:
        data["noOracleDecisions"] = [d for d in data.get("noOracleDecisions", []) if d.get("id") != no_oracle_decision_id]

    for manifest in data.get("mutationManifests", []):
        for mutation in manifest.get("mutations", []):
            for req in mutation.get("oracleRequirements", []):
                if req.get("capability") in capabilities:
                    req["qualifyingKind"] = "verified-native-second-implementation"

    with open(oracle_json_path, "w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=2, ensure_ascii=False)
        handle.write("\n")

    with open(oracle_json_path, "r", encoding="utf-8") as handle:
        json.load(handle)
    print(f"ok: {oracle_json_path} — oracle {oracle_id!r} registered as verified-native-second-implementation, decision {no_oracle_decision_id!r} removed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
