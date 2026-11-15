import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\⚡️demand-power\🐍️.py")
src = ROOT.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global src
    n = src.count(old)
    if n == 0 or (count is not None and n != count):
        sys.exit(f"MISMATCH {n} != {count}: {old[:80]!r}")
    src = src.replace(old, new)


sub('''PEAK_KW = 10.0
DESIGN_DT = 32.0
THETA_I = 20.0
THETA_E = -12.0
''', '''_example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
H_T_VAL, H_V_VAL = _example.H_T_W_K, _example.H_V_W_K
THETA_I = _example.THETA_I_WINTER_C
THETA_E = -12.0
DESIGN_DT = THETA_I - THETA_E
PEAK_KW = (H_T_VAL + H_V_VAL) * DESIGN_DT / 1000.0
GAIN_LIMIT = 12.0
''')
sub("load = np.maximum(0.0, 15.0 - theta_e)", "load = np.maximum(0.0, GAIN_LIMIT - theta_e)")
sub('''    Shape only — a cosine season plus multi-day weather driving ``θ_i − θ_e``
    above the heating limit,''', '''    Shape only — a cosine season plus multi-day weather driving ``θ_i − θ_e``
    below the building heating limit that the free gains lower to ``GAIN_LIMIT``,''')
sub("def _kw_ticks(chart, y_max_kw: float, values=(5, 10)):", "def _kw_ticks(chart, y_max_kw: float, values=None):")
sub('''    group = VGroup()
    for v in values:
        f = v / y_max_kw''', '''    group = VGroup()
    for v in values or range(2, int(y_max_kw) + 1, 2):
        f = v / y_max_kw''')
sub('def _wide_chart(y_src: str = r"P\\;[\\mathrm{kW}]"):', 'def _wide_chart(y_src: str = r"\\dot{Q}\\ \\mathrm{in\\ kW}"):')
sub('r"Q\\;[\\mathrm{kWh}]", tank["cap"]', 'r"Q\\ \\mathrm{in\\ kWh}", tank["cap"]')
sub('r"P\\;[\\mathrm{kW}]", POWER, np.array', 'r"\\dot{Q}\\ \\mathrm{in\\ kW}", POWER, np.array')
sub('r"Q\\;[\\mathrm{kWh}]", ENERGY, np.array', 'r"Q\\ \\mathrm{in\\ kWh}", ENERGY, np.array')
sub('rf"P = {label_number(p_frac() * PEAK_KW, 1)}\\,\\mathrm{{kW}}"', 'rf"\\dot{{Q}} = {label_number(p_frac() * PEAK_KW, 1)}\\,\\mathrm{{kW}}"', 2)
sub('"P_{i}", POWER', '"\\\\dot{Q}_{i}", POWER')
sub(r"Q_h\;[\mathrm{kWh}]", r"Q_{h}\ \mathrm{in\ kWh}", None) if False else None
sub(r'r"Q_{h}\;[\mathrm{kWh}]"', r'r"Q_{h}\ \mathrm{in\ kWh}"', 2)
sub(r'\text{Area} = Q_h\;[\mathrm{kWh/a}]', r'\text{Area} = Q_h', None)
sub(r'\text{Fläche} = Q_h\;[\mathrm{kWh/a}]', r'\text{Fläche} = Q_h', None)
sub(r"\Phi_{\mathrm{HL}}", r"\dot{Q}_{H}", None)
sub(r"\Phi_{{\mathrm{{HL}}}}", r"\dot{{Q}}_{{H}}", None)
sub(r"\Phi_{{\mathrm{{Anlage}}}}", r"\dot{{Q}}_{{\mathrm{{Anl}}}}", 1)
sub(r"\theta_{{\mathrm{{int}}}}", r"\theta_{{i}}", 1)
sub(r"\theta_{{\mathrm{{e}}}}", r"\theta_{{e}}", 1)
sub('''            _fact(tr("Symbol:", "Formelzeichen:"), math_label(r"\\dot{Q}_{H}", size=LABEL_FONT_SIZE, color=POWER)),''',
    '''            _fact(tr("Symbol:", "Formelzeichen:"), math_label(tr(r"\\dot{Q}_{H}\\ \\text{— in the standard } \\Phi_{\\mathrm{HL}}", r"\\dot{Q}_{H}\\ \\text{— in der Norm } \\Phi_{\\mathrm{HL}}"), size=LABEL_FONT_SIZE, color=POWER)),''')
sub("        H_T_VAL, H_V_VAL = 225.0, 87.5\n", "")
sub('''            (None, "(", P_WHITE), ("t", label_number(H_T_VAL), H_T), (None, "+", P_WHITE), ("v", label_number(H_V_VAL, 1), H_V),
            (None, ")", P_WHITE), (None, r"\\mathrm{W/K}", P_WHITE), (None, r"\\cdot", P_WHITE),
            ("d", rf"{label_number(DESIGN_DT)}\\,\\mathrm{{K}}", DELTA_T), (None, "=", P_WHITE),
            ("phi", rf"{label_number(PEAK_KW)}\\,\\mathrm{{kW}}", POWER),''',
    '''            (None, "(", P_WHITE), ("t", rf"{label_number(H_T_VAL)}\\,\\frac{{\\mathrm{{W}}}}{{\\mathrm{{K}}}}", H_T), (None, "+", P_WHITE),
            ("v", rf"{label_number(H_V_VAL)}\\,\\frac{{\\mathrm{{W}}}}{{\\mathrm{{K}}}}", H_V),
            (None, ")", P_WHITE), (None, r"\\cdot", P_WHITE),
            ("d", rf"{label_number(DESIGN_DT)}\\,\\mathrm{{K}}", DELTA_T), (None, "=", P_WHITE),
            ("phi", rf"{label_number(PEAK_KW, 2)}\\,\\mathrm{{kW}}", POWER),''')
sub('''t_lbl = math_label(rf"H_{{T}} = {label_number(H_T_VAL)}\\,\\mathrm{{W/K}}"''', '''t_lbl = math_label(rf"H_{{T}} = {label_number(H_T_VAL)}\\,\\frac{{\\mathrm{{W}}}}{{\\mathrm{{K}}}}"''')
sub('''v_lbl = math_label(rf"H_{{V}} = {label_number(H_V_VAL, 1)}\\,\\mathrm{{W/K}}"''', '''v_lbl = math_label(rf"H_{{V}} = {label_number(H_V_VAL)}\\,\\frac{{\\mathrm{{W}}}}{{\\mathrm{{K}}}}"''')
sub('''phi_read = math_label(rf"\\dot{{Q}}_{{H}} = {label_number(PEAK_KW, 1)}\\,\\mathrm{{kW}}", np.array([row_x, 0.55, 0.0])''',
    '''phi_read = math_label(rf"\\dot{{Q}}_{{H}} = {label_number(PEAK_KW, 2)}\\,\\mathrm{{kW}}", np.array([row_x, 0.55, 0.0])''')
sub('''phi_read = math_label(rf"\\dot{{Q}}_{{H}} = {label_number(PEAK_KW, 1)}\\,\\mathrm{{kW}}", np.array([0.2, 1.62, 0.0])''',
    '''phi_read = math_label(rf"\\dot{{Q}}_{{H}} = {label_number(PEAK_KW, 2)}\\,\\mathrm{{kW}}", np.array([0.2, 1.62, 0.0])''')
sub('''(None, rf"\\frac{{Q_{{h}}}}{{\\dot{{Q}}_{{H}}}}"''', '''(None, rf"\\frac{{Q_{{h}}}}{{\\dot{{Q}}_{{H}}}}"''', None) if False else None
sub('''{label_number(PEAK_KW)}\\,\\mathrm{{kW}}}}}}", P_WHITE),''', '''{label_number(PEAK_KW, 2)}\\,\\mathrm{{kW}}}}}}", P_WHITE),''')
sub('din = _din_ref(tr("Fundamentals · DIN V 18599-2", "Grundlagen · DIN V 18599-2"))',
    'din = _din_ref("DIN V 4108-6:2003-06, 3.2 (Symbole, Einheiten)")')
sub('din = _din_ref("DIN EN 12831-1 · DIN/TS 12831-1")', 'din = _din_ref("DIN EN 12831-1:2017-09, Abschn. 6 · DIN/TS 12831-1:2020-04")')
sub('din = _din_ref("DIN EN 12831-1 · DIN V 18599-2")', 'din = _din_ref("DIN EN 12831-1:2017-09, Abschn. 6 · DIN V 4108-6:2003-06, D.2")')
sub('din = _din_ref("DIN EN 12831-1 · DIN/TS 12831-1 · VDI 4645")', 'din = _din_ref("DIN EN 12831-1:2017-09, Abschn. 6 · VDI 4645:2023-04")')

LISTS = {
"Beat1_LeistungUndEnergie": [
 ("zahlen", "Two numbers describe every heating system.", "Zwei Zahlen beschreiben jede Heizung."),
 ("fehler", "Mixing them up is the classic beginner's mistake.", "Sie zu verwechseln ist der klassische Anfängerfehler."),
 ("jahr", "This is one year of heating power for our building.", "Das ist ein Jahr Heizleistung unseres Gebäudes."),
 ("saison", "It is high on cold days and zero all summer.", "An kalten Tagen ist sie hoch, den ganzen Sommer null."),
 ("hoehe", r"Power $\dot{Q}$ is the height of the curve at one moment.", r"Die Leistung $\dot{Q}$ ist die Höhe der Kurve in einem Moment."),
 ("rate", r"Its unit is the kilowatt, $\mathrm{kW}$: how fast energy flows right now.", r"Ihre Einheit ist das Kilowatt, $\mathrm{kW}$: wie schnell Energie gerade fließt."),
 ("summe", r"The energy $Q$ is the running sum of power over time.", r"Die Energie $Q$ ist die zeitliche Summe der Leistung."),
 ("kwh", r"That is the area under the curve, and its unit is the kilowatt-hour, $\mathrm{kWh}$.", r"Das ist die Fläche unter der Kurve. Ihre Einheit ist die Kilowattstunde, $\mathrm{kWh}$."),
 ("produkt", r"Power times time gives energy: $1\,\mathrm{kW}\cdot 1\,\mathrm{h} = 1\,\mathrm{kWh}$.", r"Leistung mal Zeit ergibt Energie: $1\,\mathrm{kW}\cdot 1\,\mathrm{h} = 1\,\mathrm{kWh}$."),
 ("trennung", "That single relation from the fundamentals separates the load from the demand.", "Diese eine Beziehung aus den Grundlagen trennt Last von Bedarf."),
],
"Beat2_Normheizlast": [
 ("spitze", r"The peak of that curve has a name of its own: the design heat load $\dot{Q}_H$.", r"Die Spitze dieser Kurve hat einen eigenen Namen: die Norm-Heizlast $\dot{Q}_H$."),
 ("regeln", r"It comes with its own standard and its own rules — the standard writes $\Phi_{\mathrm{HL}}$ for it.", r"Dazu gehören eine eigene Norm und eigene Regeln — die Norm schreibt dafür $\Phi_{\mathrm{HL}}$."),
 ("extremfall", "It is computed for one artificial worst case.", "Sie wird für einen künstlichen Extremfall berechnet."),
 ("normtemperatur", r"What counts is the design outdoor temperature $\theta_e = -12\,^{\circ}\mathrm{C}$ of the site, which sets the design difference $\Delta\theta$.", r"Es gilt die Norm-Außentemperatur $\theta_e = -12\,^{\circ}\mathrm{C}$ des Standorts — sie legt die Auslegungsdifferenz $\Delta\theta$ fest."),
 ("raumweise", "The calculation is done room by room.", "Gerechnet wird Raum für Raum."),
 ("verluste", "Crucially, only the losses under these design conditions count.", "Entscheidend: Nur die Verluste unter diesen Normbedingungen zählen."),
 ("solar", "Solar gains are not credited as reliable heating power.", "Solare Gewinne gelten nicht als verlässliche Heizleistung."),
 ("intern", "Internal gains are not credited either.", "Interne Gewinne werden ebenfalls nicht angerechnet."),
 ("transmission", r"The envelope contributes the transmission heat transfer coefficient $H_T = 115\,\mathrm{W/K}$ — simplified.", r"Die Hülle liefert den Transmissions-Wärmetransferkoeffizienten $H_T = 115\,\mathrm{W/K}$ — vereinfacht."),
 ("lueftung", r"The air exchange contributes the ventilation coefficient $H_V = 75\,\mathrm{W/K}$.", r"Der Luftaustausch liefert den Lüftungs-Wärmetransferkoeffizienten $H_V = 75\,\mathrm{W/K}$."),
 ("summe", r"Both together times $\Delta\theta = 32\,\mathrm{K}$ give the design heat load $\dot{Q}_H = 6.08\,\mathrm{kW}$.", r"Beide zusammen mal $\Delta\theta = 32\,\mathrm{K}$ ergeben die Norm-Heizlast $\dot{Q}_H = 6{,}08\,\mathrm{kW}$."),
 ("kilowatt", r"The result is kilowatts, $\mathrm{kW}$, not kilowatt-hours.", r"Das Ergebnis sind Kilowatt, $\mathrm{kW}$, nicht Kilowattstunden."),
 ("auslegung", "It is an important basis for sizing the whole heating system.", "Das ist eine wichtige Grundlage für die Auslegung der Heizungsanlage."),
],
"Beat3_Jahresdauerlinie": [
 ("kurve", "Load and demand live on the same curve.", "Last und Bedarf stecken in derselben Kurve."),
 ("bild", "One picture shows both at once — sort the year by size.", "Ein Bild zeigt beides — man sortiert das Jahr nach Größe."),
 ("aufreihen", "Take every hour of the year and line them up from the coldest to the mildest.", "Man reiht jede Stunde des Jahres von der kältesten bis zur mildesten auf."),
 ("dauerlinie", "That is the load duration curve.", "Das ist die Jahresdauerlinie."),
 ("hoehe", r"Its starting height is the design heat load $\dot{Q}_H$.", r"Ihre Anfangshöhe ist die Norm-Heizlast $\dot{Q}_H$."),
 ("wenige", "These are the few hours the plant must still be able to cover.", "Das sind die wenigen Stunden, die die Anlage noch abdecken muss."),
 ("flaeche_bedarf", r"Its area is the annual heating demand $Q_h$.", r"Ihre Fläche ist der Jahres-Heizwärmebedarf $Q_h$."),
 ("nutzenergie", "It is useful energy at the room, not the final energy on the meter and not the billed consumption.", "Es ist Nutzenergie am Raum, nicht die Endenergie am Zähler und nicht der abgerechnete Verbrauch."),
 ("rechteck", "The same area fits into a rectangle as tall as the peak.", "Dieselbe Fläche passt in ein Rechteck, das so hoch ist wie die Spitze."),
 ("vollstunden", r"Area divided by peak gives the full-load hours $t_{VL} = Q_h / \dot{Q}_H$.", r"Fläche geteilt durch Spitze ergibt die Vollbenutzungsstunden $t_{VL} = Q_h / \dot{Q}_H$."),
 ("bruchteil", "A building runs at full load only a fraction of the year.", "Volllast läuft ein Gebäude nur einen Bruchteil des Jahres."),
],
"Beat4_RichtigAuslegen": [
 ("kurve", "Here is our load duration curve again.", "Hier ist wieder unsere Jahresdauerlinie."),
 ("falsch", "On that same curve you can see, directly, what happens when a plant is sized wrongly.", "An derselben Kurve sieht man unmittelbar, was passiert, wenn eine Anlage falsch ausgelegt wird."),
 ("mittelwert", "Size the plant on the average load and it covers only part of the curve.", "Legt man auf den Mittelwert aus, deckt die Anlage nur einen Teil der Kurve."),
 ("defizit", "At high load a power deficit remains.", "Bei hoher Last bleibt ein Leistungsdefizit."),
 ("temperatur", r"Without reserve capacity the room temperature $\theta_i$ can no longer be held.", r"Ohne Reserve lässt sich die Raumtemperatur $\theta_i$ dann nicht halten."),
 ("reserve", "Size the plant far above the peak and a large reserve appears.", "Legt man die Anlage weit über die Spitze aus, entsteht eine große Reserve."),
 ("ueberdimensioniert", "Almost all year that reserve stays unused — the plant is oversized.", "Fast das ganze Jahr bleibt diese Reserve ungenutzt — die Anlage ist überdimensioniert."),
 ("takten", "At too high a minimum output that means more cycling.", "Bei zu hoher Mindestleistung führt das zu häufigerem Takten."),
 ("auslegungspunkt", r"The design heat load $\dot{Q}_H$ gives the central sizing point.", r"Die Norm-Heizlast $\dot{Q}_H$ liefert den zentralen Auslegungspunkt."),
 ("anlage", "The real plant also depends on the heat generator, controls and storage.", "Die reale Anlage hängt zusätzlich von Erzeuger, Regelung und Speicher ab."),
 ("bedarf", r"The area under the curve is the useful heating demand $Q_h$ — useful energy at the room.", r"Die Fläche darunter ist der Heizwärmebedarf $Q_h$ — Nutzenergie am Raum."),
 ("kosten", r"The plant turns that into final energy $Q_E$, and only that creates cost.", r"Das Heizsystem macht daraus Endenergie $Q_E$, erst daraus entstehen Kosten."),
],
}


def render(items):
    out = ["    NARRATION = ["]
    for key, en, de in items:
        out.append(f'        ("{key}",')
        out.append(f'         r"{en}",')
        out.append(f'         r"{de}"),')
    out.append("    ]")
    return "\n".join(out)


for cls, items in LISTS.items():
    i = src.index(f"class {cls}(Scene):")
    a = src.index("    NARRATION = [", i)
    b = src.index("\n    ]", a) + len("\n    ]")
    src = src[:a] + render(items) + src[b:]
ROOT.write_text(src, encoding="utf-8")
print("ok")
