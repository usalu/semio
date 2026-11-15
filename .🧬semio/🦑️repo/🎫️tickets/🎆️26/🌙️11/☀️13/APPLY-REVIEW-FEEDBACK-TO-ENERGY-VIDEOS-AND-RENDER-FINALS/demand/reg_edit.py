import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\⚖️regulations\🐍️.py")
src = ROOT.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global src
    n = src.count(old)
    if n == 0 or (count is not None and n != count):
        sys.exit(f"MISMATCH {n} != {count}: {old[:80]!r}")
    src = src.replace(old, new)


sub("THETA_E = (0.5, 1.4, 4.9, 9.4, 13.6, 16.8, 18.4, 18.0, 14.2, 9.6, 4.8, 1.6)",
    "THETA_E = (0.75, 1.65, 5.15, 9.65, 13.85, 17.05, 18.65, 18.25, 14.45, 9.85, 5.05, 1.85)")
sub("# test-reference-year shape for a central-German site — the shape, not a\n# certified data row.",
    "# test-reference-year shape for a central-German site — the shape, not a\n# certified data row; it reproduces G_t20/15 = 3500 Kd/a of the running example.")
sub('"""🔋 Monthly utilisation factor η_h = (1 − γᵃ) / (1 − γᵃ⁺¹) with γ = Q_Quelle / Q_Senke (DIN V 18599-2)."""',
    '"""🔋 Monthly utilisation factor η = (1 − γᵃ) / (1 − γᵃ⁺¹) with γ = Q_g / Q_l (DIN V 4108-6, 6.5)."""')
sub('''    """🌡️ Gradtagszahl G_t in Kd/a: the area between the heating limit and the curve, swept up to ``reach`` months."""
    xs = np.linspace(0.0, reach, max(3, int(reach * 40) + 1))
    deficit = np.array([max(0.0, THETA_HG - (_catmull(THETA_E, x) + shift)) for x in xs])''',
    '''    """🌡️ Gradtagszahl G_t20/15 in Kd/a: the sum of θ_i − θ_e over the days below the heating limit, swept up to ``reach`` months."""
    xs = np.linspace(0.0, reach, max(3, int(reach * 40) + 1))
    deficit = np.array([THETA_I - t if (t := _catmull(THETA_E, x) + shift) < THETA_HG else 0.0 for x in xs])''')
sub('''        """🟧 Area between the heating limit and the outdoor curve, filled up to the year cursor."""
        limit_f = _theta_norm(THETA_HG)''',
    '''        """🟧 Area between the indoor temperature θ_i and the outdoor curve on the heating days, filled up to the year cursor."""
        indoor_f = _theta_norm(THETA_I)''')
sub("pts += [chart[\"pt\"](x / 12, limit_f) for x, _t in reversed(run)]", "pts += [chart[\"pt\"](x / 12, indoor_f) for x, _t in reversed(run)]")

sub(r"Q_{\mathrm{Senke}}", r"Q_{l}", None)
sub(r"Q_{\mathrm{Quelle}}", r"Q_{g}", None)
sub(r"\eta_h", r"\eta", None)
sub('''    def _topic(word):
        return VGroup(body_text(word, font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
                      math_label(r"[\\mathrm{kW}]", size=LABEL_FONT_SIZE, color=P_PRIMARY)).arrange(RIGHT, buff=0.1)''',
    '''    def _topic(word, symbol):
        return VGroup(body_text(word, font_size=LABEL_FONT_SIZE, color=P_PRIMARY),
                      math_label(symbol + r"\\ \\mathrm{in\\ kW}", size=LABEL_FONT_SIZE, color=P_PRIMARY)).arrange(RIGHT, buff=0.1)''')
sub('[self._topic(tr("Heating load", "Heizlast"))], "DIN EN 12831-1", LOSS_BLUE, size)',
    '[self._topic(tr("Heating load", "Heizlast"), r"\\dot{Q}_{H}")], "DIN EN 12831-1:2017-09", LOSS_BLUE, size)')
sub('[self._topic(tr("Cooling load", "Kühllast"))], "VDI 2078", P_RED, size)',
    '[self._topic(tr("Cooling load", "Kühllast"), r"\\dot{Q}_{C}")], "VDI 2078:2015-06", P_RED, size)')
sub('"DIN 4108-2", SOLAR_YELLOW, size)', '"DIN 4108-2:2026-05", SOLAR_YELLOW, size)')
sub('tr("GEG — Building Energy Act", "GEG — Gebäudeenergiegesetz")',
    'tr("GEG (since 07/2026 GModG) — Building Energy Act", "GEG (seit 07/2026 GModG) — Gebäudeenergiegesetz")')
sub('tr("The standard series was revised in 2025 as DIN/TS 18599.", "Die Normenreihe wurde 2025 als DIN/TS 18599 überarbeitet.")',
    'tr("The standard series was revised in 10/2025 as DIN/TS 18599.", "Die Normenreihe wurde 10/2025 als DIN/TS 18599 überarbeitet.")')
sub('din = _din_ref("GEG § 20 ff.")', 'din = _din_ref("GEG (seit 7/2026 GModG) § 15 Abs. 1, § 20 Abs. 1")')
sub('din = _din_ref("DIN V 18599-2")', 'din = _din_ref("DIN V 4108-6:2003-06, 5.2 · 6.5 · DIN V 18599-2:2018-09, 5.5.3")')
sub('din = _din_ref("DIN V 18599-10 · DWD TRY")', 'din = _din_ref("DIN V 4108-6:2003-06, Anh. A · D.5 · DWD TRY 2017")')
sub('din = _din_ref("VDI 3807 · DIN 4108-6")', 'din = _din_ref("DIN V 4108-6:2003-06, Anh. D.2 (Heizperiodenbilanz) · D.5")')

sub('chart = _chart(r"\\mathrm{kWh/Monat}", dx=CENTER_DX)', 'chart = _chart(r"Q\\ \\mathrm{in\\ kWh/Monat}", dx=CENTER_DX)')
sub('chart = _chart(r"\\theta_e\\,[\\mathrm{°C}]"', 'chart = _chart(r"\\theta_e\\ \\mathrm{in\\ °C}"', 2)
sub('r"I_S\\,[\\mathrm{kWh/m^2}]"', 'r"I_s\\ \\mathrm{in\\ kWh/m^2}"')
sub('sun_tag = math_label(r"I_S"', 'sun_tag = math_label(r"I_s"')

sub('''        teaser = math_label(r"G_t=\\,?", size=BODY_FONT_SIZE + 4, color=P_ORANGE).move_to(
            np.array([readout_x + 0.9, LEGEND_Y - 0.1, 0.0]))
        self.play(FadeIn(teaser, shift=DOWN * 0.1), run_time=1.4)''',
    '''        teaser = math_label(r"G_{t,20/15}=\\,?", size=BODY_FONT_SIZE + 4, color=P_ORANGE).move_to(
            np.array([readout_x + 0.9, LEGEND_Y - 0.1, 0.0]))
        self.play(FadeIn(teaser, shift=DOWN * 0.1), Create(indoor), FadeIn(indoor_tag), run_time=1.4)''')
sub('''        readout_x = 0.85 + CENTER_DX
''', '''        readout_x = 0.85 + CENTER_DX
        indoor_f = _theta_norm(THETA_I)
        indoor = DashedLine(chart["pt"](0.0, indoor_f), chart["pt"](1.0, indoor_f), color=P_RED, stroke_width=2.4)
        indoor_tag = math_label(r"\\theta_i=20\\,\\mathrm{°C}", size=BODY_FONT_SIZE, color=P_RED)
        indoor_tag.next_to(chart["pt"](1.0, indoor_f), UP, buff=0.08).align_to(chart["pt"](1.0, indoor_f), RIGHT)
''')
sub('        frame = [chart["group"], chart["months"], theta]\n        _enter_house(self, st, 1.4, extra=[*_slide(self, chart, frame, -CENTER_DX), FadeOut(teaser)])',
    '        frame = [chart["group"], chart["months"], theta, indoor, indoor_tag]\n        _enter_house(self, st, 1.4, extra=[*_slide(self, chart, frame, -CENTER_DX), FadeOut(teaser)])')
sub('''gt = math_readout(lambda: rf"G_t={label_number(''', '''gt = math_readout(lambda: rf"G_{{t,20/15}}={label_number(''')
sub('''        row, box, items = math_panel([
            ("q", tr(r"Q_{\\text{temp.-dep.}}", r"Q_{\\text{temp.-abh.}}"), LOSS_BLUE), (None, "~", P_WHITE), ("gt", r"G_t", P_ORANGE),
            (None, r"\\to", P_WHITE), (None, tr(r"\\text{simplified principle of weather correction}", r"\\text{vereinfachtes Prinzip der Witterungsbereinigung}"), P_PRIMARY),
        ], size=FORMULA_FONT_SIZE - 4)''',
    '''        g_year = round(_degree_days(12.0), -2)
        row, box, items = math_panel([
            ("f", r"F_{Gt}", LOSS_BLUE), (None, "=", P_WHITE),
            (None, rf"{label_number(0.024, 3)}\\,\\frac{{\\mathrm{{kKh}}}}{{\\mathrm{{Kd}}}}", P_WHITE), (None, r"\\cdot", P_WHITE),
            ("gt", rf"G_{{t,20/15}}={label_number(g_year)}\\,\\frac{{\\mathrm{{Kd}}}}{{\\mathrm{{a}}}}", P_ORANGE), (None, "=", P_WHITE),
            (None, rf"{label_number(0.024 * g_year)}\\,\\frac{{\\mathrm{{kKh}}}}{{\\mathrm{{a}}}}", P_WHITE),
            (None, r"\\quad Q_{T}=F_{Gt}\\cdot H_{T}", P_PRIMARY),
        ], size=FORMULA_FONT_SIZE - 6)''')

LISTS = {
"Beat1_GEGundNormen": [
 ("intro", "Everything we just balanced is not a private calculation — in Germany it is prescribed by law.",
  "Alles, was wir gerade bilanziert haben, ist keine private Rechnung — in Deutschland ist sie gesetzlich vorgeschrieben."),
 ("geg", "The Gebäudeenergiegesetz — continued since July 2026 as the Gebäudemodernisierungsgesetz — sets the target.",
  "Das Gebäudeenergiegesetz — seit Juli 2026 als Gebäudemodernisierungsgesetz fortgeführt — setzt das Ziel."),
 ("grenze", r"It caps a building's primary energy demand $Q_P$.", r"Es begrenzt den Primärenergiebedarf $Q_P$ eines Gebäudes."),
 ("offen", "But it never says how to compute it.", "Wie man ihn berechnet, sagt es aber nicht."),
 ("din", "For that it points to DIN V 18599:2018-09.", "Dafür verweist es auf DIN V 18599:2018-09."),
 ("methode", "That is the calculation method the law references; a revision followed in October 2025.",
  "Das ist die vom Gesetz referenzierte Berechnungsmethodik; im Oktober 2025 folgte eine Überarbeitung."),
 ("nachbarn", "Beside it sit the standards from the earlier videos.", "Daneben stehen die Normen der früheren Videos."),
 ("heizlast", r"The heating load $\dot{Q}_H$ is calculated to DIN EN 12831-1 — which calls it $\Phi_{HL}$.",
  r"Die Heizlast $\dot{Q}_H$ wird nach DIN EN 12831-1 berechnet — dort heißt sie $\Phi_{HL}$."),
 ("kuehllast", r"The cooling load $\dot{Q}_C$ is calculated to VDI 2078.", r"Die Kühllast $\dot{Q}_C$ wird nach VDI 2078 berechnet."),
 ("sommer", "And summer overheating protection is verified to DIN 4108-2.", "Und der sommerliche Wärmeschutz wird nach DIN 4108-2 nachgewiesen."),
],
"Beat2_Monatsbilanz": [
 ("intro", "The standard balances month by month.", "Die Norm bilanziert Monat für Monat."),
 ("warum", "Here is why an annual average would be wrong.", "Hier ist der Grund, warum ein Jahresmittel falsch wäre."),
 ("verluste", r"These are the monthly losses $Q_l$.", r"Das sind die monatlichen Verluste $Q_l$."),
 ("winter", "They are high in winter and small in summer, following the outdoor temperature.",
  "Im Winter sind sie hoch, im Sommer klein — sie folgen der Außentemperatur."),
 ("gewinne", r"First the available gains $Q_g$ from sun and internal loads.", r"Zuerst die verfügbaren Gewinne $Q_g$ aus Sonne und internen Lasten."),
 ("nutzbar", r"Only part of them is usable — the utilisation factor $\eta$ says how much.",
  r"Nur ein Teil davon ist nutzbar — der Ausnutzungsgrad $\eta$ sagt, wie viel."),
 ("deckt", "Where green covers blue, sun and internal loads already pay the bill.",
  "Wo Grün das Blau deckt, zahlen Sonne und interne Lasten schon die Rechnung."),
 ("rest", r"Only the red remainder $Q_h$ has to be heated.", r"Nur der rote Rest $Q_h$ muss beheizt werden."),
 ("sommer", "In summer there is no heating remainder.", "Im Sommer bleibt kein Heizrest."),
 ("ungenutzt", "Those unused gains sit in the cooling balance instead of paying January.",
  "Ungenutzte Gewinne gehören in die Kühlbilanz, nicht in den Januar."),
 ("jahr", "An annual average would credit that summer surplus against January.",
  "Ein Jahresmittel würde diesen Sommerüberschuss dem Januar gutschreiben."),
 ("klein", "The demand would then look far too small.", "Der Bedarf erschiene dann viel zu klein."),
 ("summe", "So the annual heating demand is the sum of twelve separate monthly balances, never one averaged one.",
  "Der Jahresheizwärmebedarf ist deshalb die Summe von zwölf getrennten Monatsbilanzen, nie eine gemittelte."),
 ("formel", r"In formula form: for each month the losses $Q_l$ minus the usable gains $\eta \cdot Q_g$, added up.",
  r"Als Formel: je Monat Verluste $Q_l$ minus nutzbare Gewinne $\eta \cdot Q_g$, aufsummiert."),
],
"Beat3_Klimadaten": [
 ("intro", "Every one of those twelve balances needs two inputs from outside the building: temperature and sun.",
  "Jede dieser zwölf Bilanzen braucht zwei Eingangsgrößen von außen: Temperatur und Sonne."),
 ("temperatur", r"The monthly mean outdoor temperature $\theta_e$ drives the losses.",
  r"Die mittlere monatliche Außentemperatur $\theta_e$ treibt die Verluste."),
 ("delta", r"It enters every transmission term through the temperature difference $\Delta\theta = \theta_i - \theta_e$ to the indoor temperature $\theta_i$.",
  r"Sie geht über die Temperaturdifferenz $\Delta\theta = \theta_i - \theta_e$ zur Innentemperatur $\theta_i$ in jeden Transmissionsterm ein."),
 ("strahlung", r"The monthly irradiation $I_s$ per orientation drives the solar gains.",
  r"Die monatliche Einstrahlung $I_s$ je Orientierung treibt die solaren Gewinne."),
 ("orientierung", "That is why a south window counts more than a north one.", "Deshalb zählt ein Südfenster mehr als ein Nordfenster."),
 ("try", "Both come from the German weather service as a test reference year.", "Beides liefert der Deutsche Wetterdienst als Testreferenzjahr."),
 ("zwoelf", "That is twelve synthetic but representative months.", "Das sind zwölf synthetische, aber repräsentative Monate."),
 ("geg", "For certain proofs under the building energy law a standardised reference climate is fixed.",
  "Für bestimmte Nachweise nach dem Gebäudeenergiegesetz ist ein standardisiertes Referenzklima festgelegt."),
 ("potsdam", "The Potsdam reference climate zone plays the central role in it.", "Die Referenzklimazone Potsdam spielt dabei die zentrale Rolle."),
],
"Beat4_Gradtagszahl": [
 ("intro", "The weather year is a whole curve of twelve monthly values.", "Das Wetterjahr ist eine ganze Kurve aus zwölf Monatswerten."),
 ("kennzahl", r"One number compresses it: the degree-day figure $G_t$ — always stated for an indoor temperature $\theta_i$.",
  r"Eine Zahl verdichtet sie: die Gradtagszahl $G_t$ — stets bezogen auf eine Innentemperatur $\theta_i$."),
 ("heizgrenze", r"Above the heating limit $\theta_{HG} = 15\,^{\circ}\mathrm{C}$ the free gains cover the losses.",
  r"Oberhalb der Heizgrenze $\theta_{HG} = 15\,^{\circ}\mathrm{C}$ decken die freien Gewinne die Verluste."),
 ("konvention", "That is a calculation convention, not a physical limit.", "Das ist eine Rechenkonvention, keine physikalische Grenze."),
 ("flaeche", r"Below it, on every heating day the difference from the indoor temperature $\theta_i = 20\,^{\circ}\mathrm{C}$ counts.",
  r"Darunter zählt an jedem Heiztag die Differenz zur Innentemperatur $\theta_i = 20\,^{\circ}\mathrm{C}$."),
 ("prinzip", r"The area between $\theta_i$ and the outdoor curve shows the principle: $G_{t,20/15}$ for $\theta_i = 20$ and $\theta_{HG} = 15$.",
  r"Die Fläche zwischen $\theta_i$ und der Außenkurve zeigt das Prinzip: $G_{t,20/15}$ für $\theta_i = 20$ und $\theta_{HG} = 15$."),
 ("tageswerte", r"The real figure sums the daily differences $\theta_i - \theta_e$ over the heating days.",
  r"Real wird die Gradtagszahl als Summe der Tagesdifferenzen $\theta_i - \theta_e$ über die Heiztage berechnet."),
 ("kalt", "A cold winter gives a large degree-day number.", "Ein kalter Winter ergibt eine große Gradtagszahl."),
 ("mild", "A mild winter gives a small one.", "Ein milder Winter ergibt eine kleine."),
 ("nutzen", r"It makes winters comparable, and $F_{Gt} = 0.024 \cdot G_t$ gives the transmission loss $Q_T = F_{Gt} \cdot H_T$.",
  r"Sie macht Winter vergleichbar, und $F_{Gt} = 0{,}024 \cdot G_t$ liefert den Transmissionsverlust $Q_T = F_{Gt} \cdot H_T$."),
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
