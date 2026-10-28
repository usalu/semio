"""🌪️ Independently observe exact selected concept families and separate schema-function results."""
import ctypes
import hashlib
import importlib
import json
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
dependency = str(ticket / "🗑️generated/oracle")
if sys.platform == "win32":
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)


class AdvancedSweptOracle:
    def families(self, name):
        raise NotImplementedError

    def original(self, path, identity):
        raise NotImplementedError


class IfcOpenShellAdvancedSweptOracle(AdvancedSweptOracle):
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.ifcopenshell_wrapper as wrapper
        import ifcopenshell.validate
        self._library = ifcopenshell
        self._validation = ifcopenshell.validate
        self._schema = wrapper.schema_by_name("IFC4X3_ADD2")
        self._rules = importlib.import_module("ifcopenshell.express.rules.IFC4X3_ADD2")
        self.version = ifcopenshell.version

    def families(self, name):
        try:
            declaration = self._schema.declaration_by_name(name)
        except RuntimeError:
            return None
        names = []
        while declaration is not None:
            names.append(declaration.name().upper())
            declaration = declaration.supertype()
        return names

    def original(self, path, identity):
        model = self._library.open(str(path))
        representation = model.by_id(int(identity))
        items = [{"id": str(item.id()), "type": item.is_a(), "families": self.families(item.is_a())} for item in representation.Items]
        selected = representation.RepresentationIdentifier == "Body" and representation.RepresentationType == "AdvancedSweptSolid" and all(any(name in item["families"] for name in ("IFCSWEPTAREASOLID", "IFCSWEPTDISKSOLID")) for item in items)
        schema_function = bool(self._rules.IfcShapeRepresentationTypes(representation.RepresentationType, representation.Items))
        logger = self._validation.json_logger()
        self._validation.validate(model, logger, express_rules=True)
        return {"representation": str(representation.id()), "identifier": representation.RepresentationIdentifier, "type": representation.RepresentationType, "items": items, "selectedConceptStatus": "pass" if selected else "fail", "schemaFunction": schema_function, "fullExpressObservationErrors": len(logger.statements), "physicalGeometryEvaluated": False}


def main():
    directory = Path(sys.argv[1])
    official = Path(sys.argv[2])
    laws = json.loads((directory / "cases.json").read_text(encoding="utf-8"))
    provenance = json.loads((directory / "provenance.json").read_text(encoding="utf-8"))
    oracle = IfcOpenShellAdvancedSweptOracle()
    neutral = []
    for case in laws["cases"]:
        families = [oracle.families(name) for name in case["items"]]
        known = all(names is not None for names in families)
        accepted = known and all(any(name in names for name in ("IFCSWEPTAREASOLID", "IFCSWEPTDISKSOLID")) for names in families)
        actual = "not-applicable" if case["applicable"] is False else "incomplete" if case["applicable"] is None or not known else "pass" if case["identifier"] == "Body" and case["type"] == "AdvancedSweptSolid" and accepted else "fail"
        assert actual == case["expected"], (case["id"], actual)
        neutral.append({"id": case["id"], "selectedConceptStatus": actual, "externalFamilies": families})
    originals = []
    for row in provenance["originals"]:
        path = official / row["file"]
        result = oracle.original(path, row["representation"])
        assert result["selectedConceptStatus"] == row["expected"], (row, result)
        assert result["schemaFunction"] is True, (row, result)
        originals.append({"file": row["file"], "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **result})
    output = {"oracle": "ifcopenshell", "version": oracle.version, "nativeExecution": False, "nativeOutputCompared": False, "scope": "external declaration membership plus original-file parser/schema-function/full-EXPRESS observations; no geometry or full-view admission", "neutral": neutral, "originals": originals}
    Path(sys.argv[3]).write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG]", json.dumps({"version": oracle.version, "neutralMatched": len(neutral), "originals": [{"file": row["file"], "concept": row["selectedConceptStatus"], "function": row["schemaFunction"], "errors": row["fullExpressObservationErrors"]} for row in originals]}))


if __name__ == "__main__":
    main()
