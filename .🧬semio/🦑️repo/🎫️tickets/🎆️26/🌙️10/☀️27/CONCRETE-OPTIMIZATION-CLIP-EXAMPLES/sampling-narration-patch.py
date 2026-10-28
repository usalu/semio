"""Apply the narration audit rows to the sampling scene (checked against model and picture) and align the on-screen German words."""
from pathlib import Path

path = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir()) / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py"
text = path.read_text(encoding="utf-8")
swaps = [
    ('"Two choices shape this south façade: window share and overhang depth.", "Zwei Entscheidungen prägen diese Südfassade: Fensteranteil, Vordachtiefe."',
     '"Which window share and overhang depth suit this south façade?", "Welcher Fensteranteil und welche Vordachtiefe passen zu dieser Südfassade?"'),
    ('"One simulation plays a whole year of sun, heat and light.", "Eine Simulation spielt ein Jahr aus Sonne, Wärme und Licht durch."',
     '"One simulation runs through a whole year of sun, heat and daylight.", "Eine Simulation durchläuft ein ganzes Jahr mit Sonne, Wärme und Tageslicht."'),
    ('"Winter sun saves heat, summer sun adds cooling, daylight saves lamps.", "Wintersonne spart Heizen, Sommersonne bringt zusätzliches Kühlen, Tageslicht spart Licht."',
     '"Winter sun cuts heating electricity, summer sun adds cooling, daylight cuts lighting.", "Wintersonne senkt Heizstrom, Sommersonne erhöht Kühlstrom, Tageslicht senkt Beleuchtungsstrom."'),
    ('"One number: electricity per year, carried by each point.", "Eine Zahl: Strom pro Jahr, getragen von jedem Punkt."',
     '"One number per design: electricity in kWh/a, carried by each point.", "Eine Zahl je Entwurf: Strombedarf in kWh/a, getragen von jedem Punkt."'),
    ('"A full grid takes three levels per axis and combines them all.", "Ein volles Raster nimmt drei Stufen je Achse und kombiniert alle."',
     '"A full grid uses three levels per axis and combines them all.", "Ein volles Raster nutzt drei Stufen je Achse und kombiniert alle."'),
    ('"Run them one by one: room, sun rays and electricity bar for each.", "Wir rechnen sie nacheinander: Raum, Sonnenstrahlen und Strombalken für jeden Entwurf."',
     '"Simulate them one by one: room, sun rays and electricity bar for each.", "Wir simulieren sie nacheinander: Raum, Sonnenstrahlen und Strombalken für jeden Entwurf."'),
    ('"The ninth run empties the budget.", "Der neunte Lauf leert das Budget."',
     '"The ninth simulation empties the budget.", "Die neunte Simulation leert das Budget."'),
    ('"More factors: orientation with four levels, insulation with three.", "Mehr Faktoren: Orientierung mit vier, Dämmung mit drei Stufen."',
     '"More design variables: orientation with four levels, insulation with three.", "Mehr Entwurfsvariablen: Ausrichtung mit vier, Dämmung mit drei Stufen."'),
    ('"Every extra level multiplies the runs: here twelve times our budget.", "Jede weitere Stufe vervielfacht die Läufe: hier zwölfmal unser Budget."',
     '"Every extra level multiplies the simulations: here twelve times our budget.", "Jede weitere Stufe vervielfacht die Simulationen: hier zwölfmal unser Budget."'),
    ('"Random sampling draws nine independent pairs, recorded so everyone sees the same points.", "Zufälliges Sampling zieht neun unabhängige Paare, gespeichert: alle sehen dieselben Punkte."',
     '"Random sampling draws nine independent designs, recorded so everyone sees the same points.", "Zufälliges Sampling zieht neun unabhängige Entwürfe, gespeichert: alle sehen dieselben Punkte."'),
    ('"This box stays empty: no draw pairs a wide window with a deep overhang.", "Leere Box: keine Ziehung hat großes Fenster und tiefes Vordach."',
     '"This box stays empty: no draw combines a large window share with a deep overhang.", "Leere Box: Keine Ziehung kombiniert großen Fensteranteil mit tiefem Vordach."'),
    ('"Latin hypercube sampling: nine strata per axis, one sample in each.", "Latin-Hypercube-Sampling teilt jede Achse in neun Schichten, je eine Stichprobe."',
     '"Latin hypercube sampling: nine strata per axis, exactly one sample in each stratum.", "Latin-Hypercube-Sampling teilt jede Achse in neun Schichten, je Schicht genau eine Stichprobe."'),
    ('"This vertical strip is one window range and holds exactly one design.", "Dieser senkrechte Streifen ist ein Fensterbereich und enthält genau einen Entwurf."',
     '"This vertical strip is one window-share band and holds exactly one design.", "Dieser senkrechte Streifen ist ein Band des Fensteranteils und enthält genau einen Entwurf."'),
    ('"The horizontal strip holds exactly one, too: each axis is covered evenly.", "Auch der waagerechte Streifen enthält genau einen: jede Achse gleichmäßig abgedeckt."',
     '"The horizontal strip also holds exactly one: each axis is covered evenly.", "Auch der waagerechte Streifen enthält genau einen: Jede Achse ist gleichmäßig abgedeckt."'),
    ('"To judge the sets, uncover the surface with a dense reference grid.", "Zur Beurteilung legen wir die Fläche mit dichtem Referenzraster frei."',
     '"To judge the point sets, reveal the surface with a dense reference grid.", "Zur Beurteilung der Punktmengen legen wir die Fläche mit einem dichten Referenzraster frei."'),
    ('"Darker means less electricity. The lowest point lies inside the range.", "Dunkler heißt weniger Strom. Der tiefste Punkt liegt im Inneren."',
     '"Darker means less electricity. The lowest point lies inside the range.", "Dunkler heißt weniger Strombedarf. Der tiefste Punkt liegt im Inneren des Bereichs."'),
    ('"Compare each set\'s best design with the optimum of the dense grid.", "Vergleiche den besten Entwurf jeder Menge mit dem Optimum des dichten Rasters."',
     '"Compare each point set\'s best design with the optimum of the dense reference grid.", "Vergleiche den besten Entwurf jeder Punktmenge mit dem Optimum des dichten Referenzrasters."'),
    ('"Jeder Beste ist der beste von neun, doch keiner erreicht das Optimum."', '"Jeder beste Entwurf ist der beste von neun, doch keiner erreicht das Optimum."'),
    ('        ["limits_3", "And the model itself still needs validation.", "Und das Modell selbst braucht noch eine Prüfung."],\n',
     '        ["limits_3", "The model itself still needs validation.", "Und das Modell selbst braucht noch eine Prüfung."],\n'
     '        ["limits_4", "For this south façade, nine simulations found a good design, not a proven optimum.", "Für diese Südfassade fanden neun Simulationen einen guten Entwurf, aber kein bewiesenes Optimum."],\n'),
    ('"Vereinfachtes Jahres-Lehrmodell · Klima wie Zürich"', '"Vereinfachtes Jahresmodell · Klima wie Zürich"'),
    ('self.tr("Orientation", "Orientierung")', 'self.tr("Orientation", "Ausrichtung")'),
    ('("heating", self.tr("Heating", "Heizen"), HEAT), ("cooling", self.tr("Cooling", "Kühlen"), COOL), ("lighting", self.tr("Lighting", "Licht"), LIGHT)',
     '("heating", self.tr("Heating", "Heizstrom"), HEAT), ("cooling", self.tr("Cooling", "Kühlstrom"), COOL), ("lighting", self.tr("Lighting", "Beleuchtung"), LIGHT)'),
    ('((self.tr("Heating", "Heizen"), HEAT), (self.tr("Cooling", "Kühlen"), COOL), (self.tr("Lighting", "Licht"), LIGHT))',
     '((self.tr("Heating", "Heizstrom"), HEAT), (self.tr("Cooling", "Kühlstrom"), COOL), (self.tr("Lighting", "Beleuchtung"), LIGHT))'),
    ('        self.step("limits_3", mn.FadeIn(closing), mn.FadeIn(note), seconds=1.5)\n',
     '        self.step("limits_3", mn.FadeIn(closing), mn.FadeIn(note), seconds=1.5)\n'
     '        pick = min(self.marks["lhs"], key=lambda item: item["sample"]["electricity"])\n'
     '        answer = self._chip(self.tr("best of nine", "bester von neun"), *self._q(pick["sample"]["electricity"], "kWh/a", 1), "·", "Δ", *self._q(round(pick["sample"]["electricity"], 1) - round(optimum["electricity"], 1), "kWh/a", 1),\n'
     '                            at=self._slot(), size=14, color=SETS["lhs"]["color"])\n'
     '        self.P.set_value(1.0)\n'
     '        self.step("limits_4", mn.FadeOut(cards), mn.FadeOut(closing), mn.FadeOut(region), mn.FadeIn(answer), mn.FadeIn(self.bars), self.set_design(self.T, pick["design"], 1.5), seconds=3)\n'),
]
for old, new in swaps:
    assert old in text, old[:70]
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
print("applied", len(swaps))
