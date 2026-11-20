"""📝 Round 3, appliances/vacuum-cleaner: writes the root corrections of the validated candidate into the catalog asset file (only the members that differ, never `volumes`, each in the file's own formatting, member order kept, the file re-read right before writing) and the new height and length into the catalog index entry; `volumes` then goes in through `volumes-insert.py`.

Usage: python3 r3-vacuum-cleaner-apply.py
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json"))
INDEX = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-vacuum-cleaner", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def asset_file(want):
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + json.dumps(want[key], indent=2, ensure_ascii=False).replace("\n", "\n    ") + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    return changed


def index_entry(want):
    text = open(INDEX, encoding="utf-8").read()
    start = text.index('"id": "vacuum-cleaner"')
    end = text.index('"completion"', start)
    block = text[start:end]
    block = re.sub(r'"heightM": [0-9.]+', f'"heightM": {want["dimensions"]["heightM"]}', block, count=1)
    block = re.sub(r'"widthM": [0-9.]+', f'"widthM": {want["dimensions"]["widthM"]}', block, count=1)
    new = text[:start] + block + text[end:]
    json.loads(new)
    if new != text:
        open(INDEX, "w", encoding="utf-8").write(new)
    return new != text


def main():
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    changed = asset_file(want)
    idx = index_entry(want)
    print("written", CATALOG, "members:", ", ".join(changed) or "none", "| index:", "updated" if idx else "unchanged")


if __name__ == "__main__":
    main()
