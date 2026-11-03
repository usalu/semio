import os
import sys

path = sys.argv[1]
src = open(path, encoding="utf-8").read()

PAIRS = [
    ('section_heading("Erst die Last senken, dann lüften")', 'section_heading(tr("Reduce the load first, then ventilate", "Erst die Last senken, dann lüften"))'),
    ('meter("Kühllast"', 'meter(tr("Cooling load", "Kühllast")'),
    ('_step("1", "Hülle + Sonnenschutz", "Last sinkt", P_PRIMARY)', '_step("1", tr("Envelope + sun shading", "Hülle + Sonnenschutz"), tr("load falls", "Last sinkt"), P_PRIMARY)'),
    ('_step("2", "Fenster · Quer · Nacht", "freie Kühlung", P_CYAN)', '_step("2", tr("Window · cross · night", "Fenster · Quer · Nacht"), tr("free cooling", "freie Kühlung"), P_CYAN)'),
    ('_step("3", "Zu-/Abluft mit WRG", "Hauptweg", P_ORANGE)', '_step("3", tr("Supply/extract air with HRV", "Zu-/Abluft mit WRG"), tr("main path", "Hauptweg"), P_ORANGE)'),
    ('_back_chip("← Lüftung · Kapitel 4: Fenster · Quer · Auftrieb · Nacht")', '_back_chip(tr("← Ventilation · Chapter 4: Window · Cross · Stack · Night", "← Lüftung · Kapitel 4: Fenster · Quer · Auftrieb · Nacht"))'),
    ('_back_chip("← Lüftung · Kapitel 5: Zu-/Abluft · WRG")', '_back_chip(tr("← Ventilation · Chapter 5: Supply/extract air · HRV", "← Lüftung · Kapitel 5: Zu-/Abluft · WRG"))'),
    ('_back_chip("← Lüftung · Kapitel 4")', '_back_chip(tr("← Ventilation · Chapter 4", "← Lüftung · Kapitel 4"))'),
    ('_back_chip("← Lüftung · Kapitel 5")', '_back_chip(tr("← Ventilation · Chapter 5", "← Lüftung · Kapitel 5"))'),
    ('section_heading("Sorptionsgestützte Kühlung (DEC)")', 'section_heading(tr("Sorption-assisted cooling (DEC)", "Sorptionsgestützte Kühlung (DEC)"))'),
    ('Text("Sorptionsrad",', 'Text(tr("Desiccant wheel", "Sorptionsrad"),'),
    ('Text("Plattentauscher",', 'Text(tr("Plate exchanger", "Plattentauscher"),'),
    ('Text("Verdunstungskühlung",', 'Text(tr("Evaporative cooling", "Verdunstungskühlung"),'),
    (r'math_label(r"32\,\mathrm{°C}\;\text{feucht}"', r'math_label(rf"32\,\mathrm{{°C}}\;\text{{{tr(' + "'humid', 'feucht'" + r')}}}"'),
    (r'math_label(r"48\,\mathrm{°C}\;\text{trocken}"', r'math_label(rf"48\,\mathrm{{°C}}\;\text{{{tr(' + "'dry', 'trocken'" + r')}}}"'),
    ('Text("Außenluft",', 'Text(tr("Outdoor air", "Außenluft"),'),
    ('Text("Zuluft",', 'Text(tr("Supply air", "Zuluft"),'),
    ('Text("Abluft 26 °C",', 'Text(tr(f"Extract air {unit_text(\'26\', \'°C\')}", "Abluft 26 °C"),'),
    (r'Text("Fortluft\nfeucht · warm",', r'Text(tr("Exhaust air\nhumid · warm", "Fortluft\nfeucht · warm"),'),
    (r'Text("Mutation:\ndie Schädlichen überleben",', r'Text(tr("Mutation:\nthe harmful survive", "Mutation:\ndie Schädlichen überleben"),'),
    ('section_heading("Strategie: freie Kühlung zuerst")', 'section_heading(tr("Strategy: free cooling first", "Strategie: freie Kühlung zuerst"))'),
    ('chip("Sommertag",', 'chip(tr("Summer day", "Sommertag"),'),
    (r'Text("Außenluft kühler\nals der Raum?",', r'Text(tr("Outdoor air cooler\nthan the room?", "Außenluft kühler\nals der Raum?"),'),
    ('chip("Natürlich lüften",', 'chip(tr("Ventilate naturally", "Natürlich lüften"),'),
    ('chip("Öffnung · Luftweg · Zeitplan",', 'chip(tr("Opening · air path · schedule", "Öffnung · Luftweg · Zeitplan"),'),
    ('chip("Schließen + verschatten",', 'chip(tr("Close + shade", "Schließen + verschatten"),'),
    ('chip("Speichermasse puffert",', 'chip(tr("Thermal mass buffers", "Speichermasse puffert"),'),
    ('chip("Reserve: Zu-/Abluft mit WRG",', 'chip(tr("Reserve: supply/extract air with HRV", "Reserve: Zu-/Abluft mit WRG"),'),
    ('chip("Komfort",', 'chip(tr("Comfort", "Komfort"),'),
    ('Text("ja",', 'Text(tr("yes", "ja"),'),
    ('Text("nein",', 'Text(tr("no", "nein"),'),
]

for old, new in PAIRS:
    n = src.count(old)
    if n != 1:
        print("MISS" if n == 0 else "MULTI", n, old)
        continue
    src = src.replace(old, new)

if "unit_text = _manim_module.unit_text" not in src:
    src = src.replace("tr = _manim_module.tr\n", "tr = _manim_module.tr\nunit_text = _manim_module.unit_text\n", 1)

tmp = path + ".lc3tmp"
open(tmp, "w", encoding="utf-8", newline="").write(src)
os.replace(tmp, path)
print("ok")
