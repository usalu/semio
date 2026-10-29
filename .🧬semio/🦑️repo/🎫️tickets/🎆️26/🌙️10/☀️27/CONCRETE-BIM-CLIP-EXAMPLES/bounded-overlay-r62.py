"""🧪 Retained failed exact tiled-overlay experiment; not a runtime implementation."""
import math,json
from pathlib import Path
from shapely.geometry import Polygon,box,shape
from shapely.strtree import STRtree

def _tiled_xor_area(actual, expected):
    """🧩 Exact symmetric-difference area over disjoint one-metre cells."""
    from shapely.ops import unary_union

    left = [value for value in actual if not value.is_empty and value.area > 0]
    right = [value for value in expected if not value.is_empty and value.area > 0]
    shapes = left + right
    if not shapes:
        return 0.0
    trees = (STRtree(left), STRtree(right))
    def local(values, tree, cell):
        clipped = [values[int(index)].intersection(cell) for index in tree.query(cell, predicate="intersects")]
        occupied = [value for value in clipped if not value.is_empty and value.area > 0]
        return unary_union(occupied) if len(occupied) > 1 else occupied[0] if occupied else Polygon()
    low_x = math.floor(min(value.bounds[0] for value in shapes))
    low_y = math.floor(min(value.bounds[1] for value in shapes))
    high_x = math.ceil(max(value.bounds[2] for value in shapes))
    high_y = math.ceil(max(value.bounds[3] for value in shapes))
    area = 0.0
    for x in range(low_x, high_x):
        for y in range(low_y, high_y):
            cell = box(x, y, x + 1, y + 1)
            area += local(left, trees[0], cell).symmetric_difference(local(right, trees[1], cell)).area
    return area


data=json.loads((Path(__file__).parent/'🗑️generated/parking-profile-current-union-r62.json').read_text())
print(_tiled_xor_area([shape(p) for p in data['geometries']], []))
