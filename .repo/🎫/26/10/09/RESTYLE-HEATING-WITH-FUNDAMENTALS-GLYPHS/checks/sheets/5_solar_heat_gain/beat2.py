#region Beat2 — Bestrahlung und Fläche
def _bands(x0, x1, y0, y1, t: float, **style):
    """🔲 Four filled bands of thickness ``t`` lining the rectangle ``x0..x1 × y0..y1`` — a wall or frame around a hole."""
    return VGroup(*[
        Rectangle(width=w, height=h, **style).move_to([cx, cy, 0])
        for w, h, cx, cy in (
            (x1 - x0, t, (x0 + x1) / 2, y1 - t / 2), (x1 - x0, t, (x0 + x1) / 2, y0 + t / 2),
            (t, y1 - y0 - 2 * t, x0 + t / 2, (y0 + y1) / 2), (t, y1 - y0 - 2 * t, x1 - t / 2, (y0 + y1) / 2),
        )
    ])


class Beat2_BestrahlungUndFlaeche(Scene):
    """🪟 Irradiance G, area A and glass share F_f — read off the drawn window as symbols, then one formula."""

    NARRATION = [
        ("aperture",
         "Start from the full window opening in the wall.",
         "Zuerst die gesamte Fensteröffnung in der Wand."),
        ("g",
         "Irradiance G is the solar power that reaches each square metre of the facade.",
         "Die Bestrahlungsstärke G ist die Sonnenleistung, die auf jeden Quadratmeter der Fassade trifft."),
        ("a",
         "Area A is the window opening that the radiation can hit.",
         "Die Fläche A ist die Fensteröffnung, auf die die Strahlung trifft."),
        ("ff",
         "The frame blocks; only the glass share F-f lets the radiation through.",
         "Der Rahmen blockiert, nur der Glasanteil F-f lässt die Strahlung durch."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Bestrahlungsstärke G und Fläche A", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "aperture"))
        self.play(FadeIn(caption), run_time=0.3)

        #region opening
        s = 1.1
        oc = np.array([-2.9, 0.35, 0.0])
        ow, oh = WIN_W * s, WIN_H * s
        ox0, ox1, oy0, oy1 = oc[0] - ow / 2, oc[0] + ow / 2, oc[1] - oh / 2, oc[1] + oh / 2
        slab = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)
        wall = VGroup(
            Rectangle(width=3.6, height=oc[1] + 1.65 - oy1, **slab).move_to([oc[0], (oy1 + oc[1] + 1.65) / 2, 0]),
            Rectangle(width=3.6, height=oy0 - (oc[1] - 1.65), **slab).move_to([oc[0], (oy0 + oc[1] - 1.65) / 2, 0]),
            Rectangle(width=1.8 - ow / 2, height=oh, **slab).move_to([(oc[0] - 1.8 + ox0) / 2, oc[1], 0]),
            Rectangle(width=1.8 - ow / 2, height=oh, **slab).move_to([(oc[0] + 1.8 + ox1) / 2, oc[1], 0]),
        )
        opening = Rectangle(width=ow, height=oh, color=COLOR_A, stroke_width=3).move_to(oc)
        self.play(Create(wall), Create(opening), run_time=1.3)
        hold_for(self, N, "aperture", used=1.3 + 0.3)
        #endregion

        #region irradiance
        caption = swap_caption(self, caption, subtitle_text(N, "g"))
        sun_c = np.array([4.4, 1.6, 0.0])
        sun = _sun(sun_c)
        frame_t = FRAME_FACE * s
        frame_hits = [np.array(p) for p in ((ox1 - frame_t / 2, 0.95, 0), (oc[0] + 0.25, oy1 - frame_t / 2, 0),
                                            (oc[0] - 0.35, oy0 + frame_t / 2, 0), (ox0 + frame_t / 2, 0.2, 0))]
        glass_hits = [np.array(p) for p in ((-3.12, 0.92, 0), (-2.6, 0.78, 0), (-3.15, -0.12, 0),
                                            (-2.58, -0.2, 0), (-2.88, -0.28, 0))]
        aim, starts = _parallel_starts(sun_c, frame_hits + glass_hits, gap=0.55)
        rays = VGroup(*[_soft_ray(a, h, width=2.0) for a, h in zip(starts, frame_hits + glass_hits)])
        frame_rays, glass_rays = rays[:len(frame_hits)], rays[len(frame_hits):]
        all_paths = [[a, h] for a, h in zip(starts, frame_hits + glass_hits)]
        glass_paths = all_paths[len(frame_hits):]
        g_lbl = math_label("G", sun_c + DOWN * 1.15, size=FORMULA_FONT_SIZE, color=COLOR_G)

        def sunshine(rt):
            return [_pulses(all_paths, rt, every=1.8)]

        self.play(FadeIn(sun, scale=0.7), run_time=0.7)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in rays], lag_ratio=0.08), FadeIn(g_lbl),
                  run_time=1.6)
        hold_for(self, N, "g", during=sunshine)
        #endregion

        #region area A
        caption = swap_caption(self, caption, subtitle_text(N, "a"))
        area = Rectangle(width=ow, height=oh, stroke_width=0, fill_color=COLOR_A, fill_opacity=0.35).move_to(oc)
        a_lbl = math_label("A", oc, size=FORMULA_FONT_SIZE, color=COLOR_A)
        self.play(GrowFromEdge(area, DOWN), FadeIn(a_lbl), sunshine(1.2)[0], run_time=1.2)
        hold_for(self, N, "a", during=sunshine)
        #endregion

        #region glass share F_f
        caption = swap_caption(self, caption, subtitle_text(N, "ff"))
        frame = _bands(ox0, ox1, oy0, oy1, frame_t, color=PASTEL_WHITE, stroke_width=1.5, fill_color=PASTEL_WHITE,
                       fill_opacity=0.45)
        glass = Rectangle(width=(WIN_W - 2 * FRAME_FACE) * s, height=(WIN_H - 2 * FRAME_FACE) * s,
                          color=COLOR_WIN, fill_color=COLOR_WIN, fill_opacity=0.15, stroke_width=2).move_to(oc)
        ff_lbl = math_label(r"F_{\mathrm{f}}", oc, size=FORMULA_FONT_SIZE, color=COLOR_FF)
        self.play(FadeOut(area), FadeIn(frame), run_time=0.7)
        self.play(TransformFromCopy(opening, glass), ReplacementTransform(a_lbl, ff_lbl),
                  frame_rays.animate.set_stroke(color=GREY_C, opacity=[0.0, 0.35, 0.35]), run_time=1.0)
        self.add(frame_rays, frame, glass, glass_rays)
        self.bring_to_front(ff_lbl)
        a_side = math_label("A", oc + LEFT * (ow / 2 + 0.45), size=FORMULA_FONT_SIZE, color=COLOR_A)
        self.play(FadeIn(a_side), run_time=0.5)

        def through_glass(rt):
            return [_pulses(glass_paths, rt, every=1.8), glass.animate.set_fill(COLOR_WIN, opacity=0.3)]

        panel = math_panel([
            ("phi", r"\Phi", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ])
        _fly_into(self, panel, {"g": g_lbl, "a": a_side, "ff": ff_lbl})
        ring = highlight_param(panel[2], "ff", color=COLOR_FF)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "ff", during=through_glass)
        self.play(FadeOut(ring), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
