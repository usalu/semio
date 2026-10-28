"""🔍️ Inspect the installed independent function interface before defining supported oracle claims."""
import inspect
import json
from pathlib import Path
import ifcopenshell
import ifcopenshell.express.rules.IFC4X3_ADD2 as rules

class IssuedExpressOracle:
    def interface(self) -> dict:
        names = ["IfcListToArray", "IfcMakeArrayOfArray", "IfcUniquePropertyName", "IfcUniquePropertySetNames", "IfcUniquePropertyTemplateNames", "IfcUniqueQuantityNames", "IfcUniqueDefinitionNames", "IfcSameValue", "IfcSameValidPrecision", "IfcSingleProjectInstance", "IfcRepresentationContextSameWCS"]
        functions = []
        for name in names:
            function = getattr(rules, name, None)
            functions.append(dict(name=name, available=callable(function), source=inspect.getsource(function) if callable(function) else None))
        return dict(ifcopenshell=ifcopenshell.version, functions=functions)

result = IssuedExpressOracle().interface()
(Path(__file__).parent / "🗑️generated/issued-express-oracle-interface.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps(result))
