"""Extracts the include lists of the five *_SOURCE consts (as they stand) and checks that every file exists."""
import re, os, json, sys
from lib import *
CONSTS = {
    "renderer": R + "/🦀️.rs",
    "shell": SH + "/🦀️.rs",
    "scenes": EL + "/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "engine_canvas": EL + "/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs",
    "interpreter": EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
}
NAMES = {"renderer": "RENDERER_SOURCE", "shell": "SHELL_SOURCE", "scenes": "SCENES_SOURCE", "engine_canvas": "ENGINE_CANVAS_SOURCE", "interpreter": "INTERPRETER_SOURCE"}
out = {}
for key, path in CONSTS.items():
    text = read(path)
    m = re.search(r"const " + NAMES[key] + r": &str = concat!\((.*?)\n\);", text, re.S)
    assert m, key
    files = []
    for im in re.finditer(r'include_str!\("([^"]+)"\)', m.group(1)):
        p = os.path.normpath(os.path.join(os.path.dirname(path), im.group(1)))
        files.append(os.path.relpath(p, E))
    missing = [f for f in files if not os.path.exists(E + "/" + f)]
    out[key] = files
    print(key, len(files), "missing:", missing)
json.dump(out, open(SCRATCH + "/source-lists.json", "w"), ensure_ascii=False, indent=1)
