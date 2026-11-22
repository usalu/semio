"""🪑 Wheel loader, second round pass 1: parts the coarser scales lacked although they are several features large (fuel tank at 1:100 and 1:200; seat, pedestal and steering column at 1:100), and the decisions sentences of this pass. Hand-chosen numbers; safe to run again."""
import json
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
SRC = T + "/volumes/machinery/wheel-loader.json"
def put(lst, solid, after):
    for k, x in enumerate(lst):
        if x["id"] == solid["id"]:
            lst[k] = solid
            return
    at = next((k + 1 for k, x in enumerate(lst) if x["id"] == after), len(lst))
    lst.insert(at, solid)
def base(i, material, role, tone):
    return {"id": i, "bone": "rear-frame", "material": material, "role": role, "tone": tone}
def tank(radius):
    solid = {**base("fuel-tank", "paint-steel", "body", 0.45), "kind": "box", "size": [0.64, 0.42, 1.32], "transform": {"translation": [-2.9, 1.2, 0.0]}}
    if radius: solid["cornerRadius"] = radius
    return solid
SEAT = [
    {**base("seat-pedestal", "paint-steel", "detail", 0.3), "kind": "box", "size": [0.3, 0.3, 0.3], "transform": {"translation": [-0.8, 2.11, 0.0]}},
    {**base("seat", "rubber", "detail", 0.2), "kind": "box", "size": [0.5, 0.17, 0.5], "cornerRadius": 0.05, "transform": {"translation": [-0.8, 2.335, 0.0]}},
    {**base("seat-back", "rubber", "detail", 0.2), "kind": "box", "size": [0.15, 0.66, 0.46], "cornerRadius": 0.05, "transform": {"translation": [-1.07, 2.72, 0.0]}},
    {**base("steering-column", "paint-steel", "detail", 0.2), "kind": "capsule", "p0": [-0.1, 1.97, 0.0], "p1": [-0.38, 2.62, 0.0], "r0": 0.03, "r1": 0.03},
]
NOTES = {
    "50": {"en": "Second round, pass 1: the axle housings are planetary axles of a 20 t loader with 300 mm arm tubes, hub flanges and a 720 mm differential bowl, so the ground clearance is 0.45 m (was 0.54 m). The rear axle sits in two saddle pads under the rails and the front axle on two pads between the tower plates instead of a plate running through the bowl; the frame floor ends before the front bowl and a separate nose skid closes the front. The driveline is modelled: transmission case on a cross bracket under the cab, a short rear propeller shaft, a middle shaft to the universal joint on the articulation axis and a front shaft, each with flanges, and the pinion noses of both axles; the hitch bulkheads have the shaft opening, the lower hitch pin ends under the shaft, and the lift cylinders stand on two stub pins of the tower plates instead of a tube across the shaft.", "de": "Zweite Runde, Durchgang 1: Die Achsgehäuse sind Planetenachsen eines 20-t-Laders mit 300-mm-Achsrohren, Nabenflanschen und einer Differentialglocke von 720 mm, die Bodenfreiheit beträgt damit 0,45 m (vorher 0,54 m). Die Hinterachse liegt in zwei Sattelstücken unter den Längsträgern und die Vorderachse auf zwei Auflagen zwischen den Turmblechen statt auf einem Blech quer durch die Glocke; der Rahmenboden endet vor der vorderen Glocke, eine eigene Bugkufe schließt den Bug. Der Antriebsstrang ist modelliert: Getriebegehäuse an einer Querkonsole unter der Kabine, kurze hintere Gelenkwelle, mittlere Welle bis zum Kreuzgelenk auf der Knickachse und vordere Welle, jeweils mit Flanschen, dazu die Ritzelhälse beider Achsen; die Gelenkschotte haben die Wellenöffnung, der untere Gelenkbolzen endet unter der Welle, und die Hubzylinder stehen auf zwei Stummelbolzen der Turmbleche statt auf einem Rohr quer durch die Welle."},
    "100": {"en": "Second round, pass 1: axle housings with the 720 mm differential bowl and plain 300 mm arm tubes on saddle pads, the frame floor open at the front axle with a nose skid, the transmission case between the rails, three plain 90 mm propeller shafts with the pinion noses merged in, bulkheads with an octagonal shaft opening, stub pins for the lift cylinders, and the parts that were missing although they are many features large: the fuel tank as a rounded block, seat, backrest, pedestal and steering column as plain blocks.", "de": "Zweite Runde, Durchgang 1: Achsgehäuse mit der 720-mm-Differentialglocke und glatten 300-mm-Achsrohren auf Sattelstücken, der Rahmenboden an der Vorderachse offen mit Bugkufe, das Getriebegehäuse zwischen den Längsträgern, drei glatte 90-mm-Gelenkwellen mit eingeschmolzenen Ritzelhälsen, Schotte mit achteckiger Wellenöffnung, Stummelbolzen für die Hubzylinder und die Teile, die fehlten, obwohl sie viele Merkmalsgrößen messen: der Kraftstofftank als gerundeter Block, Sitz, Lehne, Sockel und Lenksäule als einfache Blöcke."},
    "200": {"en": "Second round, pass 1: the axles carry their differential bowl (720 mm against 300 mm arm tubes, 24 features), which sets the ground clearance of 0.45 m; saddle pads, the open floor at the front axle with a nose skid, the transmission block, one 100 mm shaft per frame meeting on the articulation axis, stub pins for the lift cylinders and the fuel tank as a plain block.", "de": "Zweite Runde, Durchgang 1: Die Achsen tragen ihre Differentialglocke (720 mm gegenüber 300-mm-Achsrohren, 24 Merkmalsgrößen), sie bestimmt die Bodenfreiheit von 0,45 m; Sattelstücke, der offene Boden an der Vorderachse mit Bugkufe, der Getriebeblock, je Rahmen eine 100-mm-Welle, die sich auf der Knickachse treffen, Stummelbolzen für die Hubzylinder und der Kraftstofftank als einfacher Block."},
    "500": {"en": "Second round, pass 1: both axles are 300 mm tubes with the 720 mm differential bowl (nearly ten features), the rear one in two saddle pads; the driveline stays out at this scale because the frame bodies hide it.", "de": "Zweite Runde, Durchgang 1: Beide Achsen sind 300-mm-Rohre mit der 720-mm-Differentialglocke (knapp zehn Merkmalsgrößen), die hintere in zwei Sattelstücken; der Antriebsstrang entfällt in diesem Maßstab, weil die Rahmenkörper ihn verdecken."},
}
src = json.load(open(SRC))
v = src["volumes"]
put(v["100"]["body"], tank(0.06), "axle-r")
put(v["200"]["body"], tank(0), "axle-r")
for sc in ("100", "200"):
    put(v[sc]["thermal"], {"id": "th-fuel-tank", "of": "fuel-tank", "region": "front-frame", "layer": "plate"}, "th-axle-r")
for solid in SEAT:
    put(v["100"]["body"], solid, "cab-rear-glass")
    if solid["id"] == "seat-pedestal":
        put(v["100"]["thermal"], {"id": "th-seat-pedestal", "of": "seat-pedestal", "region": "cab"}, "th-cab-rear-glass")
for sc, note in NOTES.items():
    d = v[sc]["decisions"]
    d[:] = [x for x in d if not x.get("en", "").startswith("Second round, pass 1")] + [note]
json.dump(src, open(SRC, "w"), ensure_ascii=False, indent=1)
