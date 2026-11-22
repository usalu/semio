"""🦴 Adds the acromion of 1:200 (the shoulder blade reaches the clavicle) and the decision sentences of this rework to `humans/parent-with-stroller`; run once after the publish script.

Clavicle-local points solved by hand from the rest frame of `clavicle-l` (origin [0.066, 0.490, 0.025], u [-0.3804, 0.0692, 0.9222], v [0, 0.9972, -0.0748], w [-0.9248, -0.0285, -0.3794]).
"""
import json

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/parent-with-stroller/🔣️.json"
SOURCE = T + "/volumes/humans/parent-with-stroller.json"
scope = {"json": json}
text = open(T + "/rework-05-parent-with-stroller-publish.py", encoding="utf-8").read()
exec(text[text.index("def scalar") : text.index("document = json.load")], scope)
DECISIONS = {
    "50": ("Shoulder and pelvic girdle hang on their own bones: the clavicle runs from the sternoclavicular joint to the acromion, the shoulder blade rides on it, the hip bone runs from the sacroiliac joint to the hip joint; the shoulder joint sits 62 mm lower and 14.5 mm further in than before, so the jacket shoulders slope from the neck and the elbows bend at the push bar.", "Schulter- und Beckengürtel hängen an eigenen Knochen: das Schlüsselbein läuft vom Sternoklavikulargelenk zum Akromion, das Schulterblatt sitzt darauf, das Hüftbein läuft vom Iliosakralgelenk zum Hüftgelenk; das Schultergelenk liegt 62 mm tiefer und 14,5 mm weiter innen als zuvor, daher fallen die Jackenschultern vom Hals ab und die Ellbogen sind am Schiebebügel gebeugt."),
    "100": ("Fork prongs and spokes are drawn at the 15 mm feature, finger and toddler long bones keep their true shaft as slender members; the girdle is the one of 1:50 with the clavicle as one bar.", "Gabelzinken und Speichen sind in der Merkmalsgröße von 15 mm gezeichnet, Finger- und Kleinkind-Röhrenknochen behalten als schlanke Stäbe ihren wahren Schaft; der Gürtel entspricht 1:50 mit dem Schlüsselbein als einem Stab."),
    "200": ("The hand is one fist around the bar, its skeleton one plate, the caster fork one strut, the front wheel one disc and the rear wheel a tyre with a web; frame tubes are 30 mm.", "Die Hand ist eine Faust um den Bügel, ihr Skelett eine Platte, die Lenkrollengabel eine Strebe, das Vorderrad eine Scheibe und das Hinterrad ein Reifen mit Radscheibe; die Rahmenrohre sind 30 mm stark."),
    "500": ("Every wheel is one disc, the hand one fist, the frame tubes 39 mm and the toddler's limbs 75 mm; clavicle and thigh bone keep their true section as slender members.", "Jedes Rad ist eine Scheibe, die Hand eine Faust, die Rahmenrohre 39 mm und die Gliedmaßen des Kleinkinds 75 mm stark; Schlüsselbein und Oberschenkelknochen behalten als schlanke Stäbe ihren wahren Querschnitt."),
}
document = json.load(open(CATALOG, encoding="utf-8"))
model = document["asset"]["volumes"]
structure = model["200"]["structure"]
if not any(s["id"] == "acromion-l" for s in structure):
    for suffix, sign in (("l", 1), ("r", -1)):
        at = [s["id"] for s in structure].index("scapula-" + suffix)
        structure.insert(at + 1, {**structure[at], "id": "acromion-" + suffix, "p0": [0.1605, -0.029, 0.0269 * sign], "p1": [0.1759, 0, 0.0033 * sign]})
    for scale, (en, de) in DECISIONS.items():
        sample = model[scale]["decisions"][0]
        print(scale, type(sample).__name__, json.dumps(sample, ensure_ascii=False)[:160])
        if isinstance(sample, dict) and set(sample) == {"en", "de"}:
            model[scale]["decisions"].append({"en": en, "de": de})
    open(CATALOG, "w", encoding="utf-8").write(scope["dump"](document, 0) + "\n")
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + scope["dump"](model, 0) + "}\n")
