#region Beat4 – Stack effect: height makes the pressure
S_LOW = _stack_chain(2.5, T_OUT_NIGHT, T_IN_SUMMER)
S_TALL = _stack_chain(4.4, T_OUT_NIGHT, T_IN_SUMMER)
S_WINTER = _stack_chain(4.4, T_OUT_WINTER, T_IN_WINTER)
_SAY = {
    "re": (S_LOW["rho_e"], 3), "ri": (S_LOW["rho_i"], 3), "drho": (S_LOW["drho"], 3), "dp": (S_LOW["dp"], 1),
    "v": (S_LOW["v"], 1), "q": (S_LOW["vdot"], 3), "qh": (round(S_LOW["vdot_h"], -1), 0),
    "dpt": (S_TALL["dp"], 1), "dpw": (S_WINTER["dp"], 1), "ratio": (S_WINTER["dp"] / S_TALL["dp"], 1),
}


class Beat4_Auftrieb(Scene):
    NARRATION = [
        ("intro",
         "When the wind drops, buoyancy still moves air: warm room air is lighter, rises and leaves through a high opening, while cool air is drawn in through a low opening.",
         "Fällt der Wind aus, bleibt der Auftrieb: warme Raumluft ist leichter, steigt und verlässt den Raum durch eine hohe Öffnung, kühle Luft strömt durch eine tiefe nach."),
        ("shaft",
         "A stairwell or a chimney-like shaft carries the warm air up to the high outlet opening, so the flow keeps running even on a still day.",
         "Treppenhaus oder Schacht (Kamin) führt die warme Luft zur hohen Abluftöffnung — auch an windstillen Tagen."),
        ("density",
         r"Why does it rise? Warm air is lighter: the density $\rho(\theta)$ falls as the temperature $\theta$ rises.",
         r"Warum steigt sie? Warme Luft ist leichter: die Dichte $\rho(\theta)$ sinkt mit steigender Temperatur $\theta$."),
        ("drho",
         _say(r"Outside ($\theta_e = 18\,\mathrm{°C}$) air has $\rho = @re@\,\mathrm{kg/m^3}$, inside ($\theta_i = 26\,\mathrm{°C}$) $@ri@\,\mathrm{kg/m^3}$: $\Delta\rho = @drho@\,\mathrm{kg/m^3}$.", _SAY, False),
         _say(r"Außen ($\theta_e = 18\,\mathrm{°C}$) hat Luft $\rho = @re@\,\mathrm{kg/m^3}$, innen ($\theta_i = 26\,\mathrm{°C}$) $@ri@\,\mathrm{kg/m^3}$: $\Delta\rho = @drho@\,\mathrm{kg/m^3}$.", _SAY, True)),
        ("pressure",
         _say(r"Over the height $h$ between inlet and outlet this becomes a pressure: $\Delta p = \Delta\rho \cdot g \cdot h = @dp@\,\mathrm{Pa}$ for $h = 2.5\,\mathrm{m}$.", _SAY, False),
         _say(r"Über die Höhe $h$ zwischen Zu- und Abluftöffnung wird daraus ein Druck: $\Delta p = \Delta\rho \cdot g \cdot h = @dp@\,\mathrm{Pa}$ bei $h = 2{,}5\,\mathrm{m}$.", _SAY, True)),
        ("speed",
         _say(r"The pressure accelerates the air: $v = \sqrt{2\,\Delta p/\rho}$ gives $@v@\,\mathrm{m/s}$ with $\rho \approx 1.2\,\mathrm{kg/m^3}$ (20 °C).", _SAY, False),
         _say(r"Der Druck beschleunigt die Luft: $v = \sqrt{2\,\Delta p/\rho}$ ergibt $@v@\,\mathrm{m/s}$ mit $\rho \approx 1{,}2\,\mathrm{kg/m^3}$ (20 °C).", _SAY, True)),
        ("flow",
         _say(r"Speed times open area is the flow: $\dot{V} = C_d \cdot A \cdot v = @q@\,\mathrm{m^3/s} \approx @qh@\,\mathrm{m^3/h}$ for $C_d \approx 0.6$ and $A = 0.1\,\mathrm{m^2}$.", _SAY, False),
         _say(r"Geschwindigkeit mal Öffnungsfläche ist der Volumenstrom: $\dot{V} = C_d \cdot A \cdot v = @q@\,\mathrm{m^3/s} \approx @qh@\,\mathrm{m^3/h}$ bei $C_d \approx 0{,}6$ und $A = 0{,}1\,\mathrm{m^2}$.", _SAY, True)),
        ("taller",
         _say(r"Raise the outlet and $h$ grows: the shaft lifts it from $2.5\,\mathrm{m}$ to $4.4\,\mathrm{m}$, so $\Delta p$ rises to $@dpt@\,\mathrm{Pa}$ and the flow speeds up.", _SAY, False),
         _say(r"Höherer Auslass, größeres $h$: der Schacht hebt es von $2{,}5\,\mathrm{m}$ auf $4{,}4\,\mathrm{m}$, $\Delta p$ steigt auf $@dpt@\,\mathrm{Pa}$, der Strom wird schneller.", _SAY, True)),
        ("winter",
         _say(r"Winter: $\Delta\theta = 20\,\mathrm{K}$ instead of $8\,\mathrm{K}$ gives the same shaft $@dpw@\,\mathrm{Pa}$, about $@ratio@$ times the pressure. Buoyancy is strongest in the cold.", _SAY, False),
         _say(r"Im Winter: $\Delta\theta = 20\,\mathrm{K}$ statt $8\,\mathrm{K}$ ergibt im selben Schacht $@dpw@\,\mathrm{Pa}$, das $@ratio@$-fache. Bei Kälte wirkt der Auftrieb am stärksten.", _SAY, True)),
        ("comfort",
         "That is why inlets sit low, near the people, and outlets sit high at the roof: comfort without a fan.",
         "Deshalb sitzen Zuluftöffnungen tief bei den Menschen und Abluftöffnungen hoch am Dach — Komfort ohne Ventilator."),
    ]

    SHAFT_X = -1.60
    SHAFT_HW = 0.42
    TOP_LOW = 0.75
    TOP_HIGH = 2.05

    def _stack_paths(self, inlet, room_top, shaft_top):
        """〰️ Inlet → occupied zone → up the shaft → out at the top."""
        return VGroup(*[
            smooth_path([
                inlet + RIGHT * 0.06 + UP * (0.06 * i),
                np.array([-3.20, -0.62 + 0.10 * i, 0.0]),
                np.array([self.SHAFT_X, room_top + 0.10, 0.0]),
                np.array([self.SHAFT_X, shaft_top - 0.08, 0.0]),
            ])
            for i in range(3)
        ])

    def construct(self):
        apply_scene_style(self)
        stage = _morph.stage(self, TRANSITIONS)
        N = self.NARRATION

        subtitle = section_heading(tr("Stack effect: height creates pressure", "Auftrieb: Höhe erzeugt Druck"))
        din = _din_ref("DIN EN 16798-7")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        room_c = np.array([-2.90, -0.45, 0.0])
        shell = _room(room_c, 4.2, 1.7, window_side="left", window_y=-0.85, window_h=0.60)
        room, floor = shell[0], shell[1]
        room_top = room.get_top()[1]
        inlet = Rectangle(width=0.16, height=0.60, color=P_CYAN, stroke_width=2.5, fill_opacity=0.0)
        inlet.move_to(np.array([room_c[0] - 2.1, -0.85, 0.0]))
        inlet_arrow = Arrow(inlet.get_center() + LEFT * 0.95, inlet.get_center() + LEFT * 0.12, buff=0, color=P_CYAN,
                            stroke_width=4, max_tip_length_to_length_ratio=0.28)
        inlet_lbl = Text(tr("Inlet opening", "Zuluftöffnung"), font_size=LABEL_FONT_SIZE, color=P_CYAN)
        inlet_lbl.next_to(inlet_arrow, UP, buff=0.10, aligned_edge=RIGHT)
        person = _stand(room_c, 1.7, -1.75)

        shaft_walls = VGroup(*[
            Line(
                np.array([self.SHAFT_X + s * self.SHAFT_HW, room_top, 0.0]),
                np.array([self.SHAFT_X + s * self.SHAFT_HW, self.TOP_LOW, 0.0]),
                color=P_ORANGE, stroke_width=3.0,
            )
            for s in (-1, 1)
        ])
        ceiling_gap = Rectangle(width=2 * self.SHAFT_HW - 0.07, height=0.18, stroke_width=0, fill_color=P_DEEP_DARK,
                                fill_opacity=1.0).move_to(np.array([self.SHAFT_X, room_top - 0.08, 0.0]))
        outlet = DashedLine(
            np.array([self.SHAFT_X - self.SHAFT_HW, self.TOP_LOW, 0.0]),
            np.array([self.SHAFT_X + self.SHAFT_HW, self.TOP_LOW, 0.0]),
            color=P_ORANGE, stroke_width=3.0, dash_length=0.09,
        )
        outlet_arrow = Arrow(outlet.get_center() + UP * 0.05, outlet.get_center() + UP * 0.62, buff=0, color=P_ORANGE,
                             stroke_width=4, max_tip_length_to_length_ratio=0.3)
        outlet_lbl = Text(tr("Outlet opening", "Abluftöffnung"), font_size=LABEL_FONT_SIZE, color=P_ORANGE)
        outlet_lbl.next_to(outlet, LEFT, buff=0.25)
        outlet_group = VGroup(outlet, outlet_arrow, outlet_lbl)

        def _legend(season, season_color, t_out, t_in):
            def line(sym, t, rho, tag, c):
                return VGroup(
                    math_label(rf"\rho(\theta_{{{sym}}}) = {label_number(rho, 3)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}}", size=LABEL_FONT_SIZE, color=c),
                    VGroup(math_label(rf"\theta_{{{sym}}} = {label_number(t)}\,\mathrm{{°C}}", size=LABEL_FONT_SIZE, color=P_WHITE),
                           Text(tag, font_size=LABEL_FONT_SIZE, color=P_WHITE)).arrange(RIGHT, buff=0.16),
                ).arrange(DOWN, aligned_edge=LEFT, buff=0.06)
            rows = VGroup(
                Text(season, font_size=LABEL_FONT_SIZE, color=season_color),
                line("e", t_out, _rho_air(t_out), tr("outside · cool · heavy", "außen · kühl · schwer"), P_BLUE),
                line("i", t_in, _rho_air(t_in), tr("inside · warm · light", "innen · warm · leicht"), P_ORANGE),
            ).arrange(DOWN, aligned_edge=LEFT, buff=0.14)
            box = SurroundingRectangle(rows, color=P_PRIMARY, corner_radius=0, buff=_theme_module.UI_SPACING, stroke_width=_theme_module.CHROME_BORDER)
            return VGroup(box, rows).move_to(np.array([1.45, 1.20, 0.0]))

        legend = _legend(tr("Summer night", "Sommernacht"), P_BLUE, T_OUT_NIGHT, T_IN_SUMMER)
        legend_winter = _legend("Winter", P_CYAN, T_OUT_WINTER, T_IN_WINTER)
        legend_rows = legend[1]

        scaffold = VGroup(room, floor, inlet, inlet_arrow, inlet_lbl, person, shaft_walls, outlet_group, legend)
        fit_band(scaffold, bottom=SAFE_BOTTOM_FORMULA)

        warm_spots = [person.get_top() + UP * 0.05, np.array([-2.30, floor.get_top()[1] + 0.02, 0.0])]
        stack = {"paths": None, "speed": 0.65}

        def warmth(rt, r_max=0.55):
            return [ripples(warm_spots, r_max=r_max, color=P_ORANGE, cycles=rt / 1.4)]

        def rising(rt):
            return [_streams([(stack["paths"], P_CYAN, P_ORANGE)], rt, speed=stack["speed"]), *warmth(rt, 0.4)]

        height, t_out_tr, t_in_tr = ValueTracker(2.5), ValueTracker(T_OUT_NIGHT), ValueTracker(T_IN_SUMMER)

        def chain():
            return _stack_chain(height.get_value(), t_out_tr.get_value(), t_in_tr.get_value())

        x_read = 3.95
        out_dp = math_readout(lambda: rf"\Delta p = {label_number(chain()['dp'], 1)}\,\mathrm{{Pa}}",
                              np.array([x_read, 1.80, 0.0]), size=BODY_FONT_SIZE, color=P_YELLOW)
        out_v = math_readout(lambda: rf"v = {label_number(chain()['v'], 1)}\,\mathrm{{m/s}}",
                             np.array([x_read, 1.20, 0.0]), size=BODY_FONT_SIZE, color=P_CYAN)
        out_q = math_readout(lambda: rf"\dot{{V}} = {label_number(round(chain()['vdot_h'], -1))}\,\mathrm{{m^{{3}}/h}}",
                             np.array([x_read, 0.60, 0.0]), size=BODY_FONT_SIZE, color=P_GREEN)

        def ru(value, digits=0):
            return label_number(value, digits)

        def panel_density(s):
            kgm3 = r"\,\frac{\mathrm{kg}}{\mathrm{m^{3}}}"
            return math_panel([
                ("drho", r"\Delta\rho", P_BLUE), (None, "=", P_WHITE),
                ("diff", r"\rho(\theta_{e}) - \rho(\theta_{i})", P_BLUE), (None, "=", P_WHITE),
                ("val", rf"{ru(s['rho_e'], 3)}{kgm3} - {ru(s['rho_i'], 3)}{kgm3} = {ru(s['drho'], 3)}{kgm3}", P_YELLOW),
            ], color=P_BLUE)

        def panel_dp(s, h):
            return math_panel([
                ("dp", r"\Delta p", P_YELLOW), (None, "=", P_WHITE),
                ("drho", r"\Delta\rho", P_BLUE), (None, r"\cdot", P_WHITE),
                ("g", "g", P_WHITE), (None, r"\cdot", P_WHITE),
                ("h", "h", P_CYAN), (None, "=", P_WHITE),
                ("val", rf"{ru(s['drho'], 3)}\,\frac{{\mathrm{{kg}}}}{{\mathrm{{m^{{3}}}}}} \cdot {ru(G_ACC, 2)}\,\frac{{\mathrm{{m}}}}{{\mathrm{{s^{{2}}}}}} \cdot {ru(h, 1)}\,\mathrm{{m}} = {ru(s['dp'], 1)}\,\mathrm{{Pa}}", P_YELLOW),
            ], color=P_YELLOW)

        def panel_v(s):
            return math_panel([
                ("v", "v", P_CYAN), (None, "=", P_WHITE),
                ("root", r"\sqrt{\frac{2\,\Delta p}{\rho}}", P_WHITE), (None, "=", P_WHITE),
                ("val", rf"\sqrt{{\frac{{2 \cdot {ru(s['dp'], 1)}\,\mathrm{{Pa}}}}{{{ru(RHO_AIR, 1)}\,\mathrm{{kg/m^{{3}}}}}}}} = {ru(s['v'], 1)}\,\mathrm{{m/s}}", P_CYAN),
            ], color=P_CYAN)

        def panel_q(s):
            return math_panel([
                ("q", r"\dot{V}", P_GREEN), (None, "=", P_WHITE),
                ("cd", r"C_{d}", P_WHITE), (None, r"\cdot", P_WHITE),
                ("a", "A", P_CYAN), (None, r"\cdot", P_WHITE),
                ("v", "v", P_CYAN), (None, "=", P_WHITE),
                ("val", rf"{ru(CD_OPENING, 1)} \cdot {ru(A_OPENING, 1)}\,\mathrm{{m^{{2}}}} \cdot {ru(s['v'], 1)}\,\mathrm{{m/s}} = {ru(s['vdot'], 3)}\,\mathrm{{m^{{3}}/s}} \approx {ru(round(s['vdot_h'], -1))}\,\mathrm{{m^{{3}}/h}}", P_GREEN),
            ], color=P_GREEN)

        def swap_panel(old, new):
            return [ReplacementTransform(old[0], new[0]), ReplacementTransform(old[1], new[1])]

        self.play(Create(room), Create(floor), Create(inlet), GrowArrow(inlet_arrow), FadeIn(inlet_lbl), FadeIn(person), run_time=1.4)
        self.play(*warmth(1.1), run_time=1.1)
        hold_for(self, N, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.1, during=warmth)

        caption = swap_caption(self, caption, subtitle_text(N, "shaft"))
        self.play(Create(shaft_walls), FadeIn(ceiling_gap), Create(outlet), GrowArrow(outlet_arrow), FadeIn(outlet_lbl), *warmth(1.2), run_time=1.2)
        stack["paths"] = stack_low = self._stack_paths(inlet.get_center(), room_top, self.TOP_LOW)
        guides_low = flow_guides(stack_low, P_ORANGE)
        self.play(Create(guides_low), *warmth(0.6), run_time=0.6)
        self.play(*rising(2.6), run_time=2.6)
        hold_for(self, N, "shaft", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "density"))
        panel = panel_density(S_LOW)
        self.play(FadeIn(legend), *rising(1.2), run_time=1.2)
        hold_for(self, N, "density", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "drho"))
        self.play(FadeIn(panel[0]), Create(panel[1]), Indicate(legend_rows[1], color=P_BLUE), Indicate(legend_rows[2], color=P_ORANGE),
                  *rising(1.4), run_time=1.4)
        hold_for(self, N, "drho", during=rising)

        h_dim = dim_arrow(
            np.array([-0.30, inlet.get_center()[1], 0.0]),
            np.array([-0.30, self.TOP_LOW, 0.0]),
            color=P_CYAN,
        )
        h_tok = math_label(tr(r"h = 2.5\,\mathrm{m}", r"h = 2{,}5\,\mathrm{m}"), size=LABEL_FONT_SIZE, color=P_CYAN)
        h_tok.next_to(h_dim, RIGHT, buff=0.14)
        h_tok_tall = math_label(tr(r"h = 4.4\,\mathrm{m}", r"h = 4{,}4\,\mathrm{m}"), size=LABEL_FONT_SIZE, color=P_CYAN)

        caption = swap_caption(self, caption, subtitle_text(N, "pressure"))
        new = panel_dp(S_LOW, 2.5)
        self.play(*swap_panel(panel, new), Create(h_dim), FadeIn(h_tok), FadeIn(out_dp), *rising(1.2), run_time=1.2)
        panel = new
        ring_h = highlight_param(panel[2], "h", color=P_CYAN)
        self.play(Create(ring_h), *rising(0.8), run_time=0.8)
        hold_for(self, N, "pressure", during=rising)
        self.play(FadeOut(ring_h), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "speed"))
        new = panel_v(S_LOW)
        stack["speed"] = 0.9
        self.play(*swap_panel(panel, new), FadeIn(out_v), *rising(1.2), run_time=1.2)
        panel = new
        ring_v = highlight_param(panel[2], "root", color=P_CYAN)
        self.play(Create(ring_v), *rising(0.8), run_time=0.8)
        hold_for(self, N, "speed", during=rising)
        self.play(FadeOut(ring_v), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "flow"))
        new = panel_q(S_LOW)
        a_lbl = math_label(rf"A = {ru(A_OPENING, 1)}\,\mathrm{{m^{{2}}}}", size=LABEL_FONT_SIZE, color=P_CYAN)
        a_lbl.next_to(inlet_arrow, DOWN, buff=0.10, aligned_edge=RIGHT)
        self.play(*swap_panel(panel, new), FadeIn(out_q), FadeIn(a_lbl), Indicate(inlet, color=P_CYAN, scale_factor=1.3), *rising(1.2), run_time=1.2)
        panel = new
        ring_q = highlight_param(panel[2], "a", color=P_CYAN)
        self.play(Create(ring_q), *rising(0.8), run_time=0.8)
        hold_for(self, N, "flow", during=rising)
        self.play(FadeOut(ring_q), *rising(0.25), run_time=0.25)

        caption = swap_caption(self, caption, subtitle_text(N, "taller"))
        rise = self.TOP_HIGH - self.TOP_LOW
        h_tok_tall.move_to(h_tok.get_center() + UP * rise / 2)
        tall_dim = dim_arrow(
            np.array([-0.30, inlet.get_center()[1], 0.0]),
            np.array([-0.30, self.TOP_HIGH, 0.0]),
            color=P_CYAN,
        )
        new = panel_dp(S_TALL, 4.4)
        self.play(
            *[
                wall.animate.put_start_and_end_on(
                    wall.get_start(), wall.get_end() + UP * rise,
                )
                for wall in shaft_walls
            ],
            outlet_group.animate.shift(UP * rise),
            stage.transition("ventilation.natural.stack-effect.height-dimension-taller", source=h_dim, target=tall_dim),
            stage.transition("ventilation.natural.stack-effect.height-token-taller", source=h_tok, target=h_tok_tall),
            FadeOut(guides_low),
            height.animate.set_value(4.4),
            *swap_panel(panel, new),
            run_time=1.6,
        )
        panel = new
        stack["paths"] = stack_high = self._stack_paths(inlet.get_center(), room_top, self.TOP_HIGH)
        stack["speed"] = 1.1
        self.play(Create(flow_guides(stack_high, P_ORANGE)), run_time=0.5)
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, N, "taller", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        stack["speed"] = 1.8
        new = panel_dp(S_WINTER, 4.4)
        self.play(
            stage.transition("ventilation.natural.stack-effect.legend-winter", source=legend, target=legend_winter),
            t_out_tr.animate.set_value(T_OUT_WINTER), t_in_tr.animate.set_value(T_IN_WINTER),
            *swap_panel(panel, new),
            *rising(1.6),
            run_time=1.6,
        )
        panel = new
        self.play(*rising(2.4), run_time=2.4)
        hold_for(self, N, "winter", during=rising)

        caption = swap_caption(self, caption, subtitle_text(N, "comfort"))
        low_ring = VGroup(
            SurroundingRectangle(VGroup(inlet, inlet_arrow), color=P_CYAN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5),
            SurroundingRectangle(person, color=P_CYAN, buff=_theme_module.UI_SPACING, corner_radius=0, stroke_width=2.5),
        )
        high_ring = SurroundingRectangle(VGroup(outlet, outlet_arrow), color=P_ORANGE, buff=_theme_module.UI_SPACING,
                                         corner_radius=0, stroke_width=2.5)
        self.play(Create(low_ring), Create(high_ring), *rising(1.0), run_time=1.0)
        self.play(Indicate(person, scale_factor=1.0), *rising(0.9), run_time=0.9)
        hold_for(self, N, "comfort", during=rising)
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
