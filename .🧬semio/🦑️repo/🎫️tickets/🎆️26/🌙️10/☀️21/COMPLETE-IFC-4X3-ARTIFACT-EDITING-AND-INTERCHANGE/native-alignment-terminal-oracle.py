"""🚆️ Independent native-terminal input observations through an owned external-tool test interface."""
from pathlib import Path
import ctypes
import hashlib
import json
import math
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


class TerminalOracle:
    """🧭️ Return ordinary owned metadata and ordered-object observations only."""
    def declares(self, owner, attribute):
        raise NotImplementedError

    def observe(self, path, layout):
        raise NotImplementedError


class IfcOpenShellTerminalOracle(TerminalOracle):
    """🧊️ External entity/schema classes never leave this implementation."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.ifcopenshell_wrapper as wrapper
        self._library = ifcopenshell
        self._schema = wrapper.schema_by_name("IFC4X3_ADD2")
        self.version = ifcopenshell.version

    def declares(self, owner, attribute):
        return any(field.name().upper() == attribute.upper() for field in self._schema.declaration_by_name(owner).attributes())

    def observe(self, path, layout):
        model = self._library.open(str(path))
        selected = model.by_id(int(layout))
        relationships = [row for row in model.by_type("IfcRelNests") if row.RelatingObject.id() == selected.id()]
        lists = []
        for relationship in relationships:
            members = []
            for segment in relationship.RelatedObjects:
                parameter = segment.DesignParameters
                members.append({"segment": str(segment.id()), "segmentType": segment.is_a().upper(), "parameter": str(parameter.id()), "parameterType": parameter.is_a().upper(), "length": getattr(parameter, "SegmentLength", getattr(parameter, "HorizontalLength", None))})
            lists.append({"relationship": str(relationship.id()), "members": members})
        return {"layout": str(selected.id()), "layoutType": selected.is_a().upper(), "lists": lists}


def status(observation, kind, applicability):
    if applicability is False:
        return "not-applicable"
    if applicability is None or len(observation["lists"]) != 1:
        return "incomplete"
    members = observation["lists"][0]["members"]
    parameter = "IFCALIGNMENT" + kind.upper() + "SEGMENT"
    if not members or any(row["segmentType"] != "IFCALIGNMENTSEGMENT" or row["parameterType"] != parameter or not isinstance(row["length"], (int, float)) or not math.isfinite(row["length"]) for row in members):
        return "incomplete"
    return "pass" if members[-1]["length"] == 0 else "fail"


oracle = IfcOpenShellTerminalOracle()
fields = [("IfcRelNests", "RelatingObject"), ("IfcRelNests", "RelatedObjects"), ("IfcAlignmentSegment", "DesignParameters"), ("IfcAlignmentHorizontalSegment", "SegmentLength"), ("IfcAlignmentVerticalSegment", "HorizontalLength"), ("IfcAlignmentCantSegment", "HorizontalLength")]
rows = [{"id": owner + "." + field, "equal": oracle.declares(owner, field)} for owner, field in fields]
contract = json.loads((ticket / "alignment-terminal-cases.json").read_text(encoding="utf-8-sig"))
previous = json.loads((ticket / "🗑️generated/alignment-terminal/oracle.json").read_text(encoding="utf-8-sig"))
graph = ticket / "🗑️generated/alignment-terminal/graphs.ifc"
for case in contract["cases"]:
    layout = next(row["layoutId"] for row in previous["rows"] if row["id"] == case["id"])
    observed = oracle.observe(graph, layout)
    actual = status(observed, case["layout"], case["applicable"])
    rows.append({"id": case["id"], "equal": actual == case["expected"], "actual": actual, "observed": observed})
official_rows = json.loads((official / "cases.json").read_text(encoding="utf-8-sig"))["cases"]
for file, layout, kind in [("sectioned-solid-horizontal.ifc", "34", "horizontal"), ("sectioned-solid-horizontal.ifc", "45", "vertical"), ("segmented-reference-curve.ifc", "61", "cant"), ("linear-placement-of-signal.ifc", "2388", "cant")]:
    path = official / file
    expected = next(row["sha256"] for row in official_rows if row["file"] == file)
    observed = oracle.observe(path, layout)
    actual = status(observed, kind, True)
    rows.append({"id": file + ":" + layout, "equal": hashlib.sha256(path.read_bytes()).hexdigest() == expected and actual == "pass", "actual": actual, "observed": observed})
output = ticket / "🗑️generated/native-alignment-terminal"
output.mkdir(exist_ok=True)
(output / "oracle.json").write_text(json.dumps({"library": "IfcOpenShell", "version": oracle.version, "rows": rows, "inputGraphSha256": hashlib.sha256(graph.read_bytes()).hexdigest(), "nativeExecuted": False, "nativeOutputCompared": False, "fullExpressRerun": False}, indent=2), encoding="utf-8")
assert all(row["equal"] for row in rows)
print("[DEBUG] " + json.dumps({"version": oracle.version, "matched": len(rows), "nativeExecuted": False, "nativeOutputCompared": False}))

