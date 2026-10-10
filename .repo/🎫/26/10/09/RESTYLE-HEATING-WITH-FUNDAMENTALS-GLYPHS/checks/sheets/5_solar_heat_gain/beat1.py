#region Beat1 — Verlust zu Gewinn
class Beat1_VerlustZuGewinn(Scene):
    """🔄 Envelope heat loss next to free solar gain."""

    NARRATION = [
        ("loss",
         "Outside heat leaves through the envelope as transmission loss.",
         "Wärme verlässt die Hülle als Transmissionsverlust."),
        ("flip",
         "At the same time the sun supplies free heat.",
         "Gleichzeitig liefert die Sonne freie Wärme."),
        ("gain",
         "Solar heat gain arrives at roof, walls, and windows.",
         "Solarer Wärmegewinn trifft Dach, Wände und Fenster."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Von Wärmeverlust zu solarem Gewinn", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "loss"))
        self.play(FadeIn(caption), run_time=0.3)

        #region house and transmission loss
        house = house_section(np.array([-0.2, -0.62, 0.0]), scale=1.15)
        x_l, x_r = house["bottom_left"][0], house["bottom_right"][0]
        y_lo, y_hi = house["windows"][0]["center"][1], house["windows"][1]["center"][1]
        eave_l = house["top_left"] + LEFT * 0.26 * 1.15
        peak = house["roof_peak"]

        def roof_y(x):
            return eave_l[1] + (peak[1] - eave_l[1]) * (1 - abs(x - peak[0]) / (peak[0] - eave_l[0]))

        def leak(x_in, y_in, x_out, side):
            return smooth_path([[x_in, y_in, 0], [x_out, y_in + 0.04, 0], [x_out + side * 0.55, y_in + 0.22, 0],
                                [x_out + side * 1.0, y_in + 0.5, 0]])

        def roof_leak(x):
            side = np.sign(x - peak[0])
            y = roof_y(x)
            return smooth_path([[x - side * 0.25, y - 0.45, 0], [x, y, 0], [x + side * 0.3, y + 0.42, 0],
                                [x + side * 0.65, y + 0.72, 0]])

        loss_paths = [leak(x_l + 0.85, y, x_l, -1) for y in (y_lo - 0.18, y_hi + 0.12)]
        loss_paths += [leak(x_r - 0.85, y, x_r, 1) for y in (y_lo - 0.18, y_hi + 0.12)]
        loss_paths += [roof_leak(peak[0] + dx) for dx in (-1.05, 1.05)]
        loss_guides = flow_guides(loss_paths, PASTEL_CYAN, opacity=0.3)
        loss_label = VGroup(
            Text("Wärmeverlust", font_size=BODY_FONT_SIZE, color=PASTEL_CYAN),
            math_text(r"\Phi_{\mathrm{Verlust}}", font_size=BODY_FONT_SIZE, color=PASTEL_CYAN),
        ).arrange(DOWN, buff=0.14).move_to([4.75, -0.35, 0])

        def leaking(rt):
            return [flow_animation([(loss_paths, PASTEL_ORANGE, PASTEL_CYAN)], waves=4, radius=0.06,
                                   cycles=max(1.0, rt / 1.3))]

        self.play(Create(house["group"]), run_time=1.6)
        self.play(FadeIn(loss_guides), FadeIn(loss_label), *leaking(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "loss", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 2.8, during=leaking)
        #endregion

        #region sun
        sun_c = np.array([-5.45, 1.45, 0.0])
        sun = _sun(sun_c)

        def glowing(rt):
            return [Rotate(sun[3], angle=0.22 * rt, about_point=sun_c, rate_func=linear)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flip"))
        self.play(FadeOut(loss_guides), FadeOut(loss_label), FadeIn(sun, scale=0.7), run_time=1.2)
        hold_for(self, self.NARRATION, "flip", during=glowing)
        #endregion

        #region solar gain on roof, wall and windows
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gain"))
        d = np.array([1.0, -0.42, 0.0]) / np.linalg.norm([1.0, -0.42])
        roof_hits = [np.array([x, roof_y(x), 0.0]) for x in (peak[0] - 1.55, peak[0] - 0.8)]
        wall_hit = np.array([x_l, house["level_1"].get_center()[1], 0.0])
        floors = (house["bottom_left"][1], house["level_1"].get_center()[1])
        glass_hits = [np.array([x_l, w["center"][1] + dy, 0.0]) for w in house["windows"] for dy in (0.1, -0.1)]
        lands = [h + d * (fy - h[1]) / d[1] for h, fy in zip(glass_hits, np.repeat(floors, 2))]
        reach = 2.1
        outer = VGroup(*[_soft_ray(h - d * reach, h) for h in (*roof_hits, wall_hit, *glass_hits)])
        inner = VGroup(*[_soft_ray(h, p, width=2.0, opacity=0.5, fade=False) for h, p in zip(glass_hits, lands)])
        patches = VGroup(*[Line(lands[k], lands[k + 1], color=COLOR_G, stroke_width=6, stroke_opacity=0.8)
                           for k in (0, 2)])
        gain_paths = [[h - d * reach, h] for h in (*roof_hits, wall_hit)]
        gain_paths += [[h - d * reach, h, p] for h, p in zip(glass_hits, lands)]
        warm = [(lands[k] + lands[k + 1]) / 2 + UP * 0.03 for k in (0, 2)]
        gain_label = VGroup(
            Text("Solarer Gewinn", font_size=BODY_FONT_SIZE, color=COLOR_G),
            math_text(r"\Phi_{\mathrm{solar}}", font_size=BODY_FONT_SIZE, color=COLOR_G),
        ).arrange(DOWN, buff=0.14).move_to(loss_label)

        def sunshine(rt):
            return [_pulses(gain_paths, rt), ripples(warm, r_max=0.5, color=PASTEL_ORANGE, cycles=max(1.0, rt / 1.4)),
                    *glowing(rt)]

        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in outer], lag_ratio=0.1), FadeIn(gain_label),
                  run_time=1.3)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in inner], lag_ratio=0.1), FadeIn(patches),
                  run_time=0.8)
        hold_for(self, self.NARRATION, "gain", during=sunshine)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
