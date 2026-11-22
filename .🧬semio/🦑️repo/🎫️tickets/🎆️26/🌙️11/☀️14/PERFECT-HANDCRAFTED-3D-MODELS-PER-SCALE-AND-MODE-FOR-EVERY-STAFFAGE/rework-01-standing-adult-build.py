"""🏗️ standing-adult: builds the candidate asset document (rig applied, hand-stated volumes of every scale) and the volumes source.

Usage: python3 rework-01-standing-adult-build.py [--catalog]
Writes `🗑️generated/rework-01/standing-adult/🔣️.json` (candidate) and `volumes/humans/standing-adult.json` (source); with `--catalog` the candidate also replaces the catalog file (re-read immediately before).
"""
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


lib = module("rework-01-lib")
rig = module("rework-01-standing-adult-rig")
body = module("rework-01-standing-adult-body")
SLUG = "standing-adult"


def main():
    path = lib.catalog_path("👤️humans", SLUG)
    document = lib.read(path)
    asset = document["asset"]
    rig.apply(asset)
    module("rework-01-standing-adult-skeleton").declare(asset)
    kit = lib.Kit(lib.Frames(asset["rig"]))
    lists = module("rework-01-standing-adult-lists")
    asset["volumes"] = lists.volumes(kit, asset)
    lists.roots(asset)
    module("rework-01-standing-adult-2d").redraw(asset, kit)
    lib.write(os.path.join(lib.OUT, SLUG, "🔣️.json"), document)
    lib.write(os.path.join(HERE, "volumes", "humans", f"{SLUG}.json"), {"volumes": asset["volumes"]})
    if "--catalog" in sys.argv:
        fresh = lib.read(path)
        fresh["asset"] = asset
        with open(path, "w", encoding="utf8") as handle:
            handle.write(lib.catalog_text(fresh) + "\n")
    for scale, volume in asset["volumes"].items():
        print(scale, {name: len(volume[name]) for name in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
