"""📝 runner-with-blade-prosthesis: the structure decisions of every scale say what the shoulder and pelvic girdle and the coarse skeleton are now.

Usage: python3 rework-runner-with-blade-prosthesis-texts.py     appends once to the structure decision of each scale in the volumes source
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "runner-with-blade-prosthesis.json")
GIRDLE = {
    "en": " The shoulder girdle hangs on its own rig bones: each collarbone runs from the sternoclavicular joint beside the jugular notch 0.149 m out, 4 degrees up and 27 degrees back to the acromioclavicular joint and carries shoulder blade, glenoid and the upper arm; each hip bone runs from the sacroiliac joint to the hip joint and carries the thigh, so the limbs leave the trunk at the true breadths (hip joints 0.18 m, shoulder joints 0.36 m apart).",
    "de": " Der Schultergürtel hängt an eigenen Rig-Knochen: Jedes Schlüsselbein läuft vom Sternoklavikulargelenk neben der Drosselgrube 0,149 m nach aussen, 4 Grad aufwärts und 27 Grad nach hinten zum Akromioklavikulargelenk und trägt Schulterblatt, Gelenkpfanne und Oberarm; jedes Hüftbein läuft vom Iliosakralgelenk zum Hüftgelenk und trägt den Oberschenkel, sodass die Gliedmassen in den wahren Breiten vom Rumpf abgehen (Hüftgelenke 0,18 m, Schultergelenke 0,36 m auseinander).",
}
FINE = {
    "en": " The collarbone is the S-curved bone with a thick sternal and a flat acromial end, and the manubrium ends at the jugular notch 58 mm below the seventh cervical vertebra.",
    "de": " Das Schlüsselbein ist der S-förmig geschwungene Knochen mit dickem Brustbein- und flachem Schulterende, und das Manubrium endet an der Drosselgrube 58 mm unter dem siebten Halswirbel.",
}
COARSE = {
    "en": " Femur, tibia, humerus, radius and ulna keep their true shaft as slender members between the joint knobs, the femur leaves the hip joint over its neck and runs obliquely to the knee, and the pelvis is a basin that narrows from the iliac crests to the pubis.",
    "de": " Femur, Tibia, Humerus, Radius und Ulna behalten als schlanke Stäbe zwischen den Gelenkknoten ihren wahren Schaft, der Femur verlässt das Hüftgelenk über den Schenkelhals und läuft schräg zum Knie, und das Becken ist eine Schale, die sich von den Darmbeinkämmen zum Schambein verjüngt.",
}
SMALLEST = {
    "en": " The spine is one true rod from the sacrum to the skull base and also the member of the neck; collarbone, shoulder blade and glenoid stay one 75 mm yoke per side because the 0.149 m collarbone is too short to print as a rod.",
    "de": " Die Wirbelsäule ist ein wahrer Stab vom Kreuzbein bis zur Schädelbasis und zugleich das Tragglied des Halses; Schlüsselbein, Schulterblatt und Gelenkpfanne bleiben je Seite ein 75 mm starkes Joch, weil das 0,149 m lange Schlüsselbein als Stab zu kurz zum Drucken ist.",
}
ADDED = {"50": [GIRDLE, FINE], "100": [GIRDLE, FINE], "200": [GIRDLE, COARSE], "500": [GIRDLE, COARSE, SMALLEST]}

source = json.load(open(SOURCE, encoding="utf8"))
for scale, parts in ADDED.items():
    decision = source["volumes"][scale]["decisions"][2]
    print(scale, decision["en"][:50])
    for language in ("en", "de"):
        for part in parts:
            if part[language].strip() not in decision[language]:
                decision[language] += part[language]
json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(SOURCE, "a", encoding="utf8").write("\n")
