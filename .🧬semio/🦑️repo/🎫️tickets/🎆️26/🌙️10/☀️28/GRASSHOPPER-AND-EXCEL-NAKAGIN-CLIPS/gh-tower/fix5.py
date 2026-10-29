import sys

p, body = sys.argv[1], sys.argv[2]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)


def clause(key, en, de):
    global s
    i = s.index(f'        ["{key}", ')
    j = s.index("],\n", i) + 3
    s = s[:i] + f'        ["{key}", "{en}",\n         "{de}"],\n' + s[j:]


clause("hook", "One capsule is easy. How do 140 of them become two towers?", "Eine Kapsel ist einfach. Wie werden 140 Kapseln zu zwei Türmen?")
clause("facades", "A second list holds the four facade angles. Rotating one plane by each angle gives one plane per facade.",
       "Eine zweite Liste enthält die vier Fassadenwinkel. Dreht man eine Ebene um jeden Winkel, entsteht eine Ebene pro Fassade.")
clause("types", "A type pattern picks the capsule type for every plane. List Item reads the pattern by index; only the list differs.",
       "Ein Typmuster wählt für jede Ebene den Kapseltyp. List Item liest das Muster per Index; nur die Liste ändert sich.")
clause("count", "Now the capsule storeys slider: the count is storeys times facades times items, and the drawing grows with it.",
       "Nun der Slider Kapselgeschosse: Anzahl gleich Geschosse mal Fassaden mal Items, und die Zeichnung wächst mit.")
clause("ontop", "It sits on top of the core at the height the capsule storeys slider gives, so it rises with the tower.",
       "Er sitzt auf dem Kern genau auf der Höhe, die der Slider Kapselgeschosse vorgibt, und steigt mit dem Turm.")
clause("podium", "The lower building is only an extruded massing: one footprint, one height.", "Das Sockelgebäude ist nur ein extrudierter Baukörper: Grundriss und Höhe.")
clause("towers", "The same definition builds tower A and tower B. Capsule storeys and facades differ; the formulas show both.",
       "Dieselbe Definition baut Turm A und Turm B. Kapselgeschosse und Fassaden unterscheiden sich; die Formeln zeigen beides.")
clause("swap", "Metabolism: capsules are replaceable. Swap one type, and every capsule of that type changes at once.",
       "Metabolismus: Kapseln sind austauschbar. Tauscht man einen Typ, ändert sich jede Kapsel dieses Typs sofort.")
i = s.index('        ["capital", ')
s = s[:i] + '''        ["mistake", "A typical mistake: without Cull Index, tower B gets 88 capsules, and the two west capsules on the top storey collide with the service side.",
         "Typischer Fehler: Ohne Cull Index hat Turm B 88 Kapseln, und die zwei westlichen im obersten Geschoss kollidieren mit der Versorgung."],
        ["fix", "Cull Index removes exactly those two planes. Now the definition gives 86, the number the building really has.",
         "Cull Index entfernt genau diese zwei Ebenen. Nun liefert die Definition 86, die Zahl, die das Gebäude wirklich hat."],
''' + s[i:]
rep('"core": ("side", "sh", "n", "ps", "two", "tn", "tot", "hc", "cp", "xy", "co", "core"),', '"core": ("side", "sh", "n", "ps", "two", "tn", "tot", "hc", "cp", "xy", "co", "core"),')
rep('TXT = "#1B1B1B"', 'TXT = "#1B1B1B"\nMINUS = _visuals.MINUS\nUNITS = {"side": "m", "sh": "m", "pitch": "m", "ow": "m", "ch": "m", "pw": "m", "pd": "m"}\nGERMAN = {"Core side": "Kernseite", "Storey height": "Geschosshöhe", "Capsule storeys": "Kapselgeschosse", "Podium storeys": "Sockelgeschosse", "Slot pitch": "Raster",\n          "Capital overhang": "Überstand Kopf", "Capital height": "Höhe Kopf", "Podium length": "Länge Sockel", "Podium width": "Breite Sockel"}')
rep('''            return self.gh_slider(s["label"], value, s["min"], s["max"], at=at, decimals=digits, width=width or 3.6, selected=selected)''',
    '''            label = s["label"] if self.language() == "en" else GERMAN.get(s["label"], s["label"])
            return self.gh_slider(label, value, s["min"], s["max"], at=at, decimals=digits, width=width or 3.6, selected=selected, unit=UNITS.get(oid, ""))
        if spec.kind == "toggle":
            return self.gh_toggle(bool(over.get(oid, s["value"])), at=at, name=self.tr("Swap B to D", "B gegen D tauschen"), selected=selected)''')
rep('''            return self.gh_param("Brep", at=at, label=s.get("label", "Brep"), selected=selected)''', '''            return self.gh_param(spec.outputs[0].type, at=at, label=s.get("label"), selected=selected)''')
rep("box = _engine.SurroundingRectangle(part, color=_ink(COLORS[key]), buff=buff, stroke_width=3)", "box = _engine.SurroundingRectangle(part, color=_ink(COLORS[key], 0.25), buff=buff, stroke_width=4)")
rep("box.set_fill(COLORS[key], opacity=0.15)", "box.set_fill(COLORS[key], opacity=0.22)")
i = s.index("    def present_tower(self):")
s = s[:i] + open(body, encoding="utf-8").read()
open(p, "w", encoding="utf-8").write(s)
