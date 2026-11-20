"""📝 Round 3, appliances/tower-server: writes the root corrections of the validated candidate into the catalog asset file and puts the candidate's volumes into the ticket source, ready for `volumes-insert.py`. The catalog file is re-read right before writing; only the members that differ from the candidate (never `volumes`) are replaced, each in the formatting of the file; member order is kept.

Usage: python3 r3-tower-server-apply.py
"""
import json
import os
import shutil

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-tower-server", "🔣️.json")
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
    shutil.copyfile(os.path.join(HERE, "volumes", "adhoc", "r3-tower-server.json"), os.path.join(HERE, "volumes", "appliances", "tower-server.json"))
    print("written", CATALOG, "members:", ", ".join(changed) or "none")


if __name__ == "__main__":
    main()
