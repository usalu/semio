#!/usr/bin/env python3
"""sweep2-g stage: builds the staged copies of the edited library files from the LIVE files (anchored text edits, idempotent) into batch/staged/. usage: stage.py [--check]"""
import json, os, subprocess, sys
ROOT = os.environ.get("SWEEP2_ROOT", "/Users/ueli/Documents/semio")
HERE = os.path.dirname(os.path.abspath(__file__))
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
TABLE = f"{LIB}/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"
POLICY = f"{LIB}/🏗️build/🔣️policy.json"
NX = "nx.json"
NAMES = {}
LANE = f"{LIB}/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/"
DOMAIN = f'''    {{
      "id": "unit-cache-lanes",
      "contract": {{
        "en": "Cargo unit-cache lanes: which lane a process enters, how many idle lanes the pool keeps and when it trims them, which lane files retention publishes to the shared ledger. It decides where cached units live, never what a unit is; the unit key, the lane layout and the ledger name rules stay keyed.",
        "de": "Cargo-Unit-Cache-Bahnen: welche Bahn ein Prozess betritt, wie viele ruhende Bahnen der Pool haelt und wann er sie ausduennt, welche Bahn-Dateien die Aufbewahrung im gemeinsamen Hauptbuch veroeffentlicht. Sie bestimmt, wo Einheiten liegen, nie was eine Einheit ist; Einheitenschluessel, Bahn-Layout und Hauptbuch-Namensregeln bleiben im Schluessel."
      }},
      "prefixes": [
        "{LANE}🏊️pool/",
        "{LANE}📤️retention/"
      ],
      "ports": [
        "{LANE}🏊️pool/🟦️.ts",
        "{LANE}📤️retention/🟦️.ts"
      ]
    }},
'''
NAMES.update({TABLE: "scheduling-table.json", POLICY: "build-policy.json", NX: "nx.json"})
ANCHOR = '    {\n      "id": "progress-reporting",'
def read(path): return open(os.path.join(ROOT, path), encoding="utf8").read()
def table_text():
    text = read(TABLE)
    if '"id": "unit-cache-lanes"' in text: return text
    assert text.count(ANCHOR) == 1
    return text.replace(ANCHOR, DOMAIN + ANCHOR)
def exclusions(table):
    helper = os.path.join(HERE, "exclusions.ts")
    path = os.path.join(HERE, "staged", "_table.json")
    open(path, "w", encoding="utf8").write(table)
    out = subprocess.run(["bun", helper, path], capture_output=True, text=True, cwd=ROOT)
    os.remove(path)
    assert out.returncode == 0, out.stderr
    return json.loads(out.stdout)
def splice(text, lines, indent):
    key = '"schedulingSourceExclusions": ['
    start = text.index(key); open_at = start + len(key); end = text.index("]", open_at)
    body = ",\n".join(f'{indent}"{line}"' for line in lines)
    return text[:open_at] + "\n" + body + "\n" + indent[:-2] + text[end:]
def main():
    table = table_text(); lines = exclusions(table)
    staged = {TABLE: table, POLICY: splice(read(POLICY), lines, "      "), NX: splice(read(NX), lines, "      ")}
    os.makedirs(os.path.join(HERE, "staged"), exist_ok=True)
    for path, text in staged.items():
        json.loads(text)
        target = os.path.join(HERE, "staged", NAMES[path])
        if "--check" in sys.argv:
            print(path, "ok" if os.path.exists(target) and open(target, encoding="utf8").read() == text else "DIFFERS")
        else:
            open(target, "w", encoding="utf8").write(text)
    print(len(lines), "exclusion patterns")
if __name__ == "__main__":
    main()
