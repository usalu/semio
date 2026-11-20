"""🪛 Writes the round 3 root fixes of `humans/seated-guitarist` into the catalog file: every asset member of the candidate document `🗑️generated/r3-seated-guitarist/🔣️.json` other than `volumes` (spliced by `volumes-insert.py`) replaces the catalog's member of the same name; member order and the plain two-space style of those members stay, every other byte of the file is untouched.

Usage: python3 r3-seated-guitarist-write.py [--dry]
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-seated-guitarist", "🔣️.json")
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/seated-guitarist/🔣️.json"


def splice(text, key, value):
    """🔧 Replaces the text of asset member `key` (indent depth 2) by `value` in the plain two-space style of the members outside `volumes`."""
    head = '\n    ' + json.dumps(key, ensure_ascii=False) + ': '
    at = text.index(head) + len(head)
    _, end = json.JSONDecoder().raw_decode(text, at)
    return text[:at] + json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n    ") + text[end:]


def main():
    candidate = json.load(open(CANDIDATE, encoding="utf8"))["asset"]
    text = open(ASSET, encoding="utf8").read()
    asset = json.loads(text)["asset"]
    changed = [key for key in asset if key != "volumes" and key in candidate and asset[key] != candidate[key]]
    for key in changed:
        text = splice(text, key, candidate[key])
    after = json.loads(text)["asset"]
    if list(after) != list(asset) or any(after[k] != candidate[k] for k in changed) or after["volumes"] != asset["volumes"]:
        sys.exit("splice check failed; nothing written")
    print("members replaced:", ", ".join(changed) or "none")
    if "--dry" not in sys.argv:
        with open(ASSET, "w", encoding="utf8") as handle:
            handle.write(text)


if __name__ == "__main__":
    main()
