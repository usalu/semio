"""🛏️ Author 64: replaces the template line "Thermal and structure follow the 1:100 body." of single-bed, picnic-table and operating-table at 1:100 by asset-specific sentences."""
import json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
NEW = {
 "furniture/single-bed": ("Thermal: the six frame solids (head, foot post, foot rail, cross rail, side rail, middle leg) reference the bed-frame region, the three slat planks the slats region and the three mattress blocks the mattress region, each by a reference to the body solid it covers. Structure: a centre beam on the ground, a member for the head, the foot post, the side rail and the middle leg, one for each slat plank and each mattress block, a hinge joint at the head slat, one joint per mattress block and four foot supports (one at the head, one in the middle, two at the foot).",
   "Thermik: die sechs Rahmenkörper (Kopfteil, Fußpfosten, Fußrahmen, Querriegel, Seitenholm, Mittelbein) verweisen auf die Region Bettrahmen, die drei Lattenbohlen auf die Region Lattenrost und die drei Matratzenblöcke auf die Region Matratze, jeweils per Verweis auf den Körper, den sie bedecken. Tragwerk: ein Mittelbalken am Boden, je ein Stab für Kopfteil, Fußpfosten, Seitenholm und Mittelbein, je einer für jede Lattenbohle und jeden Matratzenblock, ein Scharniergelenk an der Kopflatte, ein Gelenk je Matratzenblock und vier Fußauflager (eines am Kopf, eines in der Mitte, zwei am Fuß)."),
 "furniture/picnic-table": ("Thermal: the two legs, the top beam and the cross beam reference the A-frame region, the two hinges the hinges region, the one-slab top the table-top region and the two benches the bench-seats region, all by reference to the 1:100 body solids. Structure: two foot members on the ground, members for both legs, the cross beam, the top beam, the top slab and each bench board, a hinge joint at each bench, four foot supports (two per leg pair) and mass solids for the frame, the top and both benches.",
   "Thermik: die zwei Beine, der obere Balken und der Querbalken verweisen auf die Region A-Gestell, die zwei Scharniere auf die Region Scharniere, die einteilige Platte auf die Region Tischplatte und die zwei Bänke auf die Region Sitzbänke, alle per Verweis auf die Körper von 1:100. Tragwerk: zwei Fußstäbe am Boden, Stäbe für beide Beine, den Querbalken, den oberen Balken, die Tischplatte und jedes Bankbrett, ein Scharniergelenk an jeder Bank, vier Fußauflager (je zwei pro Beinpaar) und Massenkörper für Gestell, Tischplatte und beide Bänke."),
 "furniture/operating-table": ("Thermal: the base plate and its two castors reference the table-base region, the three column boxes the table-column region, the carrier the table-frame region, the four pad slabs (head, back, seat, leg) the table-pad region, the stand parts the stand region and the lamp housing, lens and handle the light-housing and light-lens regions; the light plume is the only solid of its own. Structure: members for the table base, the three column boxes, the top carrier, the stand base, column and arm and the lamp housing, a tilt joint under the top and a joint at the stand arm, eight castor supports (four under the table, four under the stand) and seven mass solids.",
   "Thermik: die Grundplatte und ihre zwei Rollen verweisen auf die Region Tischfuß, die drei Säulenquader auf die Region Tischsäule, der Träger auf die Region Tischrahmen, die vier Polsterplatten (Kopf, Rücken, Sitz, Bein) auf die Region Polster, die Ständerteile auf die Region Ständer und Leuchtengehäuse, Linse und Griff auf die Regionen Leuchtengehäuse und Leuchtenlinse; die Leuchtenfahne ist der einzige eigene Körper. Tragwerk: Stäbe für Tischfuß, die drei Säulenquader, den Plattenträger, Ständerfuß, Ständersäule und Ständerarm sowie das Leuchtengehäuse, ein Kippgelenk unter der Platte und ein Gelenk am Ständerarm, acht Rollenauflager (vier unter dem Tisch, vier unter dem Ständer) und sieben Massenkörper."),
}
for key, (en, de) in NEW.items():
    path = f"{T}/volumes/{key}.json"
    if (time.time() - os.path.getmtime(path)) / 60 < 20 and "--force-age" not in sys.argv:
        print("SKIP", key); continue
    raw = open(path).read()
    src = json.loads(raw)
    decs = src["volumes"]["100"]["decisions"]
    for i, d in enumerate(decs):
        if d["en"] == "Thermal and structure follow the 1:100 body.":
            decs[i] = {"en": en, "de": de}
    out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
    assert open(path).read() == raw
    open(path, "w").write(out)
    print("WROTE", key)
