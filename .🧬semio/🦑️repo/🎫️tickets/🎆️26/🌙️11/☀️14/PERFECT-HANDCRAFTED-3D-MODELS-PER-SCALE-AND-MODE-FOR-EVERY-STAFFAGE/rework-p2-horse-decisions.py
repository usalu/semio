"""🐴 Horse: decisions of the second pass per scale (girdles on the lateral rest placement, pelvis, skull, ears)."""
import json

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
SOURCE = f"{T}/volumes/animals/horse.json"
MARK = "Rig of 2026-11-22"
RIG_EN = "Rig of 2026-11-22: the limbs hang from the girdles at their true breadths. Each shoulder blade lies on the ribs from its cartilage 0.09 m beside the midline down to the shoulder joint 0.19 m beside it (no collar bone: the trunk hangs between the blades); each hip bone runs from the sacroiliac joint 0.06 m beside the midline to the hip joint 0.15 m beside it, and the leg bones run through elbow, carpus, stifle, hock and fetlock down to hooves 0.142 m and 0.146 m beside the midline, so the horse stands narrower at the ground than at the chest."
RIG_DE = "Rig vom 22.11.2026: Die Gliedmaßen hängen in wahrer Breite an den Gürteln. Jedes Schulterblatt liegt auf den Rippen, vom Knorpel 0,09 m neben der Mitte hinab zum Schultergelenk 0,19 m daneben (kein Schlüsselbein: der Rumpf hängt zwischen den Blättern); jedes Hüftbein läuft vom Kreuzdarmbeingelenk 0,06 m neben der Mitte zum Hüftgelenk 0,15 m daneben, und die Beinknochen laufen durch Ellbogen, Vorderfußwurzel, Knie, Sprunggelenk und Fesselkopf zu Hufen 0,142 m und 0,146 m neben der Mitte, sodass das Pferd am Boden schmaler steht als an der Brust."
EXTRA = {
    "50": (
        " The pelvis is built per side from an ilium wing between tuber sacrale and tuber coxae that narrows to the shaft above the hip socket, an ischial plate back to the seat bone and the sacroiliac bridge onto the sacrum; the skull has a brain case, a deep face that narrows to the incisors, zygomatic arches and facial crests. A rounded mass under the front of the withers keeps withers and crest closed when the neck drops to graze.",
        " Das Becken besteht je Seite aus einer Darmbeinschaufel zwischen Kreuz- und Hüfthöcker, die sich zur Säule über der Pfanne verjüngt, einer Sitzbeinplatte bis zum Sitzbeinhöcker und der Brücke des Kreuzdarmbeingelenks auf das Kreuzbein; der Schädel hat Hirnkapsel, einen tiefen, zu den Schneidezähnen schmaler werdenden Gesichtsteil, Jochbögen und Gesichtsleisten. Eine gerundete Masse unter dem vorderen Widerrist hält Widerrist und Kamm geschlossen, wenn der Hals zum Grasen sinkt.",
    ),
    "100": (
        " The pelvis halves are an ilium wing, an ischial plate and the sacroiliac bridge with four and three sections.",
        " Die Beckenhälften sind Darmbeinschaufel, Sitzbeinplatte und Kreuzdarmbeinbrücke mit vier und drei Schnitten.",
    ),
    "200": (
        " The pelvis halves keep wing, plate and bridge at 32 mm and more.",
        " Die Beckenhälften behalten Schaufel, Platte und Brücke mit 32 mm und mehr.",
    ),
    "500": (
        " Each hip bone stays one bar on its own bone; the ears are pointed buds of 75 mm instead of capped tubes.",
        " Jedes Hüftbein bleibt ein Stab an seinem eigenen Knochen; die Ohren sind spitze Knospen von 75 mm statt gedeckelter Rohre.",
    ),
}


def main():
    document = json.load(open(SOURCE))
    for scale, (en, de) in EXTRA.items():
        decisions = [d for d in document["volumes"][scale]["decisions"] if not d["en"].startswith(MARK)]
        decisions.append({"en": RIG_EN + en, "de": RIG_DE + de})
        document["volumes"][scale]["decisions"] = decisions
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)


main()
