"""Idempotent provenance fixes for the humans catalog (audit-provenance.md)."""
import json, sys
F = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/🔣️.json"
d = json.load(open(F))

def walk(o, cb):
    if isinstance(o, dict):
        cb(o)
        for v in o.values(): walk(v, cb)
    elif isinstance(o, list):
        for v in o: walk(v, cb)

def rename_ref(a, old, new, citation=None):
    for r in a["provenance"]["references"]:
        if r["id"] == old:
            r["id"] = new
            if citation: r["citation"] = citation
    def cb(o):
        if o.get("reference") == old: o["reference"] = new
    walk(a, cb)

def drop_ref(a, rid, retag=True):
    a["provenance"]["references"] = [r for r in a["provenance"]["references"] if r["id"] != rid]
    def cb(o):
        if o.get("reference") == rid:
            del o["reference"]
            if retag and o.get("source") == "literature": o["source"] = "authored-estimate"
    walk(a, cb)

for a in d["assets"]:
    for r in a["provenance"]["references"]:
        r["citation"] = r["citation"].replace("Annex B metabolic rates", "Annex A metabolic rates")
    if a["id"] == "cyclist":
        rename_ref(a, "iso-8090-2007", "iso-8098-2014", "ISO 8098:2014 Cycles, safety requirements for bicycles for young children (frame, brake and mass classes); the adult city bicycle follows EN 14764 city and trekking bicycles; typical bicycle mass about 14 kg is an authored estimate guided by both.")
    if a["id"] == "wheelchair-user":
        rename_ref(a, "cooper-2008", "cooper-2014")
    if a["id"] == "jogger":
        rename_ref(a, "cavanagh-1982", "cavanagh-1980")
        drop_ref(a, "mcmahon-1986")
        a["provenance"]["references"].append({"id": "dalleau-1998", "citation": "Dalleau, G., Belli, A., Bourdin, M., Lacour, J.-R. (1998). The spring-mass model and the energy cost of treadmill running. European Journal of Applied Physiology 77, 257-263 (treadmill running at about 5.1 m/s). Step frequency and flight time here are authored estimates guided by it."})
    if a["id"] == "person-with-umbrella":
        drop_ref(a, "rain-wetting")
    if a["id"] == "running-child":
        for r in a["provenance"]["references"]:
            if r["id"] == "jensen-1986":
                r["citation"] = "Jensen, R. K. (1986). Body segment mass, radius and radius of gyration proportions of children. Journal of Biomechanics 19(5), 359-368. The segment mass fractions here are rounded authored estimates guided by it."
        def cb(o):
            if o.get("reference") == "jensen-1986" and o.get("source") == "literature": o["source"] = "authored-estimate"
        walk(a, cb)
        for k, rep in (("en", ("are therefore tagged as literature-based estimates", "are tagged as authored estimates guided by Jensen (1986)")), ("de", ("sind daher als literaturbasierte Schätzung gekennzeichnet", "sind als Schätzung des Autors, geleitet von Jensen (1986), gekennzeichnet"))):
            for holder in (a["provenance"],):
                for kk in ("notes", "method"):
                    v = holder.get(kk)
                    if isinstance(v, dict) and k in v: v[k] = v[k].replace(*rep)
            if isinstance(a.get("summary"), dict): a["summary"][k] = a["summary"][k].replace(*rep)
            for pk in a["provenance"].values():
                if isinstance(pk, dict) and k in pk: pk[k] = pk[k].replace(*rep)
text = json.dumps(d, ensure_ascii=False, indent=2) + "\n"
for bad in ("per brief", "to be verified", "literature-based"): print(bad, text.count(bad))
open(F, "w").write(text)
