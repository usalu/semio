"""⚖️ Observe actual independent schema helpers and global validation with explicit unsupported and divergent outcomes."""
import hashlib
import json
import sqlite3
import sys
from pathlib import Path
from typing import Protocol
import ifcopenshell
import ifcopenshell.validate
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules

class IssuedExpressOracle(Protocol):
    def function(self, case: dict) -> dict: ...
    def global_rule(self, case: dict, source: str) -> dict: ...

def plain(value):
    if value is None:
        return dict(kind="unknown")
    if isinstance(value, bool):
        return dict(kind="logical", value=str(value).lower())
    if isinstance(value, rules.express_set):
        return dict(kind="aggregate", aggregate="SET", items=[plain(item) for item in value])
    if isinstance(value, (list, tuple)):
        return dict(kind="aggregate", items=[plain(item) for item in value])
    if isinstance(value, int):
        return dict(kind="integer", value=str(value))
    if isinstance(value, float):
        return dict(kind="real", value=str(value))
    if isinstance(value, str):
        return dict(kind="string", value=value)
    raise TypeError("Independent value has no supported observation encoding: " + type(value).__name__)

def storage(value):
    if value["kind"] == "aggregate":
        return dict(kind="aggregate", items=[storage(item) for item in value["items"]])
    return value

class IfcOpenShellOracle:
    def function(self, case: dict) -> dict:
        file = ifcopenshell.file(schema="IFC4X3_ADD2")
        identities = {}
        def decode(row):
            kind = row["kind"]
            if kind == "unknown":
                return None
            if kind == "integer":
                return int(row["value"])
            if kind == "real":
                return float(row["value"])
            if kind == "string":
                return row["value"]
            if kind == "reference":
                return identities[row["key"]]
            if kind == "aggregate":
                items = [decode(item) for item in row["items"]]
                return rules.express_set(items) if row["aggregate"] == "SET" else items
            if kind != "entity":
                raise TypeError("Unsupported independent input kind")
            entity = file.create_entity(row["type"])
            for name, item in row["fields"].items():
                setattr(entity, name, decode(item))
            if "key" in row:
                identities[row["key"]] = entity
            return entity
        result = dict(id=case["id"], owner=case["owner"], arrayBoundsReported=False)
        try:
            name = next(name for name in dir(rules) if name.upper() == case["owner"])
            value = plain(getattr(rules, name)(*[decode(item) for item in case["args"]]))
            same = storage(value) == storage(case["expected"])
            result.update(value=value, status=("agree-storage-only" if value["kind"] == "aggregate" else "agree") if same else "divergent-value")
        except Exception as error:
            result.update(status="unsupported", error=dict(kind=type(error).__name__, message=str(error)))
        return result

    def global_rule(self, case: dict, source: str) -> dict:
        file = ifcopenshell.file.from_string(source)
        result = dict(id=case["id"], owner=case["owner"], inputSha256=hashlib.sha256(source.encode()).hexdigest(), entities=len(list(file)))
        try:
            name = next(name for name in dir(rules) if name.upper() == case["owner"])
            try:
                getattr(rules, name).__call__(file)
                observed = "true"
            except AssertionError:
                observed = "false"
            result.update(value=observed, status="agree" if observed == case["expected"] else "divergent-value")
        except Exception as error:
            result.update(status="unsupported", error=dict(kind=type(error).__name__, message=str(error)))
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(file, logger, express_rules=True)
        result["fullExpressDiagnostics"] = logger.statements
        return result

class SqliteArrayOracle:
    def function(self, case: dict) -> dict:
        args = case["args"]
        lower, upper = int(args[1]["value"]), int(args[2]["value"])
        with sqlite3.connect(":memory:") as connection:
            connection.execute("CREATE TABLE rows(r INTEGER PRIMARY KEY,n INTEGER)")
            connection.execute("CREATE TABLE items(r INTEGER,c INTEGER,value TEXT)")
            matrix = case["owner"] == "IFCMAKEARRAYOFARRAY"
            source_rows = args[0]["items"] if matrix else [args[0]]
            for r, row in enumerate(source_rows):
                connection.execute("INSERT INTO rows VALUES(?,?)", (r, len(row["items"])))
                connection.executemany("INSERT INTO items VALUES(?,?,?)", [(r, c, json.dumps(value)) for c, value in enumerate(row["items"])])
            if matrix:
                low2, upper2 = int(args[3]["value"]), int(args[4]["value"])
                valid = connection.execute("SELECT COUNT(*)=?-?+1 AND MIN(n)=?-?+1 AND MAX(n)=?-?+1 FROM rows", (upper, lower, upper2, low2, upper2, low2)).fetchone()[0]
                if valid:
                    arrays = []
                    for r, in connection.execute("SELECT r FROM rows ORDER BY r"):
                        arrays.append(dict(kind="aggregate", aggregate="ARRAY", lower=str(low2), items=[json.loads(value) for _, value in connection.execute("SELECT ?+c,value FROM items WHERE r=? ORDER BY c", (low2, r))]))
                    value = dict(kind="aggregate", aggregate="ARRAY", lower=str(lower), items=arrays)
                else:
                    value = dict(kind="unknown")
            else:
                valid = connection.execute("SELECT COUNT(*)=?-?+1 FROM items", (upper, lower)).fetchone()[0]
                value = dict(kind="aggregate", aggregate="ARRAY", lower=str(lower), items=[json.loads(value) for _, value in connection.execute("SELECT ?+c,value FROM items ORDER BY c", (lower,))]) if valid else dict(kind="unknown")
        return dict(id=case["id"], owner=case["owner"], status="agree" if value == case["expected"] else "divergent-value", value=value, scope="ordered-storage-and-declared-bounds-not-all-express")

def main():
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    directory = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation"
    cases = json.loads((directory / "🧪️coverage/cases.json").read_text(encoding="utf-8-sig"))
    model = json.loads((directory / "models.json").read_text(encoding="utf-8-sig"))
    oracle: IssuedExpressOracle = IfcOpenShellOracle()
    owners = json.loads((directory / "🧪️coverage/owners.json").read_text(encoding="utf-8-sig"))
    metadata = [dict(owner=row["owner"], issuedIndex=row["issuedIndex"], available=any(name.upper()==row["owner"] and callable(getattr(rules,name)) for name in dir(rules))) for row in owners["owners"]]
    result = dict(ownerInterfaces=metadata, python=sys.version, ifcopenshell=ifcopenshell.version, sqlite=sqlite3.sqlite_version, nativeExecuted=False, totalOwnerCoverage=False, functions=[oracle.function(case) for case in cases["functions"]], globals=[oracle.global_rule(case, model["header"] + case["body"] + model["footer"]) for case in cases["globals"]], arrays=[SqliteArrayOracle().function(case) for case in cases["functions"] if case["owner"] in ("IFCLISTTOARRAY", "IFCMAKEARRAYOFARRAY")])
    (ticket / "🗑️generated/issued-express-owner-oracle.json").write_text(json.dumps(result, indent=2, default=str), encoding="utf-8")
    print("[DEBUG] " + json.dumps(dict(functions={status:sum(row["status"]==status for row in result["functions"]) for status in sorted(set(row["status"] for row in result["functions"]))}, globals={status:sum(row["status"]==status for row in result["globals"]) for status in sorted(set(row["status"] for row in result["globals"]))}, arrays={status:sum(row["status"]==status for row in result["arrays"]) for status in sorted(set(row["status"] for row in result["arrays"]))})))

if __name__ == "__main__":
    main()

