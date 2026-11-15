#region Beat 1 – Zusammenfassung

CARD_Y, CARD_STEP, CARD_W, CARD_H = 1.36, 1.5, 1.42, 1.40
X_FORMULA, X_WINTER, X_SUMMER = -6.55, 1.45, 4.85
Y_HEAD = 0.22
Y_ROWS = (-0.34, -0.80, -1.26, -1.72, -2.18)

_CARD_EN = {"Volumen": "Volume", "Luftwechsel": "Air change", "Volumenstrom": "Volume flow", "Dichte": "Density",
            "Wärmekapazität": "Heat capacity", "Winter / Sommer": "Winter / Summer", "Feuchtedifferenz": "Humidity diff.",
            "Verdampfungsenthalpie": "Vaporisation enthalpy", "Rückgewinnung": "Recovery"}
CARD_SPECS = (
    ("v", r"V", "Volumen", lambda k: rf"{label_number(V_NET * k)}\,\mathrm{{m^{{3}}}}", COL_V),
    ("n", r"n", "Luftwechsel", lambda k: rf"{label_number(N_RATE * k, 1)}\,\mathrm{{h^{{-1}}}}", COL_N),
    ("vdot", r"\dot{V}", "Volumenstrom", lambda k: rf"{label_number(V_DOT * k)}\,\mathrm{{m^{{3}}/h}}", COL_VDOT),
    ("rho", r"\rho", "Dichte", lambda k: rf"{label_number(RHO_AIR * k, 1)}\,\mathrm{{kg/m^{{3}}}}", COL_RHO),
    ("c", r"c_{\mathrm{Luft}}", "Wärmekapazität", lambda k: rf"{label_number(C_LUFT * k, 2)}\,\mathrm{{Wh/(m^{{3}}K)}}", COL_C),
    ("dt", r"\Delta\theta", "Winter / Sommer",
     lambda k: rf"{label_number(DT_WINTER * k)}\,/\,{label_number(DT_SUMMER * k)}\,\mathrm{{K}}", COL_DT),
    ("dx", r"\Delta x", "Feuchtedifferenz",
     lambda k: rf"{label_number(DX_WINTER_GKG * k, 1)}\,/\,{label_number(DX_SUMMER_GKG * k, 1)}\,\mathrm{{g/kg}}", COL_DX),
    ("r0", r"r_{0}", "Verdampfungsenthalpie", lambda k: rf"{label_number(R_KJKG * k)}\,\mathrm{{kJ/kg}}", COL_R0),
    ("eta", r"\eta_{\mathrm{WRG}}", "Rückgewinnung", lambda k: label_number(ETA_WRG * k, 1), COL_ETA),
)


class Beat1_Zusammenfassung(Scene):
    NARRATION = [
        ("intro",
         "Nine quantities carry the whole ventilation topic — here they lie on one board.",
         "Neun Größen tragen das ganze Thema Lüftung — hier liegen sie auf einer Tafel."),
        ("v",
         r"The volume $V$: $440\,\mathrm{m^3}$ of room air in the house.",
         r"Das Volumen $V$: $440\,\mathrm{m^3}$ Raumluft im Haus."),
        ("n",
         r"The air change rate $n$: $0.5\,\mathrm{h^{-1}}$ — half the volume every hour.",
         r"Die Luftwechselrate $n$: $0{,}5\,\mathrm{h^{-1}}$ — das halbe Volumen in jeder Stunde."),
        ("vdot",
         r"Together they give the volume flow $\dot{V} = V \cdot n = 220\,\mathrm{m^3/h}$. Cooling writes it $q_{v,R}$.",
         r"Zusammen ergeben sie den Volumenstrom $\dot{V} = V \cdot n = 220\,\mathrm{m^3/h}$. Die Kühltechnik schreibt $q_{v,R}$."),
        ("rho",
         r"Air has mass: its density is $\rho \approx 1.2\,\mathrm{kg/m^3}$ at $20\,\mathrm{°C}$.",
         r"Luft hat Masse: ihre Dichte beträgt $\rho \approx 1{,}2\,\mathrm{kg/m^3}$ bei $20\,\mathrm{°C}$."),
        ("c",
         r"Every cubic metre carries heat: $c_{Luft} = \rho \cdot c_p \approx 0.34\,\mathrm{Wh/(m^3K)}$.",
         r"Jeder Kubikmeter trägt Wärme: $c_{Luft} = \rho \cdot c_p \approx 0{,}34\,\mathrm{Wh/(m^3K)}$."),
        ("dt",
         r"The temperature difference $\Delta\theta$ drives the sensible flow: $20\,\mathrm{K}$ in winter, $6\,\mathrm{K}$ in summer.",
         r"Die Temperaturdifferenz $\Delta\theta$ treibt den fühlbaren Strom: $20\,\mathrm{K}$ im Winter, $6\,\mathrm{K}$ im Sommer."),
        ("sens",
         r"Volume flow, heat capacity and temperature difference give the sensible heat flow $\dot{Q}_{V,sens}$: $1500\,\mathrm{W}$ in winter, $450\,\mathrm{W}$ in summer.",
         r"Daraus folgt der fühlbare Wärmestrom $\dot{Q}_{V,sens}$: $1500\,\mathrm{W}$ im Winter, $450\,\mathrm{W}$ im Sommer."),
        ("dx",
         r"Air also carries moisture. The humidity difference $\Delta x$ is $4.3\,\mathrm{g/kg}$ in winter and $4.5\,\mathrm{g/kg}$ in summer.",
         r"Luft trägt auch Feuchte. Die Feuchtedifferenz $\Delta x$ beträgt im Winter $4{,}3\,\mathrm{g/kg}$ und im Sommer $4{,}5\,\mathrm{g/kg}$."),
        ("r0",
         r"Water vapour carries energy: the enthalpy of vaporisation of water is $r_0 \approx 2450\,\mathrm{kJ/kg}$.",
         r"Wasserdampf trägt Energie: die Verdampfungsenthalpie von Wasser beträgt $r_0 \approx 2450\,\mathrm{kJ/kg}$."),
        ("lat",
         r"Together they give the latent heat flow $\dot{Q}_{V,lat} = \dot{V} \cdot \rho \cdot r_0 \cdot \Delta x$: $770\,\mathrm{W}$ in winter, $810\,\mathrm{W}$ in summer.",
         r"Damit wird daraus der latente Wärmestrom $\dot{Q}_{V,lat} = \dot{V} \cdot \rho \cdot r_0 \cdot \Delta x$: $770\,\mathrm{W}$ im Winter, $810\,\mathrm{W}$ im Sommer."),
        ("sum",
         r"Sensible plus latent is the sum $\dot{Q}_V$: $2270\,\mathrm{W}$ in winter, $1260\,\mathrm{W}$ in summer.",
         r"Fühlbar plus latent ergibt die Summe $\dot{Q}_V$: $2270\,\mathrm{W}$ im Winter, $1260\,\mathrm{W}$ im Sommer."),
        ("eta",
         r"A plate exchanger with $\eta = 0.8$ returns four fifths of the sensible part only.",
         r"Der Plattenübertrager mit $\eta = 0{,}8$ holt vier Fünftel des fühlbaren Anteils zurück — und nur den."),
        ("wrg",
         r"What remains: $300\,\mathrm{W} + 770\,\mathrm{W} = 1070\,\mathrm{W}$ in winter, $90\,\mathrm{W} + 810\,\mathrm{W} = 900\,\mathrm{W}$ in summer.",
         r"Übrig bleiben im Winter $300\,\mathrm{W} + 770\,\mathrm{W} = 1070\,\mathrm{W}$, im Sommer $90\,\mathrm{W} + 810\,\mathrm{W} = 900\,\mathrm{W}$."),
        ("conv",
         r"By convention the heating load counts only the sensible part, $\dot{Q}_{V,sens}$; the winter latent part costs energy only where air is humidified.",
         r"Die Heizlast rechnet nach Konvention nur den fühlbaren Teil $\dot{Q}_{V,sens}$; der latente Winteranteil kostet nur dort Energie, wo befeuchtet wird."),
        ("close",
         r"So heating takes the sensible winter values, cooling technology the sums $\dot{Q}_V$ of the summer values.",
         r"Die Heizung übernimmt also die fühlbaren Winterwerte, die Kühltechnik die Summen $\dot{Q}_V$ der Sommerwerte."),
    ]

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Nine quantities, one chain", "Neun Größen, eine Kette"))
        din = _din_ref("DIN EN 12831-1 · VDI 2078")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        xs = [-6.0 + CARD_STEP * i for i in range(len(CARD_SPECS))]
        cards = {key: _sym_card(sym, tr(_CARD_EN[name], name), fmt, colour, np.array([x, CARD_Y, 0.0]))
                 for (key, sym, name, fmt, colour), x in zip(CARD_SPECS, xs)}
        frames = VGroup(*[c["frame"] for c in cards.values()])

        head_w = body_text(tr("Winter · Heating", "Winter · Heizung"), font_size=LABEL_FONT_SIZE, color=COL_HEAT).move_to(
            np.array([X_WINTER, Y_HEAD, 0.0]))
        head_s = body_text(tr("Summer · Cooling", "Sommer · Kühltechnik"), font_size=LABEL_FONT_SIZE, color=COL_COOL).move_to(
            np.array([X_SUMMER, Y_HEAD, 0.0]))
        rule = Line(np.array([-6.6, Y_HEAD - 0.24, 0.0]), np.array([6.6, Y_HEAD - 0.24, 0.0]), color=P_WHITE, stroke_width=1.2,
                    stroke_opacity=0.35)

        def watts(amount, tracker, x, y, color):
            return math_readout(lambda: rf"{label_number(amount * tracker.get_value())}\,\mathrm{{W}}",
                                np.array([x, y, 0.0]), size=BODY_FONT_SIZE, color=color, edge="center")

        def watts_sum(first, second, tracker, x, y, color):
            return math_readout(
                lambda: rf"{label_number(first * tracker.get_value())}\,\mathrm{{W}}\,+\,{label_number(second * tracker.get_value())}\,\mathrm{{W}}\,=\,{label_number((first + second) * tracker.get_value())}\,\mathrm{{W}}",
                np.array([x, y, 0.0]), size=LABEL_FONT_SIZE + 1, color=color, edge="center")

        self.play(LaggedStart(*[FadeIn(f, shift=DOWN * 0.1) for f in frames], lag_ratio=0.1), run_time=1.4)
        hold_for(self, N, "intro", used=0.3 + 1.4)

        def reveal(key, run_time=1.2):
            card = cards[key]
            self.add(card["value"])
            return [card["frame"].animate.set_stroke(opacity=1.0), FadeIn(card["static"], shift=UP * 0.08),
                    card["tracker"].animate.set_value(1.0)]

        for key in ("v", "n"):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(*reveal(key), run_time=1.2)
            hold_for(self, N, key)

        row1, it1 = _place_row([("vdot", r"\dot{V}", COL_VDOT), (None, "=", None), ("v", "V", COL_V), (None, r"\cdot", None),
                                ("n", "n", COL_N)], Y_ROWS[0], X_FORMULA)
        k1 = ValueTracker(0.0)
        res1 = math_readout(lambda: rf"=\;{label_number(V_DOT * k1.get_value())}\,\mathrm{{m^{{3}}/h}}",
                            np.array([row1.get_right()[0] + 0.12, Y_ROWS[0], 0.0]), size=22, color=COL_VDOT, edge="left")
        caption = swap_caption(self, caption, subtitle_text(N, "vdot"))
        self.play(FadeIn(rule), FadeIn(head_w), FadeIn(head_s), run_time=0.6)
        self.add(res1)
        self.play(stage.transition("ventilation.handoff.summary.vdot-v", source=cards["v"]["sym"], target=it1["v"]), stage.transition("ventilation.handoff.summary.vdot-n", source=cards["n"]["sym"], target=it1["n"]),
                  FadeIn(it1["vdot"]), FadeIn(_unkeyed(row1, it1)), k1.animate.set_value(1.0), run_time=1.6)
        self.add(cards["vdot"]["value"])
        self.play(cards["vdot"]["frame"].animate.set_stroke(opacity=1.0), stage.transition("ventilation.handoff.summary.card-vdot", source=it1["vdot"], target=cards["vdot"]["sym"]),
                  FadeIn(cards["vdot"]["name"]), cards["vdot"]["tracker"].animate.set_value(1.0), run_time=1.2)
        hold_for(self, N, "vdot")

        for key in ("rho", "c", "dt"):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(*reveal(key), run_time=1.2)
            hold_for(self, N, key)

        row2, it2 = _place_row([("q", r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE), (None, "=", None), ("vdot", r"\dot{V}", COL_VDOT),
                                (None, r"\cdot", None), ("c", r"c_{\mathrm{Luft}}", COL_C), (None, r"\cdot", None),
                                ("dt", r"\Delta\theta", COL_DT)], Y_ROWS[1], X_FORMULA, size=22)
        k2 = ValueTracker(0.0)
        res2w, res2s = watts(Q_WINTER, k2, X_WINTER, Y_ROWS[1], COL_HEAT), watts(Q_SUMMER, k2, X_SUMMER, Y_ROWS[1], COL_COOL)
        caption = swap_caption(self, caption, subtitle_text(N, "sens"))
        self.add(res2w, res2s)
        self.play(*[stage.transition(f"ventilation.handoff.summary.sens-{k}", source=cards[k]["sym"], target=it2[k]) for k in ("vdot", "c", "dt")], FadeIn(it2["q"]),
                  FadeIn(_unkeyed(row2, it2)), k2.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "sens")

        for key in ("dx", "r0"):
            caption = swap_caption(self, caption, subtitle_text(N, key))
            self.play(*reveal(key), run_time=1.2)
            hold_for(self, N, key)

        row3, it3 = _place_row([("q", r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE), (None, "=", None), ("vdot", r"\dot{V}", COL_VDOT),
                                (None, r"\cdot", None), ("rho", r"\rho", COL_RHO), (None, r"\cdot", None), ("r0", r"r_{0}", COL_R0),
                                (None, r"\cdot", None), ("dx", r"\Delta x", COL_DX)], Y_ROWS[2], X_FORMULA, size=22)
        k3 = ValueTracker(0.0)
        res3w, res3s = watts(Q_LAT_WINTER, k3, X_WINTER, Y_ROWS[2], COL_DX), watts(Q_LAT_SUMMER, k3, X_SUMMER, Y_ROWS[2], COL_DX)
        caption = swap_caption(self, caption, subtitle_text(N, "lat"))
        self.add(res3w, res3s)
        self.play(*[stage.transition(f"ventilation.handoff.summary.lat-{k}", source=cards[k]["sym"], target=it3[k]) for k in ("vdot", "rho", "r0", "dx")], FadeIn(it3["q"]),
                  FadeIn(_unkeyed(row3, it3)), k3.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "lat")

        row4, it4 = _place_row([("q", r"\dot{Q}_{\mathrm{V}}", P_WHITE), (None, "=", None), ("sens", r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE),
                                (None, "+", None), ("lat", r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE)], Y_ROWS[3], X_FORMULA, size=22)
        k4 = ValueTracker(0.0)
        res4w, res4s = watts(Q_SUM_WINTER, k4, X_WINTER, Y_ROWS[3], COL_HEAT), watts(Q_SUM_SUMMER, k4, X_SUMMER, Y_ROWS[3], COL_COOL)
        caption = swap_caption(self, caption, subtitle_text(N, "sum"))
        self.add(res4w, res4s)
        self.play(FadeIn(row4), k4.animate.set_value(1.0), run_time=1.4)
        hold_for(self, N, "sum")

        caption = swap_caption(self, caption, subtitle_text(N, "eta"))
        self.play(*reveal("eta"), run_time=1.2)
        hold_for(self, N, "eta")

        row5, it5 = _place_row([("q", r"\dot{Q}_{\mathrm{V,WRG}}", P_WHITE), (None, "=", None),
                                (None, r"\dot{Q}_{\mathrm{V,sens}}", P_WHITE), (None, r"\cdot", None),
                                ("eta", r"(1-\eta_{\mathrm{WRG}})", COL_ETA), (None, "+", None),
                                (None, r"\dot{Q}_{\mathrm{V,lat}}", P_WHITE)], Y_ROWS[4], X_FORMULA, size=22)
        k5 = ValueTracker(0.0)
        res5w = watts_sum(Q_WINTER_WRG, Q_LAT_WINTER, k5, X_WINTER - 0.05, Y_ROWS[4], COL_HEAT)
        res5s = watts_sum(Q_SUMMER_WRG, Q_LAT_SUMMER, k5, X_SUMMER + 0.05, Y_ROWS[4], COL_COOL)
        caption = swap_caption(self, caption, subtitle_text(N, "wrg"))
        self.add(res5w, res5s)
        self.play(stage.transition("ventilation.handoff.summary.wrg-eta", source=cards["eta"]["sym"], target=it5["eta"]), FadeIn(it5["q"]), FadeIn(_unkeyed(row5, it5)),
                  k5.animate.set_value(1.0), run_time=1.8)
        hold_for(self, N, "wrg")

        ring_sens = SurroundingRectangle(res2w, color=COL_HEAT, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        ring_lat = DashedVMobject(SurroundingRectangle(res3w, color=COL_DX, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3), num_dashes=24)
        note = body_text(tr("heating load: sensible only (convention)", "Heizlast: nur fühlbar (Konvention)"), font_size=LABEL_FONT_SIZE - 4, color=COL_HEAT)
        note.scale_to_fit_width(min(note.width, 2.0)).move_to(np.array([3.15, (Y_ROWS[1] + Y_ROWS[2]) / 2, 0.0]))
        caption = swap_caption(self, caption, subtitle_text(N, "conv"))
        self.play(Create(ring_sens), Create(ring_lat), FadeIn(note), run_time=1.2)
        hold_for(self, N, "conv")

        ring_sum = SurroundingRectangle(res4s, color=COL_COOL, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=3)
        caption = swap_caption(self, caption, subtitle_text(N, "close"))
        self.play(Create(ring_sum), Indicate(res4s, color=COL_COOL, scale_factor=1.15), run_time=1.2)
        hold_for(self, N, "close")

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.6)

#endregion