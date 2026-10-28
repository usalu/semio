"""Apply the index rows of audit-narration.md to the scene NARRATION (each old string must occur exactly once)."""
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
SCENE = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py"
text = SCENE.read_text(encoding="utf-8")
pairs = [
    ('"de": "Mehrere Kriterien und ein Normierter Index"', '"de": "Mehrere Kriterien und ein normierter Index"'),
    ('"Five façade variants for the same seminar room.", "Fünf Fassadenvarianten für denselben Seminarraum."',
     '"Which of five façade designs suits this seminar room best?", "Welcher von fünf Fassadenentwürfen passt am besten zu diesem Seminarraum?"'),
    ('"Electricity and cost: lower is better.", "Strom und Kosten: weniger ist besser."',
     '"Electricity intensity and cost: lower is better.", "Strombedarf pro m² und Fassadenkosten: kleiner ist besser."'),
    ('"Daylight: higher is better. Carbon: lower is better.", "Tageslicht: mehr ist besser. CO₂: weniger ist besser."',
     '"Daylight factor: higher is better. Embodied carbon: lower is better.", "Tageslichtquotient: mehr ist besser. Graue Emissionen: kleiner ist besser."'),
    ('"No variant is best everywhere.", "Keine Variante ist überall die beste."', '"No design is best everywhere.", "Kein Entwurf ist überall der beste."'),
    ('"Kilowatt-hours per square metre plus francs? That sum has no unit.", "Kilowattstunden pro Quadratmeter plus Franken? Die Summe hat keine Einheit."',
     '"Kilowatt-hours per square metre per year, plus francs? That sum has no unit.", "Kilowattstunden pro Quadratmeter und Jahr plus Franken? Die Summe hat keine Einheit."'),
    ('"Count in thousands (kCHF, tonnes) instead and the ranking changes.", "Zähle stattdessen in Tausendern (kCHF, Tonnen), und die Rangfolge ändert sich."',
     '"Count cost in kCHF and carbon in tonnes instead, and the ranking changes.", "Zähle stattdessen Kosten in kCHF und Emissionen in Tonnen, und die Rangfolge ändert sich."'),
    ('"Give each criterion its own ruler, starting with electricity.", "Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strom."',
     '"Give each criterion its own ruler, starting with electricity.", "Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strombedarf pro m²."'),
    ('"Each variant sits at its real value; the room shows what it means.", "Jede Variante liegt auf ihrem echten Wert; der Raum zeigt die Bedeutung."',
     '"Each design sits at its real value; the room shows what the value means.", "Jeder Entwurf liegt auf seinem echten Wert; der Raum zeigt, was er bedeutet."'),
    ('"Step one: subtract the best value of the set.", "Schritt eins: Ziehe den besten Wert der Menge ab."',
     '"Step one: subtract the best of these five values.", "Schritt eins: Ziehe den besten dieser fünf Werte ab."'),
    ('"Cost gets the same ruler: the cheapest façade becomes zero.", "Die Kosten erhalten dasselbe Lineal: Die günstigste Fassade wird null."',
     '"Façade cost gets the same ruler: the cheapest façade becomes zero.", "Die Fassadenkosten erhalten dasselbe Lineal: Die günstigste Fassade wird null."'),
    ('"Daylight: more is better, so the ruler is flipped.", "Tageslicht: mehr ist besser, also wird das Lineal umgedreht."',
     '"Daylight factor: more is better, so the ruler is mirrored.", "Tageslichtquotient: mehr ist besser, also wird das Lineal gespiegelt."'),
    ('"Carbon follows suit: four comparable values from zero to one.", "CO₂ ebenso: vier vergleichbare Werte zwischen null und eins."',
     '"Embodied carbon follows the same rule: its five values now lie between zero and one.", "Graue Emissionen folgen derselben Regel: ihre fünf Werte liegen nun zwischen null und eins."'),
    ('"Here: electricity 40 %, cost 30 %, daylight 20 %, carbon 10 %.", "Hier: Strom 40 %, Kosten 30 %, Tageslicht 20 %, CO₂ 10 %."',
     '"Here: electricity intensity 40 %, cost 30 %, daylight factor 20 %, embodied carbon 10 %.", "Hier: Strombedarf pro m² 40 %, Kosten 30 %, Tageslichtquotient 20 %, Graue Emissionen 10 %."'),
    ('"Schiebe die gefüllten Teile zusammen: Ihre Länge ist der Index."', '"Schiebe die gefüllten Teile zusammen: ihre Länge ist der Index."'),
    ('"Another façade wins, though no variant changed.", "Eine andere Fassade gewinnt, obwohl sich keine Variante geändert hat."',
     '"Another façade wins, though no design changed.", "Eine andere Fassade gewinnt, obwohl sich kein Entwurf geändert hat."'),
    ('"Knappe Entscheide sind fragil: Prüfe die Rangfolge, bevor du entscheidest."', '"Knappe Entscheidungen sind fragil: Prüfe die Rangfolge, bevor du entscheidest."'),
    ('"The rulers depend on the variants compared: add a sixth façade."', '"The rulers depend on the designs compared: add a sixth façade."'),
    ('obwohl sie gleich blieben."', 'obwohl sie unverändert blieben."'),
    ('"Document weights, compared variants and ranges with every decision.", "Dokumentiere Gewichte, Varianten und Wertebereiche bei jeder Entscheidung."',
     '"Document weights, compared designs and value ranges with every decision.", "Dokumentiere Gewichte, verglichene Entwürfe und Wertebereiche bei jeder Entscheidung."'),
    ('        ["next", "To see trade-offs before weighting, look at Pareto fronts.", "Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten."]',
     '        ["pointer", "To see trade-offs before weighting, look at Pareto fronts.", "Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten."],\n'
     '        ["next", "For this room at these weights, the winner has the lowest index.", "Für diesen Raum gewinnt bei diesen Gewichten der kleinste Index."]'),
]
for old, new in pairs:
    assert text.count(old) == 1, old
    text = text.replace(old, new)
SCENE.write_text(text, encoding="utf-8", newline="\n")
print("patched", len(pairs))
