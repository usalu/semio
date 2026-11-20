"""🖋️ Writes the 2D redraw and the dimensions of r3-pig-2d.ts (🗑️generated/r3-pig/representations.json) into the pig asset file: only `representations` and `dimensions` change, every other member and its formatting stays.

Usage: python3 r3-pig-apply.py   (run only after `bun r3-validate.ts --file 🗑️generated/r3-pig/🔣️.json` printed VALID)
"""
import importlib.util
import json

spec = importlib.util.spec_from_file_location("io", "r3-pig-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)
data = json.load(open("🗑️generated/r3-pig/representations.json", encoding="utf-8"))
io.put({"dimensions": data["dimensions"], "representations": data["representations"]})
print("representations and dimensions written")
