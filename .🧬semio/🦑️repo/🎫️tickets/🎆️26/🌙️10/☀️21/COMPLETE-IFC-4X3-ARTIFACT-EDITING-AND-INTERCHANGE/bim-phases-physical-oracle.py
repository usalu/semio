"""🧪️ External phase-property observations use an owned physical-file interface."""
import hashlib
import json
from pathlib import Path
import sys

class PhaseOracle:
    def inspect(self, path):
        raise NotImplementedError

class IfcOpenShellOracle(PhaseOracle):
    def inspect(self, path):
        import ifcopenshell
        import ifcopenshell.validate
        model = ifcopenshell.open(str(path))
        project = model.by_type("IfcProject")[0]
        sets = [relation.RelatingPropertyDefinition for relation in project.IsDefinedBy if relation.RelatingPropertyDefinition.Name == "semio_BimProject"]
        assert len(sets) == 1
        properties = {prop.Name: prop for prop in sets[0].HasProperties}
        nominal = properties["PhaseCount"].NominalValue
        count = nominal.wrappedValue
        values = properties["PhaseNames"].ListValues if count else ()
        names = [value.wrappedValue for value in values]
        types = [value.is_a() for value in values]
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, logger, express_rules=True)
        return {"version": ifcopenshell.version, "schema": model.schema_identifier, "entities": len(list(model)), "phase": project.Phase, "count": count, "countType": nominal.is_a(), "phases": names, "itemTypes": types, "diagnostics": logger.statements}

def main():
    root = Path(__file__).parent / "🗑️generated" / "bim-phases-physical"
    inputs = json.loads((root / "input.json").read_text(encoding="utf-8"))
    oracle = IfcOpenShellOracle()
    rows = []
    for expected in inputs:
        path = root / (expected["id"] + ".ifc")
        row = {"id": expected["id"], "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **oracle.inspect(path)}
        rows.append(row)
        assert row["schema"] == "IFC4X3_ADD2"
        assert row["phase"] is None and row["countType"] == "IfcInteger"
        assert row["count"] == len(expected["phases"]) and row["phases"] == expected["phases"]
        assert all(item == "IfcText" for item in row["itemTypes"])
        assert not row["diagnostics"], row["diagnostics"]
    (root / "oracle.json").write_text(json.dumps({"python": sys.executable, "observations": rows}, indent=2, default=str), encoding="utf-8")
    print("[DEBUG] " + json.dumps([{"id": row["id"], "entities": row["entities"], "diagnostics": len(row["diagnostics"]), "version": row["version"]} for row in rows]))

if __name__ == "__main__":
    main()

