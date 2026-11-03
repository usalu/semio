def std_dec(c):
    d = {}
    d["50"] = {
        "silhouette": ("Front view: %s with %s; plan view: %s." % (c["front"][0], c["trunk"][0], c["plan"][0]), "Vorderansicht: %s mit %s; Draufsicht: %s." % (c["front"][1], c["trunk"][1], c["plan"][1])),
        "retained": ("%s in front, %s in plan; every leaf mass drawn again with its own outline and an inner arc on the main masses, thin limb lines under or over them." % (c["trunk"][0].capitalize(), c["base"][0]), "%s vorn, %s in der Draufsicht; jede Laubmasse mit eigener Kontur und bei den Hauptmassen mit einem Innenbogen neu gezeichnet, dazu dünne Astlinien unter beziehungsweise über ihnen." % (c["trunk"][1].capitalize(), c["base"][1])),
        "merged": ("The smaller diagonal masses overlap into the outline of the main masses.", "Die kleineren diagonalen Massen gehen in die Kontur der Hauptmassen über."),
        "omitted": ("Single leaves and bark texture are not repeated in front and plan.", "Einzelblätter und Rindentextur werden in Vorder- und Draufsicht nicht wiederholt."),
    }
    d["100"] = {
        "silhouette": ("Front view with %s, plan view with %s." % (c["front100"][0], c["plan100"][0]), "Vorderansicht mit %s, Draufsicht mit %s." % (c["front100"][1], c["plan100"][1])),
        "retained": ("All main and lateral leaf masses, %s in front and %s in plan, short limb lines." % (c["trunk"][0], c["base"][0]), "Alle Haupt- und Seitenlaubmassen, %s vorn und %s in der Draufsicht, kurze Astlinien." % (c["trunk"][1], c["base"][1])),
        "merged": ("The diagonal masses are part of their neighbours.", "Die diagonalen Massen gehören zu ihren Nachbarn."),
        "omitted": ("Inner arcs of the small masses, bark lines and single leaves.", "Innenbögen der kleinen Massen, Rindenlinien und Einzelblätter."),
    }
    d["200"] = {
        "silhouette": ("Front and plan view as the main leaf masses only, %s." % c["main200"][0], "Vorder- und Draufsicht nur mit den Hauptlaubmassen, %s." % c["main200"][1]),
        "retained": ("%s and the main masses in both views." % c["trunk"][0].capitalize(), "%s und die Hauptmassen in beiden Ansichten." % c["trunk"][1].capitalize()),
        "merged": ("Limbs and small masses vanish inside the main masses.", "Äste und kleine Massen verschwinden in den Hauptmassen."),
        "omitted": ("Limb lines and every detail line.", "Astlinien und jede Detaillinie."),
    }
    d["500"] = {
        "silhouette": ("Front view as one crown silhouette on %s; plan view as one lobed crown disc." % c["trunk500"][0], "Vorderansicht als eine Kronensilhouette auf %s; Draufsicht als eine gelappte Kronenscheibe." % c["trunk500"][1]),
        "retained": ("The outline of the crown from the side and from above.", "Die Kontur der Krone von der Seite und von oben."),
        "merged": ("All masses and limbs are one shape.", "Alle Massen und Äste sind eine Form."),
        "omitted": ("Every inner structure.", "Jede innere Struktur."),
    }
    return d
