"""Replace the English and German text of selected pareto narration clauses (run with -I and the scene path)."""
import re
import sys
from pathlib import Path

NEW = {
    "room": ("Our seminar room has one glazed façade: we choose window, insulation and overhang.", "Der Seminarraum hat eine Glasfassade: Wir wählen Fenster, Dämmung, Dachüberstand."),
    "objectives_2": ("Every façade becomes one point: cost across, electricity up.", "Jede Fassade wird ein Punkt: Kosten rechts, Stromverbrauch oben."),
    "grid_2": ("Point at any tile: room, price and electricity follow.", "Zeige auf eine Kachel: Raum, Preis und Strom folgen."),
    "extremes": ("The cheapest and the most frugal façade lie far apart: none is both.", "Die günstigste und sparsamste Fassade liegen weit auseinander: keine ist beides."),
    "dominated_3": ("So the first is dominated: nobody would pick it.", "Die erste ist dominiert: Niemand würde sie wählen."),
    "rule": ("Dominance: no worse in cost and no worse in electricity.", "Dominanz: bei Kosten und Strom nicht schlechter."),
    "rule_2": ("And strictly better in at least one of them.", "Und in mindestens einem echt besser."),
    "front": ("Sweep from cheap to expensive: keep only façades that beat all cheaper ones.", "Von günstig zu teuer: Es bleiben, die alle günstigeren schlagen."),
    "approximate": ("Between the points, an unevaluated design may still beat a front member.", "Zwischen den Punkten kann ein ungeprüfter Entwurf Frontmitglieder schlagen."),
    "sets": ("The room is design space, the plot is objective space; evaluation maps between them.", "Raum: Entwurfsraum, Diagramm: Zielraum; die Auswertung bildet ab."),
    "payback": ("At 0.25 CHF per kWh the savings repay this step in 12 years.", "Bei 0,25 CHF/kWh amortisiert sich der Schritt in 12 Jahren."),
    "payback_2": ("A façade lasts about 30 years: only a faster payback is worth buying.", "Eine Fassade hält etwa 30 Jahre: Nur schnellere Amortisation lohnt."),
    "knee": ("Walk on: each step saves less, until payback exceeds the lifetime.", "Weiter: Jeder Schritt bringt weniger, bis die Lebensdauer überschritten ist."),
    "knee_2": ("Here is the knee: before it money buys a lot, after it almost nothing.", "Hier das Knie: Davor bringt Geld viel, danach kaum noch."),
    "knee_3": ("A higher tariff moves the knee: same front, different best choice.", "Höherer Tarif verschiebt das Knie: gleiche Front, andere beste Wahl."),
    "third": ("Add daylight as a third goal: tile colour shows the daylight factor in %.", "Drittes Ziel Tageslicht: Die Kachelfarbe zeigt den Tageslichtquotienten in %."),
    "third_2": ("In 2D it loses: a dimmer room was cheaper and used less.", "In 2D verliert er: Ein dunklerer Raum war billiger und sparsamer."),
    "third_3": ("With daylight counted, nobody beats it on all three goals.", "Mit Tageslicht schlägt ihn keiner in allen drei Zielen."),
    "weights": ("To pick one façade, weigh the goals, as in the index video.", "Zum Wählen gewichten wir die Ziele, wie im Indexvideo."),
    "weights_2": ("The winner always sits on the front and slides along it with the weights.", "Der Sieger liegt stets auf der Front und gleitet entlang."),
    "decision_2": ("Tariff, lifetime, weights and judgement choose: the front shows what is possible.", "Tarif, Lebensdauer, Gewichte, Urteil entscheiden: Die Front zeigt das Mögliche."),
}
target = Path(sys.argv[1])
text = target.read_text(encoding="utf-8")
for key, (en, de) in NEW.items():
    pattern = re.compile(r'("' + re.escape(key) + r'",\n\s+)"[^\n]*",\n(\s+)"[^\n]*"')
    if not pattern.search(text):
        raise SystemExit("missing " + key)
    text = pattern.sub(lambda m: f'{m.group(1)}"{en}",\n{m.group(2)}"{de}"', text, count=1)
target.write_text(text, encoding="utf-8")
print("ok")
