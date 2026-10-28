"""🔬 Validate genuine mapper output with independent IFC schema, topology and complete EXPRESS."""
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(ticket / "🗑️generated" / "ifc083"))
import ifcopenshell
import ifcopenshell.ifcopenshell_wrapper as wrapper
import ifcopenshell.validate

output = ticket / "🗑️generated" / "fem-isolated"
schema = wrapper.schema_by_name("IFC4X3_ADD2")
inverse = next(row for row in schema.declaration_by_name("IfcStructuralPointConnection").all_inverse_attributes() if row.name() == "ConnectsStructuralMembers")
assert inverse.bound1() == 1
rows = []
for stage in ["before", "after"]:
    outcomes = json.loads((output / stage / "outcomes.json").read_text("utf8"))
    for row in outcomes["rows"]:
        diagnostics = []
        incidence = []
        if row["artifact"]:
            model = ifcopenshell.open(str(output / stage / (row["id"] + ".ifc")))
            logger = ifcopenshell.validate.json_logger()
            ifcopenshell.validate.validate(model, logger, express_rules=True)
            diagnostics = [{"attribute": item.get("attribute"), "message": item["message"]} for item in logger.statements]
            incidence = sorted((node.Name, len(node.ConnectsStructuralMembers)) for node in model.by_type("IfcStructuralPointConnection"))
            if stage == "after":
                assert row["code"] is None, row
                assert not diagnostics, (row["id"], diagnostics)
                assert all(count >= 1 for _, count in incidence), incidence
                assert len(model.by_type("IfcStructuralCurveMember")) == len(row["members"])
                assert len(incidence) == len(row["nodes"])
                (output / stage / (row["id"] + ".independent.ifc")).write_text(model.to_string(), encoding="utf8")
        if stage == "before" and row["code"] == "fem.isolated-node":
            assert any("ConnectsStructuralMembers" in (item["attribute"] or "") for item in diagnostics), row["id"]
        if stage == "after":
            assert row["actualCode"] == row["code"], row
            assert row["artifact"] == (row["code"] is None), row
        rows.append({"stage": stage, "id": row["id"], "artifact": row["artifact"], "code": row["actualCode"], "expressErrors": len(diagnostics), "incidence": incidence, "diagnostics": diagnostics})
result = {"implementation": "IfcOpenShell", "version": ifcopenshell.version, "requiredIncidentMembers": inverse.bound1(), "rows": rows, "nativeExecution": False}
(output / "oracle.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf8")
print(json.dumps({"implementation": result["implementation"], "version": result["version"], "requiredIncidentMembers": 1, "beforeArtifacts": sum(row["artifact"] for row in rows if row["stage"] == "before"), "afterArtifacts": sum(row["artifact"] for row in rows if row["stage"] == "after"), "afterExpressErrors": sum(row["expressErrors"] for row in rows if row["stage"] == "after")}))
