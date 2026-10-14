#region Beat 1 – Warum Rechnung und Abrechnung auseinanderlaufen

SEG_COLOURS = (P_RED, P_CYAN, WATER_HOT, P_YELLOW)
SEG_NORM = (0.26, 0.14, 0.10, 0.12)
BAR_X0, BAR_UNIT, BAR_H = -0.60, 4.20, 0.34
BAR_Y_CALC, BAR_Y_MEAS = 0.92, 0.18


class Beat1_BerechnungVsAbrechnung(Scene):
    NARRATION = [
        ("intro",
         "Does the calculated demand match the bill? Rarely exactly — because the calculation deliberately uses standardised conditions.",
         "Stimmt der berechnete Bedarf mit der Rechnung überein? Selten genau — denn die Berechnung nutzt bewusst Normbedingungen."),
        ("normen",
         "The calculation fixes the use: a twenty-degree setpoint, a standard air change, standard hot water, a reference climate.",
         "Die Berechnung legt die Nutzung fest: zwanzig Grad als Sollwert, genormter Luftwechsel, genormtes Warmwasser, ein Referenzklima."),
        ("real",
         "Real households heat unevenly, air unevenly and live unevenly. So the measured points scatter around the diagonal.",
         "Echte Haushalte heizen, lüften und leben ungleichmäßig. Die Messpunkte streuen deshalb um die Diagonale."),
        ("prebound",
         "The scatter is not symmetric: poorly insulated buildings consume on average less than calculated — people heat more sparingly.",
         "Die Streuung ist nicht symmetrisch: Schlecht gedämmte Gebäude verbrauchen im Mittel weniger als berechnet — man heizt sparsamer."),
        ("rebound",
         "In efficient buildings it is on average above — for instance through higher comfort temperatures.",
         "In effizienten Gebäuden liegt er im Mittel darüber — etwa durch höhere Komforttemperaturen."),
        ("fazit",
         "The certificate therefore compares buildings, not households. That is exactly what it was built to do.",
         "Der Ausweis vergleicht deshalb Gebäude, nicht Haushalte. Genau dafür ist er gemacht."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Warum Rechnung und Abrechnung auseinanderlaufen", title)
        din = _din_ref("DIN V 18599-10:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        scale = 1.05
        hs = house_section(np.array([-4.5, -0.95, 0.0]), scale=scale)
        bl, br, tl, tr, peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]
        lo_win, up_win = hs["windows"]
        y_floor, y_mid = float(bl[1]), float(hs["level_1"].get_center()[1])
        room_h = hs["w_height"] / 2
        wall_in = float(br[0]) - 0.08 * scale
        person = _stand(y_floor, room_h, -4.00)
        rad = radiator(np.array([float(bl[0]) + 1.0, y_floor + 0.27, 0.0])).scale(0.85)
        th_h = 0.85
        th = thermometer_glyph(np.array([-4.30, y_mid + 0.28, 0.0]), height=th_h, color=P_RED, level=0.5)
        tick_y = y_mid + 0.28 + 0.05 + (th_h - 0.12) * 0.5
        th_tick = Line(np.array([-4.17, tick_y, 0.0]), np.array([-4.07, tick_y, 0.0]), color=CALC_CYAN, stroke_width=2.4)
        temp = th["level"]
        temp_read = math_readout(
            lambda: rf"{de_num(10 + 20 * temp.get_value())}\,\mathrm{{°C}}",
            np.array([-3.98, tick_y - 0.07, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
        spout_y, spout_x = y_mid + 0.80, -3.10
        spout = VGroup(
            Line(np.array([wall_in, spout_y, 0.0]), np.array([spout_x, spout_y, 0.0]), color=P_WHITE, stroke_width=1.8),
            Line(np.array([spout_x, spout_y, 0.0]), np.array([spout_x, spout_y - 0.08, 0.0]), color=P_WHITE, stroke_width=1.8),
        )
        basin = VGroup(
            Line(np.array([spout_x - 0.26, y_mid + 0.22, 0.0]), np.array([spout_x - 0.18, y_mid + 0.08, 0.0]), color=P_WHITE, stroke_width=1.6),
            Line(np.array([spout_x - 0.18, y_mid + 0.08, 0.0]), np.array([spout_x + 0.18, y_mid + 0.08, 0.0]), color=P_WHITE, stroke_width=1.6),
            Line(np.array([spout_x + 0.18, y_mid + 0.08, 0.0]), np.array([spout_x + 0.26, y_mid + 0.22, 0.0]), color=P_WHITE, stroke_width=1.6),
        )
        hot = ValueTracker(0.5)
        tap_stream = _stream(_polyline([(spout_x, spout_y - 0.12, 0), (spout_x, y_mid + 0.12, 0)]),
                             hot, color=WATER_HOT, count=4, speed=0.9, radius=0.04)
        opening = ValueTracker(0.3)
        air_rate = ValueTracker(0.0)
        air_streams = VGroup()
        for win in (lo_win, up_win):
            x, y = win["x"], float(win["center"][1])
            air_in = smooth_path([(x - 0.50, y - 0.10, 0), (x + 0.15, y - 0.10, 0), (x + 0.85, y - 0.06, 0), (x + 1.45, y + 0.02, 0)])
            air_out = smooth_path([(x + 1.45, y + 0.14, 0), (x + 0.85, y + 0.17, 0), (x + 0.15, y + 0.16, 0), (x - 0.50, y + 0.20, 0)])
            air_streams.add(
                _stream(air_in, air_rate, color=COLD_BLUE, color_end=P_ORANGE, count=4, speed=0.30, streak=True, radius=0.06),
                _stream(air_out, air_rate, color=P_ORANGE, count=4, speed=0.30, streak=True, radius=0.06),
            )
        for win in (lo_win, up_win):
            _drive_sash(win, opening)

        sun_c = np.array([-1.70, 1.10, 0.0])
        sun = sun_glyph(sun_c).scale(0.55)
        slope = (peak, tr + RIGHT * 0.26 * scale)
        aim = np.array([-0.80, -0.60, 0.0])
        perp = np.array([0.60, -0.80, 0.0])
        rays = VGroup(*[
            radiation_ray(sun_c + aim * 0.62 + perp * s, _polyline_hit(sun_c + aim * 0.62 + perp * s, aim, *slope))
            for s in (-0.20, 0.0, 0.20)
        ])
        cover = ValueTracker(0.0)
        cloud_core = Rectangle(width=0.75, height=0.10, stroke_width=0, fill_opacity=0).move_to(ORIGIN)
        cloud_base = cloud_glyph(cloud_core, pad=(0.42, 0.26), bump=0.40)
        cloud_base.set_fill(P_DEEP_DARK, opacity=0.95)
        cloud = cloud_base.copy()
        cloud_far, cloud_near = np.array([0.90, 1.85, 0.0]), np.array([-1.66, 1.14, 0.0])

        def _place_cloud(mob):
            k = float(cover.get_value())
            drop = k * k * (3 - 2 * k)
            where = cloud_far + (cloud_near - cloud_far) * np.array([k, drop, 0.0])
            mob.become(cloud_base.copy().move_to(where))

        cloud.add_updater(_place_cloud)
        rays.add_updater(lambda mob: mob.set_stroke(opacity=0.9 * (1.0 - 0.85 * float(cover.get_value()))))
        _place_cloud(cloud)
        warm_spots = [rad.get_top() + UP * 0.02, person.get_top() + UP * 0.04]

        def heat(rt, r_max=0.36):
            return [ripples(warm_spots, r_max=r_max, cycles=max(1.0, rt / 1.4))]

        def _bar_label(text, colour, y):
            label = body_text(text, font_size=LABEL_FONT_SIZE, color=colour)
            label.move_to(np.array([BAR_X0, y, 0.0]), aligned_edge=LEFT)
            return label

        def _bar(total, colour, y):
            return Rectangle(width=total * BAR_UNIT, height=BAR_H, color=colour, stroke_width=1.6,
                             fill_color=colour, fill_opacity=0.55).move_to(
                np.array([BAR_X0 + total * BAR_UNIT / 2, y, 0.0]))

        def _segments(lengths, y, opacity=0.78):
            cursor, out = 0.0, VGroup()
            for length, colour in zip(lengths, SEG_COLOURS):
                width = max(0.004, length * BAR_UNIT)
                out.add(Rectangle(width=width, height=BAR_H, color=colour, stroke_width=1.2,
                                  fill_color=colour, fill_opacity=opacity).move_to(
                    np.array([BAR_X0 + cursor * BAR_UNIT + width / 2, y, 0.0])))
                cursor += length
            return out

        def _measured_lengths():
            factors = ((10 + 20 * temp.get_value()) / 20.0, (0.4 + opening.get_value()) / 0.7,
                       hot.get_value() / 0.5, 1.0 + 0.3 * cover.get_value())
            return [c * f for c, f in zip(SEG_NORM, factors)]

        calc_total = sum(SEG_NORM)
        calc_lbl = _bar_label("berechnet", CALC_CYAN, BAR_Y_CALC + 0.36)
        meas_lbl = _bar_label("gemessen", REAL_ORANGE, BAR_Y_MEAS + 0.36)
        calc_bar = _bar(calc_total, CALC_CYAN, BAR_Y_CALC)
        meas_bar = _bar(0.68, REAL_ORANGE, BAR_Y_MEAS)
        calc_segs = _segments(SEG_NORM, BAR_Y_CALC)
        meas_segs = always_redraw(lambda: _segments(_measured_lengths(), BAR_Y_MEAS))

        def _legend(src, colour, at):
            row = VGroup(Dot(radius=0.07, color=colour), math_label(src, size=LABEL_FONT_SIZE, color=P_WHITE))
            return row.arrange(RIGHT, buff=0.14).move_to(at, aligned_edge=LEFT)

        legend = VGroup(
            _legend(r"\text{Innentemperatur}\;20\,\mathrm{°C}\;\text{Sollwert}", SEG_COLOURS[0], np.array([-2.15, -0.45, 0.0])),
            _legend(r"\text{Luftwechsel genormt}", SEG_COLOURS[1], np.array([2.15, -0.45, 0.0])),
            _legend(r"\text{Warmwasser pauschal je}\;\mathrm{m^{2}}", SEG_COLOURS[2], np.array([-2.15, -0.90, 0.0])),
            _legend(r"\text{Referenzklima}", SEG_COLOURS[3], np.array([2.15, -0.90, 0.0])),
        )

        self.play(Create(hs["group"]), run_time=1.5)
        self.play(FadeIn(rad), FadeIn(person), *heat(1.0), run_time=1.0)
        self.play(FadeIn(calc_lbl), FadeIn(meas_lbl), GrowFromEdge(calc_bar, LEFT), GrowFromEdge(meas_bar, LEFT),
                  *heat(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 1.0 + 1.2,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "normen"))
        self.add(th["column"], temp_read)
        self.play(FadeOut(calc_bar), FadeIn(th["group"]), FadeIn(th_tick), FadeIn(legend[0], shift=UP * 0.1),
                  GrowFromEdge(calc_segs[0], LEFT), *heat(1.4), run_time=1.4)
        self.add(air_streams)
        self.play(air_rate.animate.set_value(0.5), FadeIn(legend[1], shift=UP * 0.1),
                  GrowFromEdge(calc_segs[1], LEFT), *heat(1.4), run_time=1.4)
        self.add(tap_stream)
        self.play(FadeIn(spout), FadeIn(basin), FadeIn(legend[2], shift=UP * 0.1), GrowFromEdge(calc_segs[2], LEFT),
                  *heat(1.4), run_time=1.4)
        self.add(rays)
        self.play(FadeIn(sun), Create(rays), FadeIn(legend[3], shift=UP * 0.1), GrowFromEdge(calc_segs[3], LEFT),
                  *heat(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "normen", used=1.4 * 4 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "real"))
        self.add(meas_segs, cloud)
        self.play(FadeOut(meas_bar), *heat(0.6), run_time=0.6)
        self.play(temp.animate.set_value(0.6), *heat(1.0), run_time=1.0)
        self.play(opening.animate.set_value(0.55), air_rate.animate.set_value(0.9), *heat(1.0), run_time=1.0)
        self.play(hot.animate.set_value(0.35), person.animate.shift(RIGHT * 0.5), *heat(1.0), run_time=1.0)
        self.play(cover.animate.set_value(1.0), *heat(1.4), run_time=1.4)

        axes = labeled_axes([BAR_X0, -1.42, 0], x_len=BAR_UNIT, y_len=2.95, x_label="berechnet  →", color=P_WHITE)
        y_cap = body_text("gemessen", font_size=LABEL_FONT_SIZE - 2, color=P_WHITE)
        y_cap.set_opacity(0.85).rotate(PI / 2).next_to(axes["y_axis"], LEFT, buff=0.14)
        diagonal = DashedLine(axes["pt"](0.0, 0.0), axes["pt"](1.0, 1.0), color=P_WHITE, stroke_width=2, stroke_opacity=0.55)
        diag_tag = body_text("Rechnung = Messung", font_size=LABEL_FONT_SIZE - 4, color=P_WHITE)
        diag_tag.set_opacity(0.6).next_to(axes["pt"](0.88, 0.88), UL, buff=0.06)
        schematic_tag = body_text("schematische Darstellung", font_size=LABEL_FONT_SIZE - 4, color=P_TEAL)
        schematic_tag.set_opacity(0.8).next_to(axes["x_label"], DOWN, buff=0.12)
        meas_total = sum(_measured_lengths())
        x_seg = Line(axes["pt"](0, 0), axes["pt"](calc_total, 0), color=CALC_CYAN, stroke_width=8)
        y_seg = Line(axes["pt"](0, 0), axes["pt"](0, meas_total), color=REAL_ORANGE, stroke_width=8)
        me_dot = Dot(axes["pt"](calc_total, meas_total), radius=0.09, color=REAL_ORANGE)
        proj = VGroup(
            DashedLine(axes["pt"](calc_total, 0), me_dot.get_center(), color=CALC_CYAN, stroke_width=1.8),
            DashedLine(axes["pt"](0, meas_total), me_dot.get_center(), color=REAL_ORANGE, stroke_width=1.8),
        )
        meas_frozen = _segments(_measured_lengths(), BAR_Y_MEAS)
        self.remove(meas_segs)
        self.add(meas_frozen)
        self.play(
            FadeOut(legend), FadeOut(calc_lbl), FadeOut(meas_lbl),
            Create(axes["group"]), FadeIn(y_cap),
            ReplacementTransform(calc_segs, x_seg),
            ReplacementTransform(meas_frozen, y_seg),
            run_time=1.6,
        )
        self.play(Create(proj), FadeIn(me_dot, scale=0.4), Create(diagonal), FadeIn(diag_tag), FadeIn(schematic_tag),
                  run_time=1.0)
        rng = np.random.default_rng(11)
        xs = np.linspace(0.12, 0.94, 22)
        ys = 0.30 + 0.52 * xs + rng.normal(0.0, 0.055, len(xs))
        points = VGroup(*[
            Dot(axes["pt"](float(x), float(np.clip(y, 0.05, 0.97))), radius=0.058, color=REAL_ORANGE, fill_opacity=0.9)
            for x, y in zip(xs, ys)
        ])
        self.play(LaggedStart(*[FadeIn(d, scale=0.5) for d in points], lag_ratio=0.05), *heat(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "real", during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "prebound"))
        trend = Line(axes["pt"](0.10, 0.352), axes["pt"](0.96, 0.799), color=REAL_ORANGE, stroke_width=3)
        pre_card = stat_card("schlecht gedämmt · im Mittel", "gemessen < berechnet", color=GOOD_GREEN)
        pre_card.move_to(np.array([5.25, 0.55, 0.0]))
        leak_l = np.array([[float(bl[0]), y, 0.0] for y in (float(lo_win["center"][1]) - 0.45, float(up_win["center"][1]) + 0.45)])
        leak_r = np.array([[float(br[0]), y, 0.0] for y in (y_floor + 0.5, y_mid + 0.6)])

        def leak(rt, r_max=0.55):
            return [ripples(list(leak_l), r_max=r_max, color=P_RED, facing=PI, cycles=max(1.0, rt / 1.4)),
                    ripples(list(leak_r), r_max=r_max, color=P_RED, facing=0.0, cycles=max(1.0, rt / 1.4))]

        pre_dot = axes["pt"](0.85, 0.70)
        self.play(
            FadeOut(x_seg), FadeOut(y_seg), FadeOut(proj), Create(trend), FadeIn(pre_card),
            temp.animate.set_value(0.4), me_dot.animate.move_to(pre_dot), person.animate.shift(LEFT * 0.5),
            *leak(1.6),
            run_time=1.6,
        )
        ring = Circle(radius=0.17, color=P_WHITE, stroke_width=2.5).move_to(me_dot.get_center())
        self.play(Create(ring), *leak(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "prebound", during=lambda rt: leak(rt) + heat(rt, 0.28))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "rebound"))
        off = 0.09
        envelope = VMobject(color=P_TEAL, stroke_width=6, stroke_opacity=0.85).set_points_as_corners([
            bl + LEFT * off, tl + LEFT * off, tl + LEFT * (0.26 * scale + off) + UP * 0.02, peak + UP * off,
            tr + RIGHT * (0.26 * scale + off) + UP * 0.02, tr + RIGHT * off, br + RIGHT * off,
        ])
        re_card = stat_card("effizient · im Mittel", "gemessen > berechnet", color=WARM_RED)
        re_card.move_to(np.array([5.25, -0.45, 0.0]))
        re_dot = axes["pt"](0.20, 0.47)
        self.play(
            Create(envelope), FadeIn(re_card),
            temp.animate.set_value(0.65), me_dot.animate.move_to(re_dot), ring.animate.move_to(re_dot),
            *leak(1.6, 0.25),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "rebound", during=lambda rt: heat(rt, 0.36))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "fazit"))
        verdict = note_line("genormte Randbedingungen machen Gebäude vergleichbar — nicht Haushalte", color=P_TEAL)
        self.play(
            FadeIn(verdict), FadeOut(trend), FadeOut(pre_card), FadeOut(re_card), FadeOut(ring),
            temp.animate.set_value(0.5), opening.animate.set_value(0.3), air_rate.animate.set_value(0.5),
            hot.animate.set_value(0.5), cover.animate.set_value(0.0), person.animate.shift(RIGHT * 0.0),
            *[d.animate.move_to(axes["pt"](float((d.get_center()[0] - BAR_X0) / BAR_UNIT),
                                           float((d.get_center()[0] - BAR_X0) / BAR_UNIT))) for d in (*points, me_dot)],
            *heat(1.6),
            run_time=1.6,
        )
        hold_for(self, self.NARRATION, "fazit", during=heat)

        for mob in (*air_streams, tap_stream, rays, cloud, sun_c and temp_read, th["column"]):
            mob.clear_updaters()
        self.play(FadeOut(caption), FadeOut(verdict), run_time=0.3)
        self.wait(0.5)

#endregion
