from pathlib import Path

path = Path("🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py")
text = open(str(path), encoding="utf8", newline="").read()


def swap(old, new):
    global text
    assert text.count(old) == 1, old
    text = text.replace(old, new)


swap('''    unit = RoundedRectangle(corner_radius=0.08, width=0.78, height=1.5, color=P_WHITE, stroke_width=1.8,
                            fill_color=P_DEEP_DARK, fill_opacity=1.0)
    unit.move_to(np.array([-3.15, floor_y + 0.75, 0.0]))
    fan_center = unit.get_top() + DOWN * 0.38
    fan_ring = Circle(radius=0.2, color=P_CYAN, stroke_width=1.6).move_to(fan_center)
    fan_blades = VGroup(*[
        Line(fan_center, fan_center + 0.17 * np.array([np.cos(a), np.sin(a), 0.0]), color=P_CYAN, stroke_width=2.0)
        for a in np.linspace(0, TAU, 3, endpoint=False)
    ])
    coil_c = unit.get_bottom() + UP * 0.42
    coil = VMobject(color=P_BLUE, stroke_width=1.6).set_points_as_corners(
        [coil_c + np.array([-0.27 + i * 0.09, 0.08 if i % 2 else -0.08, 0.0]) for i in range(7)])
''', '''    unit = fitted_staffage("cooling-heating-comparison-cooler", width=0.9, height=1.5, rate=0.0)
    unit.move_to(np.array([-3.15, floor_y + 0.75, 0.0]))
    unit.shift(UP * (floor_y - unit.get_bottom()[1]))
    fan_center = unit.get_top() + DOWN * 0.38
    coil_c = unit.get_bottom() + UP * 0.42
''')
swap('''        "unit": VGroup(unit, fan_ring, fan_blades, coil), "fan_blades": fan_blades, "fan_center": fan_center,
''', '''        "unit": unit, "coil_c": coil_c,
''')
swap('''        def fan_spin(turns: float, run_time: float):
            return Rotate(ahu["fan_blades"], angle=turns * TAU, about_point=ahu["fan_center"],
                          run_time=run_time, rate_func=linear)

        def running(rt):
            return [fan_spin(rt / 1.3, rt), flow_animation(streams, waves=5, cycles=rt / 1.6)]
''', '''        def running(rt):
            return [flow_animation(streams, waves=5, cycles=rt / 1.6)]
''')
swap('''        coil = ahu["unit"][-1]
        coil_temp = ValueTracker(32.0)''', '''        ahu["unit"].rate = COOLER_RATE
        coil_temp = ValueTracker(32.0)''')
swap("lambda: coil.get_center() + LEFT * 1.45,", 'lambda: ahu["coil_c"] + LEFT * 1.45,')
swap('''            coil.animate.set_color(P_BLUE).set_stroke(width=3.2),
            coil_temp.animate.set_value(18),''', '''            coil_temp.animate.set_value(18),''')
swap("LAPTOP_RATE = 24.0\n", "LAPTOP_RATE = 24.0\nCOOLER_RATE = 15.0\n")
open(str(path), "w", encoding="utf8", newline="").write(text)
print("ok")
