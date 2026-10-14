p='scene_7.py'
s=open(p).read()
def rep(a,b):
    global s
    assert a in s, a[:80]
    s=s.replace(a,b,1)

rep("STAGE_Y, STAGE_UNIT = -0.70, 0.0115","STAGE_Y, STAGE_UNIT = -0.55, 0.0115")

# tiles
rep('''        xs = (-4.95, -1.65, 1.65, 4.95)
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
''','''        xs = (-4.95, -1.65, 1.65, 4.95)
        small = 0.42
        anchor = np.array([xs[0] - 0.55, 1.62, 0.0])
        lamp = lamp_glyph(anchor, drop=0.25)
        lamp["group"].scale(1.5, about_point=anchor)
        gauge = thermometer_glyph(np.array([xs[0] + 0.62, 0.22, 0.0]), height=1.15, color=P_RED, level=0.0)
        glow = [ripples([lamp["bulb"].get_center()], r_max=0.55, color=P_YELLOW, down=True, cycles=2.0)]
        hs_heat = house_section(np.array([xs[1], 0.78, 0.0]), scale=small)
        hs_cool = house_section(np.array([xs[2] + 0.30, 0.78, 0.0]), scale=small)
        hs_small = house_section(np.array([xs[3], 0.78, 0.0]), scale=small)
        sun_small_c = np.array([xs[2] - 1.05, 1.45, 0.0])
        sun_small = sun_glyph(sun_small_c).scale(0.34)
        cool_win = hs_cool["windows"][1]
        sun_beams = sun_rays(sun_small_c, cool_win["x"], [cool_win["sill"] + 0.04, cool_win["center"][1], cool_win["lintel"] - 0.04],
                             hs_cool["bottom_left"][1] + 0.02, gap=0.36)
        beam_paths = [[s_, h_, l_] for s_, h_, l_ in zip(sun_beams["starts"], sun_beams["hits"], sun_beams["lands"])]
        heat_paths = [smooth_path([(xs[1] + 0.30, y, 0), (xs[1] + 0.80, y + 0.02, 0), (xs[1] + 1.30, y + 0.04, 0)])
                      for y in (0.50, 0.78, 1.06)]
''')
rep('''        leaving = [(heat_paths, P_RED, P_ORANGE)]
        self.play(flow_animation(leaving, waves=3, radius=0.045, cycles=2.0), tiles[1][0].animate.set_stroke(width=3.6),
                  run_time=1.8)
        self.play(shine(sun_beams), tiles[2][0].animate.set_stroke(width=3.6), run_time=1.8)
        hold_for(self, self.NARRATION, "kette", used=1.6 + 1.8 + 1.8 + 0.35,
                 during=lambda rt: [flow_animation(leaving + [(cool_paths, P_YELLOW, P_ORANGE)], waves=3, radius=0.045,
                                                   cycles=max(1.0, rt / 0.9))])''','''        leaving = [(heat_paths, P_RED, P_ORANGE)]
        self.play(flow_animation(leaving, waves=3, radius=0.05, cycles=2.0), tiles[1][0].animate.set_stroke(width=3.6),
                  run_time=1.8)
        self.play(shine(sun_beams), tiles[2][0].animate.set_stroke(width=3.6), run_time=1.8)
        hold_for(self, self.NARRATION, "kette", used=1.6 + 1.8 + 1.8 + 0.35,
                 during=lambda rt: [flow_animation(leaving, waves=3, radius=0.05, cycles=max(1.0, rt / 0.9)),
                                    pulse_flashes(beam_paths, P_YELLOW, repeats=max(1, int(rt / 1.6)), width=3.5)])''')

# stage geometry
rep("hs = house_section(np.array([-5.15, STAGE_Y + 0.84, 0.0]), scale=0.70)","hs = house_section(np.array([-5.15, STAGE_Y + 0.768, 0.0]), scale=0.64)")
rep("rad = radiator(np.array([float(bl[0]) + 0.75, STAGE_Y + 0.17, 0.0])).scale(0.55)","rad = radiator(np.array([float(bl[0]) + 0.62, STAGE_Y + 0.17, 0.0])).scale(0.5)")
rep("boiler = _boiler(np.array([float(br[0]) - 0.50, STAGE_Y + 0.32 + 0.02, 0.0]))","boiler = _boiler(np.array([float(br[0]) - 0.42, STAGE_Y + 0.32 + 0.02, 0.0]))")
rep("person = _stand(STAGE_Y, room_h, -5.20)","person = _stand(STAGE_Y, room_h, float(bl[0]) + 1.25)")
s=s.replace("float(br[0]) - 0.50","float(br[0]) - 0.42")
rep("np.array([x, 1.40, 0.0]), color=c, stroke_width=2.2,","np.array([x, 1.55, 0.0]), color=c, stroke_width=2.2,")
rep("np.array([x, 1.62, 0.0]))\n                         for x, t, c in zip(CUT_X","np.array([x, 1.75, 0.0]))\n                         for x, t, c in zip(CUT_X")
rep("move_to(np.array([CLASS_X, STAGE_Y + 2.35, 0.0]))","move_to(np.array([CLASS_X, STAGE_Y + 2.20, 0.0]))")

# panel unit + size
s=s.replace(r"\mathrm{kWh/(m^{2}\,a)}",r"\mathrm{kWh/(m^{2}\cdot a)}")
rep("], size=26, edge_buff=2.32)","], size=24, edge_buff=2.32)")

# no blend
a=s.index("        mode = ValueTracker(0.0)\n")
b=s.index("        row, box, items = math_panel([\n            (\"qh\"")
s=s[:a]+'''
        def ratio(num, den):
            return num() / den() if den() > 1e-6 else 0.0

'''+s[b:]
a=s.index("        def e_p():")
b=s.index("        qh_read = top_read(")
s=s[:a]+'''        def e_p():
            return ratio(stack_b["total"], stack_a["total"])

        def f_p():
            return ratio(stack_c["total"], stack_b["total"])

        def top_read(expr, stack, x_left, colour):
            return math_readout(expr, lambda: np.array([x_left, stack["top"]() + 0.30, 0.0]), size=LABEL_FONT_SIZE,
                                color=colour, edge="left")

'''+s[b:]
rep('''qe_read = top_read(lambda: rf"Q_{{E}}={de_num(stack_a['total']())}\\cdot {de_num(e_p(), 2)}={de_num(q_e())}",''','''qe_read = top_read(lambda: rf"Q_{{E}}={de_num(stack_a['total']())}\\cdot {de_num(e_p(), 2)}={de_num(stack_b['total']())}",''')
rep('''qp_read = top_read(lambda: rf"Q_{{P}}={de_num(q_e())}\\cdot {de_num(f_p(), 2)}={de_num(q_p())}",''','''qp_read = top_read(lambda: rf"Q_{{P}}={de_num(stack_b['total']())}\\cdot {de_num(f_p(), 1)}={de_num(stack_c['total']())}",''')
rep("                  mode.animate.set_value(1.0), run_time=1.0)\n","                  run_time=1.0)\n")
open(p,'w').write(s)
