"""🔬 Compare released form declarations with independently parsed IfcOpenShell PSD metadata."""
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(ticket / "🗑️generated" / "ifc083"))
import ifcopenshell
from ifcopenshell.util.pset import get_template


class TemplateOracle:
    def __init__(self):
        self.rows = {row.Name: row for model in get_template("IFC4X3_ADD2").templates for row in model.by_type("IfcPropertySetTemplate")}

    def entry(self, template, name):
        return next(row for row in self.rows[template].HasPropertyTemplates if row.Name == name)

    def domain(self, row):
        if row.PrimaryMeasureType:
            return row.PrimaryMeasureType
        if row.TemplateType.startswith("Q_"):
            entity = ifcopenshell.schema_by_name("IFC4X3_ADD2").declaration_by_name("IfcQuantity" + row.TemplateType[2:].title())
            attribute = next(item for item in entity.all_attributes() if item.name().endswith("Value"))
            return attribute.type_of_attribute().declared_type().name()
        return None


source = pathlib.Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any") / "🧬️schema/🏷️templates/🔣️.json"
catalog = json.loads(source.read_text("utf8"))
oracle = TemplateOracle()
checks = 0
mismatches = []
for template in catalog["templates"]:
    external = oracle.rows[template["name"]]
    if external.TemplateType != template["kind"]:
        mismatches.append([template["name"], "kind", external.TemplateType, template["kind"]])
    for entry in template["entries"]:
        row = oracle.entry(template["name"], entry["name"])
        checks += 1
        for key, actual, expected in [
            ("kind", entry["kind"], row.TemplateType),
            ("domain", entry["dataType"], oracle.domain(row)),
            ("secondary", entry.get("secondaryType"), row.SecondaryMeasureType),
        ]:
            if (actual or None) != (expected or None):
                mismatches.append([template["name"], entry["name"], key, actual, expected])
        if "enumeration" in entry:
            actual = entry["enumeration"]["literals"]
            expected = [str(item.wrappedValue) for item in row.Enumerators.EnumerationValues]
            if actual != expected:
                mismatches.append([template["name"], entry["name"], "literals", actual, expected])
result = {"implementation": "IfcOpenShell", "version": ifcopenshell.version, "templates": len(catalog["templates"]), "entryChecks": checks, "mismatches": mismatches, "nativeExecution": False, "scope": "Independent released declaration metadata; does not prove mounted controls or native execution."}
output = ticket / "🗑️generated/staged-template-form-oracle.json"
output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", "utf8")
print("[DEBUG] " + json.dumps({key: len(value) if key == "mismatches" else value for key, value in result.items()}))
