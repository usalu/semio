"""📝 Round 3, furniture/bar-stool: writes the validated candidate into the catalog asset file and the volumes source. The catalog file is re-read right before writing; only the top-level members that differ from the candidate are replaced, each in the formatting it had (`volumes` is left to `volumes-insert.py`).

Usage: python3 r3-bar-stool-apply.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bar-stool/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-bar-stool", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "furniture", "bar-stool.json")
NS = {}
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0], NS)


def indent2(value):
    return json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n    ")


def main():
    want = json.load(open(CANDIDATE, encoding="utf-8"))
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = []
    for key in want["asset"]:
        if key == "volumes" or have.get(key) == want["asset"][key]:
            continue
        start, end = NS["member_range"](text, "    ", key)
        old = text[start:end]
        style = indent2 if indent2(have[key]) in old else (lambda v: NS["dump"](v, 2))
        text = text[:start] + "    " + json.dumps(key) + ": " + style(want["asset"][key]) + text[end:]
        changed.append(key)
    got = json.loads(text)["asset"]
    for key in want["asset"]:
        if key != "volumes" and got[key] != want["asset"][key]:
            raise SystemExit(f"member {key} differs from the candidate")
    if list(got) != list(have):
        raise SystemExit("member order changed")
    open(CATALOG, "w", encoding="utf-8").write(text)
    json.dump({"volumes": want["asset"]["volumes"]}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", CATALOG, "members:", ", ".join(changed) or "none")


if __name__ == "__main__":
    main()
