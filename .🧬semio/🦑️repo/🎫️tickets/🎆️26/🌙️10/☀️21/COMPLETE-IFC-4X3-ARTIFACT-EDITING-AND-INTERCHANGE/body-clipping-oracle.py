"""✂️ Independently observe selected Clipping membership and separate schema-rule boundaries."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("semio_surface_oracle", ticket / "body-surface-model-oracle.py")
backend = importlib.util.module_from_spec(spec)
spec.loader.exec_module(backend)


class ClippingOracle:
    def families(self, name):
        raise NotImplementedError

    def observe(self, path, identity, authored):
        raise NotImplementedError


class IfcOpenShellClippingOracle(backend.IfcOpenShellSurfaceModelOracle, ClippingOracle):
    def observe(self, path, identity, authored):
        model = self._library.open(str(path))
        representation = model.by_id(int(identity))
        items = [{"id": str(item.id()), "type": item.is_a(), "families": self.families(item.is_a())} for item in representation.Items]
        selected = representation.RepresentationIdentifier == "Body" and representation.RepresentationType == "Clipping" and all("IFCBOOLEANCLIPPINGRESULT" in row["families"] for row in items)
        logger = self._validation.json_logger()
        self._validation.validate(model, logger, express_rules=True)
        result = {"representation": str(representation.id()), "identifier": representation.RepresentationIdentifier, "type": representation.RepresentationType, "items": items, "selectedConceptStatus": "pass" if selected else "fail", "schemaFunctionForActualType": bool(self._rules.IfcShapeRepresentationTypes(representation.RepresentationType, representation.Items)), "schemaFunctionForClipping": bool(self._rules.IfcShapeRepresentationTypes("Clipping", representation.Items)), "fullExpressObservationErrors": len(logger.statements), "physicalGeometryEvaluated": False}
        result["fullExpressObservationDiagnostics"] = [{"message": str(row.get("message", "")), "instance": str(row.get("instance", ""))} for row in logger.statements]
        if authored:
            model.by_id(14).Operator = "UNION"
            logger = self._validation.json_logger()
            self._validation.validate(model, logger, express_rules=True)
            assert len(logger.statements) > 0
            result["counterexample"] = {"mutation": "authored item14 Operator=UNION", "selectedItemFamilyUnchanged": True, "fullExpressObservationErrors": len(logger.statements)}
        return result


def main():
    directory, official, destination = map(Path, sys.argv[1:4])
    laws = json.loads((directory / "cases.json").read_text(encoding="utf-8"))
    provenance = json.loads((directory / "provenance.json").read_text(encoding="utf-8"))
    oracle = IfcOpenShellClippingOracle()
    neutral = []
    for row in laws["cases"]:
        families = [oracle.families(name) for name in row["items"]]
        known = all(names is not None for names in families)
        accepted = known and all("IFCBOOLEANCLIPPINGRESULT" in names for names in families)
        actual = "not-applicable" if row["applicable"] is False else "incomplete" if row["applicable"] is None or not known else "pass" if row["identifier"] == "Body" and row["type"] == "Clipping" and accepted else "fail"
        assert actual == row["expected"], (row["id"], actual)
        neutral.append({"id": row["id"], "selectedConceptStatus": actual, "externalFamilies": families})
    inputs = []
    for authored, rows in ((False, provenance["originals"]), (True, provenance["authored"])):
        for row in rows:
            path = (directory if authored else official) / row["file"]
            result = oracle.observe(path, row["representation"], authored)
            assert result["selectedConceptStatus"] == row["expected"]
            assert result["schemaFunctionForActualType"] and result["schemaFunctionForClipping"]
            assert result["fullExpressObservationErrors"] == 0, (row, result)
            inputs.append({"file": row["file"], "authored": authored, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **result})
    output = {"oracle": "ifcopenshell", "version": oracle.version, "nativeExecution": False, "nativeOutputCompared": False, "scope": "metadata and original/authored schema observations; no geometry/full-view admission", "neutral": neutral, "inputs": inputs}
    destination.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG]", json.dumps({"version": oracle.version, "neutralMatched": len(neutral), "inputs": [{"file": row["file"], "authored": row["authored"], "concept": row["selectedConceptStatus"], "function": row["schemaFunctionForClipping"], "errors": row["fullExpressObservationErrors"], "counterexample": row.get("counterexample")} for row in inputs]}))


if __name__ == "__main__":
    main()
