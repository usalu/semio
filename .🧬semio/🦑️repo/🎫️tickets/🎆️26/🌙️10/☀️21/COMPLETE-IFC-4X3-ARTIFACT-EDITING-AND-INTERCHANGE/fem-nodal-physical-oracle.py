"""🧪️ Optional force slots, bits and original action links are external observations."""
import hashlib
import json
from pathlib import Path
import struct
import sys

class NodalOracle:
    def inspect(self, path, expected):
        raise NotImplementedError

class IfcOpenShellOracle(NodalOracle):
    def inspect(self, path, expected):
        import ifcopenshell
        import ifcopenshell.validate
        model = ifcopenshell.open(str(path))
        forces = model.by_type("IfcStructuralLoadSingleForce")
        actions = model.by_type("IfcStructuralPointAction")
        assert len(forces) == len(actions) == 1
        force, action = forces[0], actions[0]
        selected = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"].index(expected["dof"])
        values = list(force)[1:]
        assert all(value is None for i, value in enumerate(values) if i != selected)
        assert values[selected] is not None
        actual_bits = str(struct.unpack("<Q", struct.pack("<d", values[selected]))[0])
        assert actual_bits == expected["bits"], (actual_bits, expected)
        links = [link for link in model.by_type("IfcRelConnectsStructuralActivity") if link.RelatedStructuralActivity == action]
        assert len(links) == 1 and links[0].RelatingElement.Name == expected["node"]
        assert action.AppliedLoad == force and force.Name == action.Name == expected["loadId"]
        assignments = [link for link in model.by_type("IfcRelAssignsToGroup") if action in link.RelatedObjects]
        assert len(assignments) == 1 and assignments[0].RelatingGroup.is_a() == "IfcStructuralLoadCase"
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, logger, express_rules=True)
        return {"id": expected["id"], "version": ifcopenshell.version, "schema": model.schema_identifier, "entities": len(list(model)), "force": force.id(), "action": action.id(), "target": links[0].RelatingElement.id(), "targetName": links[0].RelatingElement.Name, "case": assignments[0].RelatingGroup.id(), "components": values, "selectedBits": actual_bits, "componentTypes": [force.attribute_type(i + 1) for i in range(6)], "diagnostics": logger.statements}

def main():
    root = Path(__file__).parent / "🗑️generated" / "fem-nodal-physical"
    inputs = json.loads((root / "input.json").read_text(encoding="utf-8"))
    rows = []
    oracle = IfcOpenShellOracle()
    for expected in inputs:
        path = root / (expected["id"] + ".ifc")
        row = {"sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **oracle.inspect(path, expected)}
        rows.append(row)
        assert row["schema"] == "IFC4X3_ADD2" and not row["diagnostics"]
    (root / "oracle.json").write_text(json.dumps({"python": sys.executable, "observations": rows}, indent=2, default=str), encoding="utf-8")
    print("[DEBUG] " + json.dumps({"files": len(rows), "version": rows[0]["version"], "diagnostics": sum(len(row["diagnostics"]) for row in rows), "exactBits": True, "optionalSlots": True, "nodeActionCaseLinks": True}))

if __name__ == "__main__":
    main()

