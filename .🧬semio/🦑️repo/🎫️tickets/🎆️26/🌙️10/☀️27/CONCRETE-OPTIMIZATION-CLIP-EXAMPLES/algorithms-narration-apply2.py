"""Second narration pass: compare_2 states the model's gap to the map minimum; three over-long lines shortened."""
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
path = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py"
text = path.read_text(encoding="utf-8")
swaps = [
 ("After 50 simulations, gradient descent's best design lies within 1 % of the map minimum.", "After 50 simulations, gradient descent's best design is within 1 % of the map minimum."),
 ("Nach 50 Simulationen liegt der beste Entwurf des Gradientenabstiegs innerhalb von 1 % des Kartenminimums.", "Nach 50 Simulationen liegt der Gradientenabstieg innerhalb von 1 % des Kartenminimums."),
 ("Pattern search gets there first; after 50 simulations all three lie within 2 kWh/a of each other.", "Pattern search gets there first; all three end within 3.2 kWh/a of the map minimum."),
 ("Die Mustersuche ist zuerst am Ziel; nach 50 Simulationen liegen alle drei höchstens 2 kWh/a auseinander.", "Die Mustersuche ist zuerst am Ziel; alle drei enden innerhalb von 3,2 kWh/a des Kartenminimums."),
 ("Answer for this room: 61 % window share, 0.91 m overhang, 496 kWh/a; the architect judges the rest.", "Answer: 61 % window share, 0.91 m overhang, 496 kWh/a; the architect judges the rest."),
 ("Antwort für diesen Raum: 61 % Fensteranteil, 0,91 m Vordach, 496 kWh/a; den Rest beurteilt der Architekt.", "Antwort: 61 % Fensteranteil, 0,91 m Vordach, 496 kWh/a; den Rest beurteilt der Architekt."),
]
for old, new in swaps:
    assert text.count(old) == 1, old
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
