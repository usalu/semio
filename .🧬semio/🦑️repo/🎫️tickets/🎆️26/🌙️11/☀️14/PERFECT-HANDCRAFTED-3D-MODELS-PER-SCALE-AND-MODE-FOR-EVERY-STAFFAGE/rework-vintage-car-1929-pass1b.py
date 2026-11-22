"""🌡️ Second-round pass 1, thermal stage on vehicles/vintage-car-1929: bumpers, lamps and the spare-wheel cover enter the thermal list of every scale in the regions their 2D shapes already carry.

Usage: python3 rework-vintage-car-1929-pass1b.py            writes the candidate document under 🗑️generated/r3-vintage-car-1929/
       python3 rework-vintage-car-1929-pass1b.py --apply    writes the volumes source (publish with volumes-insert.py)
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/vintage-car-1929/🔣️.json"
SOURCE = os.path.join(T, "volumes/vehicles/vintage-car-1929.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-vintage-car-1929/🔣️.json")

REGION_OF = (
    ("radiator-core", "radiator"),
    ("radiator-cap", "radiator"),
    ("bumper", "body-panels"),
    ("tail-lamp", "body-panels"),
    ("spare-cover", "body-panels"),
    ("headlamp-bowl", "fenders"),
    ("headlamp-l", "fenders"),
    ("headlamp-r", "fenders"),
    ("lamp-bar", "fenders"),
    ("lamp-stalk", "fenders"),
    ("lamp-post", "fenders"),
    ("fender-lamp-l", "fenders"),
    ("fender-lamp-r", "fenders"),
    ("fender-lamp-foot", "fenders"),
)

RADIATOR_AREA_M2 = 1.1
RADIATOR_AREA_OLD = '"areaM2": 0.75,'

DECISION = {
    "en": "Thermal: the nickel bumpers, the tail lamp and the spare-wheel cover count with the painted panels, the headlamps with their bar and stalks with the wings, as in the 2D drawings, so the thermal model shows the whole car. The radiator region now holds its core as well as the shell (hand value 1.1 m²: two core faces 0.58, shell band 0.26, face rings 0.17, neck and cap 0.03, recess edges 0.06).",
    "de": "Thermik: die vernickelten Stoßstangen, die Schlussleuchte und die Reserveradabdeckung zählen zu den lackierten Blechen, die Scheinwerfer mit Stange und Stützen zu den Kotflügeln, wie in den 2D-Zeichnungen, sodass das thermische Modell den ganzen Wagen zeigt. Die Region Kühler enthält nun neben der Schale auch das Kühlernetz (Handwert 1,1 m²: zwei Netzflächen 0,58, Schalenband 0,26, Stirnringe 0,17, Stutzen und Deckel 0,03, Rücksprungkanten 0,06).",
}


def region_of(ident):
    """🗺️ The thermal region of a trim solid, or None when it stays out of the thermal list."""
    for prefix, region in REGION_OF:
        if ident.startswith(prefix):
            return region
    return None


def edit(volumes):
    """✍️ One thermal twin per trim solid of every scale."""
    for scale, lists in volumes.items():
        twins = {entry.get("of") for entry in lists["thermal"]}
        added = []
        for solid in lists["body"]:
            region = region_of(solid["id"])
            if region and solid["id"] not in twins:
                twin = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
                if region != "radiator":
                    twin["layer"] = "paint"
                lists["thermal"].append(twin)
                added.append(solid["id"])
        lists["decisions"] = [d for d in lists["decisions"] if not d["en"].startswith("Thermal: the nickel bumpers")] + [DECISION]
        print("[DEBUG] 1:%s thermal twins added: %s" % (scale, added))


def write(path, value):
    """💾 One JSON document."""
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, ensure_ascii=False, indent=1) + "\n")


def main():
    """🏁 Candidate by default, source with --apply."""
    source = json.load(open(SOURCE, encoding="utf-8"))
    edit(source["volumes"])
    text = open(CATALOG, encoding="utf-8").read()
    assert text.count(RADIATOR_AREA_OLD) <= 1
    text = text.replace(RADIATOR_AREA_OLD, '"areaM2": %s,' % RADIATOR_AREA_M2)
    if "--apply" in sys.argv:
        write(SOURCE, source)
        with open(CATALOG, "w", encoding="utf-8") as handle:
            handle.write(text)
    else:
        document = json.loads(text)
        assert next(r for r in document["asset"]["thermal"]["regions"] if r["id"] == "radiator")["areaM2"] == RADIATOR_AREA_M2
        document["asset"]["volumes"] = source["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        write(CANDIDATE, document)


main()
