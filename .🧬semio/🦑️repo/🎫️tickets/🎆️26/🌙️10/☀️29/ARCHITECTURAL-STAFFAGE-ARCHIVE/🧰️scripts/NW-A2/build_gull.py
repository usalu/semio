import sys
from gull_asset import *
from gull_reps import *

DIM = (0.51, 0.66)
def full():
    a = base()
    asset = {"id": ID, "category": "animals", "title": tx("Herring Gull", "Silbermöwe"),
        "summary": tx("An adult herring gull (Larus argentatus, 1.0 kg, wingspan 1.45 m): upright standing, soaring on spread wings and a landing flare to touchdown; rig with two-piece neck, arm and hand wings, webbed legs; plumage thermal model (41 degC core), nares breathing, back plume, breathing patterns for rest, soaring and the flare. The side elevation shows spread wings in oblique projection.",
                      "Eine erwachsene Silbermöwe (Larus argentatus, 1,0 kg, Spannweite 1,45 m): aufrechtes Stehen, Segeln mit gespreizten Flügeln und ein Landeanflug bis zum Aufsetzen; Rig mit zweiteiligem Hals, Flügeln aus Arm und Hand, Schwimmbeinen; Gefieder-Wärmemodell (41 Grad Kern), Nasenatmung, Rückenfahne, Atemmuster für Ruhe, Segeln und Abfangen. Der Seitenaufriss zeigt gespreizte Flügel in schräger Projektion."),
        "tags": ["animal", "bird", "gull", "coastal", "flight", "glide", "landing", "urban"],
        "provenance": provenance(REFS, ("Handcrafted: rig, outlines, poses and numbers are authored by hand; parameters without a verifiable source are tagged authored-estimate and name the reference they are guided by.",
                                        "Handgefertigt: Rig, Umrisse, Posen und Zahlen sind von Hand erstellt; Parameter ohne prüfbare Quelle sind als authored-estimate gekennzeichnet und nennen die Referenz, an der sie sich orientieren.")),
        "dimensions": {"heightM": DIM[0], "widthM": DIM[1], "depthM": 0.2, "massKg": MASS}}
    asset.update({k: a[k] for k in ["rig", "anchors", "materials", "poses", "clips", "thermal"]})
    asset["representations"] = {"500": rep500(), "200": rep200(), "100": rep100(), "50": rep50()}
    asset["symbols"] = []
    asset["structure"] = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        tx("Segments are rigid with authored masses summing to 1.0 kg; on the ground both feet carry the weight; in flight no support exists (ballistic phases) and the quasi-static check reports it.",
           "Segmente sind starr mit Massen von zusammen 1,0 kg; am Boden tragen beide Füße; im Flug gibt es kein Auflager (ballistische Phasen), was die quasi-statische Prüfung meldet."),
        tx("Joint moments are the moments of all distal weights and reactions about each joint; no muscle or aerodynamic model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; weder ein Muskel- noch ein Aerodynamikmodell wird angenommen.")]}
    asset["presentation"] = {"defaultPose": "standing", "defaultClip": "soar", "defaultScale": 100, "viewpoints": ["elevation"]}
    asset["exports"] = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
    asset["completion"] = "authored"
    return asset
if __name__ == "__main__":
    A = full(); write_asset(ID, A)
    for k in ("200", "100", "50"):
        print(k, [round(v, 3) for v in bounds_world(A, k, "standing")])
        for pid in ("standing", "gliding", "landing"):
            view = (-0.7, -0.1, 0.5, 0.6) if pid == "standing" else (-0.8, 0.9, 0.6, 2.0) if pid == "gliding" else (-0.8, 0.15, 0.6, 1.2)
            render(A, k, pid, f"{GEN}/gull-{k}-{pid}.png", view, px_per_mm={"200": 200, "100": 100, "50": 50}[k])
