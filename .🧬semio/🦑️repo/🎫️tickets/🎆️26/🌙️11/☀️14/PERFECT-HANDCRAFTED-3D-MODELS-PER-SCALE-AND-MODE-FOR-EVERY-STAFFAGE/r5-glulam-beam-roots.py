"""🧪 Round 5 root fix of components/glulam-beam outside `volumes`: the mean modulus of elasticity of GL24h is 11 500 MPa in EN 14080:2013 Table 5 (11 600 MPa was the value of the withdrawn EN 1194), in the material and in the structural assumption that names it.

Usage: python3 r5-glulam-beam-roots.py [--write]   (default: candidate under 🗑️generated/r5-glulam-beam/🔣️.json; idempotent)
See: r5-brief.md, r3-components-glulam-beam.md
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glulam-beam/🔣️.json"
OUT = os.path.join(HERE, "🗑️generated", "r5-glulam-beam", "🔣️.json")
E = 11_500_000_000


def fix(raw):
    """🔧 Modulus in the material, the reference citation and the first structural assumption (en, de), as text replacements so the file keeps its formatting."""
    for old, new in (('"elasticModulusPa": 11600000000,', f'"elasticModulusPa": {E},'),
                     ("GL24h E0,mean = 11.6 GPa, mean", "GL24h E0,mean = 11.5 GPa, mean"),
                     ("E0,mean = 11.6 GPa, section", "E0,mean = 11.5 GPa (EN 14080 Table 5), section"),
                     ("E0,mean = 11,6 GPa, Querschnitt", "E0,mean = 11,5 GPa (EN 14080 Tabelle 5), Querschnitt")):
        raw = raw.replace(old, new)
    json.loads(raw)
    return raw


def main():
    """🚀 Build the candidate, or rewrite the catalog file with --write (re-read right before writing)."""
    with open(CATALOG, encoding="utf-8") as f:
        raw = fix(f.read())
    path = CATALOG if "--write" in sys.argv else OUT
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(raw)
    print(path)


if __name__ == "__main__":
    main()
