"""🧾️ The ids of the body solids of a vehicle at a scale on given bones, joined by `+` for outline.ts, without textures and without ids that start with one of the excluded prefixes: ids.py <id> <scale> <bone,…> [excluded prefix,…]"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import asset

slug, scale, bones = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
skip = tuple(sys.argv[4].split(",")) if len(sys.argv) > 4 else ()
print("+".join(s["id"] for s in asset(slug)["volumes"][scale]["body"] if s["bone"] in bones and s.get("role") != "texture" and not (skip and s["id"].startswith(skip))))
