"""🧭️ Checks the boundary picker raster with GDAL’s independent AAIGrid reader."""

import json
import sys
from pathlib import Path

import rasterio
from rasterio.io import MemoryFile

fixture_path = Path(__file__).resolve().parents[7] / "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔣️.json"
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))["maximumPicker"]
side = fixture["side"]
west, south, east, north = fixture["bounds"]
cell = fixture["cellSize"]
north_row = " ".join([fixture["northSample"]] * side)
other_row = " ".join([fixture["otherSample"]] * side)
lines = [
    f"NCOLS {side}", f"NROWS {side}", f"XLLCENTER {fixture['west']}",
    f"YLLCENTER {fixture['south']}", f"CELLSIZE {cell}", f"NODATA_VALUE {fixture['noData']}",
    north_row, *([other_row] * (side - 1)), "",
]
payload = "\n".join(lines).ljust(fixture["bytes"]).encode("ascii")
assert len(payload) == fixture["bytes"]
assert (len(payload) + 32767) // 32768 == fixture["chunkCount"]
with rasterio.Env(AAIGRID_DATATYPE="Float64"), MemoryFile(payload) as memory, memory.open() as dataset:
    assert dataset.driver == "AAIGrid"
    assert dataset.shape == (side, side)
    assert dataset.xy(0, 0) == (west, north)
    assert dataset.xy(side - 1, side - 1) == (east, south)
    heights = dataset.read(1, masked=True)
    assert not heights.mask.any()
    millimetres = (heights * 1000).round()
    assert (millimetres[0] == fixture["northMillimetres"]).all()
    assert (millimetres[1:] == fixture["otherMillimetres"]).all()
print(f"GDAL AAIGrid: {side} × {side}, {len(payload)} bytes, {fixture['chunkCount']} picker chunks, exact signed millimetres", file=sys.stderr)
if sys.argv[1:] == ["--write-raster"]:
    sys.stdout.buffer.write(payload)
