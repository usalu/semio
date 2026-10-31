from squirrel_asset import *
from squirrel_reps import *
DIM = (0.275, 0.21)
K = 0.80
def full():
    a = base()
    asset = {"id": ID, "category": "animals", "title": tx("Red Squirrel", "Eichhörnchen"),
        "summary": tx("An adult Eurasian red squirrel (Sciurus vulgaris, 0.30 kg): two-part spine, bushy two-part tail, fore limbs with long fingers and digitigrade hind limbs; sitting upright eating a nut, a bounding run and climbing a vertical trunk (the trunk is not part of the asset: the claws are supports with a bark friction of 1.5); fur thermal model with 38.5 degC core, nose breathing and a back plume.",
                      "Ein erwachsenes Europäisches Eichhörnchen (Sciurus vulgaris, 0,30 kg): zweiteilige Wirbelsäule, buschiger zweiteiliger Schwanz, Vorderglieder mit langen Fingern und Hinterglieder; aufrecht sitzend beim Nussfressen, ein Sprunglauf und Klettern an einem senkrechten Stamm (der Stamm ist nicht Teil des Assets: die Krallen sind Auflager mit Rindenreibung 1,5); Fell-Wärmemodell mit 38,5 Grad Kern, Nasenatmung und Rückenfahne."),
        "tags": ["animal", "mammal", "rodent", "squirrel", "climbing", "bounding", "park", "wood"],
        "provenance": provenance(REFS, ("Handcrafted: rig, outlines, poses and numbers are authored by hand; parameters without a verifiable source are tagged authored-estimate and name the reference they are guided by.", "Handgefertigt: Rig, Umrisse, Posen und Zahlen sind von Hand erstellt; Parameter ohne prüfbare Quelle sind als authored-estimate gekennzeichnet und nennen die Referenz, an der sie sich orientieren.")),
        "dimensions": {"heightM": DIM[0], "widthM": DIM[1], "depthM": 0.12, "massKg": MASS}}
    asset.update({k: a[k] for k in ["rig", "anchors", "materials", "poses", "clips", "thermal"]})
    asset["representations"] = {"500": rep500(), "200": rep200(), "100": rep100(), "50": rep50()}
    asset["symbols"] = []
    asset["structure"] = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        tx("Segments are rigid with authored masses summing to 0.30 kg; on the ground the feet carry the weight; on a trunk the claws of all four limbs are supports with the normal pointing away from the trunk and a friction coefficient of 1.5; airborne phases of the bound are ballistic.",
           "Segmente sind starr mit Massen von zusammen 0,30 kg; am Boden tragen die Füße; am Stamm sind die Krallen aller vier Gliedmaßen Auflager mit der Normalen vom Stamm weg und der Reibungszahl 1,5; die Flugphasen des Sprungs sind ballistisch."),
        tx("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen.")]}
    asset["presentation"] = {"defaultPose": "sitting-eating", "defaultClip": "nibble", "defaultScale": 50, "viewpoints": ["elevation"]}
    asset["exports"] = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
    asset["completion"] = "authored"
    scale_asset(asset, K)
    return asset
if __name__ == "__main__":
    A = full(); write_asset(ID, A)
    for k in ("100", "50"):
        print(k, [round(v, 3) for v in bounds_world(A, k, "sitting-eating")])
        for pid, view in (("sitting-eating", (-0.25, -0.02, 0.2, 0.34)), ("running", (-0.3, -0.02, 0.5, 0.3)), ("climbing", (-0.25, 0.0, 0.2, 0.7))):
            render(A, k, pid, f"{GEN}/squirrel-{k}-{pid}.png", view, px_per_mm={"200": 400, "100": 250, "50": 120}[k])
