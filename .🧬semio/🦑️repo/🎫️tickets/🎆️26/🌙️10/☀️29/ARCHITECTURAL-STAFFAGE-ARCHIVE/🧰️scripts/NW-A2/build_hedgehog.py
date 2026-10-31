import sys
from hedgehog_asset import *
from hedgehog_reps import *
DIM = (0.139, 0.278)
def full(adv=0.12):
    a = base(adv)
    asset = {"id": ID, "category": "animals", "title": tx("European Hedgehog", "Westeuropäischer Igel"),
        "summary": tx("An adult European hedgehog (Erinaceus europaeus, 1.0 kg): spiny dome with front and rear spine skirts that swing down to close the animal into a defensive ball, pointed snout, four short limbs; foraging, alert and rolled-up poses, a slow foraging walk and the roll-up reaction; spine and fur thermal model with 35.5 degC core, nose breathing and a back plume. Too small for 1:500.",
                      "Ein erwachsener Westeuropäischer Igel (Erinaceus europaeus, 1,0 kg): Stachelkuppel mit vorderem und hinterem Stachelsaum, die sich zum Abwehrball um das Tier schließen, spitze Schnauze, vier kurze Gliedmaßen; Suchen, Wachsamkeit und eingerollt, ein langsamer Suchgang und die Einrollreaktion; Stachel- und Fell-Wärmemodell mit 35,5 Grad Kern, Nasenatmung und Rückenfahne. Zu klein für 1:500."),
        "tags": ["animal", "mammal", "hedgehog", "spines", "defensive", "garden", "nocturnal"],
        "provenance": provenance(REFS, ("Handcrafted: rig, outlines, poses and numbers are authored by hand; parameters without a verifiable source are tagged authored-estimate and name the reference they are guided by.", "Handgefertigt: Rig, Umrisse, Posen und Zahlen sind von Hand erstellt; Parameter ohne prüfbare Quelle sind als authored-estimate gekennzeichnet und nennen die Referenz, an der sie sich orientieren.")),
        "dimensions": {"heightM": DIM[0], "widthM": DIM[1], "depthM": 0.15, "massKg": MASS}}
    asset.update({k: a[k] for k in ["rig", "anchors", "materials", "poses", "clips", "thermal"]})
    asset["representations"] = {"500": rep500(), "200": rep200(), "100": rep100(), "50": rep50()}
    asset["symbols"] = []
    asset["structure"] = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        tx("Segments are rigid with authored masses summing to 1.0 kg; on the ground all four feet carry the weight; rolled up, the weight rests on the tips of the two spine skirts (unilateral supports with a friction coefficient of 0.6).",
           "Segmente sind starr mit Massen von zusammen 1,0 kg; am Boden tragen alle vier Füße; eingerollt ruht das Gewicht auf den Spitzen der beiden Stachelsäume (einseitige Auflager mit Reibungszahl 0,6)."),
        tx("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen.")]}
    asset["presentation"] = {"defaultPose": "foraging", "defaultClip": "forage-walk", "defaultScale": 50, "viewpoints": ["elevation"]}
    asset["exports"] = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
    asset["completion"] = "authored"
    return asset
if __name__ == "__main__":
    adv = float(sys.argv[1]) if len(sys.argv) > 1 else 0.12
    A = full(adv); write_asset(ID, A)
    for k in ("100", "50"):
        print(k, [round(v, 3) for v in bounds_world(A, k, "foraging")])
        for pid in ("foraging", "alert", "rolled"):
            render(A, k, pid, f"{GEN}/hedgehog-{k}-{pid}.png", (-0.12, -0.02, 0.26, 0.2), px_per_mm={"200": 700, "100": 400, "50": 220}[k])
