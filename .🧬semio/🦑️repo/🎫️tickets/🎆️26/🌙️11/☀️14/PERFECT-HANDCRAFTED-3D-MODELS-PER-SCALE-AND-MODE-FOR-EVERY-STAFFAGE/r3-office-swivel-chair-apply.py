"""📝 Round 3, furniture/office-swivel-chair: writes the validated candidate into the catalog asset file. The file is re-read right before writing; only `provenance`, `dimensions`, `rig`, `representations` and, inside `thermal`, three region areas and one assumption change, each member in the formatting it had; `volumes` is spliced afterwards by `volumes-insert.py`.

Usage: python3 r3-office-swivel-chair-apply.py
"""
import json
import os
import re
import shutil

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/office-swivel-chair/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-office-swivel-chair", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def indent2(value):
    return json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n    ")


def main():
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    for key, style in (("provenance", dump), ("dimensions", dump), ("rig", dump), ("representations", None)):
        start, end = member_range(text, "    ", key)
        body = indent2(want[key]) if style is None else style(want[key], 2)
        text = text[:start] + "    " + json.dumps(key) + ": " + body + text[end:]
    start, end = member_range(text, "    ", "thermal")
    thermal = text[start:end]
    for old, new in zip(have["thermal"]["regions"], want["thermal"]["regions"]):
        if old["areaM2"] != new["areaM2"]:
            at = thermal.index('"id": ' + json.dumps(old["id"]))
            found = re.compile(r'"areaM2": ' + re.escape(json.dumps(old["areaM2"])) + r"(?=[,\s])").search(thermal, at)
            thermal = thermal[: found.start()] + '"areaM2": ' + json.dumps(new["areaM2"]) + thermal[found.end():]
    for old, new in zip(have["thermal"]["assumptions"], want["thermal"]["assumptions"]):
        for lang in ("en", "de"):
            if old[lang] != new[lang]:
                a, b = json.dumps(old[lang], ensure_ascii=False), json.dumps(new[lang], ensure_ascii=False)
                assert thermal.count(a) == 1
                thermal = thermal.replace(a, b)
    text = text[:start] + thermal + text[end:]
    got = json.loads(text)["asset"]
    for key in want:
        if key != "volumes" and got[key] != want[key]:
            raise SystemExit(f"member {key} differs from the candidate")
    if list(got) != list(have):
        raise SystemExit("member order changed")
    open(CATALOG, "w", encoding="utf-8").write(text)
    os.makedirs(os.path.join(HERE, "volumes", "furniture"), exist_ok=True)
    shutil.copyfile(os.path.join(HERE, "volumes", "adhoc", "r3-office-swivel-chair.json"), os.path.join(HERE, "volumes", "furniture", "office-swivel-chair.json"))
    print("written", CATALOG)


if __name__ == "__main__":
    main()
