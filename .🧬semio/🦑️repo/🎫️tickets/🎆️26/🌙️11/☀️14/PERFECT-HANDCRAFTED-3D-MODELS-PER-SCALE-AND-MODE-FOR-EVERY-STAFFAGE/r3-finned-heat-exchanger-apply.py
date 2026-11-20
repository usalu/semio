"""📝 Round 3, machinery/finned-heat-exchanger: writes the root corrections of the validated candidate (`🗑️generated/r3-finned-heat-exchanger/🔣️.json`) into the catalog asset file and the dimensions into this asset's row of the catalog manifest. The catalog file is re-read right before writing; only the members that differ from the candidate (never `volumes`) are replaced, each in the formatting of the file; member order is kept. `volumes` goes in through `volumes-insert.py`.

Usage: python3 r3-finned-heat-exchanger-apply.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/finned-heat-exchanger/🔣️.json"))
MANIFEST = os.path.normpath(os.path.join(os.path.dirname(CATALOG), "../../🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-finned-heat-exchanger", "🔣️.json")
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
    manifest_row(want["dimensions"])


def manifest_row(dimensions):
    """📇 Sets heightM and widthM of this asset's row in the catalog manifest (re-read right before writing, nothing else touched)."""
    text = open(MANIFEST, encoding="utf-8").read()
    start = text.index('"id": "finned-heat-exchanger"')
    end = text.index('"powerW"', start)
    row = text[start:end]
    new = row
    for key in ("heightM", "widthM"):
        a = new.index(f'"{key}": ') + len(f'"{key}": ')
        b = new.index(",", a)
        new = new[:a] + json.dumps(dimensions[key]) + new[b:]
    if new != row:
        out = text[:start] + new + text[end:]
        json.loads(out)
        open(MANIFEST, "w", encoding="utf-8").write(out)
        print("manifest row", row[row.index('"heightM"'):].split("\n")[0].strip(), "->", dimensions["heightM"], dimensions["widthM"])


if __name__ == "__main__":
    main()
