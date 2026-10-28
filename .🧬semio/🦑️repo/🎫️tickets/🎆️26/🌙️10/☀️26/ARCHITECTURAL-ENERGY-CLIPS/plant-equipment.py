from pathlib import Path
p=Path("🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py")
s=p.read_text(encoding="utf-8")
s=s.replace('_SLAB = dict(color=PASTEL_WHITE, stroke_width=2, fill_color=PASTEL_WHITE, fill_opacity=0.14)', '_SLAB = dict(color=PASTEL_WHITE, stroke_width=1.3, fill_color=PASTEL_WHITE, fill_opacity=0.7)')
s=s.replace('tank = energy_tank(P(TANK_X, -1.13), height=0.72, width=0.5, color=PASTEL_YELLOW, level=0.9)', 'tank_h, boiler_h, boiler_w = 1.5 * metre, 1.0 * metre, 0.6 * metre\n    tank = energy_tank(P(TANK_X, -1.6 + tank_h / 2), height=tank_h, width=0.65 * metre, color=PASTEL_YELLOW, level=0.9)')
a='''parts["boiler"] = Rectangle(width=1.6, height=0.7, color=PASTEL_ORANGE, stroke_width=2, fill_color=PASTEL_ORANGE,
                                fill_opacity=0.12).move_to(P(-2.85, -1.2))
    parts["chimney"] = VGroup(
        Line(P(-2.46, -0.85), P(-2.46, 2.3), color=PASTEL_WHITE, stroke_width=1.5),
        Line(P(-2.24, -0.85), P(-2.24, 2.3), color=PASTEL_WHITE, stroke_width=1.5),
    )'''
b='''parts["boiler"] = Rectangle(width=boiler_w, height=boiler_h, color=PASTEL_ORANGE, stroke_width=1.3,
                                fill_color=PASTEL_ORANGE, fill_opacity=0.55).move_to(P(-2.85, -1.6 + boiler_h / 2))
    flue_bottom, flue_top = -1.6 + boiler_h, 2.3
    parts["chimney"] = VGroup(
        Rectangle(width=0.14, height=flue_top - flue_bottom, **_SLAB).move_to(P(-2.85, (flue_top + flue_bottom) / 2)),
        Rectangle(width=0.24, height=0.05, **_SLAB).move_to(P(-2.85, flue_top + 0.025)),
    )'''
assert a in s
s=s.replace(a,b)
s=s.replace('tank_top = tank["cap"].get_top()', 'tank_top = tank["cap"].get_top()\n    boiler_out, tank_in = parts["boiler"].get_left(), tank["frame"].get_right()\n    plant_link = [boiler_out, np.array([tank_in[0], boiler_out[1], 0.0]), tank_in]')
s=s.replace('Line(P(-3.65, -1.2), P(TANK_X + 0.25, -1.2), color=PIPE, stroke_width=4)', 'VMobject(color=PIPE, stroke_width=4).set_points_as_corners(plant_link)')
s=s.replace('VMobject().set_points_as_corners([P(-3.65, -1.2), P(TANK_X + 0.25, -1.2)])', 'VMobject().set_points_as_corners(plant_link)')
s=s.replace('_label("Wärmeerzeuger", size=15, color=PASTEL_ORANGE).move_to(parts["boiler"])', '_label("Wärmeerzeuger", size=15, color=PASTEL_ORANGE).next_to(parts["boiler"], UP, buff=0.12)')
s=s.replace('np.array([-2.35, -0.85 + _HOUSE_DY, 0.0]), np.array([-2.35, 1.0 + _HOUSE_DY, 0.0]),\n                             np.array([-2.35, 2.3 + _HOUSE_DY, 0.0]), np.array([-2.15, 2.6 + _HOUSE_DY, 0.0])', 'h["boiler"].get_top(), np.array([-2.85, 1.0 + _HOUSE_DY, 0.0]),\n                             np.array([-2.85, 2.3 + _HOUSE_DY, 0.0]), np.array([-2.65, 2.6 + _HOUSE_DY, 0.0])')
p.write_text(s,encoding="utf-8")
