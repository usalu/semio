"""Shared assembly helpers for the NW-H3 assets (references, document skeleton, decision texts check)."""
from h3lib import *

REFS_COMMON = {r["id"]: r["citation"] for r in BASE["provenance"]["references"]}
REFS_COMMON["mifflin-1990"] = REFS = {"mifflin-1990": "Mifflin, M. D., St Jeor, S. T. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51(2), 241-247."}["mifflin-1990"]


def refs(ids, extra=None):
    out = []
    for i in ids:
        out.append({"id": i, "citation": (extra or {}).get(i) or REFS_COMMON[i]})
    return out


def material(id, en, de, emis, absorp, src="authored-estimate", ref=None, cp=None, rho=None, k=None):
    m = {"id": id, "title": T(en, de), "emissivity": emis, "solarAbsorptivity": absorp}
    if rho: m["densityKgM3"] = rho
    if k: m["conductivityWmK"] = k
    if cp: m["specificHeatJKgK"] = cp
    m["source"] = src
    if ref: m["reference"] = ref
    return m


def ordered_asset(d):
    keys = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"]
    return OrderedDict((k, d[k]) for k in keys)


def folder_of(cat):
    return next(d for d in os.listdir(CAT_ROOT) if d.endswith(cat))


def write_asset(aid, cat, asset):
    folder = folder_of(cat)
    os.makedirs(f"{CAT_ROOT}/{folder}/{aid}", exist_ok=True)
    doc = OrderedDict([("$schema", "../../../🧬️schema/🔣️.json#/$defs/AssetDocument"), ("version", "semio.staffage/v2"), ("asset", ordered_asset(asset))])
    with open(f"{CAT_ROOT}/{folder}/{aid}/🔣️.json", "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")


CAT_ROOT = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
