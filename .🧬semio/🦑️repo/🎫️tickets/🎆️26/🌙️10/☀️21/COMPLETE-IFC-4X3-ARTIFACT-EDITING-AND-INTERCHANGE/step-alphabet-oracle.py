"""🔬 Compare original alphabet source, owned output and native IFC authoring."""
import ctypes
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
MODULES = str(TICKET / "🗑️generated/oracle")
if sys.platform == "win32":
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(MODULES, short, len(short))
    MODULES = short.value or MODULES
sys.path.insert(0, MODULES)
import ifcopenshell

OUTPUT = TICKET / "🗑️generated/step-alphabets"
expected = json.loads((OUTPUT / "expected.json").read_text(encoding="utf-8"))
results = []
for source in ("source", "owned"):
    model = ifcopenshell.file.from_string((OUTPUT / (source + ".ifc")).read_text(encoding="utf-8"))
    actual = {entity.Name: entity.NominalValue.wrappedValue for entity in model}
    differences = [{"name": row["name"], "expected": row["value"], "actual": actual.get(row["name"])} for row in expected if actual.get(row["name"]) != row["value"]]
    if source == "owned":
        assert not differences, differences
    results.append({"input": source, "entities": len(actual), "differences": differences})
authored = ifcopenshell.file(schema="IFC4X3_ADD2")
for row in expected:
    authored.create_entity("IfcPropertySingleValue", Name=row["name"], NominalValue=authored.create_entity("IfcText", row["value"]))
(OUTPUT / "independent.ifc").write_text(authored.to_string(), encoding="utf-8")
result = {"oracle": "IfcOpenShell", "version": ifcopenshell.version, "defined": len(expected), "nativeAuthored": len(list(authored)), "inputs": results}
(OUTPUT / "oracle.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] " + json.dumps({"oracle": ifcopenshell.version, "defined": len(expected), "sourceDifferences": len(results[0]["differences"]), "ownedDifferences": len(results[1]["differences"])}))
