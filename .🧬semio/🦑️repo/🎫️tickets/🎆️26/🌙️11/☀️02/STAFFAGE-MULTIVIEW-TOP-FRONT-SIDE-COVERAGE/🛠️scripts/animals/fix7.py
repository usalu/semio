import json

C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"


def T(en, de):
    return {"en": en, "de": de}


def edit(cat, slug, scale, reason, feature, size):
    p = C + cat + "/" + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8-sig"))
    r = d["asset"]["representations"][scale]
    assert r["status"] == "unavailable"
    r["reason"] = reason
    r["limitingFeature"] = feature
    r["limitingSizeM"] = size
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


edit(
    "🐕️animals", "pigeon", "500",
    T(
        "A pigeon is 0.335 m x 0.254 m = 0.67 x 0.51 mm on paper at 1:500, under the 3 mm an asset needs to be drawn. The body (0.14 m deep = 0.28 mm) is the only part above the 0.2 mm feature width of the office laser profile; the head (0.05 m = 0.1 mm) and the legs (0.014 m = 0.03 mm) are below it and also below the 0.12 mm feature width of the plot profile, so the bird would print as an oval speck without head, wing or tail.",
        "Eine Taube misst 0,335 m x 0,254 m = 0,67 x 0,51 mm auf dem Papier bei 1:500, unter den 3 mm, die ein Objekt zum Zeichnen braucht. Der Rumpf (0,14 m tief = 0,28 mm) ist der einzige Teil über der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils; Kopf (0,05 m = 0,1 mm) und Beine (0,014 m = 0,03 mm) liegen darunter und auch unter der Merkmalsbreite von 0,12 mm des Plot-Profils, die Taube druckte also als ovaler Fleck ohne Kopf, Flügel und Schwanz.",
    ),
    T("Head and legs (head 0.05 m = 0.1 mm, legs 0.014 m = 0.03 mm)", "Kopf und Beine (Kopf 0,05 m = 0,1 mm, Beine 0,014 m = 0,03 mm)"),
    0.014,
)
for scale, mm, w, h, ratio in (("200", "0.45", "0.09 m", "0.09 m", "0.45 mm"), ("500", "0.18", "0.09 m", "0.09 m", "0.18 mm")):
    pass

edit(
    "🔌️appliances", "room-thermostat", "200",
    T(
        "At 1:200 the 90 mm thermostat is 0.45 x 0.45 mm on paper, under the 3 mm an asset needs to be drawn. The dial (42 mm = 0.21 mm) just reaches the 0.2 mm feature width of the office laser profile, but the index mark (10 mm = 0.05 mm), the pointer (4 mm = 0.02 mm) and the 0.15 mm gap between dial and collar are below it, so the face would print as two touching discs; use the 1:100 or 1:50 drawing at this scale.",
        "Bei 1:200 ist der 90 mm große Thermostat 0,45 x 0,45 mm auf dem Papier, unter den 3 mm, die ein Objekt zum Zeichnen braucht. Der Drehknopf (42 mm = 0,21 mm) erreicht die Merkmalsbreite von 0,2 mm des Büro-Laser-Profils gerade, aber Index (10 mm = 0,05 mm), Zeiger (4 mm = 0,02 mm) und die Lücke von 0,15 mm zwischen Knopf und Kragen liegen darunter, das Gesicht druckte also als zwei sich berührende Scheiben; bei diesem Maßstab dient die Zeichnung 1:100 oder 1:50.",
    ),
    T("Index mark and pointer (10 mm = 0.05 mm, 4 mm = 0.02 mm)", "Indexmarke und Zeiger (10 mm = 0,05 mm, 4 mm = 0,02 mm)"),
    0.004,
)
edit(
    "🔌️appliances", "room-thermostat", "500",
    T(
        "At 1:500 the 90 mm thermostat is 0.18 x 0.18 mm on paper, under the 3 mm an asset needs to be drawn. Its dial (42 mm = 0.084 mm) is below the feature width of every print profile (0.12 mm on plot, 0.2 mm on office laser), so only a speck would remain; use the 1:100 or 1:50 drawing.",
        "Bei 1:500 ist der 90 mm große Thermostat 0,18 x 0,18 mm auf dem Papier, unter den 3 mm, die ein Objekt zum Zeichnen braucht. Sein Drehknopf (42 mm = 0,084 mm) liegt unter der Merkmalsbreite jedes Druckprofils (0,12 mm Plot, 0,2 mm Büro-Laser), es bliebe nur ein Fleck; es dient die Zeichnung 1:100 oder 1:50.",
    ),
    T("Dial of 42 mm diameter (0.084 mm)", "Drehknopf von 42 mm Durchmesser (0,084 mm)"),
    0.042,
)
edit(
    "🐕️animals", "herring-gull", "500",
    T(
        "A standing herring gull is 0.66 m x 0.52 m = 1.3 x 1.0 mm on paper at 1:500, under the 3 mm an asset needs to be drawn. The neck and head (0.07 m = 0.14 mm) are above the 0.12 mm feature width of the plot profile but below the 0.2 mm of the office laser and the 0.4 mm of the draft profile, and the legs (0.012 m = 0.024 mm) are below every profile; the spread wings of the gliding pose are foreshortened in elevation to the chord (0.18 m = 0.36 mm). On office laser or draft prints only a body speck would remain.",
        "Eine stehende Silbermöwe misst 0,66 m x 0,52 m = 1,3 x 1,0 mm auf dem Papier bei 1:500, unter den 3 mm, die ein Objekt zum Zeichnen braucht. Hals und Kopf (0,07 m = 0,14 mm) liegen über der Merkmalsbreite von 0,12 mm des Plot-Profils, aber unter den 0,2 mm des Büro-Lasers und den 0,4 mm des Entwurfsprofils, und die Beine (0,012 m = 0,024 mm) liegen unter jedem Profil; die gespreizten Flügel der Gleitpose sind im Aufriss auf die Flügeltiefe (0,18 m = 0,36 mm) verkürzt. Auf Büro-Laser- oder Entwurfsdrucken bliebe nur ein Körperfleck.",
    ),
    T("Legs of the gull (0.012 m = 0.024 mm, below every profile)", "Beine der Möwe (0,012 m = 0,024 mm, unter jedem Profil)"),
    0.012,
)
edit(
    "🐕️animals", "hedgehog", "200",
    T(
        "A foraging hedgehog is 0.285 m x 0.13 m = 1.4 x 0.65 mm on paper at 1:200, under the 3 mm an asset needs to be drawn; the rolled ball is 0.14 m = 0.7 mm. The snout (0.03 m = 0.15 mm) is above the 0.12 mm feature width of the plot profile but below the 0.2 mm of the office laser, the paws and limbs (0.02 m = 0.1 mm) are below the feature width of every profile, and on office laser the dome, the two skirts and the head leave no room for the 0.15 mm ink gap between neighbouring outlines (0.1 mm on plot) in the foraging, alert and rolled poses.",
        "Ein Nahrung suchender Igel misst 0,285 m x 0,13 m = 1,4 x 0,65 mm auf dem Papier bei 1:200, unter den 3 mm, die ein Objekt zum Zeichnen braucht; die eingerollte Kugel misst 0,14 m = 0,7 mm. Die Schnauze (0,03 m = 0,15 mm) liegt über der Merkmalsbreite von 0,12 mm des Plot-Profils, aber unter den 0,2 mm des Büro-Lasers, Pfoten und Beine (0,02 m = 0,1 mm) liegen unter der Merkmalsbreite jedes Profils, und auf Büro-Laser lassen Kuppel, beide Säume und Kopf in den Posen Suchen, Wachsam und Eingerollt keinen Platz für die Tintenlücke von 0,15 mm zwischen benachbarten Konturen (0,1 mm beim Plot).",
    ),
    T("Paws and limbs (0.02 m = 0.1 mm, below every profile)", "Pfoten und Beine (0,02 m = 0,1 mm, unter jedem Profil)"),
    0.02,
)
edit(
    "🐕️animals", "hedgehog", "500",
    T(
        "A foraging hedgehog is 0.28 m x 0.15 m = 0.56 x 0.3 mm on paper at 1:500, far under the 3 mm an asset needs to be drawn; the rolled ball is 0.14 m = 0.28 mm. The snout (0.03 m = 0.06 mm) and the limbs (0.02 m = 0.04 mm) are below the feature width of every print profile (0.12 mm on plot, 0.2 mm on office laser) and the spine edge cannot be resolved, so only a speck would remain; use the 1:100 drawing.",
        "Ein Nahrung suchender Igel misst 0,28 m x 0,15 m = 0,56 x 0,3 mm auf dem Papier bei 1:500, weit unter den 3 mm, die ein Objekt zum Zeichnen braucht; die eingerollte Kugel misst 0,14 m = 0,28 mm. Schnauze (0,03 m = 0,06 mm) und Beine (0,02 m = 0,04 mm) liegen unter der Merkmalsbreite jedes Druckprofils (0,12 mm Plot, 0,2 mm Büro-Laser) und die Stachelkante ist nicht auflösbar, es bliebe nur ein Fleck; es dient die Zeichnung 1:100.",
    ),
    T("Snout and limbs (0.02-0.03 m = 0.04-0.06 mm, below every profile)", "Schnauze und Beine (0,02-0,03 m = 0,04-0,06 mm, unter jedem Profil)"),
    0.02,
)
edit(
    "🐕️animals", "red-squirrel", "200",
    T(
        "A sitting red squirrel is 0.21 m x 0.275 m = 1.05 x 1.4 mm on paper at 1:200, under the 3 mm an asset needs to be drawn. Its limbs (0.016-0.024 m = 0.08-0.12 mm), ear tuft, eye and nose are at or below the 0.12 mm feature width of the plot profile and below the 0.2 mm of the office laser, and the filled masses of tail, body and head that remain are too close together for the 0.15 mm ink gap of the office laser in some pose (running, climbing). Without limbs the animal would not show how it sits, runs or climbs.",
        "Ein sitzendes Eichhörnchen misst 0,21 m x 0,275 m = 1,05 x 1,4 mm auf dem Papier bei 1:200, unter den 3 mm, die ein Objekt zum Zeichnen braucht. Seine Glieder (0,016-0,024 m = 0,08-0,12 mm), Ohrpinsel, Auge und Nase liegen auf oder unter der Merkmalsbreite von 0,12 mm des Plot-Profils und unter den 0,2 mm des Büro-Lasers, und die verbleibenden gefüllten Massen von Schwanz, Körper und Kopf liegen in manchen Posen (Laufen, Klettern) zu dicht für die Tintenlücke von 0,15 mm des Büro-Lasers. Ohne Glieder zeigte das Tier nicht, wie es sitzt, läuft oder klettert.",
    ),
    T("Limbs and head details (0.016-0.024 m = 0.08-0.12 mm)", "Glieder und Kopfdetails (0,016-0,024 m = 0,08-0,12 mm)"),
    0.02,
)
edit(
    "🐕️animals", "red-squirrel", "500",
    T(
        "A sitting red squirrel is 0.21 m x 0.275 m = 0.42 x 0.55 mm on paper at 1:500, far under the 3 mm an asset needs to be drawn; the whole animal covers about a quarter of a square millimetre. The body depth (0.09 m = 0.18 mm) is above the 0.12 mm feature width of the plot profile but below the 0.2 mm of the office laser, while the head (0.05 m = 0.1 mm) and the limbs (0.016 m = 0.03 mm) are below the feature width of every profile, so nothing identifiable can print; use the 1:100 drawing.",
        "Ein sitzendes Eichhörnchen misst 0,21 m x 0,275 m = 0,42 x 0,55 mm auf dem Papier bei 1:500, weit unter den 3 mm, die ein Objekt zum Zeichnen braucht; das ganze Tier bedeckt etwa einen Viertel Quadratmillimeter. Die Körpertiefe (0,09 m = 0,18 mm) liegt über der Merkmalsbreite von 0,12 mm des Plot-Profils, aber unter den 0,2 mm des Büro-Lasers, während Kopf (0,05 m = 0,1 mm) und Glieder (0,016 m = 0,03 mm) unter der Merkmalsbreite jedes Profils liegen, es lässt sich also nichts Erkennbares drucken; es dient die Zeichnung 1:100.",
    ),
    T("Head and limbs (0.05 m = 0.1 mm, 0.016 m = 0.03 mm, below every profile)", "Kopf und Glieder (0,05 m = 0,1 mm, 0,016 m = 0,03 mm, unter jedem Profil)"),
    0.016,
)
print("ok")
