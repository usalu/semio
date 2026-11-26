"""📛️ Verify the required empty IfcProject label using the actual IFC dialect validators."""

from sys import argv

from ifcopenshell import open as read
from ifcopenshell.validate import json_logger, validate

for filename, schema in zip(argv[1:], ("IFC2X3", "IFC4"), strict=True):
    model = read(filename)
    assert model.schema == schema
    project, = model.by_type("IfcProject")
    assert project.Name == "", (schema, project.Name)
    log = json_logger()
    validate(model, log, express_rules=True)
    assert log.statements == [], log.statements
    print(f"[DEBUG] {schema} unnamed project has an empty label and passes EXPRESS validation")
