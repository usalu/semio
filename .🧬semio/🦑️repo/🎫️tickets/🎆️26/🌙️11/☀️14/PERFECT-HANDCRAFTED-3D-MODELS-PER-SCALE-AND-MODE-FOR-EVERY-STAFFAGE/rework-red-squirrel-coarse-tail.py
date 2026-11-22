"""🧹 Red squirrel, 1:100 to 1:500: single hair locks of the brush (7 mm wide), muzzle pads and the nostril recess are below
the smallest feature and leave the lists; the decisions texts say what the lists hold after the second round, pass 2.

Usage: python3 rework-red-squirrel-coarse-tail.py candidate|publish [scales without locks, default 100,200,500]
"""
import json
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-squirrel/🔣️.json"
SOURCE = "volumes/animals/red-squirrel.json"
CANDIDATE = "🗑️generated/r3-red-squirrel/🔣️.json"
FEATURE = {"100": 15, "200": 30, "500": 75}
GONE_AT_100 = ("nasal-recess", "near-muzzle-pad", "far-muzzle-pad")


def gone(name, scale, no_locks):
    return (name.startswith("brush-lock-") and scale in no_locks) or (scale == "100" and name in GONE_AT_100)


def clear(volume, scale, no_locks):
    volume["body"] = [s for s in volume["body"] if not gone(s["id"], scale, no_locks)]
    volume["thermal"] = [s for s in volume["thermal"] if not gone(s.get("of", ""), scale, no_locks)]


def add(entry, en, de):
    entry["en"], entry["de"] = entry["en"] + " " + en, entry["de"] + " " + de


def texts(volumes, no_locks):
    for scale in FEATURE:
        volume = volumes[scale]
        count = len(volume["body"])
        brush_en = "the brush is six links without single hair locks, which are 7 mm wide and far below the feature" if scale in no_locks else "the brush is six links with a few hair locks that carry the fringe of its outline"
        brush_de = "der Schwanz besteht aus sechs Gliedern ohne einzelne Haarsträhnen, die mit 7 mm Breite weit unter dem kleinsten Merkmal liegen" if scale in no_locks else "der Schwanz besteht aus sechs Gliedern mit wenigen Haarsträhnen, die den Saum seines Umrisses tragen"
        volume["decisions"][0] = {
            "en": f"Body at 1:{scale} (smallest feature {FEATURE[scale]} mm): {count} solids. Rump, loin, chest, neck and head are lofts with sections of their own; the loin sits on the lumbar bone, so the back arches between pelvis and thorax; the white underside is three shells (rump, loin, chest); per side upper arm, forearm, hand with one mitten for the fingers, thigh, shank, foot and a sole that runs to the toe tips; two leaf ears; {brush_en}. Corneas and nose pad remain as small solids only because they carry the thermal regions of eyes and nose.",
            "de": f"Körper bei 1:{scale} (kleinstes Merkmal {FEATURE[scale]} mm): {count} Körper. Becken, Lende, Brust, Hals und Kopf sind Lofts mit eigenen Schnitten; die Lende sitzt auf dem Lendenknochen, sodass sich der Rücken zwischen Becken und Brustkorb wölbt; die weiße Unterseite besteht aus drei Schalen (Becken, Lende, Brust); je Seite Oberarm, Unterarm, Hand mit einem Fäustling für die Finger, Oberschenkel, Unterschenkel, Fuß und eine Sohle bis zu den Zehenspitzen; zwei Blattohren; {brush_de}. Hornhäute und Nasenspiegel bleiben nur als kleine Körper, weil sie die thermischen Regionen Augen und Nase tragen.",
        }
        structure = next(d for d in volume["decisions"] if "Structure at" in d["en"])
        if scale == "100":
            add(structure, "The lumbar spine (L3 to L7) is a bar of its own on the lumbar bone; the sacrum is 19.5 mm long and 16.7 mm across the alae.", "Die Lendenwirbelsäule (L3 bis L7) ist ein eigener Stab auf dem Lendenknochen; das Kreuzbein ist 19,5 mm lang und über die Flügel 16,7 mm breit.")
        else:
            add(structure, "One bar spans pelvis and lumbar spine, because a lumbar bar of 34 mm is shorter than two features; the sacrum is 19.5 mm long and 16.7 mm across the alae.", "Ein Stab überspannt Becken und Lendenwirbelsäule, weil ein Lendenstab von 34 mm kürzer als zwei Merkmale ist; das Kreuzbein ist 19,5 mm lang und über die Flügel 16,7 mm breit.")
    fine = volumes["50"]["decisions"]
    add(next(d for d in fine if d["en"].startswith("Body")), "The trunk is cut at the lumbosacral joint: the rump stays on the pelvis, the loin sits on the lumbar bone, and each is tucked 8 mm into the other so that the arching back never opens a gap.", "Der Rumpf ist am Lenden-Kreuzbein-Gelenk geteilt: Das Becken bleibt auf dem Beckenknochen, die Lende sitzt auf dem Lendenknochen, und beide greifen 8 mm ineinander, damit der gewölbte Rücken nie einen Spalt öffnet.")
    add(next(d for d in fine if d["en"].startswith("Thermal")), "Underfur and guard hair of rump and belly patch are cut at the lumbosacral joint like the body.", "Unterwolle und Deckhaar von Becken und Bauchfleck sind wie der Körper am Lenden-Kreuzbein-Gelenk geteilt.")
    add(next(d for d in fine if d["en"].startswith("Structure")), "Second round, pass 2: the lumbar column L3 to L7 lies on its own bone with a joint solid at the lumbosacral joint; radius and ulna (with olecranon) and tibia and fibula are separate bones; the hand is a carpus with four metacarpals, the foot a tarsus with calcaneus and five metatarsals in front of the phalanges; the skull narrows behind the orbits and carries zygomatic arches that stand clear of it, the mandible is two rami that meet at the symphysis, and upper and lower incisors are modelled; the sacrum is 19.5 mm long and 16.7 mm across the alae.", "Zweite Runde, Durchgang 2: Die Lendenwirbel L3 bis L7 liegen auf einem eigenen Knochen mit einem Gelenkkörper am Lenden-Kreuzbein-Gelenk; Speiche und Elle (mit Ellenhaken) sowie Schienbein und Wadenbein sind getrennte Knochen; die Hand ist eine Handwurzel mit vier Mittelhandknochen, der Fuß eine Fußwurzel mit Fersenbein und fünf Mittelfußknochen vor den Zehengliedern; der Schädel verjüngt sich hinter den Augenhöhlen und trägt frei stehende Jochbögen, der Unterkiefer besteht aus zwei Ästen, die sich an der Symphyse treffen, obere und untere Schneidezähne sind modelliert; das Kreuzbein ist 19,5 mm lang und über die Flügel 16,7 mm breit.")


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    source = json.load(open(SOURCE, encoding="utf-8"))
    if source["volumes"] != document["asset"]["volumes"]:
        raise SystemExit("source and catalog volumes differ")
    no_locks = sys.argv[2].split(",") if len(sys.argv) > 2 else list(FEATURE)
    volumes = source["volumes"]
    if "Second round, pass 2" in json.dumps(volumes["50"]["decisions"]):
        raise SystemExit("already applied")
    for scale in FEATURE:
        clear(volumes[scale], scale, no_locks)
        print("[DEBUG]", scale, {k: len(volumes[scale][k]) for k in ("body", "thermal", "structure")})
    texts(volumes, no_locks)
    if sys.argv[1] == "candidate":
        document["asset"]["volumes"] = volumes
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        return
    json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("source written; run volumes-insert.py")


main()
