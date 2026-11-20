"""🖋️ Round 3, appliances/shower-head: writes the validated candidate (`🗑️generated/r3-shower-head/🔣️.json`) into the catalog asset file. The catalog file is re-read right before writing; every member of the asset is taken from the candidate (rig, drawings and volumes change together, because the volumes are stated in the corrected bone frames), member order and the formatting of the file are kept.

Usage: python3 r3-shower-head-apply.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-shower-head", "🔣️.json")


def main():
    want = json.load(open(CANDIDATE, encoding="utf-8"))
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)
    if json.dumps(have, ensure_ascii=False, indent=2) + "\n" != text:
        raise SystemExit("the catalog file is no longer in its plain two-space formatting")
    if list(have) != list(want) or list(have["asset"]) != list(want["asset"]):
        raise SystemExit("member order differs")
    changed = [key for key in have["asset"] if have["asset"][key] != want["asset"][key]]
    for key in changed:
        have["asset"][key] = want["asset"][key]
    open(CATALOG, "w", encoding="utf-8").write(json.dumps(have, ensure_ascii=False, indent=2) + "\n")
    print("written", CATALOG, "members:", ", ".join(changed) or "none")


if __name__ == "__main__":
    main()
