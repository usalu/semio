#region Beat3 — g-Wert
# Ten parallel rays carry 10 % each, so every share is a count of rays: two
# bounce back, three end in the glass, five pass; the warm glass then emits one
# ripple inward and two outward.
RAY_SHARE = 0.10


def _glazing(x: float, y0: float, y1: float):
    """🪟 Double glazing in section: two panes, spacer and frame blocks."""
    panes = VGroup(*[
        Rectangle(width=0.07, height=y1 - y0, stroke_color=COLOR_WIN, stroke_width=2,
                  fill_color=COLOR_WIN, fill_opacity=0.15).move_to([x + dx, (y0 + y1) / 2, 0])
        for dx in (-0.13, 0.13)
    ])
    frames = VGroup(*[
        Rectangle(width=0.5, height=0.22, stroke_color=PASTEL_WHITE, stroke_width=2,
                  fill_color=PASTEL_WHITE, fill_opacity=0.3).move_to([x, y, 0])
        for y in (y0 - 0.11, y1 + 0.11)
    ])
    return panes, frames


class Beat3_GWert(Scene):
    """🪟 Energy split at the glazing: reflected, absorbed, transmitted — g is what reaches the room."""

    NARRATION = [
        ("section",
         "A section through the glazing: outside on the left, inside on the right.",
         "Ein Schnitt durch die Verglasung: links außen, rechts innen."),
        ("split",
         "Of the solar radiation about 20 percent is reflected, 30 percent taken up by the glass and 50 percent let straight through.",
         "Von der Sonnenstrahlung werden rund 20 % reflektiert, 30 % vom Glas aufgenommen und 50 % direkt durchgelassen."),
        ("gval",
         "The warm glass gives its heat off to both sides. Inside count 50 plus 10 percent — g is about 0.6.",
         "Das warme Glas gibt seine Wärme nach beiden Seiten ab. Innen zählen 50 plus 10 % — g ist rund 0,6."),
        ("phi",
         "The g-value is dimensionless and enters the equation as one more factor.",
         "Der g-Wert ist dimensionslos und kommt als weiterer Faktor in die Gleichung."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Gesamtenergiedurchlassgrad g", title)
        din = _din_ref("DIN EN 410 · DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "section"))
        self.play(FadeIn(caption), run_time=0.3)

        #region glazing section
        x_in, x_out, x_end = -0.2, 0.2, 6.0
        panes, frames = _glazing(0.0, -1.35, 1.75)
        side_out = Text("außen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([-6.1, 1.55, 0])
        side_in = Text("innen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([6.1, 1.55, 0])
        self.play(Create(panes), FadeIn(frames), FadeIn(side_out), FadeIn(side_in), run_time=1.2)
        hold_for(self, N, "section", used=1.2 + 0.3)
        #endregion

        #region split at the glass
        caption = swap_caption(self, caption, subtitle_text(N, "split"))
        n_r, n_a = round(SHARE_R / RAY_SHARE), round(SHARE_ABS / RAY_SHARE)
        ys = [1.25 - 0.2 * i for i in range(round(1 / RAY_SHARE))]
        sun_c = np.array([-6.15, float(np.mean(ys)), 0.0])
        sun = _sun(sun_c)
        x0 = sun_c[0] + 0.55
        incoming = VGroup(*[_soft_ray([x0, y, 0], [x_in, y, 0], width=2.0) for y in ys])
        r_dir = np.array([-1.0, 0.3, 0.0]) / np.linalg.norm([-1.0, 0.3])
        refl_ends = [np.array([x_in, y, 0]) + r_dir * (x_in + 2.9) / -r_dir[0] for y in ys[:n_r]]
        reflected = VGroup(*[radiation_ray([x_in, y, 0], e, color=COLOR_REFL, stroke_width=2.0).set_stroke(opacity=0.7)
                             for y, e in zip(ys, refl_ends)])
        t_ys = ys[n_r + n_a:]
        transmitted = VGroup(*[VGroup(_soft_ray([x_in, y, 0], [x_out, y, 0], width=2.0, opacity=0.6, fade=False),
                                      _soft_ray([x_out, y, 0], [x_end, y, 0], width=2.0, opacity=0.7, fade=False))
                               for y in t_ys])
        in_lbl = Text("100 %", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE).move_to([-4.6, ys[-1] - 0.32, 0])
        refl_lbl = Text("reflektiert 20 %", font_size=LABEL_FONT_SIZE, color=COLOR_REFL)
        refl_lbl.move_to([-3.1 - refl_lbl.width / 2, float(np.mean([e[1] for e in refl_ends])), 0])
        trans_lbl = Text("durchgelassen 50 %", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE).move_to([3.1, t_ys[0] + 0.35, 0])
        abs_lbl = Text("im Glas 30 %", font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT).move_to([0.0, 2.15, 0])
        light_paths = [[[x0, y, 0], [x_in, y, 0], e] for y, e in zip(ys, refl_ends)]
        light_paths += [[[x0, y, 0], [x_in, y, 0]] for y in ys[n_r:n_r + n_a]]
        light_paths += [[[x0, y, 0], [x_end, y, 0]] for y in t_ys]

        def sunshine(rt):
            return [_pulses(light_paths, rt, every=1.8)]

        self.play(FadeIn(sun, scale=0.7), run_time=0.5)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in incoming], lag_ratio=0.06), FadeIn(in_lbl),
                  run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in reflected], lag_ratio=0.2), FadeIn(refl_lbl),
                  LaggedStart(*[Create(r, rate_func=linear) for r in transmitted], lag_ratio=0.1), FadeIn(trans_lbl),
                  panes.animate.set_fill(COLOR_GLASS_HEAT, opacity=0.7), FadeIn(abs_lbl), run_time=1.8)
        hold_for(self, N, "split", during=sunshine)
        #endregion

        #region secondary heat and g
        caption = swap_caption(self, caption, subtitle_text(N, "gval"))
        n_in, n_out = round(SHARE_IN / RAY_SHARE), round(SHARE_OUT / RAY_SHARE)
        face = 0.13 + 0.035
        heat_y = -1.05
        warm_in = [np.array([face, heat_y - 0.25 * k, 0]) for k in range(n_in)]
        warm_out = [np.array([-face, heat_y + 0.1 - 0.25 * k, 0]) for k in range(n_out)]
        in_heat_lbl = Text("Wärme nach innen 10 %", font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT)
        in_heat_lbl.move_to([0.8 + in_heat_lbl.width / 2, heat_y, 0])
        out_heat_lbl = Text("Wärme nach außen 20 %", font_size=LABEL_FONT_SIZE, color=COLOR_GLASS_HEAT)
        out_heat_lbl.move_to([-0.8 - out_heat_lbl.width / 2, heat_y - 0.05, 0])

        def glass_heat(rt):
            cycles = max(1.0, rt / 1.4)
            return [*sunshine(rt), ripples(warm_in, r_max=0.4, color=COLOR_GLASS_HEAT, cycles=cycles, facing=0.0),
                    ripples(warm_out, r_max=0.4, color=COLOR_GLASS_HEAT, cycles=cycles, facing=PI)]

        self.play(FadeIn(in_heat_lbl), FadeIn(out_heat_lbl), panes.animate.set_fill(COLOR_GLASS_HEAT, opacity=0.45),
                  *glass_heat(1.6), run_time=1.6)
        g_top, g_bot = t_ys[0] + 0.1, heat_y - 0.2
        brace = BraceBetweenPoints([6.2, g_top, 0], [6.2, g_bot, 0], direction=RIGHT, color=COLOR_GVAL)
        g_lbl = math_label(rf"g \approx {de_num(G_VALUE, 1)}", size=FORMULA_FONT_SIZE, color=COLOR_GVAL)
        g_lbl.next_to(brace, UP, buff=0.15).shift(LEFT * 0.3)
        self.play(GrowFromCenter(brace), FadeIn(g_lbl, shift=UP * 0.1), *glass_heat(1.0), run_time=1.0)
        hold_for(self, N, "gval", during=glass_heat)
        #endregion

        #region into the formula
        caption = swap_caption(self, caption, subtitle_text(N, "phi"))
        g_sym = math_label("g", size=FORMULA_FONT_SIZE, color=COLOR_GVAL)
        g_sym.move_to(g_lbl.get_left() + RIGHT * g_sym.width / 2)
        panel = math_panel([
            ("phi", r"\Phi", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g_irr", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("g", "g", COLOR_GVAL), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ])
        _fly_into(self, panel, {"g": g_sym})
        ring = highlight_param(panel[2], "g", color=COLOR_GVAL)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "phi", during=glass_heat)
        self.play(FadeOut(ring), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
