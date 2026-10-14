

#region Beat 2 – Hydraulischer Abgleich

PUMP_SPIN = 4.0
UNIT_SPEED = 0.36
FLOW_UNBALANCED = (2.0, 1.36, 0.73, 0.36)
FLOW_BALANCED = (1.0, 1.0, 1.0, 1.0)


def _pipe_stream(points, rate, *, color, color_end=None):
    """🚰 Water drops along a corner pipe — speed in units per second follows the volume flow ``rate`` (1 = design flow)."""
    pts = [np.array(p, dtype=float) for p in points]
    length = float(sum(np.linalg.norm(b - a) for a, b in zip(pts, pts[1:])))
    return _stream(_polyline(pts), rate, color=color, color_end=color_end, count=max(2, round(length / 0.30)),
                   speed=UNIT_SPEED / length, radius=0.045)


def _room_row(x_left, width, count, cy, height, t=0.14):
    """🏠 Row of rooms in section — shared ceiling and floor slabs, partition walls between the rooms."""
    style = dict(color=P_WHITE, stroke_width=2, fill_color=P_WHITE, fill_opacity=0.14)
    total = width * count
    ceiling = Rectangle(width=total + t, height=t, **style).move_to(np.array([x_left + total / 2, cy + height / 2, 0.0]))
    floor = Rectangle(width=total + t, height=t, **style).move_to(np.array([x_left + total / 2, cy - height / 2, 0.0]))
    walls = VGroup(*[Rectangle(width=t, height=height - t, **style).move_to(np.array([x_left + k * width, cy, 0.0]))
                     for k in range(count + 1)])
    return {"shell": VGroup(ceiling, walls, floor), "floor_top": cy - height / 2 + t / 2,
            "ceil_bottom": cy + height / 2 - t / 2, "x_left": x_left, "width": width, "t": t}


def _valve(center, opening):
    """🎚️ Radiator valve on the riser — the handle turns with the ``opening`` tracker (1 wide open … 0 shut)."""
    c = np.array(center, dtype=float)
    body = VGroup(Circle(radius=0.10, color=P_TEAL, stroke_width=2.2).set_fill(P_DEEP_DARK, opacity=1.0).move_to(c),
                  Line(c + LEFT * 0.10, c + RIGHT * 0.10, color=P_TEAL, stroke_width=2.2))
    handle = always_redraw(lambda: VGroup(
        Line(c, c + 0.26 * np.array([np.sin((1 - opening.get_value()) * PI / 2), np.cos((1 - opening.get_value()) * PI / 2), 0.0]),
             color=P_TEAL, stroke_width=3),
    ))
    return body, handle


def _thermo(bottom, height, level):
    """🌡️ Room thermometer whose column colour runs cold → comfortable → hot with the ``level`` tracker."""
    b = np.array(bottom, dtype=float)
    ramp = color_ramp([(0.0, COLD_BLUE), (0.5, GOOD_GREEN), (1.0, WARM_RED)])
    tube = RoundedRectangle(width=0.2, height=height, corner_radius=0.1, color=P_WHITE, stroke_width=2).move_to(b + UP * height / 2)
    bulb = Circle(radius=0.15, color=P_WHITE, stroke_width=2).set_fill(P_DEEP_DARK, opacity=1.0).move_to(b)
    tick_y = b[1] + 0.05 + (height - 0.12) * 0.5
    tick = Line(np.array([b[0] + 0.12, tick_y, 0.0]), np.array([b[0] + 0.22, tick_y, 0.0]), color=CALC_CYAN, stroke_width=2.4)

    def column():
        v = float(np.clip(level.get_value(), 0.0, 1.0))
        h = max(0.02, (height - 0.12) * v)
        col = Rectangle(width=0.1, height=h, stroke_width=0, fill_color=ramp(v), fill_opacity=0.95).move_to(b + UP * (h / 2 + 0.05))
        dot = Circle(radius=0.115, stroke_width=0, fill_color=ramp(v), fill_opacity=0.95).move_to(b)
        return VGroup(dot, col)

    return {"frame": VGroup(tube, bulb), "tick": tick, "column": always_redraw(column), "top": b + UP * height}


class Beat2_HydraulischerAbgleich(Scene):
    NARRATION = [
        ("intro",
         "One more gap between paper and practice, and this one is pure hydraulics.",
         "Noch eine Lücke zwischen Papier und Praxis — und diese ist reine Hydraulik."),
        ("unabgeglichen",
         "Water takes the path of least resistance. The rooms nearest the pump get far more flow than they were calculated for.",
         "Wasser nimmt den Weg des geringsten Widerstands. Die Räume nahe der Pumpe bekommen weit mehr Durchfluss als berechnet."),
        ("folge",
         "So the near rooms overheat while the far ones stay cold — and often the flow temperature simply gets turned up.",
         "Die nahen Räume überhitzen, die fernen bleiben kalt — und oft wird einfach die Vorlauftemperatur hochgedreht."),
        ("abgleich",
         "Hydraulic balancing presets every valve to the flow that room's calculated heat load actually needs.",
         "Der hydraulische Abgleich stellt jedes Ventil auf den Durchfluss ein, den die berechnete Raumheizlast wirklich braucht."),
        ("gewinn",
         "The flow temperature can then come down — an important condition for a heat pump to reach a good seasonal performance factor.",
         "Danach kann die Vorlauftemperatur sinken — eine wichtige Voraussetzung für eine gute Jahresarbeitszahl der Wärmepumpe."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Hydraulischer Abgleich", title)
        din = _din_ref("GEG § 60c · VOB/C DIN 18380")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        width, count, t = 2.3, 4, 0.14
        row = _room_row(-4.0, width, count, 0.85, 1.7, t)
        floor_top, ceil_bottom = row["floor_top"], row["ceil_bottom"]
        clear_h = ceil_bottom - floor_top
        xs = [-4.0 + width * (i + 0.5) for i in range(count)]
        y_sup, y_ret = -0.55, -0.95
        pump_c = np.array([-5.5, y_sup, 0.0])

        washes = VGroup(*[
            Rectangle(width=width - t, height=clear_h, stroke_width=0, fill_color=GOOD_GREEN, fill_opacity=0.0).move_to(
                np.array([x, (floor_top + ceil_bottom) / 2, 0.0]))
            for x in xs
        ])
        rads = VGroup(*[radiator(np.array([x - 0.62, floor_top + 0.27, 0.0])).scale(0.85) for x in xs])
        rad_x = [float(r.get_center()[0]) for r in rads]
        rad_bottom = float(rads[0].get_bottom()[1])
        rad_mid = float(rads[0].get_center()[1])
        people = VGroup(*[_stand(floor_top - 0.02, clear_h, x + 0.30) for x in xs])
        levels = [ValueTracker(0.5) for _ in xs]
        thermos = [_thermo(np.array([x + 0.84, floor_top + 0.30, 0.0]), 1.0, lv) for x, lv in zip(xs, levels)]

        x_in = [rx - 0.26 for rx in rad_x]
        x_out = [rx + 0.26 for rx in rad_x]
        supply = VGroup(
            Line(pump_c + RIGHT * 0.30, np.array([x_in[-1], y_sup, 0.0]), color=WATER_HOT, stroke_width=3.2, stroke_opacity=0.55),
            *[Line(np.array([xi, y_sup, 0.0]), np.array([xi, rad_bottom, 0.0]), color=WATER_HOT, stroke_width=3.2, stroke_opacity=0.55)
              for xi in x_in],
        )
        back = VGroup(
            Line(np.array([x_out[-1], y_ret, 0.0]), np.array([pump_c[0], y_ret, 0.0]), color=COLD_BLUE, stroke_width=3.2, stroke_opacity=0.55),
            Line(np.array([pump_c[0], y_ret, 0.0]), pump_c + DOWN * 0.30, color=COLD_BLUE, stroke_width=3.2, stroke_opacity=0.55),
            *[Line(np.array([xo, rad_bottom, 0.0]), np.array([xo, y_ret, 0.0]), color=COLD_BLUE, stroke_width=3.2, stroke_opacity=0.55)
              for xo in x_out],
        )
        pump = VGroup(Circle(radius=0.30, color=P_ORANGE, stroke_width=2.2).set_fill(P_ORANGE, opacity=0.35).move_to(pump_c))
        impeller = VGroup(Line(LEFT * 0.20, RIGHT * 0.20), Line(DOWN * 0.20, UP * 0.20)).set_stroke(P_WHITE, width=2.6).move_to(pump_c)
        impeller.add_updater(lambda m, dt: m.rotate(-dt * PUMP_SPIN, about_point=pump_c))
        pump_tag = body_text("Pumpe", font_size=LABEL_FONT_SIZE - 2, color=P_ORANGE).move_to(pump_c + UP * 0.58)

        vl = ValueTracker(0.55)
        vl_th = _thermo(np.array([-6.45, -0.92, 0.0]), 1.0, vl)
        vl_tag = math_label(r"\vartheta_{\mathrm{VL}}", at=np.array([-6.45, 0.28, 0.0]), size=BODY_FONT_SIZE, color=WATER_HOT)

        flows = [ValueTracker(0.0) for _ in xs]
        sections = [ValueTracker(0.0) for _ in xs]
        for k, sec in enumerate(sections):
            sec.add_updater(lambda m, k=k: m.set_value(sum(f.get_value() for f in flows[k:])))
        opens = [ValueTracker(1.0) for _ in xs]
        valves = [_valve(np.array([xi, -0.28, 0.0]), op) for xi, op in zip(x_in, opens)]

        streams = VGroup()
        sup_starts = [pump_c + RIGHT * 0.30] + [np.array([xi, y_sup, 0.0]) for xi in x_in[:-1]]
        for k in range(count):
            a = sup_starts[k]
            b = np.array([x_in[k], y_sup, 0.0])
            if np.linalg.norm(b - a) > 0.05:
                streams.add(_pipe_stream([a, b], sections[k], color=WATER_HOT))
        for k in range(count):
            streams.add(_pipe_stream(
                [(x_in[k], y_sup, 0), (x_in[k], rad_mid, 0), (x_out[k], rad_mid, 0), (x_out[k], y_ret, 0)],
                flows[k], color=WATER_HOT, color_end=COLD_BLUE))
        ret_ends = [np.array([pump_c[0], y_ret, 0.0])] + [np.array([xo, y_ret, 0.0]) for xo in x_out[:-1]]
        for k in range(count):
            streams.add(_pipe_stream([np.array([x_out[k], y_ret, 0.0]), ret_ends[k]], sections[k], color=COLD_BLUE))

        gauge_y = -1.38
        gauges, readouts, halves = [], [], []
        for k, x in enumerate(xs):
            g = meter("x", length=1.3, thickness=0.20, color=GOOD_GREEN, vertical=False)
            g["group"].remove(g["label"])
            VGroup(g["track"], g["fill"]).move_to(np.array([x - 0.30, gauge_y, 0.0]))
            g["fill"].move_to(g["track"].get_center())
            half = ValueTracker(0.0)
            half.add_updater(lambda m, f=flows[k]: m.set_value(f.get_value() / 2))
            bind_meter(g, half)
            mid_tick = Line(g["track"].get_center() + UP * 0.10, g["track"].get_center() + UP * 0.20, color=CALC_CYAN, stroke_width=2.4)
            read = math_readout(lambda f=flows[k]: de_num(f.get_value(), 1), np.array([x + 0.50, gauge_y - 0.07, 0.0]),
                                size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
            gauges.append(VGroup(g["track"], g["fill"], mid_tick))
            readouts.append(read)
            halves.append(half)
        gauge_title = math_label(r"\dot{V}/\dot{V}_{\mathrm{soll}}", at=np.array([-5.30, gauge_y - 0.09, 0.0]),
                                 size=LABEL_FONT_SIZE, color=CALC_CYAN)

        def heat(rt):
            return [ripples([rads[k].get_top() + UP * 0.02], r_max=0.14 + 0.36 * float(np.clip(levels[k].get_value(), 0.0, 1.0)),
                            color=P_RED, cycles=max(1.0, rt / 1.4)) for k in range(count)]

        self.play(Create(row["shell"]), FadeIn(washes), run_time=1.0)
        self.play(FadeIn(rads), FadeIn(people), *[FadeIn(th["frame"]) for th in thermos],
                  *[FadeIn(th["tick"]) for th in thermos], run_time=0.8)
        self.add(*[th["column"] for th in thermos], vl_th["column"], *flows, *sections, *halves, *opens)
        self.play(Create(supply), Create(back), FadeIn(pump), FadeIn(impeller), FadeIn(pump_tag),
                  FadeIn(vl_th["frame"]), FadeIn(vl_th["tick"]), FadeIn(vl_tag), run_time=1.2)
        self.play(*[FadeIn(v[0]) for v in valves], *[FadeIn(v[1]) for v in valves], run_time=0.5)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.0 + 0.8 + 1.2 + 0.5)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "unabgeglichen"))
        self.add(streams, *readouts, *[g for g in gauges], gauge_title)
        self.play(FadeIn(gauge_title), *[FadeIn(g) for g in gauges], run_time=0.6)
        self.play(*[fl.animate.set_value(f) for fl, f in zip(flows, FLOW_UNBALANCED)], run_time=1.8)
        hold_for(self, self.NARRATION, "unabgeglichen", used=0.6 + 1.8 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "folge"))
        verdicts = VGroup(*[
            body_text(text, font_size=LABEL_FONT_SIZE, color=colour).move_to(np.array([x, ceil_bottom + 0.36, 0.0]))
            for x, text, colour in zip(xs, ("zu heiß", "zu heiß", "zu kalt", "zu kalt"), (WARM_RED, WARM_RED, COLD_BLUE, COLD_BLUE))
        ])
        tints = [(WARM_RED, 0.16), (WARM_RED, 0.10), (COLD_BLUE, 0.10), (COLD_BLUE, 0.16)]
        self.play(
            *[lv.animate.set_value(v) for lv, v in zip(levels, (0.78, 0.66, 0.36, 0.28))],
            *[w.animate.set_fill(c, opacity=o) for w, (c, o) in zip(washes, tints)],
            LaggedStart(*[FadeIn(v) for v in verdicts], lag_ratio=0.2), *heat(2.0), run_time=2.0)
        reaction = note_line("Reaktion im Alltag: Vorlauftemperatur hochdrehen", color=WARM_RED)
        self.play(FadeIn(reaction), vl.animate.set_value(0.85),
                  *[lv.animate.set_value(v) for lv, v in zip(levels, (0.90, 0.76, 0.46, 0.36))], *heat(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "folge", used=2.0 + 1.6 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "abgleich"))
        self.play(FadeOut(reaction), FadeOut(verdicts), run_time=0.35)
        preset = note_line("Voreinstellung je Ventil = berechneter Volumenstrom des Raums", color=GOOD_GREEN)
        self.play(
            *[op.animate.set_value(v) for op, v in zip(opens, (0.30, 0.45, 0.80, 1.0))],
            *[fl.animate.set_value(f) for fl, f in zip(flows, FLOW_BALANCED)],
            *[lv.animate.set_value(0.5) for lv in levels],
            *[w.animate.set_fill(GOOD_GREEN, opacity=0.07) for w in washes],
            FadeIn(preset), *heat(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "abgleich", used=0.35 + 2.4, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "gewinn"))
        jaz = ValueTracker(0.25)
        jaz_meter = meter("JAZ", length=1.8, thickness=0.40, color=GOOD_GREEN)
        jaz_meter["group"].move_to(np.array([6.25, 0.10, 0.0]))
        bind_meter(jaz_meter, jaz)
        panel, box, items = math_panel([
            ("ab", r"\text{Abgleich}", GOOD_GREEN), (None, r"\to", P_WHITE),
            ("vl", r"\vartheta_{\mathrm{VL}}\;\text{sinkt}", WATER_HOT), (None, r"\to", P_WHITE),
            ("jaz", r"\mathrm{JAZ}\uparrow", GOOD_GREEN),
        ])
        duty = note_line("für viele Heizungsanlagen verpflichtend — GEG § 60c", color=P_TEAL)
        self.play(FadeOut(preset), FadeIn(duty), Create(box), FadeIn(panel), FadeIn(jaz_meter["group"]), run_time=1.0)
        ring_vl = highlight_param(items, "vl", color=WATER_HOT)
        self.play(Create(ring_vl), vl.animate.set_value(0.40), *heat(1.6), run_time=1.6)
        ring_jaz = highlight_param(items, "jaz", color=GOOD_GREEN)
        self.play(Create(ring_jaz), jaz.animate.set_value(0.85), *heat(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "gewinn", used=1.0 + 1.6 + 1.6 + 0.35, during=heat)

        for mob in (*streams, *sections, *halves, *readouts, *[th["column"] for th in thermos], vl_th["column"]):
            mob.clear_updaters()
        self.play(FadeOut(caption), FadeOut(duty), run_time=0.3)
        self.wait(0.5)

#endregion
