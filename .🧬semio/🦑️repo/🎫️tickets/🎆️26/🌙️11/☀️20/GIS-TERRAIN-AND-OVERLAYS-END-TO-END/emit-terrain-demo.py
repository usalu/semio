from pathlib import Path
import json
import math
import re

root = Path(__file__).resolve().parents[7]
ticket = Path(__file__).parent
terrain = root / "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any"
scenery = json.loads((terrain / "🖼️assets/🎬️demo/🌍️scenery.json").read_text(encoding="utf-8"))
definition = (terrain / "🧬️schema/🧭️terrain-descriptor/🦀️.rs").read_text(encoding="utf-8")
source = re.search(r'pub const GIS_3D_TERRAIN_TILE_URL_TEMPLATE: &str = "([^"]+)";', definition)[1]
terrain_code = (root / "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs").read_text(encoding="utf-8")
zoom = {name: int(re.search(fr"TERRAIN_TILE_{name.upper()}_ZOOM: u32 = ([0-9]+);", terrain_code)[1]) for name in ("min", "max")}
lon, lat = scenery["projectOrigin"]["lon"], scenery["projectOrigin"]["lat"]
overlays = {
    "positions": [{"id": item["id"], "name": item["label"], "coordinates": [item["lon"], item["lat"]]} for item in scenery["positions"]],
    "routes": [{"id": item["id"], "name": item["label"], "coordinates": item["points"]} for item in scenery["routes"]],
    "regions": [{"id": item["id"], "name": item["label"], "rings": [item["points"]]} for item in scenery["regions"]],
}
scene = {
    "tileUrlTemplate": source,
    "projectOriginLon": lon,
    "projectOriginLat": lat,
    "exaggeration": 1.5,
    "colorRamp": "hypsometric",
    "minZoom": zoom["min"],
    "maxZoom": zoom["max"],
    "attribution": {"label": "Mapzen Terrain Tiles", "url": "https://github.com/tilezen/joerd/blob/master/docs/attribution.md"},
    "overlays": overlays,
}
coordinates = [
    *(item["coordinates"] for item in overlays["positions"]),
    *(coordinate for route in overlays["routes"] for coordinate in route["coordinates"]),
    *(coordinate for region in overlays["regions"] for ring in region["rings"] for coordinate in ring),
]
radius = max(math.hypot((x - lon) * 111320 * math.cos(math.radians(lat)), (y - lat) * 111320) for x, y in coordinates)
distance = max(radius * 2.5, 1000.0)
elevation = 200.0 * scene["exaggeration"]
camera = {
    "position": [distance * 0.65, -distance * 0.65, elevation + distance],
    "target": [0.0, 0.0, elevation],
    "up": [0.0, 0.0, 1.0],
    "fov": 45.0,
    "terrainFrame": [lon, lat, radius],
}
destination = ticket / "🗑️generated"
destination.mkdir(exist_ok=True)
(destination / "terrain-scene.json").write_text(json.dumps({"terrain": scene, "camera": camera}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
