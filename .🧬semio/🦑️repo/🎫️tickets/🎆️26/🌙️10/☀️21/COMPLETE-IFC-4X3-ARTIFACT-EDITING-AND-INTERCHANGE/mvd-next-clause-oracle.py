"""🏗️ Pin one released clause and observe original swept-body representations."""
from pathlib import Path
import ctypes
import hashlib
import json
import sys
import urllib.request

ticket = Path(__file__).resolve().parent
root = next(path for path in ticket.parents if (path / "✏️s").is_dir())
output = ticket / "🗑️generated/mvd-next"
output.mkdir(exist_ok=True)
commit = "610ee3f95db9d0380b1eb8de4dc6e17833531518"
url = f"https://raw.githubusercontent.com/buildingSMART/Standards-builds/{commit}/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry/content.html"
with urllib.request.urlopen(url) as response:
    data = response.read()
(output / "swept-solid.html").write_bytes(data)
dependency = str(ticket / "🗑️generated/oracle")
if sys.platform == "win32":
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)
import ifcopenshell
base = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"
rows = []
for filename in ["extruded-solid.ifc", "slab-openings.ifc"]:
    path = base / "🧪️fixtures/official" / filename
    model = ifcopenshell.open(str(path))
    representations = []
    for representation in model.by_type("IfcShapeRepresentation"):
        if representation.RepresentationIdentifier != "Body":
            continue
        representations.append({"id": representation.id(), "identifier": representation.RepresentationIdentifier, "type": representation.RepresentationType, "items": [{"id": item.id(), "class": item.is_a(), "accepted": item.is_a("IfcExtrudedAreaSolid") or item.is_a("IfcRevolvedAreaSolid")} for item in representation.Items]})
    rows.append({"file": filename, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "header": list(model.header.file_schema.schema_identifiers), "representations": representations})
result = {"url": url, "commit": commit, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data), "library": "IfcOpenShell", "version": ifcopenshell.version, "originals": rows, "fullExpressValidityClaimed": False, "nativeExecuted": False}
with (output / "source-oracle.json").open("x", encoding="utf-8") as file:
    json.dump(result, file, indent=2)
print("[DEBUG] " + json.dumps(result))
