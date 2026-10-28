"""🔢️ SQLite independently aggregates closed coverage evidence without source-validation authority."""
import json
import sqlite3
import sys
from pathlib import Path
from typing import Protocol

class CoverageOracle(Protocol):
    def summary(self, case: dict) -> dict: ...

class SqliteCoverageOracle:
    def summary(self, case: dict) -> dict:
        with sqlite3.connect(":memory:") as connection:
            connection.execute("CREATE TABLE evidence(applicable INTEGER, implemented INTEGER, status TEXT, failed INTEGER)")
            connection.executemany("INSERT INTO evidence VALUES(?,?,?,?)", [(row["applicable"], row["implementation"] == "implemented", row["status"], row["hasFailure"]) for row in case["rows"]])
            values = connection.execute("""SELECT COUNT(*), COALESCE(SUM(implemented=1 AND applicable=1),0), COALESCE(SUM(status='pass'),0), COALESCE(SUM(failed=1),0), COALESCE(SUM(status='not-applicable'),0), COALESCE(SUM(applicable IS NULL OR status='incomplete' OR (implemented=0 AND (applicable IS NULL OR applicable<>0))),0) FROM evidence""").fetchone()
        names = ("requested", "evaluated", "passed", "failed", "notApplicable", "incomplete")
        count = dict(zip(names, values))
        selected = case["scope"] == "selected-contracts"
        count.update(denominator=count["requested"] if selected else None, authority="explicit-selection" if selected else "unresolved-release")
        complete = selected and count["requested"] > 0 and count["incomplete"] == 0
        failure = count["failed"] > 0
        status = "fail" if failure else "incomplete" if not complete else "pass" if count["passed"] else "not-applicable"
        return dict(id=case["id"], coverage=count, complete=complete, valid=complete and not failure and count["passed"] > 0, hasFailure=failure, status=status)

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
directory = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧾️requirements"
cases = json.loads((directory / "cases.json").read_text(encoding="utf-8-sig"))
oracle: CoverageOracle = SqliteCoverageOracle()
rows = [oracle.summary(case) for case in cases["coverage"]]
actual = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8-sig"))["rows"]
assert rows == actual
print(json.dumps(dict(python=sys.version, sqlite=sqlite3.sqlite_version, nativeExecuted=False, coverageOnly=True, rows=rows), indent=2))
