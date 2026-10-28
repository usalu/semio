"""Apply the verified narration rows (audit-narration.md section 6, with the 2 kWh/a claim corrected) to the algorithms scene."""
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
path = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py"
text = path.read_text(encoding="utf-8")
swaps = [
 ("colour is annual electricity in kWh/a", "colour shows annual electricity in kWh/a"),
 ("die Farbe ist der Jahresstrom in kWh/a", "die Farbe zeigt den Strombedarf in kWh/a"),
 ("less window, deeper overhang.", "less window share, deeper overhang."),
 ("weniger Fenster, tieferes Vordach.", "weniger Fensteranteil, tieferes Vordach."),
 ("starts at 75 % windows and 0.1 m overhang.", "starts at a 75 % window share and 0.1 m overhang."),
 ("startet bei 75 % Fenster und 0,1 m Vordach.", "startet bei 75 % Fensteranteil und 0,1 m Vordach."),
 ("reveal the slope: 5 runs per step.", "reveal the slope: five simulations per step."),
 ("verraten die Steigung: 5 Läufe pro Schritt.", "verraten die Steigung: fünf Simulationen pro Schritt."),
 ("More window raises the electricity here", "More window share raises the electricity here"),
 ("Mehr Fenster erhöht hier den Stromverbrauch", "Mehr Fensteranteil erhöht hier den Strombedarf"),
 ("overshoots the valley floor, to too little window.", "overshoots the valley floor, to too small a window share."),
 ("schießt über den Talboden hinaus, zu wenig Fenster.", "schießt über den Talboden hinaus, auf einen zu kleinen Fensteranteil."),
 ("manche Schritte erhöhen den Strom sogar.", "manche Schritte erhöhen sogar den Strombedarf."),
 ("After 50 simulations its best design is within 1 % of the map minimum.", "After 50 simulations, gradient descent's best design lies within 1 % of the map minimum."),
 ("Nach 50 Simulationen liegt sein bestes Design innerhalb von 1 % des Kartenminimums.", "Nach 50 Simulationen liegt der beste Entwurf des Gradientenabstiegs innerhalb von 1 % des Kartenminimums."),
 ("Four of five runs went into slopes; noise or a second valley can mislead them.", "Four of every five simulations probe slopes; noise or a second valley can mislead them."),
 ("Vier von fünf Läufen gingen in Steigungen; Rauschen oder ein zweites Tal täuschen sie.", "Vier von fünf Simulationen tasten Steigungen ab; Rauschen oder ein zweites Tal können sie täuschen."),
 ("it tries windows and overhang, up and down: 4 runs.", "it tries window share and overhang, up and down: four simulations."),
 ("probiert sie Fenster und Vordach, größer und kleiner: 4 Läufe.", "probiert sie Fensteranteil und Vordach, größer und kleiner: vier Simulationen."),
 ("After 49 simulations it is within 1 % of the minimum, with no slope.", "Pattern search first reaches within 1 % of the minimum after 16 simulations, with no slope."),
 ("Nach 49 Simulationen liegt sie innerhalb von 1 % des Minimums, ohne jede Steigung.", "Die Mustersuche erreicht 1 % über dem Minimum nach 16 Simulationen, ganz ohne Steigung."),
 ("keeps 8 designs; window share", "keeps eight designs; window share"),
 ("hält 8 Entwürfe; Fensteranteil", "hält acht Entwürfe; Fensteranteil"),
 ("simulated electricity: 8 runs.", "simulated electricity: eight simulations."),
 ("simulierten Strom: 8 Läufe.", "simulierten Strombedarf: acht Simulationen."),
 ("die mit weniger Strom wird Elternteil.", "die mit weniger Strombedarf wird ein Elternteil."),
 ("the child takes windows from one parent, overhang from the other.", "the child takes window share from one parent, overhang from the other."),
 ("Das Kind erbt die Fenster vom einen, das Vordach", "Das Kind erbt den Fensteranteil vom einen, das Vordach"),
 ("Six children, one simulation each: 6 runs per generation.", "Six children, one simulation each: six simulations per generation."),
 ("Sechs Kinder, je eine Simulation: 6 Läufe pro Generation.", "Sechs Kinder, je eine Simulation: sechs Simulationen pro Generation."),
 ("so the record never worsens.", "so the best value never gets worse."),
 ("Elitismus: Die zwei besten Eltern überleben, der Rekord wird nie schlechter.", "Elitismus: Die beiden besten Eltern überleben, so verschlechtert sich der Bestwert nie."),
 ("Repeat for 7 generations:", "Repeat for seven generations:"),
 ("Wiederhole 7 Generationen:", "Wiederhole sieben Generationen:"),
 ("the best card is within 1 % of the minimum.", "the best card is within 1 % of the map minimum."),
 ("innerhalb von 1 % des Minimums.", "innerhalb von 1 % des Kartenminimums."),
 ("pattern search needs 16 runs, the others over twice as many.", "pattern search needs 16 simulations, the others more than twice as many."),
 ("Bis 1 %: Mustersuche braucht 16 Läufe, die anderen mehr als doppelt so viele.", "Bis 1 %: Die Mustersuche braucht 16 Simulationen, die anderen mehr als doppelt so viele."),
 ("best electricity so far against simulations used.", "the best electricity so far, plotted against simulations used."),
 ("bester Strom bisher über den Simulationen.", "der bisher beste Strombedarf, aufgetragen gegen die Simulationen."),
 ("Pattern search arrives first; after 50 runs all three lie within 2 kWh/a.", "Pattern search gets there first; after 50 simulations all three lie within 2 kWh/a of each other."),
 ("Die Mustersuche ist zuerst da; nach 50 Läufen liegen alle drei innerhalb von 2 kWh/a.", "Die Mustersuche ist zuerst am Ziel; nach 50 Simulationen liegen alle drei innerhalb von 2 kWh/a zueinander."),
 ("only the 1,681-run map could show it.", "only the full map of 1,681 simulations could show it."),
 ("nur die Karte mit 1.681 Läufen könnte es zeigen.", "nur die volle Karte mit 1.681 Simulationen könnte es zeigen."),
 ("Choose by smoothness, noise and runs;", "Choose by smoothness, noise and simulation budget;"),
 ("Wähle nach Glattheit, Rauschen und Läufen;", "Wähle nach Glätte, Rauschen und Simulationsbudget;"),
 ("Algorithms choose what to simulate next; model and architect decide what counts.", "Answer for this room: 61 % window share, 0.91 m overhang, 496 kWh/a; the architect judges the rest."),
 ("Algorithmen wählen die nächste Simulation; Modell und Architekt entscheiden, was zählt.", "Antwort für diesen Raum: 61 % Fensteranteil, 0,91 m Vordach, 496 kWh/a; den Rest beurteilt der Architekt."),
]
for old, new in swaps:
    assert text.count(old) == 1, old
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
print("applied", len(swaps))
