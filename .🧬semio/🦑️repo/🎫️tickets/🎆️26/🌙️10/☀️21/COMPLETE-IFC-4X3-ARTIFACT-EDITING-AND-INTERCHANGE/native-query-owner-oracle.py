"""🧪️ External released metadata and SQL source ordering corroborate query outputs, not native funding."""
import hashlib
import json
from pathlib import Path
import sqlite3
import sys

class MetadataQueryOracle:
    def query(self, case):
        raise NotImplementedError

class IfcOpenShellSqliteOracle(MetadataQueryOracle):
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.ifcopenshell_wrapper as wrapper
        self.version = ifcopenshell.version
        self.schema = wrapper.schema_by_name("IFC4X3_ADD2")
        self.declarations = {entry.name().upper(): entry for entry in self.schema.declarations()}
    def fields(self, name, direct):
        declaration = self.declarations.get(name.upper())
        if declaration is None or not hasattr(declaration, "all_attributes"):
            return None
        result = []
        for index, attribute in enumerate(declaration.attributes() if direct else declaration.all_attributes()):
            owner = declaration
            while owner is not None and not any(field.name() == attribute.name() for field in owner.attributes()):
                owner = owner.supertype()
            result.append({"index": index, "name": attribute.name(), "owner": owner.name().upper() if owner else None, "optional": attribute.optional()})
        return result
    def query(self, case):
        entity = case["entity"]
        parts = [entity] + entity.get("complex", [])
        database = sqlite3.connect(":memory:")
        database.executescript("CREATE TABLE parts(part INTEGER,known INTEGER); CREATE TABLE fields(part INTEGER,idx INTEGER,name TEXT,owner TEXT,value TEXT);")
        for ordinal, part in enumerate(parts):
            fields = self.fields(part["name"], bool(entity.get("complex")))
            database.execute("INSERT INTO parts VALUES(?,?)", (ordinal, int(fields is not None)))
            for field in fields or []:
                value = part["args"][field["index"]] if field["index"] < len(part["args"]) else None
                database.execute("INSERT INTO fields VALUES(?,?,?,?,?)", (ordinal, field["index"], field["name"].upper(), field["owner"], value))
        matches = list(database.execute("SELECT part,idx,value FROM fields WHERE name=? AND (? IS NULL OR owner=?) ORDER BY part,idx LIMIT 2", (case["name"].upper(), case.get("owner", "").upper() or None, case.get("owner", "").upper() or None)))
        unknown = database.execute("SELECT count(*) FROM parts WHERE known=0").fetchone()[0] > 0
        database.close()
        return {"status": "unknown" if unknown else "ambiguous" if len(matches) == 2 else "unique" if matches else "missing", "values": [row[2] for row in matches], "pointers": ["/entities/7/" + ("complex/" + str(row[0]-1) + "/" if row[0] else "") + "args/" + str(row[1]) for row in matches]}

def main():
    ticket = Path(__file__).parent
    repo = ticket
    while not (repo / "✏️s").exists():
        repo = repo.parent
    path = repo / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🔎️query/🧪️tests/cases.json"
    fixture = json.loads(path.read_text(encoding="utf-8"))
    oracle = IfcOpenShellSqliteOracle()
    rows = []
    for case in fixture["cases"]:
        actual = oracle.query(case)
        assert actual == {key: case[key] for key in ("status", "values", "pointers")}, (case["id"], actual)
        rows.append({"id": case["id"], **actual})
    force = oracle.fields("IfcStructuralLoadSingleForce", True)
    assert [(field["name"], field["owner"], field["optional"]) for field in force] == [(name, "IFCSTRUCTURALLOADSINGLEFORCE", True) for name in ("ForceX", "ForceY", "ForceZ", "MomentX", "MomentY", "MomentZ")]
    result = {"python": sys.executable, "version": oracle.version, "sqlite": sqlite3.sqlite_version, "schema": "IFC4X3_ADD2", "fixtureSha256": hashlib.sha256(path.read_bytes()).hexdigest(), "queries": rows, "forceDeclarations": force, "nativeExecuted": False, "fundingCertified": False}
    output = ticket / "🗑️generated" / "native-query-owner-oracle.json"
    output.write_text(json.dumps(result, indent=2), encoding="utf-8")
    print("[DEBUG] " + json.dumps({"queries": len(rows), "optionalForceDeclarations": len(force), "version": oracle.version, "sqlite": sqlite3.sqlite_version, "nativeExecuted": False}))

if __name__ == "__main__":
    main()

