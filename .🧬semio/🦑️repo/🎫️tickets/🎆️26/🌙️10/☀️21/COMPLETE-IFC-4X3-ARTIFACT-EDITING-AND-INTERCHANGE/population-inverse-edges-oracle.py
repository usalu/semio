"""🪢️ Duplicate edge and wrong-role inverse observations on parsed physical input."""
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
import ifcopenshell

text = "ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('audit.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;#1=IFCMATERIALLAYER($,1.,$,$,$,$,$);#10=IFCMATERIALLAYERSET((#1,#1),$,$);#11=IFCMATERIALPROFILESET($,$,(#1),$);ENDSEC;END-ISO-10303-21;"
model = ifcopenshell.file.from_string(text)
target = model.by_id(1)
result = {"library": "IfcOpenShell", "version": ifcopenshell.version, "inverseIds": sorted(item.id() for item in target.ToMaterialLayerSet), "allIncomingOwnerIds": sorted(item.id() for item in model.get_inverse(target)), "wrongRoleIsIntentionallyInvalidDomain": True, "fullExpressValidityClaimed": False, "nativeExecuted": False}
with (ticket / "🗑️generated/population-inverse-edges-observed.ifc").open("x", encoding="utf-8") as output:
    output.write(text)
with (ticket / "🗑️generated/population-inverse-edges-observed-oracle.json").open("x", encoding="utf-8") as output:
    json.dump(result, output, indent=2)
print("[DEBUG] " + json.dumps(result))
assert result["inverseIds"] == [10, 10]
assert result["allIncomingOwnerIds"] == [10, 11]
