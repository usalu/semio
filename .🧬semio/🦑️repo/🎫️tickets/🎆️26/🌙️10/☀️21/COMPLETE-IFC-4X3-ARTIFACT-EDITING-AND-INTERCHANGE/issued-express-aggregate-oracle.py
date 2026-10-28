"""🧮️ Corroborate known aggregate algebra and preserve independent indeterminate-representation limits."""
import importlib.util
import json
import sqlite3
from pathlib import Path
ticket = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("issued_owner_oracle", ticket / "issued-express-owner-oracle.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
root = ticket.parents[6]
directory = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️coverage"
cases = json.loads((directory / "language.json").read_text(encoding="utf-8-sig"))

class AggregateOracle:
    def sql(self, case: dict) -> dict:
        if "error" in case:
            return dict(id=case["id"], status="unsupported-indeterminate-membership", reason="SQLite NULL set membership does not establish normative EXPRESS aggregate output completion")
        a, b = case["bindings"]["A"], case["bindings"]["B"]
        with sqlite3.connect(":memory:") as connection:
            if case["expression"] == "A * B":
                connection.execute("CREATE TABLE a(k TEXT,v TEXT)")
                connection.execute("CREATE TABLE b(k TEXT,v TEXT)")
                for name, source in (("a", a), ("b", b)):
                    connection.executemany("INSERT INTO " + name + " VALUES(?,?)", [(item["kind"], item["value"]) for item in source["items"]])
                set_result = a["aggregate"] == "SET" or b["aggregate"] == "SET"
                values = connection.execute("WITH l AS(SELECT k,v,COUNT(*) n FROM a GROUP BY k,v),r AS(SELECT k,v,COUNT(*) n FROM b GROUP BY k,v) SELECT l.k,l.v,CASE WHEN ? THEN 1 ELSE MIN(l.n,r.n) END FROM l JOIN r ON l.k=r.k AND l.v=r.v ORDER BY l.k,l.v", (set_result,)).fetchall()
                value = dict(kind="aggregate", aggregate="SET" if set_result else a["aggregate"], lower="1", items=[dict(kind=kind, value=word) for kind, word, count in values for _ in range(count)])
            elif case["expression"] == "SIZEOF(A + B)":
                connection.execute("CREATE TABLE names(value TEXT)")
                connection.executemany("INSERT INTO names VALUES(?)", [(a["value"],)] + [(item["value"],) for item in b["items"]])
                value = dict(kind="integer", value=str(connection.execute("SELECT COUNT(DISTINCT value) FROM names").fetchone()[0]))
            else:
                connection.execute("CREATE TABLE a(i INTEGER PRIMARY KEY,value TEXT)")
                connection.execute("CREATE TABLE b(i INTEGER PRIMARY KEY,value TEXT)")
                for name, source in (("a", a), ("b", b)):
                    connection.executemany("INSERT INTO " + name + " VALUES(?,?)", [(index, item.get("value")) for index, item in enumerate(source["items"])])
                false, unknown = connection.execute("SELECT SUM(CASE WHEN a.value<>b.value THEN 1 ELSE 0 END),SUM(a.value IS NULL OR b.value IS NULL) FROM a JOIN b USING(i)").fetchone()
                value = dict(kind="logical", value="false" if false else "unknown" if unknown else "true")
        return dict(id=case["id"], status="agree" if value == case["expected"] else "divergent-value", value=value)

    def ifc(self, case: dict) -> dict:
        def decode(value):
            if value["kind"] == "aggregate":
                items = [decode(item) for item in value["items"]]
                return module.rules.express_set(items) if value["aggregate"] == "SET" else items
            if value["kind"] == "integer":
                return int(value["value"])
            return value.get("value")
        a, b = (decode(case["bindings"][name]) for name in ("A", "B"))
        try:
            if case["expression"] == "A * B":
                raw = a * b
            elif case["expression"] == "SIZEOF(A + B)":
                raw = len(a + b)
            elif case["expression"] == "A = B":
                raw = a == b
            else:
                raw = a + b
            value = module.plain(raw)
            if case["expression"] == "SIZEOF(A + B)":
                status = "agree" if value == case["expected"] else "divergent-value"
            elif case["expression"] == "A * B":
                status = "agree-storage-only" if module.storage(value) == module.storage(case["expected"]) else "divergent-value"
            else:
                status = "unsupported-indeterminate-mapping"
            return dict(id=case["id"], status=status, value=value, runtimeType=type(raw).__name__)
        except Exception as error:
            return dict(id=case["id"], status="unsupported", error=dict(kind=type(error).__name__, message=str(error)))

oracle = AggregateOracle()
result = dict(ifcopenshell=module.ifcopenshell.version, sqlite=sqlite3.sqlite_version, nativeExecuted=False, totalLanguageConformance=False, sqliteRows=[oracle.sql(case) for case in cases], ifcRows=[oracle.ifc(case) for case in cases])
(ticket / "🗑️generated/issued-express-aggregate-oracle.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps(result))
