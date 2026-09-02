#!/usr/bin/env python3
"""🔨️ D3 helper: registers one `cross-semio-implementation` oracle entry in a subset's
`🧪️oracle/🔣️.json`, narrows/clears the matching `noOracleDecisions` entry, and leaves everything
else in the file untouched. Mirrors the pattern `📗️din16798`'s oracle registration already uses.
Validates the file is well-formed JSON both before and after. Idempotent: re-running with the same
oracle id replaces that entry rather than duplicating it.
"""
from __future__ import annotations

import json
import sys


def main() -> int:
    (
        oracle_json_path,
        oracle_id,
        capability,
        component_rel_path,
        rationale,
        no_oracle_decision_id,
    ) = sys.argv[1:7]

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
        "kind": "cross-semio-implementation",
        "engine": {"family": "none", "implementation": "in-repository second implementation", "version": "0"},
        "productionReachable": False,
        "networkDuringExecution": False,
    }
    data["oracles"] = [o for o in data.get("oracles", []) if o.get("id") != oracle_id] + [entry]

    if no_oracle_decision_id:
        data["noOracleDecisions"] = [d for d in data.get("noOracleDecisions", []) if d.get("id") != no_oracle_decision_id]

    with open(oracle_json_path, "w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=2, ensure_ascii=False)
        handle.write("\n")

    with open(oracle_json_path, "r", encoding="utf-8") as handle:
        json.load(handle)
    print(f"ok: {oracle_json_path} — oracle {oracle_id!r} registered, decision {no_oracle_decision_id!r} removed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
