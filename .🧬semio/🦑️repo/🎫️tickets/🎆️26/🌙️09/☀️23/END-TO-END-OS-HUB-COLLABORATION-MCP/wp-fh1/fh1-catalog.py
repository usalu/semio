"""🗂️ FH1: writes the framework fault catalog of the overlay from `catalog_texts.T` (texts) and S20's reviewed class list
`s14-s20-sets/class/family-A.json` for every code the latest family-A census says a framework crate raises (sorted by code); reports raised codes without text and texts nothing raises."""
import json, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
import catalog_texts
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
CATALOG = O + "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json"
census = json.load(open("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census/family-A.json"))
raised = sorted(json.load(open("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census/framework-raised.json")))
missing = [code for code in raised if code not in catalog_texts.T]
unraised = sorted(code for code in catalog_texts.T if code not in raised)
classes = {entry["code"]: entry["class"] for entry in json.load(open("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class/family-A.json"))}
faults = [{"code": code, "class": classes[code], "en": catalog_texts.T[code][0], "de": catalog_texts.T[code][1]} for code in raised if code in catalog_texts.T]
with open(CATALOG, "w") as handle:
    handle.write('{\n  "schema": "semio.fault-catalog.v1",\n  "faults": [\n')
    handle.write(",\n".join("    " + json.dumps(entry, ensure_ascii=False) for entry in faults))
    handle.write("\n  ]\n}\n")
print(f"catalog: {len(faults)} entries; raised without text: {missing}; text not raised (not written): {unraised}")
