"""🧱️ Independent released entity inheritance observations for twenty family projections."""
from pathlib import Path
import ctypes
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

class FamilyMetadataOracle:
    """🧭️ Return independent name ancestry, never external entity classes."""
    def ancestors(self, name):
        raise NotImplementedError

class IfcOpenShellFamily(FamilyMetadataOracle):
    """🧊️ Read entity parents from the independent ADD2 implementation."""
    def __init__(self):
        import ifcopenshell
        from ifcopenshell import ifcopenshell_wrapper
        self.version = ifcopenshell.version
        self._schema = ifcopenshell_wrapper.schema_by_name("IFC4X3_ADD2")

    def ancestors(self, name):
        try:
            entity = self._schema.declaration_by_name(name.upper())
        except RuntimeError:
            return None
        result = []
        while entity:
            result.append(entity.name().upper())
            entity = entity.supertype()
        return result

base = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts/🧱️body"
cases = json.loads((base / "family-cases.json").read_text(encoding="utf-8-sig"))
oracle = IfcOpenShellFamily()
rows = []
for case in cases["cases"]:
    ancestry = [oracle.ancestors(name) for name in case["names"]]
    known = set(name for chain in ancestry if chain is not None for name in chain)
    if any(chain is None for chain in ancestry):
        result = "incomplete"
    elif case["policy"] == "tessellation":
        result = "pass" if "IFCTESSELLATEDFACESET" in known else "fail"
    else:
        allowed = bool(known & {"IFCEXTRUDEDAREASOLID", "IFCREVOLVEDAREASOLID"})
        excluded = bool(known & {"IFCEXTRUDEDAREASOLIDTAPERED", "IFCREVOLVEDAREASOLIDTAPERED"})
        result = "pass" if allowed and not excluded else "fail"
    rows.append({"id": case["id"], "ancestors": ancestry, "actual": result, "expected": case["expected"], "match": result == case["expected"]})
output = {"library": "IfcOpenShell", "version": oracle.version, "scope": "metadata union projection only", "physicalComplexValidityClaimed": False, "nativeExecuted": False, "rows": rows}
with (ticket / "🗑️generated/native-body-family-independent-oracle.json").open("x", encoding="utf-8") as file:
    json.dump(output, file, indent=2)
print("[DEBUG] " + json.dumps({"rows": len(rows), "matches": sum(row["match"] for row in rows)}))
assert len(rows) == 20 and all(row["match"] for row in rows)
