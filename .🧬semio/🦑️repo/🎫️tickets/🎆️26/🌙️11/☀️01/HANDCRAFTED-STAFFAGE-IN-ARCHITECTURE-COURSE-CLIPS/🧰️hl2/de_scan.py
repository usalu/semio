"""🔍 Scan every {en, de} pair in the staffage catalog for untranslated or English-leaking German text."""
import glob
import json
import os
import re
import sys

CATALOG = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"

ENGLISH = set("""the and with from that this which are its into onto between during while where their these those is be have has each only
should would could will shall being been within without about through across over under
""".split())
CONTENT = set("""marker arrow dot notes mounted walking standing sitting kneeling lying seated rear bottom shows showing shown
housing cover painted plastic tyre wheel wood sofa frame
""".split())
GERMAN = set("""der die das und mit für ist sind eine einen einem dem den des von nicht auf im zum zur oder wird werden wenn
""".split())
WORD = re.compile(r"[A-Za-zÄÖÜäöüß]+")


def pairs(node, path):
    if isinstance(node, dict):
        if set(node.keys()) >= {"en", "de"} and all(isinstance(node[k], str) for k in ("en", "de")):
            yield path, node["en"], node["de"]
        for key, value in node.items():
            yield from pairs(value, f"{path}.{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from pairs(value, f"{path}[{index}]")


def words(text):
    return [w.lower() for w in WORD.findall(text)]


def main():
    files = sorted(glob.glob(os.path.join(CATALOG, "**", "🔣️.json"), recursive=True))
    total = 0
    findings = []
    for file in files:
        with open(file, encoding="utf-8") as handle:
            data = json.load(handle)
        rel = os.path.relpath(file, CATALOG)
        for path, en, de in pairs(data, ""):
            total += 1
            if not de.strip() or not en.strip():
                findings.append(("EMPTY", rel, path, en, de))
                continue
            if en.strip() == de.strip():
                findings.append(("IDENTICAL", rel, path, en, de))
                continue
            de_words = words(de)
            en_words = words(en)
            en_hits = sorted({w for w in de_words if w in ENGLISH or w in CONTENT})
            de_hits = sorted({w for w in en_words if w in GERMAN})
            if en_hits:
                findings.append(("DE-ENGLISH", rel, path, en, de, en_hits))
            if de_hits:
                findings.append(("EN-GERMAN", rel, path, en, de, de_hits))
    for finding in findings:
        kind, rel, path, en, de = finding[:5]
        extra = f" words={finding[5]}" if len(finding) > 5 else ""
        print(f"{kind} {rel} {path}{extra}")
        print(f"  en: {en}")
        print(f"  de: {de}")
    print(f"pairs={total} files={len(files)} findings={len(findings)}")


if __name__ == "__main__":
    sys.exit(main())
