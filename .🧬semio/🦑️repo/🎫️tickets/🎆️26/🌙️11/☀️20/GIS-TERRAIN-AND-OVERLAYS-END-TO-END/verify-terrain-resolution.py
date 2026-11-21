"""🗻 Verify the rectangular fractional elevation fixture through GDAL."""

import json
from pathlib import Path

import numpy
from rasterio import Env
from rasterio.io import MemoryFile


fixture = json.loads(Path(__file__).with_name("terrain-fractional-grid.json").read_text(encoding="utf-8"))
expected = fixture["elevation"]
with Env(AAIGRID_DATATYPE="Float64"), MemoryFile(fixture["asciiGrid"].encode("ascii")) as grid, grid.open() as raster:
    assert raster.driver == "AAIGrid"
    assert (raster.width, raster.height) == (expected["width"], expected["height"])
    west, south = raster.xy(raster.height - 1, 0)
    east, north = raster.xy(0, raster.width - 1)
    numpy.testing.assert_allclose([west, south, east, north], expected["bounds"], rtol=0, atol=1e-12)
    heights = raster.read(1, masked=True)[::-1]
    assert not numpy.ma.getmaskarray(heights).any()
    numpy.testing.assert_allclose(heights * 1000, expected["millimetres"], rtol=0, atol=1e-9)
print(json.dumps({"driver": "AAIGrid", **expected}, ensure_ascii=False))
