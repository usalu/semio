"""🚆️ Corroborate terminal-clause declaring owners through independent released metadata."""
from pathlib import Path
import ctypes
import json
import sys

ticket = Path(__file__).resolve().parent
dependency = str(ticket / "🗑️generated/oracle")
if sys.platform == "win32":
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)

class MetadataOracle:
    """🧭️ Keep external schema classes behind an ordinary observation interface."""
    def owners(self, entity, attribute):
        raise NotImplementedError

class IfcOpenShellMetadata(MetadataOracle):
    """🧊️ Observe actual direct declarations along the released inheritance chain."""
    def __init__(self):
        import ifcopenshell
        from ifcopenshell import ifcopenshell_wrapper
        self.version = ifcopenshell.version
        self._schema = ifcopenshell_wrapper.schema_by_name("IFC4X3_ADD2")

    def owners(self, entity, attribute):
        declaration = self._schema.declaration_by_name(entity)
        rows = []
        while declaration:
            if any(item.name() == attribute for item in declaration.attributes()):
                rows.append(declaration.name())
            declaration = declaration.supertype()
        return rows

oracle = IfcOpenShellMetadata()
fields = [("IfcRelNests", "RelatingObject"), ("IfcRelNests", "RelatedObjects"), ("IfcAlignmentSegment", "DesignParameters"), ("IfcAlignmentHorizontalSegment", "SegmentLength"), ("IfcAlignmentVerticalSegment", "HorizontalLength"), ("IfcAlignmentCantSegment", "HorizontalLength")]
rows = [{"entity": entity, "attribute": attribute, "declaredOwners": oracle.owners(entity, attribute)} for entity, attribute in fields]
for row in rows:
    row["match"] = row["declaredOwners"] == [row["entity"]]
result = {"library": "IfcOpenShell", "version": oracle.version, "schema": "IFC4X3_ADD2", "rows": rows, "physicalValidityEvaluated": False, "nativeExecuted": False}
with (ticket / "🗑️generated/terminal-owner-oracle.json").open("x", encoding="utf-8") as output:
    json.dump(result, output, indent=2)
print("[DEBUG] " + json.dumps(result))
assert all(row["match"] for row in rows)
