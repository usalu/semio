#region Beat6 — Wärmespeicherung
class Beat6_Waermespeicherung(Scene):
    """🧱 Floor slab stores winter sun by day and radiates at night."""

    NARRATION = [
        ("day",
         "Dense floor slab absorbs winter sun through the window.",
         "Die dichte Bodenplatte absorbiert Wintersonne durchs Fenster."),
        ("night",
         "At night the stored heat radiates back into the room.",
         "Nachts strahlt die gespeicherte Wärme zurück in den Raum."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Wärmespeicherung und Strahlung", title)
        din = _din_ref("DIN EN ISO 13786")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "day"))
        self.play(FadeIn(caption), run_time=0.3)

        #region room, slab and winter sun
        room = room_section(np.array([0.7, -0.3, 0.0]), w=5.8, h=2.0, slab=0.55, window=(0.3, 0.88))
        room["glass"].set_stroke(COLOR_WIN).set_fill(COLOR_WIN)
        floor_slab = room["floor"]
        floor_label = Text("Betonbodenplatte", font_size=BODY_FONT_SIZE, color=PASTEL_WHITE)
        floor_label.move_to([room["x_l"] + 0.1 + floor_label.width / 2, floor_slab.get_center()[1], 0])
        phase_1 = Text("Tag: Absorption", font_size=BODY_FONT_SIZE, color=COLOR_G)
        phase_2 = Text("Nacht: Abstrahlung", font_size=BODY_FONT_SIZE, color=COLOR_HEAT)
        phase_1.next_to(room["ceiling"], UP, buff=0.35)
        phase_2.move_to(phase_1)

        sun_c = np.array([-5.5, 1.0, 0.0])
        sun = _sun(sun_c)
        sun_label = Text("Wintersonne", font_size=BODY_FONT_SIZE, color=COLOR_G).next_to(sun, UP, buff=0.15)
        ys = np.linspace(room["win_hi"] - 0.12, room["win_lo"] + 0.12, 4)
        rays = sun_rays(sun_c, room["glass_x"], ys, room["y_f"], gap=0.5)
        rays["out"].set_stroke(color=COLOR_G)
        rays["in"].set_stroke(color=COLOR_G)
        lands = rays["lands"]
        patch = Line(lands[-1], lands[0], color=COLOR_G, stroke_width=7, stroke_opacity=0.8)
        ray_paths = [[s, h, p] for s, h, p in zip(rays["starts"], rays["hits"], rays["lands"])]
        charge = [interpolate(lands[-1], lands[0], f) + DOWN * 0.03 for f in (0.45, 0.85)]

        def daylight(rt):
            return [_pulses(ray_paths, rt, lead=0.0),
                    ripples(charge, r_max=0.42, color=COLOR_HEAT, cycles=max(1.0, rt / 1.2), down=True)]

        self.play(Create(room["group"]), FadeIn(floor_label), FadeIn(phase_1), FadeIn(sun, scale=0.7),
                  FadeIn(sun_label), run_time=1.6)
        self.play(shine(rays), run_time=1.2)
        self.play(FadeIn(patch, scale=0.6), floor_slab.animate.set_fill(COLOR_HEAT, opacity=0.55),
                  *daylight(1.8), run_time=1.8)
        hold_for(self, self.NARRATION, "day", used=1.6 + 1.2 + 1.8 + 0.3, during=lambda rt: [
            *daylight(rt), floor_slab.animate.set_fill(COLOR_HEAT, opacity=0.75)])
        #endregion

        #region night
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "night"))
        moon = moon_glyph(sun_c, color=COLOR_FRAME)
        moon_label = Text("Nachthimmel", font_size=BODY_FONT_SIZE, color=COLOR_FRAME).next_to(moon, UP, buff=0.2)
        emit = [np.array([x, room["y_f"] + 0.03, 0.0]) for x in np.linspace(room["x_l"] + 0.7, room["x_r"] - 0.7, 5)]
        rad_label = Text("Gespeicherte Wärme", font_size=BODY_FONT_SIZE, color=COLOR_HEAT)
        rad_label.move_to([room["center"][0], room["y_c"] - 0.4, 0])

        def night_glow(rt):
            return [ripples(emit, r_max=0.85, color=COLOR_HEAT, cycles=max(1.0, rt / 1.5)),
                    floor_slab.animate.set_fill(COLOR_HEAT, opacity=0.4)]

        self.play(FadeOut(rays["group"]), FadeOut(patch), FadeOut(sun), FadeOut(sun_label),
                  ReplacementTransform(phase_1, phase_2), FadeIn(moon), FadeIn(moon_label), run_time=1.4)
        self.play(FadeIn(rad_label), ripples(emit, r_max=0.85, color=COLOR_HEAT, cycles=1.2), run_time=1.8)
        hold_for(self, self.NARRATION, "night", during=night_glow)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
