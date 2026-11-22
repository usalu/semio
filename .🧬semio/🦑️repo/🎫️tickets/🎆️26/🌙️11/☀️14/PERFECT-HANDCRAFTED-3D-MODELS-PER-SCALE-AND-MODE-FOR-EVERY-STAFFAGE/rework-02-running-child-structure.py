"""🦴️ Running child: the skeleton of all four scales (author 02 rework): the measures of this child for `rework-02-skeleton.py`, plus the frame of the backpack hung from the clavicles."""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, f"rework-02-{name}.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


lib, skeleton = load("lib"), load("skeleton")
ASSET = "humans/running-child"
K = 0.64
FINGERS = [(0.018, 1.0), (0.005, 1.08), (-0.008, 1.0), (-0.020, 0.82)]
P = {
    "k": K, "chest": 0.66, "pelvis": 0.07, "torso": 0.29, "neck": 0.07, "shoulder_u": 0.2255, "shoulder_z": 0.105, "hip_z": 0.055,
    "thigh": 0.2565, "shank": 0.262, "arm_length": 0.1848, "forearm": 0.1456, "hand": 0.12, "arm": "arm",
    "skull": {"cranium": (0.066, 0.004, 0), "cranium_radii": (0.066, 0.084, 0.066), "face": (0.000, -0.045, 0), "face_radii": (0.028, 0.026, 0.040), "condyle": (0.016, -0.012, 0.042), "angle": (-0.022, -0.014, 0.036), "chin": (-0.042, -0.066, 0.008)},
    "foot": {"heel": (0.0088, -0.062), "ball": (0.0985, 0.0), "toe": 0.045, "lift": 0.014, "up": (-0.569, 0.823), "anchor": "ball-{}"},
    "hand_points": {"knuckle": 0.064, "palm_v": -0.001, "thumb": [(0.012, 0.016, -0.005), (0.040, 0.027, -0.011), (0.064, 0.029, -0.021)], "fingers": [[(0.064, v, -0.003), (0.064 + 0.018 * q, v, -0.008), (0.064 + 0.026 * q, v, -0.008 - 0.012 * q), (0.064 + 0.023 * q, v, -0.008 - 0.023 * q)] for v, q in FINGERS], "mitten_tip": (0.090, 0.001, -0.012)},
    "floor": {500: 0.0376}, "sole_material": "canvas-sneaker",
}
TEXT = {
    50: ("Structure 1:50: the child's skeleton as one connected load path, in child proportions (large cranium, short face, narrow ribcage, short long bones): skull with mandible, cervical and trunk column with its curves and spinous processes, twelve rib pairs to sternum, costal arch or free, clavicles, scapulae with spine and glenoid, sacrum, ilia with crests, pubic and ischial rami, femur with head, neck, trochanter and condyles, patella, tibia, fibula, talus, calcaneus, metatarsals, toes, humerus, ulna, radius, carpus, metacarpus and phalanges; the backpack panel hangs by its two straps from the clavicles; the feet stand on their contact anchors.",
         "Tragwerk 1:50: das Skelett des Kindes als ein zusammenhängender Lastpfad in Kinderproportionen (grosser Hirnschädel, kurzes Gesicht, schmaler Brustkorb, kurze Röhrenknochen): Schädel mit Unterkiefer, Hals- und Rumpfwirbelsäule mit ihren Krümmungen und Dornfortsätzen, zwölf Rippenpaare zum Brustbein, zum Rippenbogen oder frei, Schlüsselbeine, Schulterblätter mit Gräte und Pfanne, Kreuzbein, Darmbeine mit Kämmen, Scham- und Sitzbeinäste, Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Kniescheibe, Schienbein, Wadenbein, Sprungbein, Fersenbein, Mittelfussknochen, Zehen, Oberarmknochen, Elle, Speiche, Handwurzel, Mittelhand und Fingerglieder; die Rucksackplatte hängt mit ihren zwei Riemen an den Schlüsselbeinen; die Füsse stehen auf ihren Kontaktankern."),
    100: ("Structure 1:100: the same connected skeleton for a 15 mm feature: six rib pairs, no spinous processes, two metatarsal rays per foot, one finger blade beside the thumb; pack panel and straps kept.",
          "Tragwerk 1:100: dasselbe zusammenhängende Skelett für 15 mm Mindestmass: sechs Rippenpaare, keine Dornfortsätze, zwei Mittelfussstrahlen je Fuss, eine Fingerfläche neben dem Daumen; Rucksackplatte und Riemen bleiben."),
    200: ("Structure 1:200: the load path in members of 30 mm: skull with jaw, column, ribcage basket, clavicles and scapulae, pelvis basin with hip struts, one member per limb bone, pack panel on its straps.",
          "Tragwerk 1:200: der Lastpfad in Gliedern von 30 mm: Schädel mit Kiefer, Wirbelsäule, Brustkorb als Korb, Schlüsselbeine und Schulterblätter, Beckenschale mit Hüftstreben, ein Glied je Gliedmassenknochen, Rucksackplatte an ihren Riemen."),
    500: ("Structure 1:500: the same members as at 1:200 at the printable thickness of this scale.",
          "Tragwerk 1:500: dieselben Glieder wie bei 1:200 in der druckbaren Stärke dieses Massstabs."),
}


def pack_frame(sc):
    """🎒️ Panel and straps of the backpack: the straps run from the panel top over the shoulders to the middle of the clavicles."""
    f = max(skeleton.FLOOR[sc], P["floor"].get(sc, 0))
    panel = lib.box("sk-pack-panel", "pack", "bone", 0.85, (0.040, 0.034, 0), (0.220, max(0.012, 2 * f), 0.150), 0.004)
    panel.update(member="pack", part="member")
    out = [panel]
    for s, t in ((1, "l"), (-1, "r")):
        strap = lib.capsule(f"sk-strap-{t}", "pack", "bone", 0.85, (0.140, 0.030, 0.052 * s), (0.1835, -0.0597, 0.0544 * s), max(0.004, f))
        strap.update(member="pack", part="member")
        out.append(strap)
    return out


def main():
    source = lib.load_source(ASSET)
    for sc in (50, 100, 200, 500):
        entry = source["volumes"][str(sc)]
        masses = [s for s in entry["structure"] if s.get("part") == "mass"]
        entry["structure"] = skeleton.human_skeleton(P, sc, []) + pack_frame(sc) + masses
        en, de = TEXT[sc]
        entry["decisions"] = [d for d in entry["decisions"] if d["en"].startswith(("Body ", "Thermal "))] + [{"en": en, "de": de}]
        print(f"[DEBUG] {ASSET} 1:{sc}: structure {len(entry['structure'])}")
    lib.save_source(ASSET, source)


if __name__ == "__main__":
    main()
