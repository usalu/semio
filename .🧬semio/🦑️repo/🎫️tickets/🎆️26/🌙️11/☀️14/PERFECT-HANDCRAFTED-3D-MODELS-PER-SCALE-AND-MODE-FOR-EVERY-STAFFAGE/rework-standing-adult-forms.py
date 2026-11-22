"""🍑 standing-adult, pass 1 second round: hand-chosen corrections of forms read in the sheets.

- 1:50, 1:100 body: the buttocks were two balls (0.058 deep, 0.144 wide) standing out of the seat on every side; they are now the lower gluteal round inside the seat (0.060 deep, 0.150 high, 0.132 wide; top and flanks buried, the seat loft carries the 0.11 m depth), so only the round under the seat and the gluteal fold at 0.81 m show.
- 1:500 body: the shank ended 14 mm above the shoe; it now runs into the shoe.
- 1:500 structure: six rib hoops of 10 mm read as a pretzel; the thorax is one rib-cage basket (apex at rib 1, widest at rib 8), like the pelvic girdle of that scale.
- 1:200 structure: the iliac wing was a leaf beside the hip bone; it is a blade from the acetabulum to the crest, its plane running from the anterior superior spine back and inwards to the posterior one.

Usage: python3 rework-standing-adult-forms.py [--apply]   (without --apply: candidate `🗑️generated/rework-standing-adult/forms/🔣️.json`)
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("pelvis_round", os.path.join(T, "rework-standing-adult-pelvis.py"))
pelvis = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pelvis)
rig = pelvis.rig
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-adult", "forms", "🔣️.json")
ALONG = [0.5, 0.5, 0.5, 0.5]
BONE = {"material": "bone", "role": "body", "tone": 0.85}


def find(solids, ident):
    return next(x for x in solids if x["id"] == ident)


def buttocks(volumes):
    for scale in ("50", "100"):
        for tag, s in (("l", 1), ("r", -1)):
            solid = find(volumes[scale]["body"], "buttock-" + tag)
            solid["radii"] = [0.060, 0.075, 0.066]
            solid["transform"]["translation"] = [-0.020, 0.046, 0.068 * s]


def shank_500(volumes):
    for tag in ("l", "r"):
        find(volumes["500"]["body"], "shank-" + tag)["sections"][-1]["at"][0] = 0.428


def rib_cage_500(volumes):
    solids = volumes["500"]["structure"]
    at = next(i for i, x in enumerate(solids) if x["id"].startswith("rib-"))
    solids[:] = [x for x in solids if not x["id"].startswith("rib-")]
    rows = [(-0.030, -0.010, 0.085, 0.126), (0.040, -0.018, 0.100, 0.140), (0.120, -0.020, 0.103, 0.134), (0.200, -0.025, 0.080, 0.105), (0.262, -0.025, 0.052, 0.064)]
    sections = [{"at": [u, v, 0.0], "rx": rx, "ry": ry, "exponent": 2.2, "rotation": ALONG} for u, v, rx, ry in rows]
    solids.insert(at, {"id": "rib-cage", "kind": "loft", "bone": "torso", **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": "torso", "part": "member"})


def wings_200(volumes, frames):
    solids = volumes["200"]["structure"]
    rows = [((-0.004, 0.915, 0.080), 0.030, 0.015), ((-0.008, 0.955, 0.094), 0.046, 0.015), ((-0.012, 0.995, 0.104), 0.062, 0.015), ((-0.014, 1.028, 0.108), 0.056, 0.015), ((-0.014, 1.040, 0.109), 0.034, 0.015)]
    for tag, s in (("l", 1), ("r", -1)):
        frame = pelvis.Frame(frames, "ilium-" + tag)
        sections = []
        for i, (at, rx, ry) in enumerate(rows):
            a, b = rows[max(i - 1, 0)][0], rows[min(i + 1, len(rows) - 1)][0]
            sections.append(frame.section(pelvis.side(at, s), pelvis.side([0.88, 0, 0.47], s), pelvis.side([b[k] - a[k] for k in range(3)], s), rx, ry, 2.6))
        index = next(i for i, x in enumerate(solids) if x["id"] == "iliac-wing-" + tag)
        solids[index] = frame.loft("iliac-wing-" + tag, sections)


DECISIONS = {
    ("200", 2): {
        "en": "Structure at 1:200: the skeleton as its lines of thrust: skull, cervical, thoracic and lumbar column along their curves, sacrum, six rib hoops per side closing on the sternum, one girdle line per side on the clavicle bone (clavicle, acromion, scapula), each hip bone on its own pelvic-girdle bone as a bar from the sacroiliac joint over the acetabulum to the symphysis with the iliac wing as a blade from the acetabulum up to the crest, its plane running from the anterior superior spine back and inwards; humerus, forearm, hand, femur, tibia and foot; the long bones keep their true shaft, compact parts are 30 mm thick; masses of the segments above 2 kg.",
        "de": "Tragwerk bei 1:200: das Skelett als seine Drucklinien: Schädel, Hals-, Brust- und Lendensäule entlang ihrer Krümmungen, Kreuzbein, sechs Rippenreifen je Seite, die am Brustbein schließen, eine Gürtellinie je Seite am Schlüsselbeinknochen (Schlüsselbein, Schulterhöhe, Schulterblatt), jedes Hüftbein an seinem eigenen Beckengürtelknochen als Stab vom Kreuz-Darmbein-Gelenk über die Pfanne zur Symphyse mit der Darmbeinschaufel als Blatt von der Pfanne hinauf zum Kamm, dessen Ebene vom vorderen oberen Darmbeinstachel nach hinten innen läuft; Oberarm, Unterarm, Hand, Oberschenkel, Schienbein und Fuß; die Röhrenknochen behalten ihren wahren Schaft, gedrungene Teile sind 30 mm dick; Massen der Segmente über 2 kg.",
    },
    ("500", 2): {
        "en": "Structure at 1:500, typified for a 75 mm feature: the skull as one block, the spine as one rod at its true section along the lumbar lordosis, thoracic kyphosis and cervical lordosis, the rib cage as one basket from the first rib to the costal margin (0.13 m wide at the apex, 0.28 m at the eighth rib) instead of single ribs, the pelvis with both hip bones as one bowl, one girdle line per side, humerus, forearm with hand, femur and tibia as slender bones at their true shaft, each foot as one bar on its heel and ball supports; masses of pelvis, lumbar segment, thorax and thighs.",
        "de": "Tragwerk bei 1:500, typisiert für ein Merkmal von 75 mm: der Schädel als ein Block, die Wirbelsäule als ein Stab in wahrem Querschnitt entlang Lendenlordose, Brustkyphose und Halslordose, der Brustkorb als ein Korb von der ersten Rippe bis zum Rippenbogen (0,13 m breit an der Spitze, 0,28 m an der achten Rippe) statt einzelner Rippen, das Becken mit beiden Hüftbeinen als eine Schale, eine Gürtellinie je Seite, Oberarm, Unterarm mit Hand, Oberschenkel und Schienbein als schlanke Knochen in wahrem Schaft, jeder Fuß als ein Stab auf seinen Auflagern an Ferse und Ballen; Massen von Becken, Lendensegment, Brustkorb und Oberschenkeln.",
    },
}
SWAPS = {
    ("50", 0): [("Jeans: seat with gluteal masses,", "Jeans: the seat carries the gluteal depth of 0.11 m behind the hip joint, the lower gluteal round closes it towards the fold at 0.81 m;"), ("Jeans: Gesäß mit Gesäßmuskeln,", "Jeans: Das Gesäß trägt die Gesäßtiefe von 0,11 m hinter dem Hüftgelenk, die untere Gesäßrundung schließt es zur Gesäßfalte bei 0,81 m;")],
    ("100", 0): [("jeans seat with gluteal masses,", "jeans seat with the lower gluteal round,"), ("Jeansgesäß mit Gesäßmuskeln,", "Jeansgesäß mit der unteren Gesäßrundung,")],
    ("50", 2): [("ride on the shoulder-girdle bone. Hip bones with iliac wing, ischium and pubic rami meeting at the symphysis;", "ride on the clavicle bone, which runs from the sternoclavicular to the acromioclavicular joint. Each hip bone rides on its pelvic-girdle bone from the sacroiliac joint to the hip joint: iliac wing with crest, acetabulum, ischium with tuberosity and pubic rami meeting at the symphysis;"), ("sitzen auf dem Schultergürtelknochen. Hüftbeine mit Darmbeinschaufel, Sitzbein und Schambeinästen, die sich an der Symphyse treffen;", "sitzen auf dem Schlüsselbeinknochen, der vom Brustbein-Schlüsselbein-Gelenk zum Schultereckgelenk läuft. Jedes Hüftbein sitzt auf seinem Beckengürtelknochen vom Kreuz-Darmbein-Gelenk zum Hüftgelenk: Darmbeinschaufel mit Kamm, Hüftpfanne, Sitzbein mit Sitzbeinhöcker und Schambeinäste, die sich an der Symphyse treffen;")],
    ("500", 0): [("shank with the calf, and one shoe.", "shank with the calf running into one shoe."), ("Unterschenkel mit der Wade und ein Schuh.", "Unterschenkel mit der Wade, der in einen Schuh läuft.")],
}


def decisions(volumes):
    for (scale, index), text in DECISIONS.items():
        volumes[scale]["decisions"][index] = text
    for (scale, index), swaps in SWAPS.items():
        entry = volumes[scale]["decisions"][index]
        for old, new in swaps:
            for language in ("en", "de"):
                if old in entry[language]:
                    entry[language] = entry[language].replace(old, new)


def main():
    text = open(rig.SOURCE, encoding="utf8").read()
    source = json.loads(text)
    document = json.load(open(rig.CATALOG, encoding="utf8"))
    frames = rig.frames(document["asset"]["rig"]["bones"])
    volumes = source["volumes"]
    buttocks(volumes)
    shank_500(volumes)
    rib_cage_500(volumes)
    wings_200(volumes, frames)
    decisions(volumes)
    for scale, name, ident in (("50", "thermal", "th-buttock-l"), ("500", "thermal", "th-shank-l"), ("500", "thermal", "th-shoe-l")):
        print(json.dumps(find(volumes[scale][name], ident))[:400])
    if "--apply" in sys.argv:
        if open(rig.SOURCE, encoding="utf8").read() != text:
            raise SystemExit("the source changed: run again")
        json.dump(source, open(rig.SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        open(rig.SOURCE, "a", encoding="utf8").write("\n")
        print("applied to", rig.SOURCE)
        return
    document["asset"]["volumes"] = volumes
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(CANDIDATE)


if __name__ == "__main__":
    main()
