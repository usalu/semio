"""🧪️ One unchanged authored zero-force action is observed behind a physical source interface."""
import hashlib
import json
from pathlib import Path
import sys

class ZeroForceOracle:
    def inspect(self, path):
        raise NotImplementedError

class IfcOpenShellOracle(ZeroForceOracle):
    def inspect(self, path):
        import ifcopenshell
        import ifcopenshell.validate
        model = ifcopenshell.open(str(path))
        forces = model.by_type("IfcStructuralLoadSingleForce")
        actions = model.by_type("IfcStructuralPointAction")
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, logger, express_rules=True)
        return {"version": ifcopenshell.version, "schema": model.schema_identifier, "entities": len(list(model)), "forces": [{"id": force.id(), "name": force.Name, "components": list(force)[1:]} for force in forces], "actions": [{"id": action.id(), "name": action.Name, "appliedLoad": action.AppliedLoad.id()} for action in actions], "diagnostics": logger.statements}

def main():
    root = Path(__file__).parent / "🗑️generated" / "fem-zero-record"
    path = root / "zero.ifc"
    result = {"python": sys.executable, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **IfcOpenShellOracle().inspect(path)}
    (root / "external-before.json").write_text(json.dumps(result, indent=2, default=str), encoding="utf-8")
    assert result["schema"] == "IFC4X3_ADD2" and len(result["forces"]) == len(result["actions"]) == 1
    assert result["forces"][0]["components"] == [0, 0, 0, 0, 0, 0]
    assert not result["diagnostics"]
    print("[DEBUG] " + json.dumps({"version": result["version"], "forces": len(result["forces"]), "actions": len(result["actions"]), "diagnostics": len(result["diagnostics"])}))

if __name__ == "__main__":
    main()

