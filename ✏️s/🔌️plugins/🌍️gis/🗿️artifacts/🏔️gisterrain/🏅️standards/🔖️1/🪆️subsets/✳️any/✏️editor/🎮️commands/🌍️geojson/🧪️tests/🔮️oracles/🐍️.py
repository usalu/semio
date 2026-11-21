"""🌍 Verify the neutral Terrain GeoJSON fixture with GeoJSON and GEOS."""

import json
import math
from pathlib import Path
import sys

import geojson
from shapely.geometry import shape

fixture = Path(__file__).resolve().parent.parent / "🧫️fixtures" / "🔣️.geojson"
text = fixture.read_text(encoding="utf-8")
collection = geojson.loads(text)
polygon, point, line = collection.features
assert isinstance(collection, geojson.FeatureCollection)
assert (polygon.id, point.id) == ("yard", 9007199254740993)
assert point.properties["lon"] == "survey note" and point.properties["id"] == "external code"
assert tuple(shape(point.geometry).coords)[0] == (5.56, 50.62, 87.25)
assert tuple(shape(line.geometry).coords)[1] == (5.57, 50.63, 92.0)
assert len(shape(polygon.geometry).interiors) == 1
assert math.isclose(shape(polygon.geometry).area, 0.0008)
assert polygon.properties["ring"] == "property metadata"
expected = json.loads(text)
line["properties"] = None
assert json.loads(geojson.dumps(collection)) == expected

large = json.loads(fixture.with_name("🔣️large.json").read_text(encoding="utf-8"))
routes = [
    geojson.Feature(
        id=f"contour-{index}",
        geometry=geojson.LineString(large["coordinates"]),
        properties={
            "semioTerrainContour": True,
            "elevationMetres": large["elevationMetres"],
            "note": "s" * large["noteWidth"],
        },
    )
    for index in range(large["routeCount"])
]
large_collection = geojson.dumps(geojson.FeatureCollection(routes), separators=(",", ":"))
assert len(large_collection.encode("utf-8")) > 12 * 1024
assert len(geojson.loads(large_collection).features) == large["routeCount"]

if len(sys.argv) > 1:
    exported = Path(sys.argv[1]).read_text(encoding="utf-8")
    checked = geojson.loads(exported)
    checked.features[-1]["properties"] = None
    assert json.loads(exported) == json.loads(geojson.dumps(checked)) == expected
