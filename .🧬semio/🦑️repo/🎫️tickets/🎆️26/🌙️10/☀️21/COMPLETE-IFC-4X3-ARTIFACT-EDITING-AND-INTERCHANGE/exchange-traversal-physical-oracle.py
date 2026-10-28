"""🧪️ Physical exchange observations behind an owned source-path/diagnostics interface."""
import hashlib
import json
from pathlib import Path
import sys

class PhysicalOracle:
    def inspect(self, path):
        raise NotImplementedError

class IfcOpenShellOracle(PhysicalOracle):
    def inspect(self, path):
        import ifcopenshell
        import ifcopenshell.validate
        model = ifcopenshell.open(str(path))
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, logger, express_rules=True)
        return {"schema": model.schema_identifier, "entities": len(list(model)), "diagnostics": logger.statements, "version": ifcopenshell.version}

def main():
    root = Path(__file__).parent / "🗑️generated" / "traversal-physical"
    oracle = IfcOpenShellOracle()
    rows = []
    for name in ("bim", "fem"):
        path = root / (name + ".ifc")
        row = {"name": name, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **oracle.inspect(path)}
        rows.append(row)
    (root / "oracle.json").write_text(json.dumps({"python": sys.executable, "observations": rows}, indent=2, default=str), encoding="utf-8")
    print("[DEBUG] " + json.dumps([{"name": row["name"], "entities": row["entities"], "diagnostics": len(row["diagnostics"]), "version": row["version"]} for row in rows]))
    if any(row["diagnostics"] for row in rows):
        raise SystemExit(1)

if __name__ == "__main__":
    main()
