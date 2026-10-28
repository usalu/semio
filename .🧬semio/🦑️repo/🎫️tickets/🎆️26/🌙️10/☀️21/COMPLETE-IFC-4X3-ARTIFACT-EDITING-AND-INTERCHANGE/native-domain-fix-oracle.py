"""🔬 Independent engineering sharing witness; native output is never substituted by reference output."""
import json
import pathlib
import sys

root = pathlib.Path(__file__).parent
sys.path.insert(0, str(root / "🗑️generated" / "ifc083"))
import ifcopenshell
import ifcopenshell.validate


def observe(path):
    model = ifcopenshell.open(str(path))
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, logger, express_rules=True)
    definitions = [r for r in model.by_type("IfcRelDefinesByProperties") if r.RelatingPropertyDefinition.is_a("IfcPropertySet") and r.RelatingPropertyDefinition.Name == "semio_FemSection"]
    material_rows = model.by_type("IfcRelAssociatesMaterial")
    sections = {r.RelatingPropertyDefinition.id() for r in definitions}
    materials = {r.RelatingMaterial.id() for r in material_rows}
    measures = []
    for identity in sorted(sections):
        definition = model.by_id(identity)
        measures.append({p.Name: p.NominalValue.wrappedValue for p in definition.HasProperties})
    return {"schema": model.schema_identifier, "nodes": len(model.by_type("IfcStructuralPointConnection")), "members": len(model.by_type("IfcStructuralCurveMember")), "sharedSections": len(sections), "sharedMaterials": len(materials), "sectionProperties": measures, "validation": logger.statements}


reference = root / "🗑️generated" / "ifc-inputs" / "fem-shared.ifc"
native = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else root / "🗑️generated" / "native-owned" / "native-fem-owned.ifc"
report = {"implementation": "IfcOpenShell", "version": ifcopenshell.version, "nativeExecuted": native.exists(), "nativePath": str(native), "reference": observe(reference)}
if native.exists():
    report["native"] = observe(native)
    assert report["native"]["members"] == 2
    assert report["native"]["sharedSections"] == report["native"]["sharedMaterials"] == 1
    assert report["native"]["sectionProperties"][0]["TorsionalConstant"] == 4e-5
    assert not report["native"]["validation"]
else:
    report["nativeSkipped"] = "No executed Rust owned-route output is available; reference output is not native evidence"
destination = root / "🗑️generated" / "native-domain-fix-oracle.json"
destination.write_text(json.dumps(report, indent=2, default=str), encoding="utf-8")
print("[DEBUG] Independent engineering sharing witness retained; nativeExecuted=" + str(report["nativeExecuted"]))
