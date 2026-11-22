"""🏗️ Builds one PW-09 plant: `python3 build.py <species module> <out.json> [<measure.json>] [--ship]`.

Without a measurement the dimensions are the template's (a draft for `measure.ts`); with it they are measured and the true-extent symbols are fitted to\nthe drawings. The feature floors `<module>-floors.json` (written by `floors.py`) are applied to the volumes. `--ship` also writes the catalog
asset and the manifest row of the ticket.

@see ./morph.py
@see ./species.py
"""
import importlib
import json
import sys
from pathlib import Path

from morph import VEGETATION, apply_floors, dumps, fit_symbols, load, save
from species import leftovers, manifest_row

args = [a for a in sys.argv[1:] if not a.startswith("--")]
sp = importlib.import_module(args[0])
measure = json.load(open(args[2])) if len(args) > 2 else None
doc = sp.build(load(sp.TEMPLATE), measure)
floors = Path(__file__).resolve().parent / f"{args[0]}-floors.json"
if floors.exists():
    apply_floors(doc["asset"], json.loads(floors.read_text()))
if measure is not None:
    fit_symbols(doc["asset"], measure)
Path(args[1]).write_text(dumps(doc) + "\n", encoding="utf-8")
left = leftovers(doc["asset"], sp.WORDS)
for path, word, text in left:
    print(f"[DEBUG] leftover {word} at {path}: {text}")
if "--ship" in sys.argv:
    if left:
        raise SystemExit("template words left in the texts")
    print(save(doc, doc["asset"]["id"]))
    manifest = json.loads((VEGETATION.parent / "🔣️.json").read_text(encoding="utf-8"))
    template = next(e for e in manifest["entries"] if e["id"] == sp.TEMPLATE)
    en, de = sp.DISTINCTION
    row = manifest_row(doc["asset"], {"en": en, "de": de}, template)
    out = Path(__file__).resolve().parents[2] / "📥️manifest-rows" / f"{doc['asset']['id']}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(dumps(row) + "\n", encoding="utf-8")
    print(out)
