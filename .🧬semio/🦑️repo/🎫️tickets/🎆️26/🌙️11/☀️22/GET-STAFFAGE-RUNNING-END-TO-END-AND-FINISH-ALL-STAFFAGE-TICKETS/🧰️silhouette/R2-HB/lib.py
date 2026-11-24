"""🧰️ Shared helpers of the make scripts of batch R2-HB: the catalog asset of a person as it is now, and the writer of an ops file (the op format of `ops.ts`, one op per line)."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, *[".."] * 9))
HUMANS = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans")


def asset_of(slug):
    """📖️ The asset of a person as the catalog file holds it now."""
    return json.load(open(os.path.join(HUMANS, slug, "🔣️.json"), encoding="utf8"))["asset"]


def write_ops(slug, ops, part=None):
    """📤️ Writes the ops of a person (`ops/<slug>.json`, or `ops/<slug>.<part>.json` for one part of several that `join` merges)."""
    path = os.path.join(HERE, "ops", f"{slug}{'.' + part if part else ''}.json")
    with open(path, "w", encoding="utf8") as handle:
        handle.write("[\n" + ",\n".join(json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
    print(f"{path}: {len(ops)} ops")
