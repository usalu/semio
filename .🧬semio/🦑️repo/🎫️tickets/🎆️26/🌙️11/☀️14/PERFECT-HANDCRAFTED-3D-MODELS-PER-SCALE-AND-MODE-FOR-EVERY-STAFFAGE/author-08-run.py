import sys
from author_08_build import *

D = lambda en, de: {"en": en, "de": de}
SPECS = {
 "parent-with-stroller": dict(prop_bones=["stroller", "stroller-hood", "wheel-rear", "wheel-front", "child"],
   desc=D({"50": "the stroller keeps its tube frame, basket, seat, hood ribs, wheels with rims and the child as hand-placed solids.", "100": "stroller tubes, seat, basket, wheels and child at the sizes the 1:100 drawing resolves; hanger straps and rims are gone.", "200": "frame tubes, seat, hood and wheels as single masses, the child as head and body.", "500": "stroller as a few tubes, a seat mass, two tyres and the child as one mass."},
          {"50": "der Kinderwagen behält Rohrrahmen, Korb, Sitz, Verdeckbügel, Räder mit Felgen und das Kind als von Hand gesetzte Körper.", "100": "Rohre, Sitz, Korb, Räder und Kind in der vom Massstab 1:100 aufgelösten Grösse; Haltegurte und Felgen entfallen.", "200": "Rahmenrohre, Sitz, Verdeck und Räder als einzelne Massen, das Kind aus Kopf und Körper.", "500": "der Kinderwagen aus wenigen Rohren, einer Sitzmasse, zwei Reifen und dem Kind als eine Masse."})),
}

def spec(props, e50, e100, e200, e500, d50, d100, d200, d500, **kw):
    return dict(prop_bones=props, desc=D({"50": e50, "100": e100, "200": e200, "500": e500}, {"50": d50, "100": d100, "200": d200, "500": d500}), **kw)

SPECS["parent-carrying-toddler"] = spec(["toddler-torso", "toddler-head", "toddler-arm-n", "toddler-arm-f", "toddler-thigh-n", "toddler-shank-n", "toddler-thigh-f", "toddler-shank-f"],
 "the toddler on the arm is a second small figure of lofted limbs, suit, hat and mittens; beanie, scarf, gloves and facial details are hand-placed.",
 "the toddler keeps head, hat, body, arms and legs; face and glove details are gone.",
 "parent and toddler are merged limb lofts; the toddler is body, head and four limb masses.",
 "one or two sections per bone for both figures.",
 "das Kleinkind auf dem Arm ist eine zweite kleine Figur aus Loft-Gliedern, Anzug, Mütze und Fäustlingen; Mütze, Schal, Handschuhe und Gesichtsdetails sind von Hand gesetzt.",
 "das Kleinkind behält Kopf, Mütze, Körper, Arme und Beine; Gesichts- und Handschuhdetails entfallen.",
 "Elternteil und Kleinkind sind zusammengefasste Glieder-Lofts; das Kleinkind besteht aus Körper, Kopf und vier Gliedmassen.",
 "ein bis zwei Schnitte je Knochen für beide Figuren.")
SPECS["person-cooking"] = spec(["counter", "counter-lift", "pot", "spoon"],
 "the counter has plinth, body, worktop, hob plate with burner rings, the pot is a revolve with handles and the spoon a handle with a bowl; hoodie details are hand-placed.",
 "counter as plinth, body, worktop and hob; pot and spoon as simple revolve and capsule.",
 "counter as block and worktop, pot as revolve, spoon as one capsule.",
 "counter as one block with the hob, pot and spoon as single masses.",
 "die Küchenzeile hat Sockel, Korpus, Arbeitsplatte, Kochfeld mit Brennerringen, der Topf ist ein Rotationskörper mit Griffen, der Löffel ein Stiel mit Laffe; Hoodie-Details sind von Hand gesetzt.",
 "Küchenzeile als Sockel, Korpus, Arbeitsplatte und Kochfeld; Topf und Löffel als einfacher Rotationskörper und Kapsel.",
 "Küchenzeile als Block mit Arbeitsplatte, Topf als Rotationskörper, Löffel als eine Kapsel.",
 "Küchenzeile als ein Block mit Kochfeld, Topf und Löffel als einzelne Massen.")
SPECS["person-on-crutches"] = spec(["crutch-l", "crutch-r"],
 "the forearm crutches are tubes with cuffs, brackets, grips and rubber tips; boots, straps and hood are hand-placed.",
 "crutch tubes, cuffs and tips; straps are gone.",
 "crutches as slender tubes with cuff and tip.",
 "crutches as one slender tube each.",
 "die Unterarmgehstützen sind Rohre mit Manschetten, Haltern, Griffen und Gummispitzen; Stiefel, Riemen und Kapuze sind von Hand gesetzt.",
 "Stützenrohre, Manschetten und Spitzen; Riemen entfallen.",
 "Stützen als schlanke Rohre mit Manschette und Spitze.",
 "Stützen als je ein schlankes Rohr.")
SPECS["person-with-rollator"] = spec(["frame", "wheel-rear", "wheel-front", "aid-lift", "aid-slide"],
 "the rollator keeps its tube frame, seat, backrest, basket, brake levers, caster forks and wheels with rims; glasses, beret and buttons are hand-placed.",
 "rollator tubes, seat, backrest, basket, bag and wheels; cables and braces are gone.",
 "rollator as main tubes, seat, basket and wheels.",
 "rollator as a few tubes, a seat mass and four tyres.",
 "der Rollator behält Rohrrahmen, Sitz, Lehne, Korb, Bremshebel, Lenkgabeln und Räder mit Felgen; Brille, Baskenmütze und Knöpfe sind von Hand gesetzt.",
 "Rollatorrohre, Sitz, Lehne, Korb, Tasche und Räder; Seilzüge und Streben entfallen.",
 "Rollator als Hauptrohre, Sitz, Korb und Räder.",
 "Rollator aus wenigen Rohren, einer Sitzmasse und vier Reifen.")
SPECS["person-with-umbrella"] = spec(["umbrella", "rib-front", "rib-back"],
 "the umbrella is a shaft with handle hook, hub, ferrule and two half-domes revolved about the shaft; coat hem, belt and facial details are hand-placed.",
 "umbrella shaft, handle, hub and canopy domes.",
 "umbrella as slender shaft and canopy dome.",
 "umbrella as a shaft and one canopy mass.",
 "der Schirm besteht aus Schaft mit Griffhaken, Nabe, Zwinge und zwei um den Schaft gedrehten Halbkuppeln; Mantelsaum, Gürtel und Gesichtsdetails sind von Hand gesetzt.",
 "Schirmschaft, Griff, Nabe und Schirmkuppeln.",
 "Schirm als schlanker Schaft mit Schirmkuppel.",
 "Schirm als Schaft und eine Schirmmasse.")

def go(slug, scales=SCALES):
    sp = SPECS[slug]
    cfg = auto_cfg(slug, sp["prop_bones"], desc=sp["desc"], person_over=sp.get("person_over"), extra_keep=sp.get("extra_keep"), exclude_ids=sp.get("exclude", ()))
    run(slug, cfg, scales=scales)
if __name__ == "__main__":
    go(sys.argv[1], sys.argv[2:] or SCALES)
