"""🗂️ Independently compare exact prepared-population ID outcomes through a SQL implementation."""
import json
import sqlite3
import sys
from pathlib import Path


class PopulationLookupOracle:
    def observe(self, entities, identities):
        raise NotImplementedError


class SqlitePopulationLookupOracle(PopulationLookupOracle):
    def observe(self, entities, identities):
        with sqlite3.connect(":memory:") as connection:
            connection.execute("create table source (id text not null, ordinal integer not null)")
            connection.executemany("insert into source values (?, ?)", ((row["id"], ordinal) for ordinal, row in enumerate(entities)))
            results = []
            for identity in identities:
                count, ordinal = connection.execute("select count(*), min(ordinal) from source where id = ?", (identity,)).fetchone()
                result = {"id": identity, "status": "missing" if count == 0 else "unique" if count == 1 else "ambiguous"}
                if count == 1:
                    result["ordinal"] = ordinal
                results.append(result)
            return results


def main():
    source = Path(sys.argv[1])
    destination = Path(sys.argv[2])
    laws = json.loads(source.read_text(encoding="utf-8"))
    oracle = SqlitePopulationLookupOracle()
    observations = []
    for case in laws["cases"]:
        if case["expected"]["container"] != "prepared":
            observations.append({"id": case["id"], "scope": "not-observed-container-validation"})
            continue
        actual = oracle.observe(case["entities"], case["queries"])
        assert actual == case["expected"]["matches"], (case["id"], actual)
        observations.append({"id": case["id"], "scope": "exact-id-lookup", "matches": actual})
    output = {"oracle": "sqlite", "version": sqlite3.sqlite_version, "identityStorage": "exact UTF-8 decimal TEXT", "conformance": "not-evaluated", "nativeExecution": False, "nativeOutputCompared": False, "matchedPreparedCases": sum(row["scope"] == "exact-id-lookup" for row in observations), "observations": observations}
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG] SQLite prepared-population lookup cases matched:", output["matchedPreparedCases"])


if __name__ == "__main__":
    main()
