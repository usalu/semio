"""🛰️ Crops the local Liège elevation asset from the Mapzen Skadi HGT tile."""

import gzip
import hashlib
import json
from pathlib import Path
from struct import unpack_from
from urllib.request import urlopen


SOURCE = "https://s3.amazonaws.com/elevation-tiles-prod/skadi/N50/N50E005.hgt.gz"
SOURCE_SHA256 = "d1784a3e8bcbba1efcfe1494edfec757a6a277d6321e458eb6458a4541df437c"
ARCSECONDS = 3600
STRIDE = 6
COUNT = 33
WEST, SOUTH = 5.555, 50.585
EAST = WEST + (COUNT - 1) * STRIDE / ARCSECONDS
NORTH = SOUTH + (COUNT - 1) * STRIDE / ARCSECONDS


def sample_grid():
    """📏️ Samples the HGT point raster south-to-north and west-to-east."""
    with urlopen(SOURCE, timeout=45) as response:
        source = response.read()
    assert hashlib.sha256(source).hexdigest() == SOURCE_SHA256
    tile = gzip.decompress(source)
    assert len(tile) == 2 * (ARCSECONDS + 1) ** 2
    rows = []
    for latitude in range(round(SOUTH * ARCSECONDS), round(NORTH * ARCSECONDS) + 1, STRIDE):
        row = ARCSECONDS * 51 - latitude
        elevations = []
        for longitude in range(round(WEST * ARCSECONDS), round(EAST * ARCSECONDS) + 1, STRIDE):
            col = longitude - ARCSECONDS * 5
            elevation, = unpack_from(">h", tile, 2 * (row * (ARCSECONDS + 1) + col))
            assert elevation != -32768
            elevations.append(elevation)
        rows.append(elevations)
    print("Source SHA-256", SOURCE_SHA256)
    print("Grid dimensions", len(rows[0]), len(rows))
    print("Elevation range", min(map(min, rows)), max(map(max, rows)))
    return rows


if __name__ == "__main__":
    paths = Path.cwd().glob("✏️s/*plugins/*gis/*artifacts/*gisterrain/*standards/*/*subsets/*any/*assets/*demo")
    path = next(paths) / "🏔️elevation.json"
    rows = sample_grid()
    bounds = json.dumps([WEST, SOUTH, EAST, NORTH])
    encoded_rows = ",\n".join("    " + json.dumps(row, separators=(",", ",")) for row in rows)
    path.write_text(f'{{\n  "bounds": {bounds},\n  "rows": [\n{encoded_rows}\n  ]\n}}\n', encoding="utf-8")
