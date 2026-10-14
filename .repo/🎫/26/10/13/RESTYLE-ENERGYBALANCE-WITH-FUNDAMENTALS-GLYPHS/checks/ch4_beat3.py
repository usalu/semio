#region Beat 3 – Erzeugung: Wirkungsgrad oder Arbeitszahl

class Beat3_Erzeugung(Scene):
    NARRATION = [
        ("intro",
         "The last stage is the biggest — and this is where a combustion boiler and a heat pump differ fundamentally.",
         "Die letzte Stufe ist die größte — hier unterscheiden sich Verbrennungskessel und Wärmepumpe grundlegend."),
        ("kessel",
         "A boiler burns fuel. Part of the heat leaves through the flue, so more always goes in than comes out.",
         "Ein Kessel verbrennt Brennstoff. Ein Teil der Wärme geht durch den Schornstein — es muss immer mehr hinein als heraus."),
        ("brennwert",
         "A condensing boiler cools the flue gas until its water vapour condenses — the latent heat from the fundamentals video becomes usable.",
         "Ein Brennwertkessel kühlt das Abgas, bis Wasserdampf darin kondensiert — die latente Wärme aus dem Grundlagen-Video wird nutzbar."),
        ("waermepumpe",
         "A heat pump burns nothing. It lifts ambient heat, so three quarters of what it delivers is not purchased final energy.",
         "Eine Wärmepumpe verbrennt nichts. Sie hebt Umweltwärme an — drei Viertel der Lieferung sind keine zugekaufte Endenergie."),
        ("jaz",
         "Its seasonal performance factor is the ratio of heat delivered over the year to the electricity that drove it: here a hundred to twenty-six.",
         "Ihre Jahresarbeitszahl ist das Verhältnis von Wärme zu eingesetztem Strom im Jahr: hier hundert zu sechsundzwanzig."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Erzeugung: Wirkungsgrad oder Arbeitszahl", title)
        din = _din_ref("DIN V 18599-5 · VDI 4650-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        t = {k: ValueTracker(0.0) for k in ("heat", "gas", "loss", "recov", "el", "env", "out_env", "out_el", "split")}
        Y, half = 0.55, 0.5 * Q_NUTZ * FLOW_UNIT
        boiler = _boiler(np.array([-3.2, Y, 0.0]), w=0.9, h=1.2)
        pump = _heat_pump(np.array([3.7, Y, 0.0]), w=1.0, h=1.2)
        divider = DashedLine(np.array([0.0, -1.0, 0.0]), np.array([0.0, 2.05, 0.0]), color=P_WHITE, stroke_width=1.6,
                             stroke_opacity=0.3)
        out_env = lambda: interpolate_color(ManimColor(NUTZ_BLUE), ManimColor(AMBIENT_GREEN), t["split"].get_value())
        out_el = lambda: interpolate_color(ManimColor(NUTZ_BLUE), ManimColor(P_YELLOW), t["split"].get_value())

        gas_in = _Stream([np.array([-6.3, Y, 0.0]), np.array([-3.65, Y, 0.0])],
                         [(t["gas"], ERZ_AMBER, False), (t["loss"], P_RED, False)], start=-half, cap=118, seed=11)
        heat_l = _Stream([np.array([-2.75, Y, 0.0]), np.array([-0.60, Y, 0.0])], [(t["heat"], NUTZ_BLUE, False)],
                         start=-half, cap=100, seed=12)
        flue = _Stream([np.array([-3.40, Y + 0.58, 0.0]), np.array([-3.40, 1.95, 0.0])], [(t["loss"], P_RED, False)],
                       start=-0.5 * 18 * FLOW_UNIT, cap=18, seed=13)
        recov = _Stream([np.array([-3.40, 1.62, 0.0]), np.array([-2.85, 1.62, 0.0]), np.array([-2.85, Y + 0.58, 0.0])],
                        [(t["recov"], P_CYAN, False)], start=-0.5 * 11 * FLOW_UNIT, cap=11, seed=14, radius=0.25)
        env_in = _Stream([np.array([0.9, Y - 0.25, 0.0]), np.array([3.15, Y - 0.25, 0.0])],
                         [(t["env"], AMBIENT_GREEN, False)], start=-0.5 * 74 * FLOW_UNIT, cap=74, seed=15)
        el_in = _Stream([np.array([0.9, Y + 0.42, 0.0]), np.array([3.15, Y + 0.42, 0.0])],
                        [(t["el"], P_YELLOW, False)], start=-0.5 * 26 * FLOW_UNIT, cap=26, seed=16)
        hp_out = _Stream([np.array([4.25, Y, 0.0]), np.array([6.0, Y, 0.0])],
                         [(t["out_env"], out_env, False), (t["out_el"], out_el, False)], start=-half, cap=100, seed=17)
        active: list = []

        def live(rt):
            fan = Rotate(pump[1][1], angle=-TAU * rt / 0.9, about_point=pump[1][0].get_center(), rate_func=linear)
            return [_flow(active, rt), fan]

        def label(src, at, color, edge="left"):
            return _readout(src, np.array(at, dtype=float), size=LABEL_FONT_SIZE, color=color, edge=edge)

        gas_lbl = label(lambda: rf"\text{{Erdgas}}\;{de_num(t['gas'].get_value() + t['loss'].get_value())}\,\mathrm{{kWh}}",
                        (-6.3, Y + 0.75, 0.0), ERZ_AMBER)
        heat_lbl = label(lambda: rf"\text{{Wärme}}\;{de_num(t['heat'].get_value())}\,\mathrm{{kWh}}",
                         (-0.60, Y + 0.50, 0.0), NUTZ_BLUE, "right")
        pump_lbl = label(lambda: rf"{de_num(t['out_env'].get_value() + t['out_el'].get_value())}\,\mathrm{{kWh}}",
                         (6.0, Y + 0.50, 0.0), NUTZ_BLUE, "right")
        el_lbl = label(lambda: rf"\text{{Strom}}\;{de_num(t['el'].get_value())}\,\mathrm{{kWh}}",
                       (0.9, Y + 0.75, 0.0), P_YELLOW)
        env_lbl = label(lambda: rf"\text{{Umwelt}}\;{de_num(t['env'].get_value())}\,\mathrm{{kWh}}",
                        (0.9, Y - 0.62, 0.0), AMBIENT_GREEN)

        gas_row = _ratio_row("Gaskessel", r"e_{g}", ERZ_AMBER, -0.65, out=Q_NUTZ,
                             value=lambda: (t["gas"].get_value() + t["loss"].get_value()) / Q_NUTZ,
                             shown=lambda: t["gas"].get_value() > 99.0, x0=-6.9, name_w=1.95, length=2.0)
        wp_row = _ratio_row("Wärmepumpe", r"e_{g}", AMBIENT_GREEN, -0.65, out=Q_NUTZ,
                            value=lambda: t["el"].get_value() / Q_NUTZ,
                            shown=lambda: t["el"].get_value() > 25.0, x0=0.35, name_w=1.55, length=2.0)
        cond_label = body_text("Brennwertkessel", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            gas_row["label"].get_left(), aligned_edge=LEFT)
        flue_lbl = body_text("Abgas", font_size=LABEL_FONT_SIZE - 3, color=P_RED).move_to(np.array([-3.0, 1.95, 0.0]))
        drops = droplets(np.array([-2.55, 1.35, 0.0]), n=5, spread=(0.14, 0.14), color=P_CYAN)

        self.play(Create(divider), FadeIn(boiler), FadeIn(pump), run_time=0.8)
        self.add(heat_l.ribbons, hp_out.ribbons, heat_lbl, pump_lbl)
        active.extend([heat_l, hp_out])
        self.play(t["heat"].animate.set_value(Q_NUTZ), t["out_env"].animate.set_value(74.0),
                  t["out_el"].animate.set_value(26.0), *live(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 0.8 + 1.2, during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "kessel"), live)
        self.add(gas_in.ribbons, flue.ribbons, gas_lbl, gas_row["readout"])
        active.extend([gas_in, flue])
        self.play(FadeIn(gas_row["group"]), FadeIn(flue_lbl), *live(0.8), run_time=0.8)
        self.play(t["gas"].animate.set_value(Q_NUTZ), t["loss"].animate.set_value(18.0),
                  Circumscribe(boiler, color=ERZ_AMBER, buff=0.1, stroke_width=3, fade_out=True),
                  *live(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "kessel", during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "brennwert"), live)
        cond_note = note_line("Brennwert: Wasserdampf im Abgas kondensiert → latente Wärme wird nutzbar", color=P_CYAN)
        self.add(recov.ribbons)
        active.append(recov)
        self.play(FadeIn(cond_note), FadeIn(drops), ReplacementTransform(gas_row["label"], cond_label),
                  t["loss"].animate.set_value(7.0), t["recov"].animate.set_value(11.0), *live(2.2), run_time=2.2)
        hold_for(self, self.NARRATION, "brennwert", during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "waermepumpe"), live)
        bought = body_text("zugekauft", font_size=LABEL_FONT_SIZE - 2, color=P_YELLOW).move_to(np.array([4.75, 1.50, 0.0]))
        free = body_text("nicht zugekauft", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN).move_to(
            np.array([4.75, -0.40, 0.0]))
        bought_leader = Line(bought.get_bottom() + DOWN * 0.04, np.array([4.75, Y + 0.185, 0.0]), color=P_YELLOW,
                             stroke_width=1.4, stroke_opacity=0.6)
        free_leader = Line(free.get_top() + UP * 0.04, np.array([4.75, Y - 0.065, 0.0]), color=AMBIENT_GREEN,
                           stroke_width=1.4, stroke_opacity=0.6)
        self.add(env_in.ribbons, el_in.ribbons, el_lbl, env_lbl, wp_row["readout"])
        active.extend([env_in, el_in])
        self.play(FadeOut(cond_note), FadeIn(wp_row["group"]), *live(0.3), run_time=0.3)
        self.play(t["env"].animate.set_value(74.0), t["el"].animate.set_value(26.0), t["split"].animate.set_value(1.0),
                  Circumscribe(pump, color=AMBIENT_GREEN, buff=0.1, stroke_width=3, fade_out=True),
                  *live(2.2), run_time=2.2)
        self.play(FadeIn(bought), FadeIn(free), Create(bought_leader), Create(free_leader), *live(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "waermepumpe", during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "jaz"), live)
        eq, eq_box, items = math_panel([
            ("jaz", r"\mathrm{JAZ}", AMBIENT_GREEN), (None, "=", P_WHITE),
            ("frac", rf"\frac{{\textcolor{{{NUTZ_BLUE}}}{{\text{{Wärme ab}}}}}}{{\textcolor{{{P_YELLOW}}}{{\text{{Strom zu}}}}}}",
             P_WHITE),
            ("num", r"=\frac{100}{26}\approx 3{,}8", P_WHITE),
            ("eg", r"\;\Rightarrow\;e_{g}=\frac{1}{\mathrm{JAZ}}", AMBIENT_GREEN),
        ])
        pump_lbl.clear_updaters()
        el_lbl.clear_updaters()
        sources = VGroup(pump_lbl[0].copy(), el_lbl[0].copy())
        self.play(Create(eq_box), FadeIn(items["jaz"]), FadeIn(eq[1]),
                  ReplacementTransform(sources, items["frac"]),
                  Circumscribe(hp_out.ribbons, color=NUTZ_BLUE, buff=0.06, stroke_width=3, fade_out=True),
                  Circumscribe(el_in.ribbons, color=P_YELLOW, buff=0.06, stroke_width=3, fade_out=True),
                  *live(2.0), run_time=2.0)
        self.play(FadeIn(items["num"]), *live(1.0), run_time=1.0)
        self.play(FadeIn(items["eg"]), Circumscribe(wp_row["group"], color=AMBIENT_GREEN, buff=0.08, stroke_width=3,
                                                    fade_out=True), *live(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "jaz", during=live)

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion
