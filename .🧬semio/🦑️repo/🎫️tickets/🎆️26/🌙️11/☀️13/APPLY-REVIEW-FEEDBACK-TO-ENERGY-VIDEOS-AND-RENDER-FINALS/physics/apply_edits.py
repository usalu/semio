import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = ROOT.read_text(encoding="utf-8")
failures = []


def sub(old, new, count=1):
    global text
    found = text.count(old)
    if found != count:
        failures.append((found, count, old[:90]))
        return
    text = text.replace(old, new)


# ---- Beat1 narration and symbols
sub('"Power tells us how quickly energy is transferred. Its symbol is P and its unit is the watt."',
    r'"Power tells us how quickly energy is transferred. Its symbol is $P$ and its unit is the watt."')
sub('"Leistung sagt, wie schnell Energie übertragen wird. Ihr Formelzeichen ist P, ihre Einheit Watt."',
    r'"Leistung sagt, wie schnell Energie übertragen wird. Ihr Formelzeichen ist $P$, ihre Einheit Watt."')
sub('"So energy is power times time, and power is energy divided by time."',
    r'"So energy is power times time, $E = P \cdot t$, and power is energy divided by time."')
sub('"Also: Energie ist Leistung mal Zeit, Leistung ist Energie durch Zeit."',
    r'"Also: Energie ist Leistung mal Zeit, $E = P \cdot t$, und Leistung ist Energie durch Zeit."')
sub('"The design heating load is determined for defined design conditions and is not the annual energy use."',
    r'"The design heating load $\dot{Q}_{\mathrm{H}}$ is determined for defined design conditions and is not the annual energy use."')
sub('"Die Auslegungsheizlast wird für definierte Auslegungsbedingungen bestimmt und ist nicht der Jahresenergieverbrauch."',
    r'"Die Auslegungsheizlast $\dot{Q}_{\mathrm{H}}$ wird für definierte Auslegungsbedingungen bestimmt und ist nicht der Jahresenergieverbrauch."')
sub('"The annual heating demand in kilowatt-hours is the area under the power curve over the year."',
    r'"The annual heating demand $Q_{\mathrm{h}}$ in kilowatt-hours is the area under the power curve over the year."')
sub('"Der Jahres-Heizwärmebedarf in kWh ist die Fläche unter der Leistungskurve übers Jahr."',
    r'"Der Jahres-Heizwärmebedarf $Q_{\mathrm{h}}$ in kWh ist die Fläche unter der Leistungskurve übers Jahr."')
sub(r'_m(r"P\;[\mathrm{W}]", size=FORMULA_FONT_SIZE, color=P_CYAN)', r'_m(r"P\text{ in }\mathrm{W}", size=FORMULA_FONT_SIZE, color=P_CYAN)')
sub(r'_axes(O, 5.1, 2.95, r"t\;[\mathrm{h}]", r"P\;[\mathrm{kW}]")', r'_axes(O, 5.1, 2.95, r"t\text{ in }\mathrm{h}", r"P\text{ in }\mathrm{kW}")')
sub(r'new_xl = _m(r"t\;[\mathrm{s}]", size=LABEL_FONT_SIZE)', r'new_xl = _m(r"t\text{ in }\mathrm{s}", size=LABEL_FONT_SIZE)')
sub(r'new_yl = _m(r"P\;[\mathrm{W}]", size=LABEL_FONT_SIZE)', r'new_yl = _m(r"P\text{ in }\mathrm{W}", size=LABEL_FONT_SIZE)')
sub(r'_axes(YO, 10.7, 3.0, "t", r"P\;[\mathrm{kW}]")', r'_axes(YO, 10.7, 3.0, "t", r"\dot{Q}_{\mathrm{H}}\text{ in }\mathrm{kW}")')
sub(r'tr(r"\text{Heating power}\; P \;\text{over the year (schematic)}", r"\text{Heizleistung}\; P \;\text{über das Jahr (schematisch)}")',
    r'tr(r"\text{Heating load}\; \dot{Q}_{\mathrm{H}} \;\text{over the year (schematic)}", r"\text{Heizlast}\; \dot{Q}_{\mathrm{H}} \;\text{über das Jahr (schematisch)}")')
sub(r'tr(r"\text{Design heating load}\; P_{\mathrm{HL}} \approx 8\,\mathrm{kW}", r"\text{Auslegungsheizlast}\; P_{\mathrm{HL}} \approx 8\,\mathrm{kW}")',
    r'tr(r"\text{Design heating load}\; \dot{Q}_{\mathrm{H}} \approx 8\,\mathrm{kW}", r"\text{Auslegungsheizlast}\; \dot{Q}_{\mathrm{H}} \approx 8\,\mathrm{kW}")')
sub(r'rf"Q = {label_number(cum[int(np.clip(sweep.get_value(), 0, 365))])}\,\mathrm{{kWh}}"', r'rf"Q_{{\mathrm{{h}}}} = {label_number(cum[int(np.clip(sweep.get_value(), 0, 365))])}\,\mathrm{{kWh}}"')
sub(r'tr(r"\text{Heating demand} \approx 15\,000\,\mathrm{kWh/a}", r"\text{Heizwärmebedarf} \approx 15\,000\,\mathrm{kWh/a}")',
    r'tr(r"\text{Heating demand}\; Q_{\mathrm{h}} \approx 15\,000\,\mathrm{kWh/a}", r"\text{Heizwärmebedarf}\; Q_{\mathrm{h}} \approx 15\,000\,\mathrm{kWh/a}")')
sub(r'tr(r"\text{Kraftwerksblock} \approx 1.4\,\mathrm{GW}", r"\text{Kraftwerksblock} \approx 1{,}4\,\mathrm{GW}")',
    r'tr(r"\text{Power-station unit} \approx 1.4\,\mathrm{GW}", r"\text{Kraftwerksblock} \approx 1{,}4\,\mathrm{GW}")')
sub(r'tr(r"\text{all power plants in the world} \approx 9\,\mathrm{TW}", r"\text{alle Kraftwerke der Welt} \approx 9\,\mathrm{TW}")',
    r'tr(r"\text{all power plants in the world} \approx 9\,\mathrm{TW}", r"\text{alle Kraftwerke der Welt} \approx 9\,\mathrm{TW}")')

# ---- Beat2
sub('"Work is transferred energy: at constant power, E equals P times t. One kilowatt for one hour is one kilowatt-hour."',
    r'"Work is transferred energy: at constant power, $E = P \cdot t$. One kilowatt for one hour is one kilowatt-hour."')
sub('"Arbeit ist übertragene Energie: Bei konstanter Leistung gilt E = P · t. 1 kW über 1 h ergibt 1 kWh."',
    r'"Arbeit ist übertragene Energie: Bei konstanter Leistung gilt $E = P \cdot t$. 1 kW über 1 h ergibt 1 kWh."')
sub(r'tr(r"\text{Deutschland pro Jahr, Endenergie} \approx 2.4 \cdot 10^{12}\,\mathrm{kWh}", r"\text{Deutschland pro Jahr, Endenergie} \approx 2{,}4 \cdot 10^{12}\,\mathrm{kWh}")',
    r'tr(r"\text{Germany per year, final energy} \approx 2.4 \cdot 10^{12}\,\mathrm{kWh}", r"\text{Deutschland pro Jahr, Endenergie} \approx 2{,}4 \cdot 10^{12}\,\mathrm{kWh}")')
sub(r'tr(r"\text{Welt pro Jahr, Primärenergie} \approx 1.7 \cdot 10^{14}\,\mathrm{kWh}", r"\text{Welt pro Jahr, Primärenergie} \approx 1{,}7 \cdot 10^{14}\,\mathrm{kWh}")',
    r'tr(r"\text{World per year, primary energy} \approx 1.7 \cdot 10^{14}\,\mathrm{kWh}", r"\text{Welt pro Jahr, Primärenergie} \approx 1{,}7 \cdot 10^{14}\,\mathrm{kWh}")')

# ---- Beat3
sub(r'("in", r"E_{\mathrm{ein}}", P_CYAN), (None, "=", P_WHITE), ("out", r"E_{\mathrm{aus}}", P_RED),',
    r'("in", tr(r"E_{\mathrm{in}}", r"E_{\mathrm{ein}}"), P_CYAN), (None, "=", P_WHITE), ("out", tr(r"E_{\mathrm{out}}", r"E_{\mathrm{aus}}"), P_RED),')
sub(r'("st", r"\Delta E_{\mathrm{Speicher}}", P_ORANGE),', r'("st", tr(r"\Delta E_{\mathrm{stored}}", r"\Delta E_{\mathrm{Speicher}}"), P_ORANGE),')
sub(r'("l", r"P_{\mathrm{Licht}}", P_YELLOW),', r'("l", tr(r"P_{\mathrm{light}}", r"P_{\mathrm{Licht}}"), P_YELLOW),')
sub(r'("w", r"P_{\mathrm{Wärme}}", P_RED),', r'("w", tr(r"P_{\mathrm{heat}}", r"P_{\mathrm{Wärme}}"), P_RED),')
sub(r'(None, "=", P_WHITE), (None, r"P_{\mathrm{Wärme}}", P_RED),', r'(None, "=", P_WHITE), (None, tr(r"P_{\mathrm{heat}}", r"P_{\mathrm{Wärme}}"), P_RED),')

# ---- Beat4 narration
sub('''"The balance: one kilowatt-hour of electricity plus three of ambient heat gives four of heating heat."''',
    r'''"The balance: one kilowatt-hour of electricity, the final energy $Q_{\mathrm{E}}$, plus three of ambient heat $Q_{\mathrm{U}}$ gives four of useful heat $Q_{\mathrm{N}}$."''')
sub('''"Die Bilanz: 1 kWh Strom plus 3 kWh Umweltwärme ergeben 4 kWh Heizwärme."''',
    r'''"Die Bilanz: 1 kWh Strom, die Endenergie $Q_{\mathrm{E}}$, plus 3 kWh Umweltwärme $Q_{\mathrm{U}}$ ergeben 4 kWh Nutzwärme $Q_{\mathrm{N}}$."''')
sub('"Heat divided by electricity is the coefficient of performance, COP — here four."',
    '"Useful heat divided by final energy is the coefficient of performance, COP — here four."')
sub('"Heizwärme durch Strom ist die Leistungszahl COP — hier 4."',
    '"Nutzwärme durch Endenergie ist die Leistungszahl COP — hier 4."')
sub('''         "Averaged over a year this is the seasonal performance factor JAZ — for air-source units often around three.",
         "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl JAZ — bei Luft-Wärmepumpen oft rund 3."),''',
    '''         "Averaged over a year this is the seasonal performance factor, in German the Jahresarbeitszahl JAZ — for air-source units often around three.",
         "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl, kurz JAZ — bei Luft-Wärmepumpen oft rund 3."),
        ("jaz_ref",
         "You find it as seasonal performance factor in the guideline VDI 4650 part 1; the European label calls it SCOP, DIN EN 14825.",
         "Du findest sie als Jahresarbeitszahl in der Richtlinie VDI 4650 Blatt 1; das europäische Label nennt sie SCOP, DIN EN 14825."),''')
sub('''         "Final energy is what is delivered to the building and paid for: fuel in the tank, electricity at the meter.",
         "Endenergie ist, was ans Haus geliefert und bezahlt wird: Brennstoff im Tank, Strom am Zähler."),''',
    '''         "Final energy is what is delivered to the building and paid for: fuel in the tank, electricity at the meter.",
         "Endenergie ist, was ans Haus geliefert und bezahlt wird: Brennstoff im Tank, Strom am Zähler."),
        ("useful",
         "What actually warms the rooms is useful heat, the useful energy. Final energy and useful heat are not the same.",
         "Was die Räume tatsächlich erwärmt, ist die Nutzwärme, die Nutzenergie. Endenergie und Nutzwärme sind nicht dasselbe."),''')
sub('"Per kilowatt-hour of final energy: wood chips give about 0.85 kilowatt-hours of heat, oil about 0.94, gas about 0.96."',
    r'"Per kilowatt-hour of final energy $Q_{\mathrm{E}}$ the building gets useful heat $Q_{\mathrm{N}}$: wood chips about 0.85 kilowatt-hours, oil about 0.94, gas about 0.96."')
sub('"Je 1 kWh Endenergie liefern Holzhackschnitzel rund 0,85 kWh Wärme, Heizöl rund 0,94, Erdgas rund 0,96."',
    r'"Je 1 kWh Endenergie $Q_{\mathrm{E}}$ erhält das Gebäude Nutzwärme $Q_{\mathrm{N}}$: Holzhackschnitzel rund 0,85 kWh, Heizöl rund 0,94 kWh, Erdgas rund 0,96 kWh."')
sub('"Electric heater: exactly one at the house. Fuels lose the rest with flue gas; electricity may cause flue gas elsewhere."',
    '"Electric heater: exactly one kilowatt-hour of useful heat at the house. Fuels lose the rest with flue gas; electricity may cause flue gas elsewhere."')
sub('"Elektroheizstab: am Haus genau 1. Brennstoffe verlieren den Rest mit dem Abgas; Strom kann anderswo Abgas verursachen."',
    '"Elektroheizstab: am Haus genau 1 kWh Nutzwärme. Brennstoffe verlieren den Rest mit dem Abgas; Strom kann anderswo Abgas verursachen."')
sub('"The air-source heat pump gives about three, the ground-source about four — because it adds ambient heat."',
    '"The air-source heat pump gives about three kilowatt-hours of useful heat, the ground-source about four — because it adds ambient heat."')
sub('"Die Luft-Wärmepumpe liefert rund 3, die Erd-Wärmepumpe rund 4 — weil sie Umweltwärme dazuholt."',
    '"Die Luft-Wärmepumpe liefert rund 3 kWh Nutzwärme, die Erd-Wärmepumpe rund 4 kWh — weil sie Umweltwärme dazuholt."')

# ---- Beat4 visuals
sub(r'''w_lbl = _m(r"W_{\mathrm{el}} = 1\,\mathrm{kWh}", np.array([stack_x - 0.42, base + unit / 2 - 0.07, 0.0]),''',
    r'''w_lbl = _m(tr(r"\text{Final energy}\; Q_{\mathrm{E}} = 1\,\mathrm{kWh}", r"\text{Endenergie}\; Q_{\mathrm{E}} = 1\,\mathrm{kWh}"), np.array([stack_x - 0.42, base + unit / 2 - 0.07, 0.0]),''')
sub(r'''u_lbl = _m(r"Q_{\mathrm{U}} = 3\,\mathrm{kWh}", np.array([stack_x - 0.42, base + 2.5 * unit - 0.07, 0.0]),''',
    r'''u_lbl = _m(tr(r"\text{Ambient heat}\; Q_{\mathrm{U}} = 3\,\mathrm{kWh}", r"\text{Umweltwärme}\; Q_{\mathrm{U}} = 3\,\mathrm{kWh}"), np.array([stack_x - 0.42, base + 2.5 * unit - 0.07, 0.0]),''')
sub(r'''h_lbl = _m(r"Q_{\mathrm{H}} = 4\,\mathrm{kWh}", np.array([stack_x - 0.42, h_frame.get_top()[1] + 0.05, 0.0]),''',
    r'''h_lbl = _m(tr(r"\text{Useful heat}\; Q_{\mathrm{N}} = 4\,\mathrm{kWh}", r"\text{Nutzwärme}\; Q_{\mathrm{N}} = 4\,\mathrm{kWh}"), np.array([stack_x - 0.42, h_frame.get_top()[1] + 0.05, 0.0]),''')
sub(r'''\frac{\textcolor{semio_primary}{Q_{\mathrm{H}}}}{\textcolor{semio_secondary}{W_{\mathrm{el}}}}''',
    r'''\frac{\textcolor{semio_primary}{Q_{\mathrm{N}}}}{\textcolor{semio_secondary}{Q_{\mathrm{E}}}}''')
sub(r'''w_live = _readout(lambda: rf"W_{{\mathrm{{el}}}} = {label_number(4.0 / cop_of(t_out.get_value()), 2)}\,\mathrm{{kWh}}",''',
    r'''w_live = _readout(lambda: rf"Q_{{\mathrm{{E}}}} = {label_number(4.0 / cop_of(t_out.get_value()), 2)}\,\mathrm{{kWh}}",''')
sub('''        jaz = _m(tr(r"\\mathrm{SPF} \\approx 3 \\quad \\text{(Bohne 2022)}", r"\\mathrm{JAZ} \\approx 3 \\quad \\text{(Bohne 2022)}"), np.array([stack_x - 0.9, 1.95, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_PRIMARY)
        self.play(FadeIn(jaz, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, N, "jaz")
''',
    '''        jaz = _m(tr(r"\\text{Seasonal performance factor (JAZ)}\\; \\mathrm{SPF} \\approx 3", r"\\text{Jahresarbeitszahl (JAZ)}\\; \\mathrm{JAZ} \\approx 3"), np.array([6.9, 2.0, 0.0]),
                 size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")
        self.play(FadeIn(jaz, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, N, "jaz")

        caption = swap_caption(self, caption, subtitle_text(N, "jaz_ref"))
        jaz_ref = _m(tr(r"\\text{VDI 4650 part 1 · DIN EN 14825 (SCOP)}", r"\\text{VDI 4650 Blatt 1 · DIN EN 14825 (SCOP)}"), np.array([6.9, 1.62, 0.0]),
                     size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")
        self.play(FadeIn(jaz_ref, shift=DOWN * 0.1), run_time=0.7)
        hold_for(self, N, "jaz_ref")
''')
sub('''fe_lbl = Text(tr("Final energy", "Endenergie"), font_size=BODY_FONT_SIZE, color=P_YELLOW).next_to(fe_brace, UP, buff=0.12)''',
    '''fe_lbl = _m(tr(r"\\text{Final energy}\\; Q_{\\mathrm{E}}", r"\\text{Endenergie}\\; Q_{\\mathrm{E}}"), color=P_YELLOW).next_to(fe_brace, UP, buff=0.12)''')
sub('''fe_heat_lbl = Text(tr("Useful heat", "Nutzwärme"), font_size=LABEL_FONT_SIZE, color=P_RED).move_to(np.array([1.8, 0.45, 0.0]))''',
    '''fe_heat_lbl = _m(tr(r"\\text{Useful heat}\\; Q_{\\mathrm{N}}", r"\\text{Nutzwärme}\\; Q_{\\mathrm{N}}"), np.array([1.8, 0.45, 0.0]), size=LABEL_FONT_SIZE, color=P_RED)''')
sub('''        hold_for(self, N, "final", during=lambda rt: [
            _ripples([fe_heater.get_top() + UP * 0.04], r_max=0.7, color=P_RED, cycles=max(1.0, rt / 1.4))])
''',
    '''        hold_for(self, N, "final")

        caption = swap_caption(self, caption, subtitle_text(N, "useful"))
        self.play(Indicate(fe_heat_lbl, color=P_RED), Indicate(fe_lbl, color=P_YELLOW), run_time=1.2)
        hold_for(self, N, "useful", during=lambda rt: [
            _ripples([fe_heater.get_top() + UP * 0.04], r_max=0.7, color=P_RED, cycles=max(1.0, rt / 1.4))])
''')
sub('''(tr("Air HP\\n(SPF)", "Luft-WP\\n(JAZ)"), 3.0, "hp"), (tr("Ground HP\\n(SPF)", "Erd-WP\\n(JAZ)"), 4.0, "hp"),''',
    '''(tr("Air HP\\n(SPF)", "Luft-WP\\n(JAZ)"), 3.0, "hp"), (tr("Ground HP\\n(SPF)", "Erd-WP\\n(JAZ)"), 4.0, "hp"),''')
sub('''one_lbl = _m(r"1\\,\\mathrm{kWh}", np.array([-6.2, cb + cu - 0.06, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE,''',
    '''one_lbl = _m(r"Q_{\\mathrm{E}} = 1\\,\\mathrm{kWh}", np.array([-6.9, cb + cu - 0.06, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE,''')
sub('''axis_lbl = Text(tr("Useful heat from 1 kWh final energy", "Nutzwärme aus 1 kWh Endenergie"), font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([-3.0, 2.15, 0.0]))''',
    '''axis_lbl = _m(tr(r"\\text{Useful heat}\\; Q_{\\mathrm{N}}\\; \\text{from}\\; Q_{\\mathrm{E}} = 1\\,\\mathrm{kWh}\\; \\text{final energy}", r"\\text{Nutzwärme}\\; Q_{\\mathrm{N}}\\; \\text{aus}\\; Q_{\\mathrm{E}} = 1\\,\\mathrm{kWh}\\; \\text{Endenergie}"),
                      np.array([-3.5, 2.15, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY)''')
sub('''for t, c in ((tr("Fuel", "Brennstoff"), P_ORANGE), (tr("Electricity", "Strom"), P_CYAN), (tr("Ambient heat", "Umweltwärme"), P_GREEN))''',
    '''for t, c in ((tr("Fuel (final energy)", "Brennstoff (Endenergie)"), P_ORANGE), (tr("Electricity (final energy)", "Strom (Endenergie)"), P_CYAN), (tr("Ambient heat", "Umweltwärme"), P_GREEN))''')

# ---- Beat5
sub('''"Solar gain is g times window area times irradiance."''', r'''"The solar heat flow is the g-value times window area $A$ times irradiance $I$."''')
sub('''"Solarer Gewinn: g mal Fensterfläche mal Einstrahlung."''', r'''"Der solare Wärmestrom ist der g-Wert mal Fensterfläche $A$ mal Einstrahlung $I$."''')
sub(r'''i_lbl = _m(r"I = 500\,\mathrm{W/m^{2}}", sun_c''', r'''i_lbl = _m(r"I = 500\,\frac{\mathrm{W}}{\mathrm{m^{2}}}", sun_c''')
sub('''g_lbl = _m(tr(r"g = 0.50", r"g = 0{,}50")''', '''g_lbl = _m(tr(r"g_{\\perp} = 0.50", r"g_{\\perp} = 0{,}50")''')
sub(r'''gi_lbl = _m(r"g \cdot I \approx 250\,\mathrm{W/m^{2}}"''', r'''gi_lbl = _m(r"g_{\perp} \cdot I \approx 250\,\frac{\mathrm{W}}{\mathrm{m^{2}}}"''')
sub('''norm_tag = Text("Willems et al. 2024", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).next_to(g_lbl, DOWN, buff=0.12).shift(RIGHT * 0.5)''',
    '''norm_tag = _m(tr(r"\\text{DIN EN 410: } g_{\\perp} \\text{ of glazing}", r"\\text{DIN EN 410: } g_{\\perp} \\text{ der Verglasung}"), np.array([6.9, 1.45, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")''')
sub('''(None, r"\\dot{Q}_{\\mathrm{S}}", P_ORANGE), (None, "=", P_WHITE), ("g", "g", P_YELLOW),''',
    '''(None, r"\\dot{Q}_{\\mathrm{S}}", P_ORANGE), (None, "=", P_WHITE), ("g", r"g_{\\perp}", P_YELLOW),''')
sub('''("iv", r"500\\,\\mathrm{W/m^{2}}", P_YELLOW),''', '''("iv", r"500\\,\\frac{\\mathrm{W}}{\\mathrm{m^{2}}}", P_YELLOW),''')
sub(r'''rf"\vartheta_{{\mathrm{{Raum}}}} = {label_number(t_room.get_value())}\,\mathrm{{°C}}"''',
    r'''rf"\theta_{{\mathrm{{i}}}} = {label_number(t_room.get_value())}\,\mathrm{{°C}}"''')
sub('''norm2 = Text("Willems et al. 2024", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(np.array([-3.7, -1.4, 0.0]))''',
    '''norm2 = _m(tr(r"\\text{DIN 4108-2: sun protection, reduction factor } F_{\\mathrm{C}}", r"\\text{DIN 4108-2: Sonnenschutz, Abminderungsfaktor } F_{\\mathrm{C}}"), np.array([-6.9, -1.9, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="left")''')

# ---- Beat6
sub(r'''dt_read = _readout(lambda: rf"\Delta T = ''', r'''dt_read = _readout(lambda: rf"\Delta\theta = ''')
sub('''("dt", r"\\Delta T", P_ORANGE),''', '''("dt", r"\\Delta\\theta", P_ORANGE),''')
sub(r'''m_line = _m(tr(r"m = \rho \cdot V = 2\,400\,\mathrm{kg/m^{3}} \cdot 0.1\,\mathrm{m^{3}} = 240\,\mathrm{kg}", r"m = \rho \cdot V = 2\,400\,\mathrm{kg/m^{3}} \cdot 0{,}1\,\mathrm{m^{3}} = 240\,\mathrm{kg}"),''',
    r'''m_line = _m(tr(r"m = \rho \cdot V = 2\,400\,\frac{\mathrm{kg}}{\mathrm{m^{3}}} \cdot 0.1\,\mathrm{m^{3}} = 240\,\mathrm{kg}", r"m = \rho \cdot V = 2\,400\,\frac{\mathrm{kg}}{\mathrm{m^{3}}} \cdot 0{,}1\,\mathrm{m^{3}} = 240\,\mathrm{kg}"),''')
sub(r'''lambda: (rf"Q = 240\,\mathrm{{kg}} \cdot 1\,000\,\mathrm{{J/(kg\,K)}} \cdot {label_number(dT.get_value(), 1)}\,"''',
    r'''lambda: (rf"Q = 240\,\mathrm{{kg}} \cdot 1\,000\,\frac{{\mathrm{{J}}}}{{\mathrm{{kg\,K}}}} \cdot {label_number(dT.get_value(), 1)}\,"''')
sub(r'''cax = _axes(CO, 7.15, 3.05, r"t\;[\mathrm{h}]", r"\vartheta\;[\mathrm{°C}]")''', r'''cax = _axes(CO, 7.15, 3.05, r"t\text{ in }\mathrm{h}", r"\theta\text{ in }\mathrm{°C}")''')
sub(r'''phi_read = _readout(lambda: rf"\varphi \approx {label_number(phi.get_value())}\,\mathrm{{h}}", cpt(18, 33.9),''',
    r'''phi_read = _readout(lambda: tr(rf"\text{{phase shift}}\; \Delta t \approx {label_number(phi.get_value())}\,\mathrm{{h}}", rf"\text{{Phasenverschiebung}}\; \Delta t \approx {label_number(phi.get_value())}\,\mathrm{{h}}"), cpt(18, 33.9),''')
sub('''norm = Text("Willems et al. 2024", font_size=LABEL_FONT_SIZE, color=P_PRIMARY).move_to(
            np.array([3.9, -1.35, 0.0]))''',
    '''norm = _m(tr(r"\\text{DIN EN ISO 13786: dynamic thermal behaviour (heat storage)}", r"\\text{DIN EN ISO 13786: dynamisch-thermisches Verhalten (Wärmespeicherung)}"),
                  np.array([6.9, -1.75, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")''')
sub(r'''rf"\vartheta_{{\mathrm{{Decke}}}} = {label_number(t_ceil.get_value())}\,\mathrm{{°C}}"''',
    r'''tr(rf"\theta_{{\mathrm{{ceiling}}}} = {label_number(t_ceil.get_value())}\,\mathrm{{°C}}", rf"\theta_{{\mathrm{{Decke}}}} = {label_number(t_ceil.get_value())}\,\mathrm{{°C}}")''')

# ---- Beat7
sub(r'''qax = _axes(QO, 7.4, 3.15, r"Q\;[\mathrm{kJ}]", r"T\;[\mathrm{°C}]")''', r'''qax = _axes(QO, 7.4, 3.15, r"Q\text{ in }\mathrm{kJ}", r"\theta\text{ in }\mathrm{°C}")''')
sub(r'''rf"T = {label_number(temp_of(Q.get_value()))}\,\mathrm{{°C}}"''', r'''rf"\theta = {label_number(temp_of(Q.get_value()))}\,\mathrm{{°C}}"''')
sub('''(None, "c", P_WHITE), (None, r"\\cdot", P_WHITE), (None, r"\\Delta T", P_RED), (None, "=", P_WHITE),''',
    '''(None, "c", P_WHITE), (None, r"\\cdot", P_WHITE), (None, r"\\Delta\\theta", P_RED), (None, "=", P_WHITE),''')
sub(r'''tr(r"1\,\mathrm{kg} \cdot 4.19\,\mathrm{kJ/(kg\,K)} \cdot 100\,\mathrm{K}", r"1\,\mathrm{kg} \cdot 4{,}19\,\mathrm{kJ/(kg\,K)} \cdot 100\,\mathrm{K}")''',
    r'''tr(r"1\,\mathrm{kg} \cdot 4.19\,\frac{\mathrm{kJ}}{\mathrm{kg\,K}} \cdot 100\,\mathrm{K}", r"1\,\mathrm{kg} \cdot 4{,}19\,\frac{\mathrm{kJ}}{\mathrm{kg\,K}} \cdot 100\,\mathrm{K}")''')
sub(r'''rf"\frac{{2\,257}}{{419}} \approx {label_number(ratio.get_value(), 1)}"''', r'''rf"\frac{{2\,257\,\mathrm{{kJ}}}}{{419\,\mathrm{{kJ}}}} \approx {label_number(ratio.get_value(), 1)}"''')
sub(r'''(None, r"1\,\mathrm{kg} \cdot 2\,257\,\mathrm{kJ/kg}", P_WHITE),''', r'''(None, r"1\,\mathrm{kg} \cdot 2\,257\,\frac{\mathrm{kJ}}{\mathrm{kg}}", P_WHITE),''')
sub(r'''aax = _axes(AO, 6.2, 3.25, r"\vartheta\;[\mathrm{°C}]", r"x\;[\mathrm{g/kg}]")''', r'''aax = _axes(AO, 6.2, 3.25, r"\theta\text{ in }\mathrm{°C}", r"x\text{ in }\mathrm{g/kg}")''')
sub(r'''t_e = _m(r"\vartheta_{\mathrm{e}} = ''', r'''t_e = _m(r"\theta_{\mathrm{e}} = ''')
sub(r'''t_i = _m(r"\vartheta_{\mathrm{i}} = 20''', r'''t_i = _m(r"\theta_{\mathrm{i}} = 20''')
sub(r'''w_foot = _m(r"\vartheta_{\mathrm{si}} \approx 1\,\mathrm{°C} < \vartheta_{\mathrm{Tau}} \approx 9\,\mathrm{°C}",''',
    r'''w_foot = _m(tr(r"\theta_{\mathrm{si}} \approx 1\,\mathrm{°C} < \theta_{\mathrm{dew}} \approx 9\,\mathrm{°C}", r"\theta_{\mathrm{si}} \approx 1\,\mathrm{°C} < \theta_{\mathrm{Tau}} \approx 9\,\mathrm{°C}"),''')
sub(r'''s_foot = _m(r"\vartheta_{\mathrm{Reg}} \approx 8\,\mathrm{°C} < \vartheta_{\mathrm{Tau}} \approx 16\,\mathrm{°C}",''',
    r'''s_foot = _m(tr(r"\theta_{\mathrm{coil}} \approx 8\,\mathrm{°C} < \theta_{\mathrm{dew}} \approx 16\,\mathrm{°C}", r"\theta_{\mathrm{Reg}} \approx 8\,\mathrm{°C} < \theta_{\mathrm{Tau}} \approx 16\,\mathrm{°C}"),''')

# ---- Beat8
sub('''"The heat pump transfers energy to a higher temperature level — the COP counts the heat per kilowatt-hour of electricity."''',
    '''"The heat pump transfers energy to a higher temperature level — the COP counts the useful heat per kilowatt-hour of final energy."''')
sub('''"Die Wärmepumpe überträgt Energie auf ein höheres Temperaturniveau — der COP nennt die Wärme je kWh Strom."''',
    '''"Die Wärmepumpe überträgt Energie auf ein höheres Temperaturniveau — der COP nennt die Nutzwärme je kWh Endenergie."''')
sub('''"To read more, two textbooks explain these topics step by step — the links are on screen."''',
    '''"To read more, two textbooks and the standards explain these topics — edition and designation are on screen."''')
sub('''"Zum Nachlesen: Zwei Lehrbücher erklären diese Themen Schritt für Schritt — die Links siehst du hier."''',
    '''"Zum Nachlesen: Zwei Lehrbücher und die Normen erklären diese Themen — Auflage und Kennung siehst du hier."''')
sub(r'''r"E_{\mathrm{ein}} = E_{\mathrm{aus}} + \Delta E_{\mathrm{Speicher}}"),''',
    r'''tr(r"E_{\mathrm{in}} = E_{\mathrm{out}} + \Delta E_{\mathrm{stored}}", r"E_{\mathrm{ein}} = E_{\mathrm{aus}} + \Delta E_{\mathrm{Speicher}}")),''')
sub(r'''r"\mathrm{COP} = Q_{\mathrm{H}} \,/\, W_{\mathrm{el}}"),''', r'''r"\mathrm{COP} = Q_{\mathrm{N}} \,/\, Q_{\mathrm{E}}"),''')
sub(r'''r"\dot{Q}_{\mathrm{S}} = g \cdot A \cdot I"),''', r'''r"\dot{Q}_{\mathrm{S}} = g_{\perp} \cdot A \cdot I"),''')
sub(r'''r"Q = m \cdot c \cdot \Delta T"),''', r'''r"Q = m \cdot c \cdot \Delta\theta"),''')
old_books = text[text.index("        books = ["):text.index("        VGroup(*cards).arrange")]
new_books = '''        books = [
            ("Willems WM, Schild K, Stricker D (2024)", "Formeln und Tabellen Bauphysik",
             "Wärmeschutz – Feuchteschutz – Klima – Akustik – Brandschutz", tr("8th edition, Springer Vieweg, Wiesbaden · ISBN 978-3-658-44973-5", "8. Auflage, Springer Vieweg, Wiesbaden · ISBN 978-3-658-44973-5"),
             "https://doi.org/10.1007/978-3-658-44974-2", tr("Radiation · Thermal mass · Moisture", "Strahlung · Thermische Masse · Feuchte"), P_YELLOW),
            ("Bohne D, Bohne M (2026)", "Gebäudetechnik – Technischer Ausbau und nachhaltige Gebäudesysteme",
             "", tr("13th edition, Springer Vieweg, Heidelberg · ISBN 978-3-658-51570-6", "13. Auflage, Springer Vieweg, Heidelberg · ISBN 978-3-658-51570-6"),
             "", tr("Heat pump · Ventilation · Building services", "Wärmepumpe · Lüftung · Anlagentechnik"), P_CYAN),
            (tr("Standards", "Normen und Richtlinien"), "VDI 4650 Blatt 1 (2024-02) · DIN EN 14825 · DIN EN 410",
             "DIN 4108-2 · DIN EN ISO 13786", tr("Seasonal performance · g-value · Sun protection · Heat storage", "Jahresarbeitszahl · g-Wert · Sonnenschutz · Wärmespeicherung"),
             "", "", P_GREEN),
        ]
        cards = []
        for who, book, subtitle, edition, link, topics, color in books:
            lines = [Text(who, font_size=LABEL_FONT_SIZE, color=color), Text(book, font_size=LABEL_FONT_SIZE + 2, color=P_WHITE)]
            if subtitle:
                lines.append(Text(subtitle, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))
            lines.append(Text(f"{edition} · {topics}" if topics else edition, font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).set_opacity(0.7))
            if link:
                lines.append(Text(link, font_size=LABEL_FONT_SIZE - 2, color=P_PRIMARY))
            cards.append(VGroup(*lines).arrange(DOWN, buff=0.08))
'''
text = text.replace(old_books, new_books)
sub("VGroup(*cards).arrange(DOWN, buff=0.5).move_to(np.array([0.0, 0.1, 0.0]))", "VGroup(*cards).arrange(DOWN, buff=0.3).move_to(np.array([0.0, 0.05, 0.0]))")

ROOT.write_text(text, encoding="utf-8")
print("failures:", len(failures))
for f in failures:
    print(f)
