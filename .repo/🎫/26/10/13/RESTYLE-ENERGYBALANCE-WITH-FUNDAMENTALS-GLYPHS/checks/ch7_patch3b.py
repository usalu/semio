p='scene_7.py'
s=open(p).read()
def rep(a,b):
    global s
    assert a in s, a[:80]
    s=s.replace(a,b,1)
rep('''        unit_note = math_label(r"\\text{in}\;\\mathrm{kWh/(m^{2}\\cdot a)}\;\\text{— schematisches Beispiel, Gaskessel-Variante}",
                               at=np.array([0.0, 2.30, 0.0]), size=LABEL_FONT_SIZE, color=P_TEAL)
''','''        unit_note = VGroup(
            body_text("Gaskessel-Variante, schematisches Beispiel, in", font_size=LABEL_FONT_SIZE, color=P_TEAL),
            math_label(r"\\mathrm{kWh/(m^{2}\\cdot a)}", size=LABEL_FONT_SIZE, color=P_TEAL),
        ).arrange(RIGHT, buff=0.14, aligned_edge=DOWN).move_to(np.array([0.0, 2.30, 0.0]))
''')
rep('next_to(chip_gas, DOWN, buff=0.14)','next_to(chip_gas, LEFT, buff=0.16)')
rep('next_to(chip_wp, UP, buff=0.14)','next_to(chip_wp, LEFT, buff=0.16)')
a=s.index("        cold = np.array([float(bl[0]), STAGE_Y + 0.55, 0.0])")
b=s.index("        hold_for(self, self.NARRATION, \"outro\"")
new='''        cold_wall = Line(bl + RIGHT * 0.01, bl + UP * room_h + RIGHT * 0.01, color=COLD_BLUE, stroke_width=6)
        cold_spots = [bl + UP * 0.30 + RIGHT * 0.03, bl + UP * 0.62 + RIGHT * 0.03]
        path = [bl + UP * 0.45, tops[0], tops[1], tops[2], chip_gas.get_left() + LEFT * 0.05]
        trace = VMobject(color=P_YELLOW, stroke_width=2.2, stroke_opacity=0.55).set_points_as_corners(path)
        landlord = _stand(STAGE_Y, room_h, 3.85)
        tenant = _stand(STAGE_Y, room_h, 4.55, color=P_CYAN)
        speech = _class_chip(gas_class, gas_colour, np.array([3.85, 0.62, 0.0]))
        cloud = cloud_glyph(speech, pad=(0.34, 0.26), bump=0.40)
        trail = cloud_trail(cloud, landlord.get_top() + UP * 0.12)

        def cold_air(rt):
            return [ripples(cold_spots, r_max=0.5, color=COLD_BLUE, facing=0.0, cycles=max(1.0, rt / 1.4))]

        self.play(FadeOut(VGroup(qh_read, qe_read, qp_read, wp_e_read, wp_p_read, unit_note, tag_gas, tag_wp,
                                 link_wp, chip_wp, chip_gas, link_gas)), Create(cold_wall), *cold_air(1.4), run_time=1.4)
        self.play(Create(trace), *cold_air(1.4), run_time=1.4)
        self.play(pulse_flashes([path], P_YELLOW, repeats=2, width=5.0), *cold_air(2.6), run_time=2.6)
        self.play(FadeIn(landlord), FadeIn(tenant), FadeOut(trace), run_time=0.7)
        self.play(FadeIn(cloud), FadeIn(speech), FadeIn(trail), run_time=0.9)
'''
s=s[:a]+new+s[b:]
rep('used=0.5 + 1.6 + 2.6 + 0.7 + 0.9 + 0.35, during=heat)','used=1.4 + 1.4 + 2.6 + 0.7 + 0.9 + 0.35, during=lambda rt: heat(rt) + cold_air(rt))')
open(p,'w').write(s)
