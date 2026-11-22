"""🎠 Rework 15 playground swing: the seat swings with its chains. A flat seat hitched to one chain per side is a rigid pendulum with them: in the swing the apparent gravity runs along the chain, so the seat stays square to the chains and is never counter-rotated to the horizon.

Usage: python3 rework-15-playground-swing-seat.py [--apply]
"""
import sys

from rework_15_kit import Kit

kit = Kit("objects/playground-swing")
TEXTS = {
    "pendulum-forward": ("Maximum forward swing; the seat stays square to the chains (30 degrees nose up), and the child pushes forward with 80 N.", None),
    "pendulum-back": ("Maximum backward swing with the seat square to the chains (30 degrees nose down).", "Maximaler Rückschwung, der Sitz bleibt rechtwinklig zu den Ketten (30 Grad nach vorn geneigt)."),
}
for pose in kit.asset["poses"]:
    if "seat" in pose["bones"]:
        pose["bones"]["seat"]["angleDeg"] = 0
    if pose["id"] in TEXTS:
        en, de = TEXTS[pose["id"]]
        pose["description"]["en"] = en
        if de:
            pose["description"]["de"] = de
for clip in kit.asset["clips"]:
    for key in clip["keys"]:
        if "seat" in key.get("bones", {}):
            key["bones"]["seat"]["angleDeg"] = 0
    clip["description"]["en"] = clip["description"]["en"].replace("the seat is counter-rotated to stay horizontal", "the seat stays square to the chains like the rigid pendulum it is")
    clip["description"]["de"] = clip["description"]["de"].replace("der Sitz wird gegengedreht und bleibt waagerecht", "der Sitz bleibt rechtwinklig zu den Ketten, wie das starre Pendel, das er ist")
notes = kit.asset["provenance"]["notes"]
for lang, start, end, new in (
    ("en", "The seat is a child of the left chain with a hitch joint", "The seat stays horizontal;",
     "The seat is a child of the left chain with a hitch joint that stays at 0 in the swing: both chains hang side by side from one bearing axis, so chains and seat turn about that axis as one rigid pendulum and the seat stays square to the chains (there is no fore-and-aft pair of chains that would keep it level); the right chain is kinematically coupled by the same angle. Amplitude 30 degrees (authored). Occupant: child of 25 kg (authored estimate). The hitch joint remains free for a rider who tips the seat;"),
    ("de", "Der Sitz ist Kind der linken Kette mit einem Hitch-Gelenk", "Der Sitz bleibt waagerecht;",
     "Der Sitz ist Kind der linken Kette mit einem Hitch-Gelenk, das im Schwung auf 0 bleibt: beide Ketten hängen nebeneinander an einer Lagerachse, Ketten und Sitz drehen also als ein starres Pendel um diese Achse und der Sitz bleibt rechtwinklig zu den Ketten (es gibt kein vorderes und hinteres Kettenpaar, das ihn waagerecht hielte); die rechte Kette ist kinematisch über denselben Winkel gekoppelt. Amplitude 30 Grad (Festlegung). Insasse: Kind mit 25 kg (Schätzung). Das Hitch-Gelenk bleibt frei für ein Kind, das den Sitz kippt;"),
):
    if start in notes[lang]:
        a, b = notes[lang].index(start), notes[lang].index(end) + len(end)
        notes[lang] = notes[lang][:a] + new + notes[lang][b:]
kit.finish(sys.argv)
