"""🏗️ Observe actual swept/tapered representation classification and full EXPRESS diagnostics."""
from pathlib import Path
import ctypes
import hashlib
import json
import sys

ticket = Path(__file__).resolve().parent
root = next(path for path in ticket.parents if (path / "✏️s").is_dir())
dependency = str(ticket / "🗑️generated/oracle")
if sys.platform == "win32":
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)

class SweptOracle:
    """🧭️ Return ordinary semantic observations rather than external entity objects."""
    def observe(self, item_kind, representation_type):
        raise NotImplementedError

class IfcOpenShellSwept(SweptOracle):
    """🧊️ Independent inherited roles and complete validation of original-derived models."""
    def __init__(self, source, output):
        import ifcopenshell
        import ifcopenshell.validate
        self._library = ifcopenshell
        self._validation = ifcopenshell.validate
        self.source = source
        self.output = output
        self.version = ifcopenshell.version

    def observe(self, item_kind, representation_type):
        model = self._library.open(str(self.source))
        original = model.by_id(1021)
        representation = model.by_id(1020)
        if item_kind == "IfcExtrudedAreaSolid":
            item = original
        elif item_kind == "IfcExtrudedAreaSolidTapered":
            item = model.create_entity(item_kind, SweptArea=original.SweptArea, Position=original.Position, ExtrudedDirection=original.ExtrudedDirection, Depth=original.Depth, EndSweptArea=original.SweptArea)
        else:
            point = model.create_entity("IfcCartesianPoint", Coordinates=(0.0,0.0,0.0))
            direction = model.create_entity("IfcDirection", DirectionRatios=(0.0,1.0,0.0))
            axis = model.create_entity("IfcAxis1Placement", Location=point, Axis=direction)
            arguments = dict(SweptArea=original.SweptArea, Position=original.Position, Axis=axis, Angle=1.5707963267948966)
            if item_kind.endswith("Tapered"):
                arguments["EndSweptArea"] = original.SweptArea
            item = model.create_entity(item_kind, **arguments)
        representation.Items = (item,)
        representation.RepresentationType = representation_type
        logger = self._validation.json_logger()
        self._validation.validate(model, logger, express_rules=True)
        diagnostics = [{"attribute": str(row.get("attribute", "")), "message": str(row.get("message", "")), "instance": str(row.get("instance", ""))} for row in logger.statements]
        identifier = item_kind + "-" + representation_type
        model.write(str(self.output / (identifier + ".ifc")))
        return {"id": identifier, "class": item.is_a(), "isExtruded": item.is_a("IfcExtrudedAreaSolid"), "isRevolved": item.is_a("IfcRevolvedAreaSolid"), "isSweptArea": item.is_a("IfcSweptAreaSolid"), "isExtrudedTapered": item.is_a("IfcExtrudedAreaSolidTapered"), "isRevolvedTapered": item.is_a("IfcRevolvedAreaSolidTapered"), "representation": representation.id(), "diagnostics": diagnostics, "fullExpressValid": len(diagnostics)==0}

base = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"
source = base / "🧪️fixtures/official/extruded-solid.ifc"
output = ticket / "🗑️generated/mvd-next"
oracle = IfcOpenShellSwept(source, output)
rows = [oracle.observe(kind, representation_type) for kind in ["IfcExtrudedAreaSolid", "IfcExtrudedAreaSolidTapered", "IfcRevolvedAreaSolid", "IfcRevolvedAreaSolidTapered"] for representation_type in ["SweptSolid", "AdvancedSweptSolid"]]
result = {"library": "IfcOpenShell", "version": oracle.version, "sourceSha256": hashlib.sha256(source.read_bytes()).hexdigest(), "rows": rows, "nativeExecuted": False}
with (output / "tapered-oracle.json").open("x", encoding="utf-8") as file:
    json.dump(result, file, indent=2)
print("[DEBUG] " + json.dumps([{ "id": row["id"], "valid": row["fullExpressValid"], "attributes": [item["attribute"] for item in row["diagnostics"]]} for row in rows]))
