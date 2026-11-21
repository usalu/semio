"""🧭 Compare the neutral ASCII Grid fixture with Rasterio GDAL and SciPy."""

import json
from pathlib import Path
import numpy as np
import rasterio
from scipy.ndimage import map_coordinates

fixtures = Path(__file__).resolve().parent.parent / "🧫️fixtures"
expect = json.loads((fixtures / "🔣️.json").read_text(encoding="utf-8"))
with rasterio.open(fixtures / "🔼️slope.asc") as raster:
    assert raster.read(1, masked=True).count() == raster.width * raster.height
    west, south = raster.xy(raster.height - 1, 0)
    east, north = raster.xy(0, raster.width - 1)
    assert [west, south, east, north] == expect["bounds"]
    x = np.linspace(0.0, raster.width - 1, expect["side"])
    y = np.linspace(raster.height - 1, 0.0, expect["side"])
    ys, xs = np.meshgrid(y, x, indexing="ij")
    heights = map_coordinates(raster.read(1).astype(np.float64), [ys, xs], order=1, mode="nearest")
    rounded = (np.sign(heights) * np.floor(np.abs(heights) + 0.5)).astype(np.int16)
    assert all(rounded[row, col] == value for row, col, value in expect["checkpoints"])
    assert int(rounded.sum()) == expect["checksum"]
    assert int(np.square(rounded.astype(np.int64)).sum()) == expect["squaredChecksum"]
