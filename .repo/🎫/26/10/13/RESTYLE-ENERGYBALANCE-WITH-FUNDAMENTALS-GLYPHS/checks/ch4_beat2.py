#region Beat 2 – Übergabe, Verteilung, Speicherung

class Beat2_UebergabeVerteilungSpeicherung(Scene):
    NARRATION = [
        ("intro",
         "Walk the chain backwards through a real building, starting in the room itself.",
         "Gehen wir die Kette rückwärts durch ein echtes Gebäude — beginnend im Raum selbst."),
        ("uebergabe",
         "Emission: how the surface hands heat to the air. A radiator needs hot water, underfloor heating needs far less.",
         "Übergabe: wie die Fläche Wärme an die Luft abgibt. Ein Heizkörper braucht heißes Wasser, eine Fußbodenheizung viel weniger."),
        ("verteilung",
         "Distribution: every pipe metre loses heat. Outside the envelope it is lost for good — inside it partly comes back as a gain.",
         "Verteilung: Jeder Rohrmeter verliert Wärme. Außerhalb der Hülle ist sie weg — innerhalb kommt sie teilweise als Gewinn zurück."),
        ("speicher",
         "Storage: a tank kept warm around the clock loses heat even without any draw-off, for as long as a whole week.",
         "Speicherung: Ein warm gehaltener Speicher verliert auch ohne Zapfung Wärme — selbst eine ganze Woche lang."),
        ("kennzahl",
         "Each stage gets one expense figure: how much has to go in for one unit to come out.",
         "Jede Stufe bekommt eine Aufwandszahl: wie viel hinein muss, damit eine Einheit herauskommt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Übergabe, Verteilung, Speicherung", title)
        din = _din_ref("DIN V 18599-5:2018-09 · -6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        active: list = []

        def live(rt, *heat):
            return [_flow(active, rt), *[P["heat"](n, rt) for n in heat]]

        envelope = DashedVMobject(
            Rectangle(width=ROOM_W + 0.64, height=ROOM_H + 0.26 + 0.12 + 0.26, color=P_TEAL, stroke_width=2.4).move_to(
                np.array([ROOM_C[0], P["under_slab"] + (ROOM_H + 0.64) / 2, 0.0])), num_dashes=70)
        shell_tag = body_text("beheizte Hülle", font_size=LABEL_FONT_SIZE - 3, color=P_TEAL).move_to(
            np.array([-2.35, 1.56, 0.0]))
        cellar_tag = body_text("unbeheizter Keller", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(
            np.array([ROOM_C[0], GROUND_Y - 0.30, 0.0]))
        cellar_tag.set_opacity(0.7)

        rows = {
            "ue": _stage_row("Übergabe", r"e_{ü}", UEBER_RED, 1.45, v["z2u"], LEVELS[0], tick_label=True),
            "ve": _stage_row("Verteilung", r"e_{d}", VERT_ORANGE, 0.55, v["z2v"], LEVELS[1]),
            "sp": _stage_row("Speicherung", r"e_{s}", SPEI_YELLOW, -0.35, v["z3s"], LEVELS[2]),
        }

        def reveal(name, rt):
            row = rows[name]
            return [row["unit"].animate.set_value(1.0)]

        # —— The building, heated storey over the unheated cellar ——
        self.play(Create(P["shell"]), Create(envelope), FadeIn(shell_tag), FadeIn(cellar_tag), run_time=1.5)
        self.play(FadeIn(P["person"]), FadeIn(P["radiator"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(0.8, "person"), run_time=0.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 0.8,
                 during=lambda rt: live(rt, "person"))

        # —— Emission: the radiator hands one unit to the room and takes 1,04 in ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "uebergabe"))
        rad_tag = math_label(r"\text{Heizkörper}\,\cdot\,55\,\mathrm{°C}", np.array([-4.95, 1.52, 0.0]),
                             size=LABEL_FONT_SIZE, color=UEBER_RED)
        loop = _floor_loop(-4.20, -2.55, room["y_f"] - 0.07)
        loop_tag = math_label(r"\text{Fußbodenheizung}\,\cdot\,35\,\mathrm{°C}", np.array([-3.05, 1.14, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_CYAN)
        flow_note = note_line("niedrige Vorlauftemperatur → bessere Erzeugereffizienz", color=P_CYAN)
        self.add(P["z0"].ribbons, P["z2"].ribbons, rows["ue"]["readout"], rows["ve"]["readout"], rows["sp"]["readout"])
        active.extend([P["z0"], P["z2"]])
        self.play(FadeIn(rows["ue"]["group"]), FadeIn(rad_tag), *_grow(P, ["z0n"], 1.2), *reveal("ue", 1.2),
                  *live(1.2, "person", "rad"), run_time=1.2)
        self.play(Circumscribe(P["radiator"], color=UEBER_RED, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z2n", "z2u"], 1.6), *live(1.6, "person", "rad", "ue"), run_time=1.6)
        floor_heat = lambda rt: ripples([np.array([x, room["y_f"] + 0.02, 0.0]) for x in (-3.7, -3.1, -2.5)],
                                        r_max=0.32, color=P_CYAN, cycles=max(1.0, rt / 1.4))
        self.play(Create(loop), FadeIn(loop_tag), FadeIn(flow_note), *live(1.2, "person", "rad", "ue"), floor_heat(1.2),
                  run_time=1.2)
        hold_for(self, self.NARRATION, "uebergabe",
                 during=lambda rt: [*live(rt, "person", "rad", "ue"), floor_heat(rt)])

        # —— Distribution: the pipe narrows by 8 kWh on its way up ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "verteilung"))
        inside_note = note_line("Verluste innerhalb der Hülle werden als Gewinn gutgeschrieben", color=VERT_ORANGE)
        self.play(FadeOut(flow_note), FadeOut(rad_tag), FadeOut(loop_tag), FadeOut(loop), run_time=0.3)
        self.play(FadeIn(rows["ve"]["group"]), FadeIn(inside_note), *reveal("ve", 1.0), *live(1.0, "person", "rad", "ue"),
                  run_time=1.0)
        self.play(Circumscribe(P["z2"].ribbons, color=VERT_ORANGE, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z2v"], 2.0), *live(2.0, "person", "rad", "ue", "ve", "gain"), run_time=2.0)
        hold_for(self, self.NARRATION, "verteilung",
                 during=lambda rt: live(rt, "person", "rad", "ue", "ve", "gain"))

        # —— Storage: the tank drains while it waits ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "speicher"))
        tank_note = body_text("7 Tage ohne Zapfung", font_size=LABEL_FONT_SIZE - 3, color=SPEI_YELLOW).move_to(
            np.array([TANK_X, GROUND_Y - 0.62, 0.0]))
        self.play(FadeOut(inside_note), FadeOut(cellar_tag), run_time=0.3)
        self.add(P["z3"].ribbons, P["tank"]["fill"])
        active.append(P["z3"])
        self.play(FadeIn(rows["sp"]["group"]), FadeIn(P["tank"]["group"]), FadeIn(P["boiler"]), FadeIn(tank_note),
                  *reveal("sp", 1.0), *live(1.0, "person", "rad", "ue", "ve"), run_time=1.0)
        self.play(Circumscribe(P["tank"]["group"], color=SPEI_YELLOW, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z3n", "z3u", "z3v", "z3s"], 1.8), *live(1.8, "person", "rad", "ue", "ve", "sp"),
                  run_time=1.8)
        hold_for(self, self.NARRATION, "speicher",
                 during=lambda rt: [*live(rt, "person", "rad", "ue", "ve", "sp"),
                                    P["tank"]["level"].animate.set_value(0.72)])

        # —— One expense figure per stage ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kennzahl"))
        gt_note = note_line("alle Aufwandszahlen > 1", color=P_TEAL)
        eq, eq_box, items = math_panel([
            ("eu", r"e_{ü}\approx 1{,}04", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"e_{d}\approx 1{,}08", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"e_{s}\approx 1{,}06", SPEI_YELLOW),
        ])
        self.play(FadeOut(tank_note), FadeIn(gt_note), run_time=0.3)
        self.play(Create(eq_box), *[ReplacementTransform(rows[k]["symbol"].copy(), items[i])
                                    for k, i in (("ue", "eu"), ("ve", "ed"), ("sp", "es"))],
                  FadeIn(VGroup(*[m for m in eq if all(m is not items[i] for i in ("eu", "ed", "es"))])),
                  *live(1.4, "person", "rad", "ue", "ve", "sp"), run_time=1.4)
        hold_for(self, self.NARRATION, "kennzahl",
                 during=lambda rt: live(rt, "person", "rad", "ue", "ve", "sp"))

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion


