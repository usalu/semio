"""🚄️ Independent released metadata and exact original identities for native direct-clause laws."""
from pathlib import Path
import ctypes
import hashlib
import json
import sys

ticket = Path(__file__).resolve().parent
dependency = str(ticket / "🗑️generated/oracle")
if sys.platform == "win32":
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)
root = next(path for path in ticket.parents if (path / "✏️s").is_dir())
official = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official"


class DirectMetadataOracle:
    """🧭️ Test-tool-independent interface returning only owned ordinary observations."""
    def metadata(self, owner, attribute):
        raise NotImplementedError

    def branches(self):
        raise NotImplementedError

    def observe(self, path, segment):
        raise NotImplementedError


class IfcOpenShellDirectOracle(DirectMetadataOracle):
    """🧊️ External classes remain private to the independent test implementation."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.ifcopenshell_wrapper as wrapper
        self._library = ifcopenshell
        self._schema = wrapper.schema_by_name("IFC4X3_ADD2")
        self.version = ifcopenshell.version

    def metadata(self, owner, attribute):
        declaration = self._schema.declaration_by_name(owner)
        return any(field.name().upper() == attribute.upper() for field in declaration.attributes())

    def branches(self):
        declaration = self._schema.declaration_by_name("IfcCurveMeasureSelect")
        return [item.name().upper() for item in declaration.select_list()]

    def observe(self, path, segment):
        model = self._library.open(str(path))
        selected = model.by_id(int(segment))
        parameter = selected.DesignParameters
        result = {"parameters": str(parameter.id()), "businessLength": getattr(parameter, "SegmentLength", getattr(parameter, "HorizontalLength", None)), "paths": []}
        definition = selected.Representation
        if definition:
            for representation in definition.Representations:
                for curve in representation.Items:
                    result["paths"].append({"definition": str(definition.id()), "representation": str(representation.id()), "curve": str(curve.id()), "branch": curve.SegmentLength.is_a().upper(), "length": curve.SegmentLength.wrappedValue})
        return result


oracle = IfcOpenShellDirectOracle()
fields = [
    ("IfcAlignmentSegment", "DesignParameters"),
    ("IfcProduct", "Representation"),
    ("IfcProductRepresentation", "Representations"),
    ("IfcRepresentation", "Items"),
    ("IfcCurveSegment", "SegmentLength"),
    ("IfcAlignmentHorizontalSegment", "SegmentLength"),
    ("IfcAlignmentVerticalSegment", "HorizontalLength"),
    ("IfcAlignmentCantSegment", "HorizontalLength"),
]
rows = [{"id": owner + "." + field, "equal": oracle.metadata(owner, field)} for owner, field in fields]
branches = oracle.branches()
rows.extend({"id": name, "equal": name in branches} for name in ["IFCLENGTHMEASURE", "IFCPARAMETERVALUE"])
contract = json.loads((ticket / "alignment-direct-counterpart-vectors.json").read_text(encoding="utf-8-sig"))
for case in contract["physicalCases"]:
    path = official / case["file"]
    observed = oracle.observe(path, case["segment"])
    equal = hashlib.sha256(path.read_bytes()).hexdigest() == case["sha256"] and observed["parameters"] == case["parameters"] and observed["businessLength"] == 0
    if case["expected"] == "pass":
        equal = equal and len(observed["paths"]) == 1 and all(observed["paths"][0][key] == case[key] for key in ["definition", "representation", "curve", "branch"]) and observed["paths"][0]["length"] == 0
    else:
        equal = equal and observed["paths"] == []
    rows.append({"id": case["id"], "equal": equal, "observed": observed})
result = {"library": "IfcOpenShell", "version": oracle.version, "schema": "IFC4X3_ADD2", "rows": rows, "nativeExecuted": False, "fullExpressRerun": False, "nativeOutputCompared": False}
output = ticket / "🗑️generated/native-alignment-direct"
output.mkdir(exist_ok=True)
(output / "oracle.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
assert all(row["equal"] for row in rows)
print("[DEBUG] " + json.dumps({"version": oracle.version, "matched": len(rows), "nativeExecuted": False, "nativeOutputCompared": False}))

