"""📝 Round 3, appliances/laptop: writes the root corrections of the validated candidate into the catalog asset file, the height and the length into the catalog index, and the candidate's volumes into the ticket source, ready for `volumes-insert.py`. Each file is re-read right before writing; only the members that differ from the candidate (never `volumes`) are replaced, in the formatting of the splice tool; member order is kept.

Usage: python3 r3-laptop-apply.py
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
STAFFAGE = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"))
CATALOG = os.path.join(STAFFAGE, "🗂️catalog", "🔌️appliances", "laptop", "🔣️.json")
INDEX = os.path.join(STAFFAGE, "🗂️catalog", "🔣️.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-laptop", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "laptop.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def index(dimensions):
    """📇 The height and the length of the laptop row of the catalog index."""
    text = open(INDEX, encoding="utf-8").read()
    found = re.search(r'("id": "laptop",\s*"category": "appliances",\s*"title": \{[^}]*\},\s*"heightM": )([0-9.]+)(,\s*"widthM": )([0-9.]+)', text)
    if not found:
        raise SystemExit("the laptop row of the catalog index was not found")
    new = found.group(1) + json.dumps(dimensions["heightM"]) + found.group(3) + json.dumps(dimensions["widthM"])
    if new != found.group(0):
        open(INDEX, "w", encoding="utf-8").write(text[: found.start()] + new + text[found.end():])
    return new != found.group(0)


def main():
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + dump(want[key], 2) + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    if changed:
        open(CATALOG, "w", encoding="utf-8").write(text)
    json.dump({"volumes": want["volumes"]}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("written", CATALOG, "members:", ", ".join(changed) or "none", "| index changed:", index(want["dimensions"]))


if __name__ == "__main__":
    main()
