"""Writes the independent cross-check vectors, the self-contained artifact oracle and the feature file of the five non-Eurocode artifacts. Usage: build.py [--check]; --check recomputes and compares with the committed vectors (relative 1e-9)."""
import json
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
ART = {"din4108": "🧱️din4108", "din18599": "⚡️din18599", "din16798": "🌬️din16798", "iso16757": "📇️iso16757", "vdi3805": "🏭️vdi3805"}
sys.path.insert(0, str(HERE))


def close(a, b):
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, float) or isinstance(b, float):
        return a is not None and b is not None and abs(a - b) <= 1e-9 * max(abs(a), abs(b), 1e-300) + 1e-15
    return a == b


def compute(name):
    import importlib
    mod = importlib.import_module(name)
    if name == "din4108":
        doc = mod.vectors(str(ROOT / ART[name] / SUB / "🧪️tests/🧫️fixtures/🧮️u-values/🔣️.json"))
    else:
        doc = mod.vectors()
    doc["divergences"] = mod.divergences(doc) if hasattr(mod, "divergences") else []
    return json.loads(json.dumps(doc))


def standalone(name):
    common = (HERE / "common.py").read_text(encoding="utf8")
    body = (HERE / f"{name}.py").read_text(encoding="utf8")
    body = re.sub(r"^from common import .*\n", "", body, flags=re.M)
    path_expr = '    fixture = SUBSET / "🧪️tests/🧫️fixtures/🧮️u-values/🔣️.json"\n    doc = vectors(str(fixture))\n' if name == "din4108" else "    doc = vectors()\n"
    driver = f'''

import pathlib
import sys

SUBSET = pathlib.Path(__file__).resolve().parents[2]
VECTORS = SUBSET / "🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json"


def close(a, b):
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, float) or isinstance(b, float):
        return a is not None and b is not None and abs(a - b) <= 1e-9 * max(abs(a), abs(b), 1e-300) + 1e-15
    return a == b


def main():
{path_expr}    doc["divergences"] = divergences(doc)
    doc = json.loads(json.dumps(doc))
    if "--write" in sys.argv:
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\\n", encoding="utf8")
        return 0
    ok = close(doc, json.loads(VECTORS.read_text(encoding="utf8")))
    print(json.dumps({{"ok": ok, "divergences": len(doc["divergences"])}}))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
'''
    header = '"""Independent third-party cross-check oracle (scipy / mpmath / numpy / exact fractions; no repo oracle imported). Run `python -I 🐍️.py` to verify the committed vectors, `--write` to regenerate them. Divergences between the Rust source and the standard are listed under `divergences` in the vectors and are not forced to agree."""\nimport json\nimport math\n'
    return header + common.split('"""', 2)[2].lstrip("\n").replace("import math\n", "", 1) + "\n\n" + body.split('"""', 2)[2].lstrip("\n") + driver


def main():
    check = "--check" in sys.argv
    status = 0
    for name in ART:
        doc = compute(name)
        vec = ROOT / ART[name] / SUB / "🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json"
        oracle = ROOT / ART[name] / SUB / "🔮️oracles/🌐️cross-checks/🐍️.py"
        if check:
            ok = close(doc, json.loads(vec.read_text(encoding="utf8")))
            print(name, "ok" if ok else "DIFFERENT", len(doc["divergences"]), "divergences")
            status |= 0 if ok else 1
            continue
        vec.parent.mkdir(parents=True, exist_ok=True)
        oracle.parent.mkdir(parents=True, exist_ok=True)
        vec.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        oracle.write_text(standalone(name), encoding="utf8")
        print(name, len(vec.read_text(encoding="utf8")), "bytes;", len(doc["divergences"]), "divergences")
    return status


if __name__ == "__main__":
    sys.exit(main())
