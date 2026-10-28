"""Rewrite sampling captions that leave one short word on the second line."""
from pathlib import Path

path = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir()) / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py"
text = path.read_text(encoding="utf-8")
swaps = [("Winter sun saves heating, summer sun adds cooling, daylight saves lamps.", "Winter sun saves heat, summer sun adds cooling, daylight saves lamps."),
         ("Latin hypercube sampling cuts each axis into nine strata, one sample each.", "Latin hypercube sampling: nine strata per axis, one sample in each."),
         ("Wintersonne spart Heizen, Sommersonne bringt Kühlen, Tageslicht spart Licht.", "Wintersonne spart Heizen, Sommersonne bringt zusätzliches Kühlen, Tageslicht spart Licht."),
         ("Wir rechnen nacheinander: Raum, Sonnenstrahlen und Strombalken je Entwurf.", "Wir rechnen nacheinander: Raum, Sonnenstrahlen und Strombalken je Lauf."),
         ("Zufälliges Sampling zieht neun unabhängige Paare, gespeichert für alle gleich.", "Zufälliges Sampling zieht neun unabhängige Paare, gespeichert: alle sehen dieselben Punkte."),
         ("Diese Box bleibt leer: keine Ziehung hat großes Fenster und tiefes Vordach.", "Leere Box: keine Ziehung hat großes Fenster und tiefes Vordach."),
         ("Jeder Punkt wird ein vollständiges Gebäude und durchläuft dasselbe Modell.", "Jeder Punkt wird ein volles Gebäude und durchläuft dasselbe Modell."),
         ("Zur Beurteilung legen wir die Fläche mit einem dichten Referenzraster frei.", "Zur Beurteilung legen wir die Fläche mit dichtem Referenzraster frei."),
         ("Sampling zeigt Abdeckung und aussichtsreiche Bereiche für die weitere Suche.", "Sampling zeigt Abdeckung und aussichtsreiche Bereiche für die Suche.")]
for old, new in swaps:
    assert old in text, old
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
print("rewrote", len(swaps))
