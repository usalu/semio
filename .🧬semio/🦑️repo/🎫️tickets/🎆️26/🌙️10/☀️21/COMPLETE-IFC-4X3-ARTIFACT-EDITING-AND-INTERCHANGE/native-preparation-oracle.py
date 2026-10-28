"""🎟️ Compare request-key admission through an independent SQL constraint implementation."""
import json
import sqlite3
import sys
from pathlib import Path


class RequestAdmissionOracle:
    def admit(self, rows):
        raise NotImplementedError


class SqliteRequestAdmissionOracle(RequestAdmissionOracle):
    def admit(self, rows):
        with sqlite3.connect(":memory:") as connection:
            connection.execute("create table targets (id text primary key check(length(id) between 1 and 20 and id not glob '*[^0-9]*' and substr(id,1,1) != '0' and (length(id)<20 or id<='18446744073709551615')), kind text check(kind in ('horizontal','vertical','cant')))")
            try:
                connection.executemany("insert into targets values (?, ?)", ((row["id"], row["kind"]) for row in rows))
            except sqlite3.IntegrityError:
                return False
            return 1 <= connection.execute("select count(*) from targets").fetchone()[0] <= 1_000_000


def main():
    laws = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    oracle = SqliteRequestAdmissionOracle()
    observations = []
    for case in laws["cases"]:
        admitted = oracle.admit(case["targets"])
        assert admitted == case["admitted"], (case["id"], admitted)
        observations.append({"id": case["id"], "admitted": admitted})
    output = {"oracle": "sqlite", "version": sqlite3.sqlite_version, "scope": "request-key/count/uniqueness/closed-row-policy reference only", "nativeExecution": False, "nativeOutputCompared": False, "conformance": "not-evaluated", "observations": observations}
    Path(sys.argv[2]).write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG] SQL request admission cases matched:", len(observations))


if __name__ == "__main__":
    main()
