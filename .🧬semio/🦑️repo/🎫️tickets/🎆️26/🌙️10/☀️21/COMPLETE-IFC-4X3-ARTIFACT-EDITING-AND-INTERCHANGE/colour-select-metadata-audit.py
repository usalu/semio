import json
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules

class MetadataOracle:
    def inspect(self):
        schema = ifcopenshell.ifcopenshell_wrapper.schema_by_name("IFC4X3_ADD2")
        rgb = schema.declaration_by_name("IfcColourRgb")
        ancestry = []
        current = rgb
        while current:
            ancestry.append(current.name())
            current = current.supertype()
        colour = [item.name() for item in schema.declaration_by_name("IfcColour").select_list()]
        fill = [item.name() for item in schema.declaration_by_name("IfcFillStyleSelect").select_list()]
        file = ifcopenshell.file(schema="IFC4X3_ADD2")
        value = file.create_entity("IfcColourRgb", None, 1.0, 0.0, 0.0)
        assert set(ancestry) & set(colour)
        assert "IfcColour" in fill
        return {"nativeExecution": False, "metadataOracle": "IfcOpenShell " + ifcopenshell.version,
                "rgbAncestry": ancestry, "colourMembers": colour, "fillStyleMembers": fill,
                "metadataImpliesColourMembership": True, "actualHelperTypeof": sorted(rules.typeof(value)),
                "helperIsNormativeOracle": False}

result = MetadataOracle().inspect()
path = Path(__file__).parent / "🗑️generated" / "colour-select" / "independent-metadata.json"
path.write_text(json.dumps(result, indent=2), encoding="utf-8")
print(json.dumps(result))
