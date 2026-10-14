

#region Beat 3 – Die Serie im Rückblick

COL_NUTZ, COL_END, COL_PRIM = "#38BDF8", "#DC2626", "#A78BFA"
LOSS_COLOURS = ("#FBBF24", "#F97316", "#EF4444", "#B91C1C")
LOSS_VALUES = tuple(int(v) for v in np.diff(np.concatenate([[CHAIN_NUTZ], np.round(CHAIN_NUTZ * np.cumprod(CHAIN_E_FACTORS))])))
STAGE_Y, STAGE_UNIT = -1.25, 0.016
X_A, X_B, X_C = -3.05, -0.75, 1.95
X_B_WP, X_C_WP = -0.22, 2.45
CUT_X = (-3.55, -1.45, 1.30)
METER_X, TANK_X = -1.90, 0.55
CLASS_X, CHIP_X = 6.20, 5.40
PIPE_Y = -1.40


def _stack(x, colours, *, width=0.5):
    """📊 Vertical stack of segments whose heights follow value trackers — a quantity as a drawn length."""
    heights = [ValueTracker(0.0) for _ in colours]

    def build():
        out, cursor = VGroup(), STAGE_Y
        for h, colour in zip(heights, colours):
            v = h.get_value() * STAGE_UNIT
            if v > 0.004:
                out.add(Rectangle(width=width, height=v, color=colour, stroke_width=1.2, fill_color=colour,
                                  fill_opacity=0.8).move_to(np.array([x, cursor + v / 2, 0.0])))
            cursor += v
        return out

    return {"mob": always_redraw(build), "heights": heights, "x": x,
            "total": lambda: float(sum(h.get_value() for h in heights)),
            "top": lambda: STAGE_Y + sum(h.get_value() for h in heights) * STAGE_UNIT}


def _class_scale(x, top, bottom):
    """🌈 Vertical A+ … H certificate scale — one band per class, A+ on top."""
    band_h = (top - bottom) / len(CHAIN_CLASSES)
    bands, centres = VGroup(), {}
    for i, (letter, _bound, colour) in enumerate(CHAIN_CLASSES):
        rect = Rectangle(width=0.62, height=band_h, color=colour, fill_color=colour, fill_opacity=0.9,
                         stroke_width=1.0).move_to(np.array([x, top - band_h * (i + 0.5), 0.0]))
        text = body_text(letter, font_size=LABEL_FONT_SIZE - 5, color=P_DEEP_DARK).move_to(rect.get_center())
        bands.add(VGroup(rect, text))
        centres[letter] = rect.get_center()
    return {"bands": bands, "centres": centres}


def _class_chip(letter, colour, at):
    """🔤 Efficiency-class letter on a coloured tile."""
    tile = RoundedRectangle(width=0.54, height=0.54, corner_radius=0.08, color=colour, stroke_width=1.6,
                            fill_color=colour, fill_opacity=0.95)
    mark = body_text(letter, font_size=BODY_FONT_SIZE + 6, color=P_DEEP_DARK).move_to(tile.get_center())
    return VGroup(tile, mark).move_to(at)


def _gas_meter(center, rate):
    """⏱️ Gas meter box on the house line — the needle turns with the gas ``rate``."""
    c = np.array(center, dtype=float)
    box = RoundedRectangle(width=0.50, height=0.66, corner_radius=0.06, color=P_WHITE, stroke_width=2.0).move_to(c)
    dial_c = c + UP * 0.12
    dial = Circle(radius=0.16, color=P_WHITE, stroke_width=1.6).move_to(dial_c)
    needle = Line(dial_c, dial_c + UP * 0.13, color=COL_END, stroke_width=2.6)
    needle.add_updater(lambda m, dt: m.rotate(-dt * 3.0 * float(rate.get_value()), about_point=dial_c))
    counter = VGroup(*[Rectangle(width=0.08, height=0.12, color=P_WHITE, stroke_width=1.2).move_to(c + DOWN * 0.20 + RIGHT * dx)
                       for dx in (-0.12, -0.04, 0.04, 0.12)])
    return VGroup(box, dial, counter, needle)


def _boiler(center):
    """🔥 Wall boiler with a flame — where the gas becomes heat in the house."""
    c = np.array(center, dtype=float)
    casing = Rectangle(width=0.42, height=0.64, color=P_WHITE, stroke_width=2.0).move_to(c)
    flame = Polygon(c + DOWN * 0.18 + LEFT * 0.09, c + DOWN * 0.18 + RIGHT * 0.09, c + UP * 0.02, color=P_ORANGE,
                    stroke_width=1.4, fill_color=P_ORANGE, fill_opacity=0.8)
    core = Polygon(c + DOWN * 0.18 + LEFT * 0.04, c + DOWN * 0.18 + RIGHT * 0.04, c + DOWN * 0.06, color=P_YELLOW,
                   stroke_width=1.0, fill_color=P_YELLOW, fill_opacity=0.9)
    return VGroup(casing, flame, core)


def _tile(cx, title, detail, colour):
    """🗂️ One video of the series — framed slot with its title and what it contributed."""
    frame = RoundedRectangle(width=2.95, height=2.70, corner_radius=0.12, color=colour, stroke_width=2.0).move_to(
        np.array([cx, 0.45, 0.0]))
    name = body_text(title, font_size=BODY_FONT_SIZE, color=colour).move_to(np.array([cx, -0.28, 0.0]))
    detail.move_to(np.array([cx, -0.68, 0.0]))
    return VGroup(frame, name, detail)


class Beat3_SerienAbschluss(Scene):
    NARRATION = [
        ("intro",
         "Four videos, one continuous calculation — worth seeing end to end one last time.",
         "Vier Videos, eine durchgehende Rechnung — es lohnt sich, sie einmal am Stück zu sehen."),
        ("kette",
         "The fundamentals gave us the units. Heating and cooling gave us the flows across the envelope.",
         "Die Grundlagen gaben uns die Einheiten. Heizen und Kühlen gaben uns die Ströme über die Hülle."),
        ("bilanz",
         "This video closed the balance, walked it out to the meter, out to the source, and onto the certificate.",
         "Dieses Video schloss die Bilanz, führte sie zum Zähler, zur Quelle und auf den Ausweis."),
        ("zahlen",
         "A hundred kilowatt-hours of useful heat became, on the gas path, a hundred and forty at the meter, a hundred and fifty-four at the source — and one letter. The heat pump writes a different letter.",
         "Aus hundert Kilowattstunden Nutzwärme wurden auf dem Gasweg hundertvierzig am Zähler, hundertvierundfünfzig an der Quelle — und ein Buchstabe. Die Wärmepumpe schreibt einen anderen."),
        ("outro",
         "That is the complete path from a cold wall in January to the number a landlord shows a new tenant.",
         "Das ist der vollständige Weg von einer kalten Wand im Januar bis zur Zahl, die ein Vermieter einem neuen Mieter zeigt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Serie im Rückblick", title)
        din = _din_ref("GEG · DIN V 18599:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        xs = (-4.95, -1.65, 1.65, 4.95)
        small = 0.27
        lamp = lamp_glyph(np.array([xs[0] - 0.55, 1.62, 0.0]), drop=0.25)
        gauge = thermometer_glyph(np.array([xs[0] + 0.60, 0.30, 0.0]), height=1.0, color=P_RED, level=0.0)
        glow = [ripples([lamp["bulb"].get_center()], r_max=0.5, color=P_YELLOW, down=True, cycles=2.0)]
        hs_heat = house_section(np.array([xs[1], 0.80, 0.0]), scale=small)
        hs_cool = house_section(np.array([xs[2] + 0.35, 0.80, 0.0]), scale=small)
        hs_small = house_section(np.array([xs[3], 0.80, 0.0]), scale=small)
        sun_small_c = np.array([xs[2] - 1.0, 1.40, 0.0])
        sun_small = sun_glyph(sun_small_c).scale(0.32)
        cool_win = hs_cool["windows"][0]
        sun_beams = sun_rays(sun_small_c, cool_win["x"], [cool_win["sill"] + 0.03, cool_win["center"][1], cool_win["lintel"] - 0.03],
                             hs_cool["bottom_left"][1] + 0.02, gap=0.34)
        heat_paths = [smooth_path([(xs[1] + 0.20, y, 0), (xs[1] + 0.62, y + 0.02, 0), (xs[1] + 1.10, y + 0.03, 0)])
                      for y in (0.66, 0.84, 1.02)]
        cool_paths = [smooth_path([(xs[2] + 0.55, y, 0), (xs[2] + 0.95, y - 0.02, 0), (xs[2] + 1.30, y - 0.03, 0)])
                      for y in (0.66, 0.84)]
        tiles = VGroup(
            _tile(xs[0], "1 · Grundlagen", math_label(r"\mathrm{W},\;\mathrm{J},\;\Delta T", size=LABEL_FONT_SIZE, color=P_WHITE), P_YELLOW),
            _tile(xs[1], "2 · Heizen", body_text("Verluste & Gewinne", font_size=LABEL_FONT_SIZE, color=P_WHITE), P_ORANGE),
            _tile(xs[2], "3 · Kühlen", body_text("Sommerlasten", font_size=LABEL_FONT_SIZE, color=P_WHITE), P_CYAN),
            _tile(xs[3], "4 · Energiebilanz", body_text("Gesetz & Ausweis", font_size=LABEL_FONT_SIZE, color=P_WHITE), P_TEAL),
        )
        glyphs = VGroup(lamp["group"], gauge["group"], hs_heat["group"], hs_cool["group"], sun_small, hs_small["group"])
        line_y = -1.18
        thread = smooth_path([(-6.0, line_y, 0), (0.0, line_y, 0), (6.0, line_y, 0)])
        thread_guide = flow_guides(VGroup(thread), P_WHITE, opacity=0.35)
        pace = ValueTracker(1.0)
        thread_flow = _stream(thread, pace, color=P_WHITE, count=12, speed=0.10, radius=0.05)

        self.play(LaggedStart(*[FadeIn(tile, shift=DOWN * 0.1) for tile in tiles], lag_ratio=0.18),
                  LaggedStart(*[Create(g) for g in glyphs], lag_ratio=0.18), run_time=1.9)
        self.add(gauge["column"])
        self.play(Create(thread_guide), run_time=0.6)
        self.add(thread_flow)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.9 + 0.6)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "kette"))
        self.play(
            gauge["level"].animate.set_value(0.8), lamp["bulb"].animate.set_fill(P_YELLOW, opacity=0.9),
            tiles[0][0].animate.set_stroke(width=3.6), *glow, run_time=1.6)
        leaving = [(heat_paths, P_RED, P_ORANGE)]
        self.play(flow_animation(leaving, waves=3, radius=0.045, cycles=2.0), tiles[1][0].animate.set_stroke(width=3.6),
                  run_time=1.8)
        self.play(shine(sun_beams), tiles[2][0].animate.set_stroke(width=3.6), run_time=1.8)
        hold_for(self, self.NARRATION, "kette", used=1.6 + 1.8 + 1.8 + 0.35,
                 during=lambda rt: [flow_animation(leaving + [(cool_paths, P_YELLOW, P_ORANGE)], waves=3, radius=0.045,
                                                   cycles=max(1.0, rt / 0.9))])

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "bilanz"))
        hs = house_section(np.array([-5.15, STAGE_Y + 0.90, 0.0]), scale=0.75)
        bl, br = hs["bottom_left"], hs["bottom_right"]
        room_h = hs["w_height"] / 2
        lo_win = hs["windows"][0]
        rad = radiator(np.array([float(bl[0]) + 0.80, STAGE_Y + 0.17, 0.0])).scale(0.55)
        boiler = _boiler(np.array([float(br[0]) - 0.50, STAGE_Y + 0.32 + 0.02, 0.0]))
        person = _stand(STAGE_Y, room_h, -5.20)
        gas = ValueTracker(0.0)
        gas_path = _polyline([(TANK_X, STAGE_Y + 0.10, 0), (TANK_X, PIPE_Y, 0), (float(br[0]) - 0.50, PIPE_Y, 0),
                              (float(br[0]) - 0.50, STAGE_Y + 0.05, 0)])
        pipe = VGroup(
            Line(np.array([TANK_X, STAGE_Y, 0.0]), np.array([TANK_X, PIPE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
            Line(np.array([TANK_X, PIPE_Y, 0.0]), np.array([float(br[0]) - 0.50, PIPE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
            Line(np.array([float(br[0]) - 0.50, PIPE_Y, 0.0]), np.array([float(br[0]) - 0.50, STAGE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
            Line(np.array([METER_X, PIPE_Y, 0.0]), np.array([METER_X, STAGE_Y, 0.0]), color=COL_PRIM, stroke_width=2.4),
        )
        gas_flow = _stream(gas_path, gas, color=COL_PRIM, color_end=COL_END, count=14, speed=0.09, radius=0.05)
        meter_glyph = _gas_meter(np.array([METER_X, STAGE_Y + 0.37, 0.0]), gas)
        tank = energy_tank(np.array([TANK_X, STAGE_Y + 0.75, 0.0]), height=1.30, width=0.80, color=COL_PRIM, level=1.0)
        scale = _class_scale(CLASS_X, STAGE_Y + 1.98, STAGE_Y)
        cuts = VGroup(*[DashedLine(np.array([x, STAGE_Y + 0.02, 0.0]), np.array([x, 1.50, 0.0]), color=c, stroke_width=2.2,
                                   dash_length=0.12)
                        for x, c in zip(CUT_X, (COL_NUTZ, COL_END, COL_PRIM))])
        names = VGroup(*[body_text(t, font_size=LABEL_FONT_SIZE, color=c).move_to(np.array([x, 1.78, 0.0]))
                         for x, t, c in zip(CUT_X, ("Raum", "Zähler", "Quelle"), (COL_NUTZ, COL_END, COL_PRIM))])
        ausweis = body_text("Ausweis", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([CLASS_X, STAGE_Y + 2.25, 0.0]))
        warm_spots = [rad.get_top() + UP * 0.02, person.get_top() + UP * 0.04]

        def heat(rt, r_max=0.30):
            return [ripples(warm_spots, r_max=r_max, cycles=max(1.0, rt / 1.4))]

        self.play(FadeOut(tiles[:3]), FadeOut(tiles[3][0]), FadeOut(tiles[3][1]), FadeOut(tiles[3][2]),
                  FadeOut(VGroup(lamp["group"], gauge["group"], gauge["column"], hs_heat["group"], hs_cool["group"], sun_small,
                                 sun_beams["group"])),
                  FadeOut(thread_guide), FadeOut(thread_flow),
                  ReplacementTransform(hs_small["group"], hs["group"]), run_time=1.5)
        self.remove(thread_flow)
        self.play(FadeIn(rad), FadeIn(boiler), FadeIn(person), Create(cuts[0]), FadeIn(names[0]), *heat(1.0), run_time=1.0)
        self.add(gas_flow, tank["fill"])
        self.play(Create(pipe), FadeIn(meter_glyph), gas.animate.set_value(1.0), Create(cuts[1]), FadeIn(names[1]),
                  *heat(1.4), run_time=1.4)
        self.play(FadeIn(tank["group"]), Create(cuts[2]), FadeIn(names[2]), tank["level"].animate.set_value(0.8),
                  *heat(1.4), run_time=1.4)
        self.play(FadeIn(scale["bands"], shift=LEFT * 0.2), FadeIn(ausweis), *heat(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "bilanz", used=1.5 + 1.0 + 1.4 + 1.4 + 1.2 + 0.35, during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "zahlen"))
        stack_a = _stack(X_A, [COL_NUTZ])
        stack_b = _stack(X_B, [COL_NUTZ, *LOSS_COLOURS])
        stack_c = _stack(X_C, [COL_NUTZ, *LOSS_COLOURS, COL_PRIM])
        stack_bw = _stack(X_B_WP, [COL_END], width=0.28)
        stack_cw = _stack(X_C_WP, [COL_END, COL_PRIM], width=0.28)
        self.add(stack_a["mob"], stack_b["mob"], stack_c["mob"], stack_bw["mob"], stack_cw["mob"])
        guide = DashedLine(np.array([X_A - 0.25, STAGE_Y + CHAIN_NUTZ * STAGE_UNIT, 0.0]),
                           np.array([X_C + 0.25, STAGE_Y + CHAIN_NUTZ * STAGE_UNIT, 0.0]), color=P_WHITE, stroke_width=1.6,
                           dash_length=0.10, stroke_opacity=0.5)
        mode = ValueTracker(0.0)

        def blend(gas_value, wp_value):
            w = float(np.clip(mode.get_value(), 0.0, 1.0))
            return (1 - w) * gas_value + w * wp_value

        def ratio(num, den):
            return num() / den() if den() > 1e-6 else 0.0

        row, box, items = math_panel([
            ("qh", r"Q_{h}", COL_NUTZ), (None, r"\cdot", P_WHITE), ("ep", r"e_{p}", COL_END), (None, r"\cdot", P_WHITE),
            ("fp", r"f_{p}", COL_PRIM), (None, "=", P_WHITE), ("qp", r"Q_{P}", COL_PRIM),
            (None, r"\;[\mathrm{kWh/(m^{2}\,a)}]", P_TEAL),
        ])
        read_y = float(box.get_top()[1]) + 0.24
        ep_read = math_readout(lambda: de_num(blend(ratio(stack_b["total"], stack_a["total"]),
                                                    ratio(stack_bw["total"], stack_a["total"])), 2),
                               np.array([items["ep"].get_center()[0], read_y, 0.0]), size=LABEL_FONT_SIZE, color=COL_END, edge="center")
        fp_read = math_readout(lambda: de_num(blend(ratio(stack_c["total"], stack_b["total"]),
                                                    ratio(stack_cw["total"], stack_bw["total"])), 2),
                               np.array([items["fp"].get_center()[0], read_y, 0.0]), size=LABEL_FONT_SIZE, color=COL_PRIM, edge="center")

        def top_read(src, stack, x_left, colour, wp=None):
            return math_readout(
                lambda: rf"{src}={de_num(blend(stack['total'](), wp['total']()) if wp else stack['total']())}",
                lambda: np.array([x_left, stack["top"]() + 0.30, 0.0]), size=LABEL_FONT_SIZE, color=colour, edge="left")

        qh_read = top_read("Q_{h}", stack_a, X_A - 0.25, COL_NUTZ)
        qe_read = top_read("Q_{E}", stack_b, X_B - 0.25, COL_END)
        qp_read = top_read("Q_{P}", stack_c, X_C - 0.25, COL_PRIM)
        wp_e_read = math_readout(lambda: de_num(stack_bw["total"]()), lambda: np.array([X_B_WP, stack_bw["top"]() + 0.28, 0.0]),
                                 size=LABEL_FONT_SIZE, color=COL_END, edge="center")
        wp_p_read = math_readout(lambda: de_num(stack_cw["total"]()), lambda: np.array([X_C_WP, stack_cw["top"]() + 0.28, 0.0]),
                                 size=LABEL_FONT_SIZE, color=COL_PRIM, edge="center")
        unit_note = math_label(r"\text{in}\;\mathrm{kWh/(m^{2}\,a)}\;\text{— schematisches Beispiel, Gaskessel-Variante}",
                               at=np.array([0.0, 2.30, 0.0]), size=LABEL_FONT_SIZE, color=P_TEAL)
        gas_class, gas_colour = _class_of(CHAIN_END)
        wp_class, wp_colour = _class_of(CHAIN_END_WP)
        y_gas, y_wp = scale["centres"][gas_class][1], scale["centres"][wp_class][1]
        chip_gas = _class_chip(gas_class, gas_colour, np.array([CHIP_X, y_gas, 0.0]))
        chip_wp = _class_chip(wp_class, wp_colour, np.array([CHIP_X, y_wp, 0.0]))
        link_gas = Line(chip_gas.get_right(), np.array([CLASS_X - 0.31, y_gas, 0.0]), color=gas_colour, stroke_width=2.4)
        link_wp = Line(chip_wp.get_right(), np.array([CLASS_X - 0.31, y_wp, 0.0]), color=wp_colour, stroke_width=2.4)
        tag_gas = body_text("Gaskessel", font_size=LABEL_FONT_SIZE, color=gas_colour).next_to(chip_gas, DOWN, buff=0.14)
        tag_wp = body_text("Wärmepumpe", font_size=LABEL_FONT_SIZE, color=wp_colour).next_to(chip_wp, UP, buff=0.14)

        self.add(qh_read)
        self.play(FadeIn(row), Create(box), FadeIn(unit_note), stack_a["heights"][0].animate.set_value(CHAIN_NUTZ),
                  *heat(1.4), run_time=1.4)
        ring_h = highlight_param(items, "qh", color=COL_NUTZ)
        self.play(Create(ring_h), Create(guide), *heat(0.5), run_time=0.5)
        self.add(qe_read, ep_read)
        ring_e = highlight_param(items, "ep", color=COL_END)
        self.play(FadeOut(ring_h), Create(ring_e), stack_b["heights"][0].animate.set_value(CHAIN_NUTZ), run_time=0.8)
        for h, value in zip(stack_b["heights"][1:], LOSS_VALUES):
            self.play(h.animate.set_value(value), run_time=0.6)
        self.add(qp_read, fp_read)
        ring_f = highlight_param(items, "fp", color=COL_PRIM)
        self.play(FadeOut(ring_e), Create(ring_f),
                  *[h.animate.set_value(v) for h, v in zip(stack_c["heights"][:-1], (CHAIN_NUTZ, *LOSS_VALUES))], run_time=1.0)
        self.play(stack_c["heights"][-1].animate.set_value(CHAIN_PRIM - CHAIN_END), run_time=0.9)
        self.play(FadeOut(ring_f), FadeIn(link_gas), FadeIn(chip_gas, shift=LEFT * 0.2), FadeIn(tag_gas), run_time=1.0)
        self.add(wp_e_read, wp_p_read)
        self.play(stack_bw["heights"][0].animate.set_value(CHAIN_END_WP), run_time=0.9)
        self.play(stack_cw["heights"][0].animate.set_value(CHAIN_END_WP),
                  stack_cw["heights"][1].animate.set_value(CHAIN_PRIM_WP - CHAIN_END_WP),
                  mode.animate.set_value(1.0), run_time=1.0)
        self.play(TransformFromCopy(chip_gas, chip_wp), FadeIn(link_wp), FadeIn(tag_wp), run_time=1.2)
        hold_for(self, self.NARRATION, "zahlen", used=1.4 + 0.5 + 0.8 + 0.6 * 4 + 1.0 + 0.9 + 1.0 + 0.9 + 1.0 + 1.2 + 0.35,
                 during=heat)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "outro"))
        tops = [np.array([X_A, stack_a["top"](), 0.0]), np.array([X_B, stack_b["top"](), 0.0]),
                np.array([X_C, stack_c["top"](), 0.0])]
        cold = np.array([float(bl[0]), STAGE_Y + 0.55, 0.0])
        path = [cold, tops[0], tops[1], tops[2], chip_gas.get_left() + LEFT * 0.05]
        landlord = _stand(STAGE_Y, room_h, 3.85)
        tenant = _stand(STAGE_Y, room_h, 4.55, color=P_CYAN)
        speech = _class_chip(gas_class, gas_colour, np.array([3.85, 0.50, 0.0]))
        cloud = cloud_glyph(speech, pad=(0.34, 0.26), bump=0.40)
        trail = cloud_trail(cloud, landlord.get_top() + UP * 0.12)
        cold_ripples = [ripples([cold], r_max=0.5, color=COLD_BLUE, facing=PI, cycles=2.0)]
        self.play(FadeOut(VGroup(ep_read, fp_read, qh_read, qe_read, qp_read, wp_e_read, wp_p_read, unit_note, tag_gas, tag_wp,
                                 link_wp, chip_wp, chip_gas, link_gas)), run_time=0.5)
        self.play(*cold_ripples, run_time=1.6)
        self.play(pulse_flashes([path], P_YELLOW, repeats=2, width=5.0), run_time=2.6)
        self.play(FadeIn(landlord), FadeIn(tenant), run_time=0.7)
        self.play(FadeIn(cloud), FadeIn(speech), FadeIn(trail), run_time=0.9)
        hold_for(self, self.NARRATION, "outro", used=0.5 + 1.6 + 2.6 + 0.7 + 0.9 + 0.35, during=heat)

        for mob in (gas_flow, meter_glyph[3], ep_read, fp_read, qh_read, qe_read, qp_read, wp_e_read, wp_p_read):
            mob.clear_updaters()
        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.6)

#endregion
