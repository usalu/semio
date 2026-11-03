import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
edit(sys.argv[2], [
('''             "boundsM": { "minX": -2.5, "minY": -0.5, "maxX": 2.5, "maxY": 4.5 } },''',
'''             "boundsM": { "minX": -2.5, "minY": -0.5, "maxX": 2.5, "maxY": 4.5 },
             "provenance": { "version": 1, "asset": { "id": "case-asset", "category": "humans", "revision": "sha256:0000000000000000000000000000000000000000000000000000000000000000" },
                             "view": { "style": "dark-line", "viewpoint": "elevation", "theme": "dark", "mirror": false }, "scale": { "denominator": 50 }, "state": { "kind": "clip", "id": "walk" },
                             "clock": { "fps": 10, "durationS": 0.3, "loop": true, "offsetS": 0, "frames": 3 }, "scenario": { "kind": "geometry" } } },'''),
])
