#region Beat4 — Saisonale Sonnenwinkel
class Beat4_SaisonaleWinkel(Scene):
    """🌞 Parallel sun rays at 60° and 20°: the overhang blocks summer, winter light lands on floor and lower back wall."""

    NARRATION = [
        ("path",
         "The sun path climbs high in summer and stays low in winter.",
         "Die Sonnenbahn steht im Sommer hoch und im Winter flach."),
        ("summer",
         "The sun is so far away that its rays arrive parallel. At 60 degrees the roof overhang holds nearly all of them back.",
         "Die Sonne ist so weit weg, dass ihre Strahlen parallel einfallen. Bei 60° hält der Dachüberstand fast alle ab."),
        ("winter",
         "At 20 degrees the rays fall flat onto the floor and at most the lower back wall — the upper back corner gets no direct sun.",
         "Bei 20° fallen die Strahlen flach auf den Boden und höchstens unten auf die Rückwand — oben hinten kommt keine Sonne an."),
    ]

    def construct(self):
        apply_scene_style(self)
        N = self.NARRATION

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Saisonale Sonnenwinkel", title)
        din = _din_ref("DIN V 18599-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(N, "path"))
        self.play(FadeIn(caption), run_time=0.3)

        #region room and sun path
        geo = _room_geo(1.0, -1.9, 1.05)
        tip = geo["x_w"] + ROOF_OVERHANG * geo["s"]
        room = _section_room(geo, roof_tip=tip)
        boxes = [room["roof_box"]]
        edges = [(tip, geo["y_c"])]
        heights = np.linspace(geo["y_f"] + 0.15, geo["y_c"] + geo["slab"] - 0.03, 10)
        hub = np.array([geo["x_w"], geo["y_f"], 0.0])
        radius = 3.9
        elev = ValueTracker(ELEV_SUMMER)

        def sun_at():
            return hub + radius * _sun_dir(elev.get_value())

        arc = DashedVMobject(Arc(
            radius=radius, start_angle=np.radians(ELEV_WINTER), angle=np.radians(ELEV_SUMMER - ELEV_WINTER),
            arc_center=hub, color=PASTEL_YELLOW, stroke_width=2,
        ), num_dashes=24)
        arc_label = Text("Sonnenbahn", font_size=BODY_FONT_SIZE, color=PASTEL_YELLOW)
        arc_label.next_to(hub + radius * _sun_dir(ELEV_WINTER), DR, buff=0.35)
        sun = _sun(sun_at())
        sun.add_updater(lambda m: m.move_to(sun_at()))
        elev_read = math_readout(lambda: rf"\text{{Sonnenhöhe}}\;{de_num(elev.get_value())}°",
                                 [4.3, 1.95, 0], size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer = Text("Sommer: Überhang hält steile Strahlen ab", font_size=BODY_FONT_SIZE, color=COLOR_G)
        tag_summer.move_to([-6.5 + tag_summer.width / 2, 1.95, 0])
        tag_winter = Text("Winter: flache Strahlen bis zur unteren Rückwand", font_size=BODY_FONT_SIZE, color=PASTEL_ORANGE)
        tag_winter.move_to([-6.5 + tag_winter.width / 2, 1.95, 0])
        dark_note = Text("oben hinten: keine direkte Sonne", font_size=LABEL_FONT_SIZE, color=GREY_B)
        dark_note.move_to([geo["x_b"] + 0.18 + dark_note.width / 2, geo["y_c"] - 0.3, 0])

        self.play(Create(room["group"]), Create(arc), FadeIn(arc_label), run_time=1.6)
        self.add(sun)
        self.play(FadeIn(sun, scale=0.6), FadeIn(elev_read), run_time=0.9)
        hold_for(self, N, "path", used=1.6 + 0.9 + 0.3,
                 during=lambda rt: [elev.animate(rate_func=there_and_back, run_time=rt).set_value(ELEV_WINTER)])
        #endregion

        #region summer
        caption = swap_caption(self, caption, subtitle_text(N, "summer"))
        rays_now = _sun_rays(ELEV_SUMMER, geo, boxes, heights, sun_at())
        summer_paths = _ray_paths(ELEV_SUMMER, geo, boxes, heights, sun_at())
        patch_now = _patch_mob(_sun_patch(ELEV_SUMMER, geo, edges))
        self.play(LaggedStart(*[Create(r) for r in rays_now], lag_ratio=0.1), FadeIn(patch_now), FadeIn(tag_summer),
                  run_time=1.6)
        hold_for(self, N, "summer", during=lambda rt: [_pulses(summer_paths, rt, every=1.8)])
        #endregion

        #region sweep to winter
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        rays = always_redraw(lambda: _sun_rays(elev.get_value(), geo, boxes, heights, sun_at()))
        patch = always_redraw(lambda: _patch_mob(_sun_patch(elev.get_value(), geo, edges)))
        self.play(FadeOut(rays_now), FadeOut(patch_now), run_time=0.6)
        self.add(patch, rays)
        self.play(
            elev.animate.set_value(ELEV_WINTER),
            FadeTransform(tag_summer, tag_winter),
            run_time=3.0, rate_func=smooth,
        )
        _freeze(rays, patch, sun, elev_read)
        winter_paths = _ray_paths(ELEV_WINTER, geo, boxes, heights, sun_at())
        spots = _patch_spots(_sun_patch(ELEV_WINTER, geo, edges), geo)

        def winter_sun(rt):
            cycles = max(1.0, rt / 1.4)
            return [_pulses(winter_paths, rt, every=1.8),
                    *[ripples([c], r_max=0.6, color=PASTEL_ORANGE, cycles=cycles, facing=f) for c, f in spots]]

        self.play(FadeIn(dark_note), *winter_sun(0.6), run_time=0.6)
        hold_for(self, N, "winter", during=winter_sun)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
