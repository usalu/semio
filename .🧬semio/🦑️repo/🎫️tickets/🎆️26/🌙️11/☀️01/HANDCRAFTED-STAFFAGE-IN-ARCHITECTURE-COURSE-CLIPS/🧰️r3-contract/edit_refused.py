import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
edit(sys.argv[2], [
('{ "asset": "panel-radiator", "pose": "no-such-pose", "view": "dark-line", "viewpoint": "elevation", "theme": "dark" }', '{ "asset": "panel-radiator", "pose": "no-such-pose", "view": "dark-line", "viewpoint": "elevation", "theme": "dark", "scale": 50 }'),
('{ "asset": "walking-commuter", "clip": "no-such-clip", "view": "dark-line", "viewpoint": "elevation", "theme": "dark" }', '{ "asset": "walking-commuter", "clip": "no-such-clip", "view": "dark-line", "viewpoint": "elevation", "theme": "dark", "scale": 50, "fps": 15 }'),
('"theme": "dark", "environment": "no-such-preset" }, "message": "unknown environment preset" }\n', '"theme": "dark", "scale": 50, "environment": "no-such-preset" }, "message": "unknown environment preset" },\n    { "name": "an unknown asset", "cast": { "asset": "no-such-asset", "pose": "rated", "view": "dark-line", "viewpoint": "elevation", "theme": "dark", "scale": 50 }, "message": "is not in the catalog" },\n    { "name": "a scale the asset marks unavailable", "cast": { "asset": "hosta-clump", "pose": "summer-full", "view": "dark-line", "viewpoint": "elevation", "theme": "dark", "scale": 500 }, "message": "has no authored representation at 1:500" }\n'),
])
