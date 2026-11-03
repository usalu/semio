import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
edit(sys.argv[2], [
('"key": { "entry": { "asset": "case-asset", "pose": "standing", "viewpoint": "elevation", "view": "dark-line", "theme": "dark" }', '"key": { "entry": { "asset": "case-asset", "pose": "standing", "viewpoint": "elevation", "view": "dark-line", "theme": "dark", "scale": 50 }'),
('"constant": { "asset": "case-asset", "pose": "standing", "viewpoint": "elevation", "view": "thermal", "theme": "dark", "environment": "cold" }', '"constant": { "asset": "case-asset", "pose": "standing", "viewpoint": "elevation", "view": "thermal", "theme": "dark", "scale": 50, "environment": "cold" }'),
('"view": "thermal", "theme": "dark", "timeline": [{ "t": 0, "environment": "cold" }, { "t": 4, "environment": "hot" }] }', '"view": "thermal", "theme": "dark", "scale": 50, "fps": 15, "timeline": [{ "t": 0, "environment": "cold", "ease": "step" }, { "t": 4, "environment": "hot", "ease": "linear" }] }'),
('"plain": { "asset": "case-asset", "pose": "standing", "viewpoint": "elevation", "view": "dark-line", "theme": "dark" }', '"plain": { "asset": "case-asset", "pose": "standing", "viewpoint": "elevation", "view": "dark-line", "theme": "dark", "scale": 50 }'),
])
