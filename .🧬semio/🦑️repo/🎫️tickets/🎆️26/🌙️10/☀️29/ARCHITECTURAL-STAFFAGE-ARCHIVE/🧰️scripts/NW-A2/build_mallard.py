import sys
from mallard_asset import *
from mallard_reps import *

def full(advance):
    a = base(advance)
    asset = {"id": ID, "category": "animals",
             "title": tx("Mallard Drake", "Stockerpel"),
             "summary": tx("An adult drake mallard (Anas platyrhynchos, 1.15 kg): boat-shaped body with a three-piece S-neck, flat bill, folded wings that can be raised, short legs with webbed feet and a waterline contact for swimming; standing, floating, dabbling and upending poses, a waddling walk, paddling and a dabbling cycle, with a plumage thermal model (41.5 degC core), nostril breathing and a back plume.",
                           "Ein erwachsener Stockerpel (Anas platyrhynchos, 1,15 kg): bootsförmiger Körper mit dreiteiligem S-Hals, flachem Schnabel, angelegten und hebbaren Flügeln, kurzen Beinen mit Schwimmfüßen und einem Wasserlinien-Kontakt zum Schwimmen; Stehen, Schwimmen, Gründeln und Kopfstand, Watschelgang, Paddeln und ein Gründelzyklus, mit Gefieder-Wärmemodell (41,5 Grad Kern), Nasenatmung und Rückenfahne."),
             "tags": ["animal", "bird", "duck", "waterfowl", "swimming", "waddle", "dabbling", "urban"],
             "provenance": provenance(REFS, ("Handcrafted: rig, outlines, poses and numbers are authored by hand. Parameters without a verifiable source are tagged authored-estimate and name the reference they are guided by; scale decisions follow the office-laser print profile.",
                                            "Handgefertigt: Rig, Umrisse, Posen und Zahlen sind von Hand erstellt. Parameter ohne prüfbare Quelle sind als authored-estimate gekennzeichnet und nennen die Referenz, an der sie sich orientieren; Maßstabsentscheidungen folgen dem Büro-Laser-Druckprofil.")),
             "dimensions": {"heightM": DIM[0], "widthM": DIM[1], "depthM": 0.2, "massKg": MASS}}
    asset.update({k: a[k] for k in ["rig", "anchors", "materials", "poses", "clips", "thermal"]})
    asset["representations"] = {"500": rep500(), "200": rep200(), "100": rep100(), "50": rep50()}
    asset["symbols"] = []
    asset["structure"] = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        tx("Segments are rigid with authored masses summing to 1.15 kg; on land both feet carry the weight and share it by position; in water the waterline anchor stands for the buoyant force of the displaced water (a unilateral support with a footprint of the waterplane).",
           "Segmente sind starr mit Massen von zusammen 1,15 kg; an Land tragen beide Füße und teilen die Last nach Lage; im Wasser steht der Wasserlinienanker für die Auftriebskraft des verdrängten Wassers (einseitiges Auflager mit der Fläche der Wasserlinie)."),
        tx("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen.")]}
    asset["presentation"] = {"defaultPose": "standing", "defaultClip": "waddle", "defaultScale": 100, "viewpoints": ["elevation"]}
    asset["exports"] = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
    asset["completion"] = "authored"
    return asset

DIM = (0.46, 0.60)
ADV = 0.25
if __name__ == "__main__":
    adv = float(sys.argv[1]) if len(sys.argv) > 1 else ADV
    A = full(adv)
    write_asset(ID, A)
    for k in ("200", "100", "50"):
        print(k, [round(v, 3) for v in bounds_world(A, k, "standing")])
        for pid in ("standing", "swimming", "dabbling", "wing-stretch"):
            render(A, k, pid, f"{GEN}/mallard-{k}-{pid}.png", (-0.35, -0.2, 0.45, 0.5), px_per_mm=int({"200": 260, "100": 130, "50": 65}[k]))
