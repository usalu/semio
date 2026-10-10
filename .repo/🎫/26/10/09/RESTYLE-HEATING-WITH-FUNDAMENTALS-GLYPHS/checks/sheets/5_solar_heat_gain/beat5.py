subtitle = beat_subtitle("Verschattungsfaktor", title)
        din = _din_ref("DIN V 18599-2 · DIN 4108-2")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)
        caption = caption_bar(subtitle_text(N, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        #region section
        glass_out, glass_in = -3.5, -3.3
        wall_top, wall_bottom = 1.55, -0.85
        slab = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)
        lintel = Rectangle(width=0.5, height=0.6, **slab).move_to([-3.4, wall_top + 0.3, 0])
        sill = Rectangle(width=0.5, height=0.6, **slab).move_to([-3.4, wall_bottom - 0.3, 0])
        masonry = VGroup(lintel, sill, _hatch(lintel), _hatch(sill)).set_z_index(3)
        glazing = Rectangle(width=glass_in - glass_out, height=wall_top - wall_bottom, color=COLOR_WIN, stroke_width=2.5,
                            fill_color=COLOR_WIN, fill_opacity=0.15).move_to([(glass_out + glass_in) / 2,
                                                                              (wall_top + wall_bottom) / 2, 0])
        glazing.set_z_index(3)
        slab_left, room_end_x = lintel.get_right()[0], -0.7
        ceiling_y, floor_y = lintel.get_top()[1], sill.get_bottom()[1]
        t = 0.16
        room = VGroup(
            Rectangle(width=room_end_x + t - slab_left, height=t, **slab).move_to(
                [(slab_left + room_end_x + t) / 2, ceiling_y - t / 2, 0]),
            Rectangle(width=room_end_x + t - slab_left, height=t, **slab).move_to(
                [(slab_left + room_end_x + t) / 2, floor_y - t / 2, 0]),
            Rectangle(width=t, height=ceiling_y - floor_y - t, **slab).move_to(
                [room_end_x + t / 2, (ceiling_y + floor_y - t) / 2, 0]),
        )
        lbl_out = Text("außen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([-6.2, 0.35, 0])
        lbl_in = Text("innen", font_size=BODY_FONT_SIZE, color=PASTEL_TEAL).move_to([-1.55, 1.6, 0])
        self.play(Create(masonry), Create(glazing), Create(room), run_time=1.6)
        self.play(FadeIn(lbl_out), FadeIn(lbl_in), run_time=0.6)
        hold_for(self, N, "intro", used=2.2 + 0.3)
        #endregion

        #region rays
        d = np.array([0.75, -0.661, 0.0])
        sun_c = np.array([-6.25, 1.85, 0.0])
        sun = _sun(sun_c)
        slat_x = -4.4
        slat_ys = [1.35 - i * 0.32 for i in range(8)]
        glass_ys = [y - 0.793 for y in slat_ys[:6]]

        def ray(end, back=3.0, **kw):
            return _soft_ray(end - d * back, end, **kw).set_z_index(1)

        def through(start):
            t_hit = min(t for t in ((floor_y - start[1]) / d[1], (room_end_x - start[0]) / d[0]) if t > 0.05)
            return start + d * t_hit

        def warm_spots(ends):
            return [(e + UP * 0.03, None) if e[1] <= floor_y + 1e-6 else (e + LEFT * 0.03, PI) for e in ends]

        def inside(ys, **kw):
            return VGroup(*[_soft_ray([glass_in, y, 0], through(np.array([glass_in, y, 0.0])), fade=False, **kw)
                            .set_z_index(1) for y in ys])

        direct = VGroup(*[ray(np.array([glass_out, y, 0.0]), width=2.4, opacity=0.8) for y in glass_ys])
        interior = inside(glass_ys[:5], width=2.4, opacity=0.6)
        direct_paths = [[r.get_start(), r.get_end(), s.get_end()] for r, s in zip(direct, interior)]
        direct_spots = warm_spots([s.get_end() for s in interior[::2]])
        #endregion

        #region F_sh scale
        sx0, sx1, sy = 1.3, 5.9, 0.35
        scale = VGroup(
            Line([sx0, sy, 0], [sx1, sy, 0], color=PASTEL_WHITE, stroke_width=2),
            *[Line([x, sy - 0.12, 0], [x, sy + 0.12, 0], color=PASTEL_WHITE, stroke_width=2) for x in (sx0, sx1)],
        )
        end_l = VGroup(math_label("0", size=BODY_FONT_SIZE), Text("ganz verschattet", font_size=LABEL_FONT_SIZE,
                                                                 color=COLOR_FRAME)).arrange(DOWN, buff=0.1)
        end_l.next_to([sx0, sy, 0], DOWN, buff=0.25)
        end_r = VGroup(math_label("1", size=BODY_FONT_SIZE), Text("ohne Verschattung", font_size=LABEL_FONT_SIZE,
                                                                 color=COLOR_FRAME)).arrange(DOWN, buff=0.1)
        end_r.next_to([sx1, sy, 0], DOWN, buff=0.25)
        fsh = ValueTracker(1.0)

        def sx(v):
            return sx0 + v * (sx1 - sx0)

        marker = always_redraw(lambda: Triangle(stroke_width=0, fill_color=COLOR_G, fill_opacity=1.0)
                               .scale(0.15).rotate(PI).move_to([sx(fsh.get_value()), sy + 0.24, 0]))
        marker_lbl = math_readout(lambda: rf"F_{{\mathrm{{sh}}}} = {de_num(fsh.get_value(), 2)}",
                                  lambda: np.array([sx(fsh.get_value()), sy + 0.55, 0]),
                                  size=BODY_FONT_SIZE, color=COLOR_G, edge="center")
        #endregion

        def sunlit(paths, spots):
            def during(rt):
                cycles = max(1.0, rt / 1.4)
                return [_pulses(paths, rt, every=1.8),
                        *[ripples([c], r_max=0.5, color=PASTEL_ORANGE, cycles=cycles, facing=f) for c, f in spots]]
            return during

        caption = swap_caption(self, caption, subtitle_text(N, "unshaded"))
        self.play(FadeIn(sun, scale=0.7), run_time=0.6)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in direct], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in interior], lag_ratio=0.1), run_time=1.2)
        self.play(Create(scale), FadeIn(end_l), FadeIn(end_r), *sunlit(direct_paths, direct_spots)(0.8), run_time=0.8)
        self.add(marker, marker_lbl)
        self.play(FadeIn(marker), run_time=0.4)
        hold_for(self, N, "unshaded", during=sunlit(direct_paths, direct_spots))

        #region Raffstore
        caption = swap_caption(self, caption, subtitle_text(N, "raffstore"))
        u = np.array([np.cos(48 * DEGREES), np.sin(48 * DEGREES), 0.0])
        nrm = np.array([-u[1], u[0], 0.0])
        refl_dir = d - 2 * float(np.dot(d, nrm)) * nrm
        slats = VGroup(*[Rectangle(width=0.5, height=0.08, color=COLOR_BLIND, stroke_width=1.5, fill_color=COLOR_BLIND,
                                   fill_opacity=0.85).move_to([slat_x, y, 0]).rotate(48 * DEGREES) for y in slat_ys])
        rail = Rectangle(width=0.6, height=0.24, color=COLOR_BLIND, stroke_width=2, fill_color=COLOR_BLIND,
                         fill_opacity=0.9).move_to([slat_x, 1.95, 0])
        bracket = Line([slat_x + 0.3, 1.95, 0], [-3.65, 1.95, 0], color=COLOR_BLIND, stroke_width=2.5)
        blind = VGroup(rail, bracket, slats).set_z_index(4)
        lbl_blind = Text("Raffstore", font_size=LABEL_FONT_SIZE, color=COLOR_BLIND).move_to([-4.95, -1.45, 0])
        blocked = VGroup(*[ray(np.array([glass_out, y, 0.0]), back=3.0 - 1.316, width=2.4, opacity=0.8)
                           .shift(-d * 1.316) for y in glass_ys])
        refl_starts = [np.array([-4.487, y + 0.077, 0]) for y in slat_ys[:6]]
        reflected = VGroup(*[radiation_ray(p, p + refl_dir * 1.3, color=COLOR_G, stroke_width=2.2)
                             .set_stroke(opacity=0.55).set_z_index(5) for p in refl_starts])
        bounce_paths = [[b.get_start(), b.get_end(), r.get_end()] for b, r in zip(blocked, reflected)]
        blind.shift(UP * 3.0).set_opacity(0)
        self.add(blind)
        self.play(blind.animate.shift(DOWN * 3.0).set_opacity(1.0), fsh.animate.set_value(0.0), run_time=1.6)
        self.play(FadeIn(lbl_blind, shift=UP * 0.15),
                  *[Transform(r, c) for r, c in zip(direct, blocked)],
                  LaggedStart(*[Create(a, rate_func=linear) for a in reflected], lag_ratio=0.08), FadeOut(interior),
                  run_time=1.6)
        hold_for(self, N, "raffstore", during=sunlit(bounce_paths, []))
        #endregion

        #region residue
        caption = swap_caption(self, caption, subtitle_text(N, "reduced"))
        residual = VGroup(*[radiation_ray([-4.226, y + 0.193, 0], through(np.array([-4.226, y + 0.193, 0.0])),
                                          color=COLOR_G, stroke_width=1.4).set_stroke(opacity=0.32).set_z_index(1)
                            for y in slat_ys[:5]])
        rest_paths = [[r.get_start(), r.get_end()] for r in residual]

        def shaded(rt):
            return [*sunlit(bounce_paths, [])(rt), _pulses(rest_paths, rt, width=2.5, every=1.8, lead=0.0)]

        self.play(LaggedStart(*[Create(r) for r in residual], lag_ratio=0.1), fsh.animate.set_value(FSH_BLIND),
                  run_time=1.8)
        hold_for(self, N, "reduced", during=shaded)
        #endregion

        #region winter: blind up, overhang
        caption = swap_caption(self, caption, subtitle_text(N, "winter"))
        overhang = Rectangle(width=1.25, height=0.16, **slab).move_to([glass_out - 0.62, wall_top + 0.08, 0]).set_z_index(4)
        lbl_over = Text("Dachüberstand", font_size=LABEL_FONT_SIZE, color=COLOR_FRAME)
        lbl_over.move_to([overhang.get_right()[0] + 0.2 - lbl_over.width / 2, lintel.get_top()[1] + 0.25, 0])
        winter_direct = VGroup(*[ray(np.array([glass_out, y, 0.0]), width=2.4, opacity=0.8) for y in glass_ys[1:]])
        winter_cut = ray(np.array([glass_out, glass_ys[0], 0.0]), back=3.0 - 1.45, width=2.4,
                         opacity=0.8).shift(-d * 1.45)
        winter_in = inside(glass_ys[1:5], width=2.4, opacity=0.6)
        winter_paths = [[r.get_start(), r.get_end(), s.get_end()] for r, s in zip(winter_direct, winter_in)]
        winter_paths.append([winter_cut.get_start(), winter_cut.get_end()])
        winter_spots = warm_spots([s.get_end() for s in winter_in[::2]])
        self.play(blind.animate.shift(UP * 3.0).set_opacity(0), FadeOut(lbl_blind), FadeOut(reflected),
                  FadeOut(residual), FadeOut(direct), run_time=1.2)
        self.play(FadeIn(overhang), FadeIn(lbl_over), Create(winter_cut, rate_func=linear),
                  LaggedStart(*[Create(r, rate_func=linear) for r in winter_direct], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(r, rate_func=linear) for r in winter_in], lag_ratio=0.1),
                  fsh.animate.set_value(FSH_WINTER), run_time=1.6)
        winter_sun = sunlit(winter_paths, winter_spots)
        hold_for(self, N, "winter", during=winter_sun)
        #endregion

        #region formula
        caption = swap_caption(self, caption, subtitle_text(N, "fsh"))
        _freeze(marker, marker_lbl)
        sym = math_label(r"F_{\mathrm{sh}}", size=BODY_FONT_SIZE, color=COLOR_G)
        sym.move_to(marker_lbl.get_left() + RIGHT * sym.width / 2)
        panel = math_panel([
            ("phi", r"\Phi", PASTEL_WHITE), (None, "=", PASTEL_WHITE),
            ("g_irr", "G", COLOR_G), (None, r"\cdot", PASTEL_WHITE),
            ("a", "A", COLOR_A), (None, r"\cdot", PASTEL_WHITE),
            ("ff", r"F_{\mathrm{f}}", COLOR_FF), (None, r"\cdot", PASTEL_WHITE),
            ("g", "g", COLOR_GVAL), (None, r"\cdot", PASTEL_WHITE),
            ("fsh", r"F_{\mathrm{sh}}", COLOR_G), (None, r"\;[\mathrm{W}]", PASTEL_WHITE),
        ], edge_buff=1.35)
        _fly_into(self, panel, {"fsh": sym})
        ring = highlight_param(panel[2], "fsh", color=COLOR_G)
        self.play(Create(ring), run_time=0.4)
        hold_for(self, N, "fsh", during=winter_sun)
        self.play(FadeOut(ring), run_time=0.25)
        #endregion

        self.play(FadeOut(caption), run_time=0.3)
        self.wait(0.5)
#endregion
