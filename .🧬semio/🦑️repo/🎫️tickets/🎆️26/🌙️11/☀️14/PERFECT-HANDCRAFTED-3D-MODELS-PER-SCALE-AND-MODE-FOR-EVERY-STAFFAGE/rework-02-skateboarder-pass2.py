"""🫁️ Second chisel pass of `humans/skateboarder` on the published state: at 1:200 and 1:500 the rib cage is one closed thoracic form
(the loose wire hoops and the wire sternum are gone), and the decisions of every scale say what is modelled now.
Rows of the cage in the torso frame, metres: (station up the spine, front at the sternum line, rear behind the spine, half breadth).

Usage: python3 rework-02-skateboarder-pass2.py [--publish]
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib02", os.path.join(T, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
ASSET = "humans/skateboarder"
CANDIDATE = os.path.join(T, "🗑️generated", "rework-skateboarder", "cand2", "🔣️.json")
CAGE = [(0.175, -0.100, 0.045, 0.118), (0.23, -0.122, 0.045, 0.134), (0.30, -0.122, 0.045, 0.138), (0.37, -0.095, 0.045, 0.124), (0.42, -0.072, 0.040, 0.092), (0.452, -0.045, 0.032, 0.056)]

DECISIONS = {
    "50": ("Rework of the skeleton: the clavicles run from the sternoclavicular joints at the manubrium to the acromioclavicular joints above the humeral heads (165 mm, rising 19 degrees) on their own bones, each scapula with acromion, coracoid process and glenoid hangs from its clavicle, and each hip bone (ilium, ischium, pubis) hangs from the sacroiliac joint on its own bone and carries the thigh at the acetabulum; the temporal bones carry the mandible, the patella lies on the femoral condyles and the fibula meets tibia and talus.",
           "Überarbeitung des Skeletts: Die Schlüsselbeine laufen auf eigenen Knochen von den Sternoklavikulargelenken am Manubrium zu den Akromioklavikulargelenken über den Oberarmköpfen (165 mm, 19 Grad ansteigend), jedes Schulterblatt mit Akromion, Rabenschnabelfortsatz und Gelenkpfanne hängt an seinem Schlüsselbein, und jedes Hüftbein (Darmbein, Sitzbein, Schambein) hängt auf eigenem Knochen am Iliosakralgelenk und trägt den Oberschenkel an der Hüftpfanne; die Schläfenbeine tragen den Unterkiefer, die Kniescheibe liegt auf den Femurkondylen und das Wadenbein trifft Schienbein und Sprungbein."),
    "100": ("Rework: the board is modelled for the 15 mm feature, a deck slab of 15 mm under the true top line with both kicks, per truck a baseplate, the bushing stack on the inclined kingpin, the hanger and an axle bar of 15 mm, and four wheels of 54 x 32 mm; grip tape, pivot and kingpin are below the scale. Clavicles and hip bones sit on their own lateral bones as at 1:50, with acromion, temporal bones and the fibular head and malleolus.",
            "Überarbeitung: Das Brett ist für das 15-mm-Merkmal modelliert, eine 15 mm starke Deckplatte unter der wahren Oberkante mit beiden Kicks, je Achse eine Grundplatte, der Lenkgummistapel auf dem geneigten Königsbolzen, der Achskörper und ein 15 mm starker Achsstab sowie vier Rollen von 54 x 32 mm; Griptape, Pivot und Königsbolzen liegen unter dem Maßstab. Schlüsselbeine und Hüftbeine sitzen wie in 1:50 auf eigenen seitlichen Knochen, mit Akromion, Schläfenbeinen sowie Wadenbeinkopf und Außenknöchel."),
    "200": ("Rework: the board is modelled for the 30 mm feature, a deck slab of 30 mm under the true top line, one block per truck between the wheels and four wheels of 54 x 32 mm. The rib cage is one closed thoracic form from the first rib to the costal margin around the spine, each clavicle is a rod at its true 13 mm from the top of the cage to the acromion on its own bone, the hip bones sit on their own bones and the heel reaches the deck.",
            "Überarbeitung: Das Brett ist für das 30-mm-Merkmal modelliert, eine 30 mm starke Deckplatte unter der wahren Oberkante, je Achse ein Block zwischen den Rollen und vier Rollen von 54 x 32 mm. Der Brustkorb ist eine geschlossene Thoraxform von der ersten Rippe bis zum Rippenbogen um die Wirbelsäule, jedes Schlüsselbein ist ein Stab in wahren 13 mm von der Oberkante des Brustkorbs zum Akromion auf eigenem Knochen, die Hüftbeine sitzen auf eigenen Knochen und die Ferse reicht bis zum Deck."),
    "500": ("Rework: at the 75 mm feature deck, trucks and wheels are one board mass that stands on the ground between the axles and lifts to the kicks under the true top line; trucks and wheels keep one thermal block each per axle inside it. The rib cage is one closed thoracic form around the spine, the clavicles are rods at their true 13 mm on their own bones and the pelvis block spans both hip bones.",
            "Überarbeitung: Beim 75-mm-Merkmal sind Deck, Achsen und Rollen eine Brettmasse, die zwischen den Achsen auf dem Boden steht und sich unter der wahren Oberkante zu den Kicks hebt; Achsen und Rollen behalten darin je Achse einen thermischen Block. Der Brustkorb ist eine geschlossene Thoraxform um die Wirbelsäule, die Schlüsselbeine sind Stäbe in wahren 13 mm auf eigenen Knochen und der Beckenblock überspannt beide Hüftbeine."),
}
STALE = {
    " The deck retains an 11.5 mm maple layup, a concave section and two kicks. Each truck supports two 54 mm road wheels on a 171 mm track.": "",
    ", clavicles and scapulae one shoulder-girdle bar": "",
    " Open ribs retain the vertebral and sternal attachments of this figure.": "",
    "The heat field follows the deck, hangers, axles and four wheel surfaces.": "The heat field follows the board mass with one block per axle for the trucks and one for the wheels.",
    "The rider retains separate spine, open ribs, shoulder girdle, limb, joint and mass geometry. The board transfers load through the deck, baseplates, kingpins, bushings, hangers, axles and wheels.": "The rider retains separate spine, rib cage, clavicle, limb, joint and mass geometry. The board transfers load through its one mass to the ground.",
    " Das konkave Ahorndeck bleibt 11,5 mm stark und steigt an Nose und Tail an. Jede Achse trägt zwei 54-mm-Rollen bei 171 mm Spurweite.": "",
    ", Schlüsselbeine und Schulterblätter ein Schultergürtelstab": "",
    " Offene Rippenzüge erhalten die Ansätze an Wirbelsäule und Brustbein dieser Figur.": "",
    "Das Temperaturfeld folgt Deck, Achskörpern, Achsstiften und den vier Rollen.": "Das Temperaturfeld folgt der Brettmasse mit je einem Block je Achse für die Achsen und für die Rollen.",
    "Wirbelsäule, offene Rippen, Schultergürtel, Gliedmaßen, Gelenke und Massen sind getrennt modelliert. Das Brett überträgt die Last über Deck, Grundplatten, Königsbolzen, Lenkgummis, Achskörper, Achsstifte und Rollen.": "Wirbelsäule, Brustkorb, Schlüsselbeine, Gliedmaßen, Gelenke und Massen sind getrennt modelliert. Das Brett überträgt die Last über seine eine Masse auf den Boden.",
}


def cage(scale):
    """🫁️ Thorax as one loft: egg-shaped in plan, widest at the eighth rib, narrowing to the first rib."""
    keys = [(u, (front + rear) / 2, 0, (rear - front) / 2, half, 2.3) for u, front, rear, half in (CAGE if scale == 200 else CAGE[::2] + CAGE[-1:])]
    return lib.loft("ribcage", "torso", "bone", 0.9, keys, sub=2 if scale == 200 else 1, round0=0.012, round1=0.012, member="torso", part="member")


def apply(volumes):
    for scale in (200, 500):
        structure = volumes[str(scale)]["structure"]
        first = next(i for i, s in enumerate(structure) if s["id"].startswith("open-"))
        volumes[str(scale)]["structure"] = [s for s in structure[:first] if not s["id"].startswith("open-")] + [cage(scale)] + [s for s in structure[first:] if not s["id"].startswith("open-")]
    for scale, (en, de) in DECISIONS.items():
        decisions = [d for d in volumes[scale]["decisions"] if not d["en"].startswith("Rework")]
        for decision in decisions:
            for old, new in STALE.items():
                decision["en"] = decision["en"].replace(old, new)
                decision["de"] = decision["de"].replace(old, new)
        volumes[scale]["decisions"] = decisions + [{"en": en, "de": de}]


def main():
    source = lib.load_source(ASSET)
    apply(source["volumes"])
    document = lib.load_catalog(ASSET)
    document["asset"]["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    print("[DEBUG] candidate", CANDIDATE)
    if "--publish" in sys.argv:
        lib.save_source(ASSET, source)
        lib.save_catalog(ASSET, lib.load_catalog(ASSET))
        print("[DEBUG] published")


main()
