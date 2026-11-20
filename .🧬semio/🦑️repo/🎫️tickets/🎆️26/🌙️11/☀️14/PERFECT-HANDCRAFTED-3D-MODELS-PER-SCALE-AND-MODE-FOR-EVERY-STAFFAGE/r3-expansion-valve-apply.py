"""📝 Round 3, machinery/expansion-valve: writes the root corrections of the validated candidate (`🗑️generated/r3-expansion-valve/🔣️.json`) into the catalog asset file. The catalog file is re-read right before writing; only the members that differ from the candidate (never `volumes`) are replaced, each in the formatting of the file; member order is kept. `volumes` goes in through `volumes-insert.py`.

Usage: python3 r3-expansion-valve-apply.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/expansion-valve/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-expansion-valve", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def main():
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
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
    print("written", CATALOG, "members:", ", ".join(changed) or "none")


if __name__ == "__main__":
    main()
