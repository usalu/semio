"""🧮️ Compare real physical population/inverse IDs through an independent interface."""
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

class PopulationOracle:
    """🧭️ External entities do not cross the test-tool interface."""
    def observe(self, path):
        raise NotImplementedError

class IfcOpenShellPopulation(PopulationOracle):
    """🧊️ Observe declared subtype cohorts and inverse object identities."""
    def __init__(self):
        import ifcopenshell
        self._library = ifcopenshell
        self.version = ifcopenshell.version

    def observe(self, path):
        model = self._library.open(str(path))
        target = model.by_id(1)
        return {"header": list(model.header.file_schema.schema_identifiers), "cohorts": {name: sorted(str(item.id()) for item in model.by_type(name)) for name in ["IfcMaterialDefinition", "IfcMaterialLayerSet", "IfcCartesianPoint", "IfcWall"]}, "inverseIds": sorted(str(item.id()) for item in target.ToMaterialLayerSet)}

oracle = IfcOpenShellPopulation()
rows = [{"count": count, **oracle.observe(ticket / f"🗑️generated/population-inverse-{count}.ifc")} for count in range(3)]
result = {"library": "IfcOpenShell", "version": oracle.version, "rows": rows, "fullExpressValidityClaimed": False, "nativeExecuted": False}
with (ticket / "🗑️generated/population-traversal-oracle.json").open("x", encoding="utf-8") as output:
    json.dump(result, output, indent=2)
print("[DEBUG] " + json.dumps(result))
assert [row["inverseIds"] for row in rows] == [[], ["10"], ["10", "11"]]
