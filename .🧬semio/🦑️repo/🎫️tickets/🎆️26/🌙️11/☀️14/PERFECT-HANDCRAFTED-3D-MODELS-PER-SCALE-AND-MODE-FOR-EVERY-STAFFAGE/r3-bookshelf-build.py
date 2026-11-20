"""🏗 Round 3, furniture/bookshelf: assembles the candidate asset document (corrected roots, redrawn 2D, handcrafted volumes) under 🗑️generated/r3-bookshelf/ so that `r3-validate.ts --file` and `r3-2d.ts --file` can judge it; with `--apply` it writes the root corrections into the catalog asset file (re-read right before writing, only the members that differ, never `volumes`, member order and formatting kept) and the volumes into the ticket source, ready for `volumes-insert.py`.

Usage: python3 r3-bookshelf-build.py [--apply]
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bookshelf/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-bookshelf")
SOURCE = os.path.join(HERE, "volumes", "furniture", "bookshelf.json")


def module(name):
    """📦 Loads one of the sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_bookshelf_" + name, os.path.join(HERE, f"r3-bookshelf-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def candidate(text):
    """🧩 The catalog document with every round 3 change applied in memory."""
    document = json.loads(text)
    module("roots").apply(document["asset"])
    document["asset"]["volumes"] = module("model").volumes()
    return document


def write_catalog(text, want):
    """📝 Replaces the differing members of the asset in the text of the catalog file."""
    exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0], globals())
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + json.dumps(want[key], indent=2, ensure_ascii=False).replace("\n", "\n    ") + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    return text, changed


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = candidate(text)
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    volumes = document["asset"]["volumes"]
    print("candidate", os.path.join(OUT, "🔣️.json"), {k: [sum(s.get("repeat", {}).get("count", 1) for s in v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})
    if "--apply" in sys.argv:
        result, changed = write_catalog(text, document["asset"])
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("the catalog file changed while building; run again")
        open(CATALOG, "w", encoding="utf-8").write(result)
        json.dump({"volumes": volumes}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print("written", CATALOG, "members:", ", ".join(changed) or "none", "and", SOURCE)


if __name__ == "__main__":
    main()
