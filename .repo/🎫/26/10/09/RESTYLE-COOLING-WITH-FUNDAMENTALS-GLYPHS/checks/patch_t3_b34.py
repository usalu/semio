"""🩹 [DEBUG] One-off patch: Teil 3 Beats 3–4 onto the Physical Fundamentals glyphs."""
import pathlib
import sys

p = pathlib.Path(sys.argv[1])
s = p.read_text()


def rep(old, new):
    global s
    assert s.count(old) == 1, (s.count(old), old[:70])
    s = s.replace(old, new)


start = s.index('''        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house_section(hc)''')
end = s.index('''        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)

#endregion


#region Beat 4''')
new = r'''        hc = LEFT * 0.35 + CONTENT_CENTER
        h = _build_house(hc)
        win = h["windows"][1]
        sun_pos = RIGHT * 4.3 + UP * 1.5
        sun_group = _build_sun(sun_pos)
        self.add(h["air"], h["house"])
        self.play(FadeIn(sun_group, scale=0.7), run_time=1.0)

        win_center = win["center"]
        sq_box = Square(side_length=1.3, color=P_CYAN, stroke_width=2).move_to(win_center)
        zoom_box = DashedVMobject(sq_box, num_dashes=16)
        self.play(Create(zoom_box), run_time=0.8)
        self.play(FadeOut(zoom_box), open_window(win, run_time=1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 0.8 + 1.0)

        # Warm humid outdoor air streams in through the upper half of the open sash,
        # cool room air leaves through the lower half — particles, not wavy lines.
        wx, wy = win["x"], win_center[1]
        y_in, y_out = wy + 0.12, wy - 0.12
        air_start_x = wx - 2.4
        inflow = [smooth_path([np.array([air_start_x, y_in + dy + 0.1, 0.0]), np.array([wx - 0.5, y_in + dy, 0.0]),
                               np.array([wx + 0.6, y_in + dy - 0.05, 0.0]), np.array([wx + 2.0, y_in + dy - 0.45, 0.0])])
                  for dy in (0.0, 0.08)]
        outflow = [smooth_path([np.array([wx + 2.0, y_out - 0.7 + dy, 0.0]), np.array([wx + 0.6, y_out + dy, 0.0]),
                                np.array([wx - 0.5, y_out + dy, 0.0]), np.array([air_start_x, y_out + dy - 0.1, 0.0])])
                   for dy in (0.0, -0.08)]
        moist = [smooth_path([np.array([air_start_x, y_in + 0.3, 0.0]), np.array([wx - 0.5, y_in + 0.2, 0.0]),
                              np.array([wx + 0.6, y_in + 0.14, 0.0]), np.array([wx + 2.0, y_in - 0.25, 0.0])])]
        heat_waves_in = flow_guides(inflow, P_RED, opacity=0.3)
        cold_waves_out = flow_guides(outflow, P_BLUE, opacity=0.3)
        drops = droplets(np.array([air_start_x + 0.8, y_in + 0.32, 0.0]), n=6, spread=(0.6, 0.06), seed=11)
        air_y_base_in = y_in

        def window_air(rt):
            cyc = max(1.0, rt / 1.5)
            return [flow_animation([(inflow, P_RED, P_ORANGE), (outflow, P_CYAN, P_BLUE)], waves=4, cycles=cyc),
                    flow_animation([(moist, P_BLUE)], waves=5, radius=0.05, cycles=cyc, streak=False)]

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "flow"))
        self.play(Create(heat_waves_in), Create(cold_waves_out), FadeIn(drops, lag_ratio=0.15), run_time=1.2)
        self.play(*window_air(3.6), h["air"].animate.set_fill(P_RED, opacity=0.12), run_time=3.6)
        hold_for(self, self.NARRATION, "flow", during=window_air)

        row, box, items = math_panel([
            ("ql", r"\dot{Q}_{L}", P_WHITE), (None, "=", P_WHITE),
            ("sens", r"\dot{Q}_{sens}", P_RED), (None, "+", P_WHITE),
            ("lat", r"\dot{Q}_{lat}", P_BLUE),
            (None, r"\;[\mathrm{W}]", P_WHITE),
        ], size=BODY_FONT_SIZE)
        rest = VGroup(*[
            m for m in row.submobjects
            if m is not items["sens"] and m is not items["lat"]
        ])

        # Tokens rise into the free sky left of the house, apart from each other
        # and from the streams, then morph straight into their formula slots.
        sens_tok = math_label(r"\dot{Q}_{sens}", size=BODY_FONT_SIZE, color=P_RED)
        sens_tok.move_to(np.array([air_start_x + 0.35, air_y_base_in + 0.95, 0.0]))
        lat_tok = math_label(r"\dot{Q}_{lat}", size=BODY_FONT_SIZE, color=P_BLUE)
        lat_tok.move_to(np.array([air_start_x + 1.55, air_y_base_in + 0.95, 0.0]))

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "formula"))
        self.play(
            ReplacementTransform(heat_waves_in.copy(), sens_tok),
            ReplacementTransform(drops.copy(), lat_tok),
            Create(box), FadeIn(rest), *window_air(1.2),
            run_time=1.2,
        )
        self.play(
            ReplacementTransform(sens_tok, items["sens"]),
            ReplacementTransform(lat_tok, items["lat"]), *window_air(1.3),
            run_time=1.3,
        )
        hold_for(self, self.NARRATION, "formula", during=window_air)

        for key, color in (("sens", P_RED), ("lat", P_BLUE)):
            ring = highlight_param(items, key, color=color)
            caption = swap_caption(self, caption, subtitle_text(self.NARRATION, key))
            stress = (flow_animation([(inflow, P_RED, P_ORANGE)], waves=8, radius=0.075, cycles=1.0) if key == "sens"
                      else flow_animation([(moist, P_BLUE)], waves=10, radius=0.06, cycles=1.0, streak=False))
            self.play(Create(ring), stress, run_time=1.4)
            hold_for(self, self.NARRATION, key, during=window_air)
            self.play(FadeOut(ring), run_time=0.25)

'''
s = s[:start] + new + s[end:]

rep('''        bulb = Circle(
            radius=0.38, color=P_RED, fill_color=P_DEEP_DARK, fill_opacity=1.0, stroke_width=3,
        )
        bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
        tube = RoundedRectangle(
            corner_radius=0.12, height=2.1, width=0.34, color=P_RED, stroke_width=3,
        )
        tube.move_to(np.array([lx, mid_y + 0.15, 0]))
        mercury_bulb = Circle(
            radius=0.35, color=P_RED, fill_color=P_RED, fill_opacity=0.9, stroke_width=0,
        )
        mercury_bulb.move_to(np.array([lx, mid_y - 1.05, 0]))
        temp_ticks = VGroup(*[
            Line([lx - 0.28, y, 0], [lx - 0.12, y, 0], color=P_TEAL, stroke_width=2)
            for y in np.linspace(mid_y - 0.55, mid_y + 0.95, 6)
        ])''', '''        therm = thermometer_glyph(np.array([lx, mid_y - 1.0, 0.0]), height=2.15, color=P_RED, level=1.0)
        tube = therm["group"][0]
        temp_ticks = VGroup(*[
            Line([lx - 0.28, y, 0], [lx - 0.14, y, 0], color=P_TEAL, stroke_width=2)
            for y in np.linspace(mid_y - 0.55, mid_y + 0.95, 6)
        ])''')
rep('''        temp_tracker = ValueTracker(1.7)
        column = always_redraw(lambda: Rectangle(
            width=0.22,
            height=max(0.05, temp_tracker.get_value()),
            color=P_RED,
            fill_color=P_RED,
            fill_opacity=0.9,
            stroke_width=0,
        ).move_to(np.array([lx, mid_y - 0.75 + temp_tracker.get_value() / 2, 0])))''',
'''        temp_tracker = ValueTracker(1.7)
        therm["level"].set_value(0.12 + 0.86 * temp_tracker.get_value() / 1.7)
        column = therm["column"]
        column.add_updater(lambda m: therm["level"].set_value(0.12 + 0.86 * temp_tracker.get_value() / 1.7))''')
rep('''            Create(bulb), Create(tube), Create(temp_ticks), FadeIn(mercury_bulb),''',
    '''            FadeIn(therm["group"]), Create(temp_ticks),''')
rep('''        droplet_group = VGroup(*[
            Circle(radius=0.07, color=P_CYAN, fill_color=P_CYAN, fill_opacity=0.85, stroke_width=1)
            .move_to(np.array([rx + dx, mid_y + 0.88 + dy, 0]))
            for dx, dy in [(-0.28, 0.0), (-0.06, 0.14), (0.14, 0.06), (0.30, -0.06)]
        ])''', '''        droplet_group = droplets(np.array([rx, mid_y + 0.9, 0.0]), n=6, spread=(0.36, 0.1), seed=7)''')
rep('''        hold_for(self, self.NARRATION, "delta_x", used=0.6 + 3.2 + 0.35)

        self.play(FadeOut(ring_dx), FadeOut(caption), run_time=0.4)''',
'''        hold_for(self, self.NARRATION, "delta_x", used=0.6 + 3.2 + 0.35)

        column.clear_updaters()
        self.play(FadeOut(ring_dx), FadeOut(caption), run_time=0.4)''')
p.write_text(s)
print("[DEBUG] beats 3-4 ok")
