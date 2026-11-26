"""📐️ Measure both cold-column IFC dialects with IfcOpenShell."""

from math import isclose
from sys import argv
from ifcopenshell import open as read
from ifcopenshell.geom import create_shape, settings
from ifcopenshell.util.shape import get_volume
from ifcopenshell.validate import json_logger, validate

heights = {"col-basement": 2.6 + 0.2 - 0.1, "col-ground": 2.8 - 0.3 - 0.1}
for filename, schema in zip(argv[1:], ["IFC2X3", "IFC4"], strict=True):
    model = read(filename)
    assert model.schema == schema
    log = json_logger()
    validate(model, log, express_rules=True)
    assert log.statements == [], log.statements
    columns = model.by_type("IfcColumn")
    assert len(columns) == 2
    for column in columns:
        shape = create_shape(settings(), column)
        measured = get_volume(shape.geometry)
        expected = 0.35 * 0.5 * heights[column.Tag]
        assert isclose(measured, expected, abs_tol=1e-8), (column.Tag, expected, measured)
        print(f"[DEBUG] {schema} {column.Tag} IfcOpenShell solid volume {measured:.12f}")
    if schema == "IFC4":
        member_rows = model.by_type("IfcStructuralCurveMember")
        assert len(member_rows) == 2
        for member in member_rows:
            polyline = member.Representation.Representations[0].Items[0]
            start, end = (point.Coordinates for point in polyline.Points)
            assert start[:2] == end[:2]
            assert isclose(end[2] - start[2], heights[member.Name], abs_tol=1e-8)
        print("[DEBUG] IFC4 includes both exact vertical structural axes")
