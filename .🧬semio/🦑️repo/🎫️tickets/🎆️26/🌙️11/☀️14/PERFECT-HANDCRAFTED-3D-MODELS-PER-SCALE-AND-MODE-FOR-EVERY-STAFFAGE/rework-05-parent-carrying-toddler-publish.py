"""🚀 Rework 05, parent carrying a toddler: writes the validated candidate `🗑️generated/r3-parent-carrying-toddler/🔣️.json` into the catalog asset and the volumes source with `kit.publish`.

Usage: python3 rework-05-parent-carrying-toddler-publish.py  (after `rework-05-build.sh parent-carrying-toddler --volumes` ended `VALID`)
@see ./rework-05-kit.py
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "rework-05-kit.py"))
kit = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(kit)
SLUG = "parent-carrying-toddler"

if __name__ == "__main__":
    catalog = os.path.join(kit.CATALOG, "👤️humans", SLUG, "🔣️.json")
    kit.publish(os.path.join(HERE, "🗑️generated", "r3-" + SLUG, "🔣️.json"), catalog, os.path.join(HERE, "volumes", "humans", SLUG + ".json"))
    print("published", catalog)
