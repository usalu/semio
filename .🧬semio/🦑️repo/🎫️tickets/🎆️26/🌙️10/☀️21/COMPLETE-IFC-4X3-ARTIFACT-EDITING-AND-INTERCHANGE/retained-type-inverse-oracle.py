import json
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules

class ExpressOracle:
    def evidence(self):
        schema = ifcopenshell.ifcopenshell_wrapper.schema_by_name("IFC4X3_ADD2")
        axis_select = schema.declaration_by_name("IfcAxis2Placement")
        file = ifcopenshell.file(schema="IFC4X3_ADD2")
        point = file.create_entity("IfcCartesianPoint", (0.0, 0.0, 0.0))
        axis = file.create_entity("IfcAxis2Placement3D", point)
        return {
            "nativeExecution": False,
            "oracle": "IfcOpenShell " + ifcopenshell.version,
            "axis2PlacementMembers": [item.name() for item in axis_select.select_list()],
            "actualIfcOpenShellTypeof": sorted(rules.typeof(axis)),
            "bagSetIntersection": sorted(["x", "x"] * rules.express_set(["x"])),
            "scalarSetUnion": sorted("INTEGER" + rules.express_set(["INTEGER", "REAL", "NUMBER"])),
            "typeofSelectClosureProven": False,
        }

result = ExpressOracle().evidence()
output = Path(__file__).parent / "🗑️generated" / "retained-type-inverse-oracle.json"
output.write_text(json.dumps(result, indent=2), encoding="utf-8")
print(json.dumps(result))
