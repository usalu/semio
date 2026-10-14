#region Beat 4 – Die Anlagenaufwandszahl e_p

class Beat4_Anlagenaufwandszahl(Scene):
    NARRATION = [
        ("intro",
         "Multiply the four stages together and the whole plant collapses into a single number.",
         "Multipliziert man die vier Stufen, schrumpft die ganze Anlage auf eine einzige Zahl."),
        ("kette",
         "That is the plant expense figure: how much final energy is required per unit of useful heat.",
         "Das ist die Anlagenaufwandszahl: wie viel Endenergie je Einheit Nutzenergie erforderlich ist."),
        ("gas",
         "For the gas boiler it lands at one point four — a hundred becomes about a hundred and forty.",
         "Beim Gaskessel liegt sie bei eins Komma vier — aus hundert werden etwa hundertvierzig."),
        ("wp",
         "For the heat pump it drops below one, because three quarters of the delivered heat came from the environment.",
         "Bei der Wärmepumpe fällt sie unter eins — drei Viertel der gelieferten Wärme kamen aus der Umwelt."),
        ("brueck",
         "But thirty-one kilowatt-hours of electricity and a hundred and forty of gas are still not comparable. That is the next chapter.",
         "Doch einunddreißig Kilowattstunden Strom und hundertvierzig Gas sind noch nicht vergleichbar. Das ist das nächste Kapitel."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Anlagenaufwandszahl e_p", title)
        din = _din_ref("DIN V 18599-5:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        mix, lift, grow_p = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        active: list = []
        heat_names = ("rad", "person", "ue", "ve", "sp")

        def live(rt):
            return [_flow(active, rt), *[P["heat"](n, rt) for n in heat_names]]

        q_h = _readout(lambda: rf"Q_{{h}}={de_num(v['z0n'].get_value())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                       np.array([ROOM_C[0], 1.42, 0.0]), size=BODY_FONT_SIZE, color=NUTZ_BLUE, edge="center")
        q_e = _readout(lambda: rf"Q_{{E}}={de_num(P['z4'].total())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                       np.array([0.78, -1.22, 0.0]), size=BODY_FONT_SIZE, color=END_RED, edge="left")

        rows = {
            "ue": _stage_row("Übergabe", r"e_{ü}", UEBER_RED, 1.80, v["z2u"], LEVELS[0], tick_label=True),
            "ve": _stage_row("Verteilung", r"e_{d}", VERT_ORANGE, 1.22, v["z2v"], LEVELS[1]),
            "sp": _stage_row("Speicherung", r"e_{s}", SPEI_YELLOW, 0.64, v["z3s"], LEVELS[2]),
        }
        eg_unit = ValueTracker(0.0)

        def eg_value():
            m = mix.get_value()
            return ((1 - m) * (eg_unit.get_value() + v["z4e"].get_value() / LEVELS[3])
                    + m * v["z4w"].get_value() / LEVELS[3])

        rows["eg"] = _ratio_row("Erzeugung", r"e_{g}", ERZ_AMBER, 0.06, out=LEVELS[3], value=eg_value,
                                shown=lambda: eg_unit.get_value() > 0.999 and (v["z4e"].get_value() > 0.05
                                                                              or v["z4w"].get_value() > 0.05))
        keys = ("ue", "ve", "sp", "eg")
        colours = (UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER)
        units = {"ue": rows["ue"]["unit"], "ve": rows["ve"]["unit"], "sp": rows["sp"]["unit"], "eg": eg_unit}
        product = _product_row(
            "Anlage", r"e_{p}", -0.75,
            [((lambda f=rows[k]["value"]: 1.0 + grow_p.get_value() * (f() - 1.0)), c) for k, c in zip(keys, colours)],
            lift=lambda: lift.get_value())

        # —— The whole plant, drawn once: every stream at its final width ——
        self.play(Create(P["shell"]), run_time=1.2)
        self.add(P["z0"].ribbons, P["z2"].ribbons, P["z3"].ribbons, P["z4"].ribbons, P["flue"].ribbons,
                 P["env"].ribbons, P["tank"]["fill"], q_h, q_e)
        active.extend([P["z0"], P["z2"], P["z3"], P["z4"], P["flue"]])
        self.play(FadeIn(P["person"]), FadeIn(P["radiator"]), FadeIn(P["tank"]["group"]), FadeIn(P["boiler"]),
                  FadeIn(P["meter"]), FadeIn(P["chimney"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(0.8), run_time=0.8)
        self.play(*_grow(P, ["z0n", "z2n", "z2u", "z2v", "z3n", "z3u", "z3v", "z3s", "z4n", "z4u", "z4v", "z4s",
                             "z4e", "flue"], 2.4), *live(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 0.8 + 2.4, during=live)

        # —— Four gauges, then their product as one bar ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "kette"), live)
        self.add(*[rows[k]["readout"] for k in keys], product["readout"])
        for k, c, node in zip(keys, colours, (P["radiator"], P["z2"].ribbons, P["tank"]["group"], P["boiler"])):
            self.play(FadeIn(rows[k]["group"]), units[k].animate.set_value(1.0),
                      Circumscribe(node, color=c, buff=0.1, stroke_width=3, fade_out=True), *live(0.8), run_time=0.8)
        eq, eq_box, items = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, "=", P_WHITE),
            ("eu", r"e_{ü}", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"e_{d}", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"e_{s}", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"e_{g}", ERZ_AMBER),
        ])
        schematic_note = note_line("Schematische Darstellung — DIN/TS 18599-5 rechnet deutlich detaillierter",
                                   color=P_TEAL)
        self.play(FadeIn(product["group"]), FadeIn(schematic_note), lift.animate.set_value(1.0), *live(0.8),
                  run_time=0.8)
        self.play(grow_p.animate.set_value(1.0), *live(2.0), run_time=2.0)
        self.play(Create(eq_box),
                  ReplacementTransform(product["symbol"].copy(), items["ep"]),
                  *[ReplacementTransform(rows[k]["symbol"].copy(), items[i])
                    for k, i in zip(keys, ("eu", "ed", "es", "eg"))],
                  FadeIn(VGroup(*[m for m in eq if all(m is not items[i] for i in ("ep", "eu", "ed", "es", "eg"))])),
                  *live(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "kette", during=live)

        # —— Gas: 100 become 140 ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "gas"), live)
        gas_label = body_text("Gaskessel", font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(
            rows["eg"]["label"].get_left(), aligned_edge=LEFT)
        eq2, eq2_box, items2 = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, r"\approx", P_WHITE),
            ("eu", r"1{,}04", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"1{,}08", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"1{,}06", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"1{,}18", ERZ_AMBER), (None, r"\approx 1{,}40", P_WHITE),
        ])
        self.play(FadeOut(schematic_note), ReplacementTransform(rows["eg"]["label"], gas_label),
                  ReplacementTransform(eq_box, eq2_box), ReplacementTransform(eq, eq2),
                  Indicate(P["meter"], color=END_RED), *live(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "gas", during=live)

        # —— Heat pump: the same chain with a generator that takes 0,26 ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "wp"), live)
        pump_label = body_text("Wärmepumpe", font_size=LABEL_FONT_SIZE, color=AMBIENT_GREEN).move_to(
            gas_label.get_left(), aligned_edge=LEFT)
        env_lbl = _readout(lambda: rf"\text{{Umwelt}}\;{de_num(v['env'].get_value())}\,\mathrm{{kWh}}",
                           np.array([-0.45, PIPE_Y - 0.62, 0.0]), size=LABEL_FONT_SIZE, color=AMBIENT_GREEN, edge="left")
        eq3, eq3_box, items3 = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, r"\approx", P_WHITE),
            ("eu", r"1{,}04", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"1{,}08", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"1{,}06", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"0{,}26", AMBIENT_GREEN), (None, r"\approx 0{,}31", P_WHITE),
        ])
        why = note_line("e_p < 1 nur, weil Umweltwärme ohne zugekaufte Endenergie dazukommt", color=AMBIENT_GREEN)
        self.add(env_lbl)
        active.append(P["env"])
        self.play(
            ReplacementTransform(P["boiler"], P["heat_pump"]), ReplacementTransform(P["meter"], P["power_meter"]),
            FadeOut(P["chimney"]), ReplacementTransform(gas_label, pump_label),
            *[v[n].animate.set_value(0.0) for n in ("z4n", "z4u", "z4v", "z4s", "z4e", "flue")],
            v["z4w"].animate.set_value(WP_STROM), v["env"].animate.set_value(WP_UMWELT), mix.animate.set_value(1.0),
            ReplacementTransform(eq2_box, eq3_box), ReplacementTransform(eq2, eq3), FadeIn(why),
            *live(3.0), run_time=3.0)
        hold_for(self, self.NARRATION, "wp", during=live)

        # —— Both numbers side by side — not yet comparable ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "brueck"), live)
        bridge, bridge_box, bitems = math_panel([
            ("qe", r"Q_{E}", END_RED), (None, "=", P_WHITE),
            ("qh", r"Q_{h}", NUTZ_BLUE), (None, r"\cdot", P_WHITE),
            ("ep", r"e_{p}", P_WHITE),
        ])
        gas_meter = _meter(np.array([2.1, 1.35, 0.0]), kind="gas", color=ERZ_AMBER).scale(1.5)
        power_meter = _meter(np.array([5.3, 1.35, 0.0]), kind="power", color=P_YELLOW).scale(1.5)
        gas_val = math_label(rf"{de_num(round(Q_E_GAS))}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}", np.array([2.1, 0.55, 0.0]),
                             size=BODY_FONT_SIZE, color=END_RED)
        power_val = math_label(rf"{de_num(round(Q_E_WP))}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}", np.array([5.3, 0.55, 0.0]),
                               size=BODY_FONT_SIZE, color=P_YELLOW)
        not_equal = VGroup(*[Line(np.array([3.55 - 0.16, 1.35 + dy, 0.0]), np.array([3.55 + 0.16, 1.35 + dy, 0.0]),
                                  color=P_WHITE, stroke_width=3) for dy in (0.07, -0.07)],
                           Line(np.array([3.55 - 0.1, 1.35 - 0.2, 0.0]), np.array([3.55 + 0.1, 1.35 + 0.2, 0.0]),
                                color=P_WHITE, stroke_width=3))
        gas_name = body_text("Gas", font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(np.array([2.1, 1.95, 0.0]))
        power_name = body_text("Strom", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([5.3, 1.95, 0.0]))
        bridge_note = note_line("1 kWh Strom ≠ 1 kWh Gas", color=P_TEAL)
        everything = VGroup(*[rows[k]["group"] for k in keys], product["group"])
        self.remove(*[rows[k]["readout"] for k in keys], product["readout"])
        self.play(FadeOut(why), FadeOut(everything), FadeOut(pump_label),
                  ReplacementTransform(eq3_box, bridge_box), ReplacementTransform(eq3, bridge),
                  FadeIn(gas_meter), FadeIn(power_meter), FadeIn(gas_val), FadeIn(power_val), FadeIn(not_equal),
                  FadeIn(gas_name), FadeIn(power_name), FadeIn(bridge_note), *live(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "brueck", during=live)

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion
