"""🦴️ Runner with blade prosthesis: the skeleton of all four scales (author 02 rework): the measures of this runner for `rework-02-skeleton.py`; the right tibia and fibula end in the residual limb inside the socket, and the stored prosthesis members (socket, adapter, pyramid, pylon, blade) carry the path on to the blade."""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, f"rework-02-{name}.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


lib, skeleton = load("lib"), load("skeleton")
ASSET = "humans/runner-with-blade-prosthesis"
FINGERS = [(0.026, 0.127, 0.140, 0.136, 0.048), (0.006, 0.1285, 0.1423, 0.138, 0.050), (-0.014, 0.127, 0.140, 0.136, 0.048), (-0.031, 0.123, 0.134, 0.1306, 0.042)]
P = {
    "k": 1.0, "pelvis": 0.10323, "torso": 0.51, "neck": 0.10323, "shoulder_u": 0.43358, "shoulder_z": 0.18, "hip_z": 0.09,
    "thigh": 0.435, "shank": 0.425, "arm_length": 0.33, "forearm": 0.26, "hand": 0.19, "arm": "arm", "stump": "r", "stump_length": 0.150,
    "skull": {"cranium": (0.064, 0.000, 0), "cranium_radii": (0.080, 0.090, 0.071), "face": (-0.012, -0.050, 0), "face_radii": (0.040, 0.034, 0.050), "condyle": (0.018, -0.010, 0.050), "angle": (-0.045, -0.008, 0.046), "chin": (-0.074, -0.070, 0.010)},
    "foot": {"heel": (-0.0044, -0.0861), "ball": (0.1537, 0.0), "toe": 0.080, "lift": 0.022, "up": (-0.4782, 0.8783), "anchor": "ball-{}"},
    "hand_points": {"knuckle": 0.100, "palm_v": -0.002, "thumb": [(0.020, 0.024, -0.008), (0.062, 0.040, -0.018), (0.104, 0.042, -0.034)], "fingers": [[(0.100, v, -0.005), (u1, v, -0.012), (u2, v, -0.6 * w3), (u3, v, -w3)] for v, u1, u2, u3, w3 in FINGERS], "mitten_tip": (0.145, 0.002, -0.020)},
    "floor": {500: 0.0376}, "sole_material": "running-shoe",
}
HARDWARE = ("st-", "sk-pylon", "joint-blade-")
TEXT = {
    50: ("Structure 1:50: the skeleton as one connected load path: cranium with face and mandible on the cervical column, thoracolumbar column with lordosis, kyphosis and 17 spinous processes, twelve rib pairs from the column to the sternum, the costal arch or free, clavicles from the manubrium to the acromion, scapulae with spine, neck and glenoid, sacrum between the ilia with their crests, pubic and ischial rami meeting in the symphysis, femur with head, neck, trochanter and condyles, patella, tibia and fibula, talus, calcaneus, five metatarsals and toes, humerus, ulna and radius, carpus, metacarpus, three phalanges per finger and two of the thumb on the left and intact side; the right tibia and fibula end 15 cm below the knee inside the carbon socket, which carries the path over adapter and pyramid into the blade; every member overlaps its neighbour at the joint.",
         "Tragwerk 1:50: das Skelett als ein zusammenhängender Lastpfad: Hirnschädel mit Gesichtsschädel und Unterkiefer auf der Halswirbelsäule, Brust- und Lendenwirbelsäule mit Lordose, Kyphose und 17 Dornfortsätzen, zwölf Rippenpaare von der Wirbelsäule zum Brustbein, zum Rippenbogen oder frei endend, Schlüsselbeine vom Manubrium zum Akromion, Schulterblätter mit Gräte, Hals und Gelenkpfanne, Kreuzbein zwischen den Darmbeinen mit ihren Kämmen, Scham- und Sitzbeinäste, die sich in der Symphyse treffen, Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Kniescheibe, Schien- und Wadenbein, Sprungbein, Fersenbein, fünf Mittelfussknochen und Zehen, Oberarmknochen, Elle und Speiche, Handwurzel, Mittelhand, drei Glieder je Finger und zwei am Daumen auf der linken, unversehrten Seite; rechtes Schien- und Wadenbein enden 15 cm unter dem Knie im Karbonschaft, der den Pfad über Adapter und Pyramide in die Feder führt; jedes Glied überlappt seinen Nachbarn im Gelenk."),
    100: ("Structure 1:100: the same connected skeleton for a 15 mm feature: six rib pairs, no spinous processes, two metatarsal rays and toes per foot, the fingers one blade beside the thumb.",
          "Tragwerk 1:100: dasselbe zusammenhängende Skelett für 15 mm Mindestmass: sechs Rippenpaare, keine Dornfortsätze, zwei Mittelfussstrahlen und Zehen je Fuss, die Finger eine Fläche neben dem Daumen."),
    200: ("Structure 1:200: the load path in members of 30 mm: skull with jaw, cervical and trunk column, the ribcage as one basket, clavicles and scapulae to the shoulder joints, the pelvis as one basin with its hip struts, one member per limb bone, heel and forefoot.",
          "Tragwerk 1:200: der Lastpfad in Gliedern von 30 mm: Schädel mit Kiefer, Hals- und Rumpfwirbelsäule, der Brustkorb als ein Korb, Schlüsselbeine und Schulterblätter zu den Schultergelenken, das Becken als eine Schale mit ihren Hüftstreben, ein Glied je Gliedmassenknochen, Ferse und Vorfuss."),
    500: ("Structure 1:500: the same members as at 1:200 at the printable thickness of this scale; the path still runs from the skull over column, basket and basin into both legs down to the feet.",
          "Tragwerk 1:500: dieselben Glieder wie bei 1:200 in der druckbaren Stärke dieses Massstabs; der Pfad läuft weiterhin vom Schädel über Wirbelsäule, Korb und Schale in beide Beine bis in die Füsse."),
}


def main():
    source = lib.load_source(ASSET)
    for sc in (50, 100, 200, 500):
        entry = source["volumes"][str(sc)]
        masses = [s for s in entry["structure"] if s.get("part") == "mass"]
        hardware = [s for s in entry["structure"] if s["id"].startswith(HARDWARE)]
        entry["structure"] = skeleton.human_skeleton(P, sc, []) + hardware + masses
        en, de = TEXT[sc]
        entry["decisions"] = [d for d in entry["decisions"] if d["en"].startswith(("Body ", "Thermal "))] + [{"en": en, "de": de}]
        print(f"[DEBUG] {ASSET} 1:{sc}: structure {len(entry['structure'])}")
    lib.save_source(ASSET, source)


if __name__ == "__main__":
    main()
