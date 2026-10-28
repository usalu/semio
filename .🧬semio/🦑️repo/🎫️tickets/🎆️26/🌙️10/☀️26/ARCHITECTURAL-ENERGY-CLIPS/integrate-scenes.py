from pathlib import Path
import ast

root = Path("🎓️teaching/🏛️architecture/⚡️energy")

def replace(path, old, new):
    content = path.read_text(encoding="utf-8")
    if old not in content:
        raise RuntimeError(f"Missing edit anchor: {path}: {old[:80]}")
    path.write_text(content.replace(old, new), encoding="utf-8")

physics = root / "🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py"
source = physics.read_text(encoding="utf-8")
shared = {"_window": "window_glyph", "_open_window": "open_window", "_build_cross_section_house": "house_section", "_room": "room_section", "_radiator": "radiator", "_person": "person_glyph"}
lines = source.splitlines(keepends=True)
for node in reversed(ast.parse(source).body):
    if isinstance(node, ast.FunctionDef) and node.name in shared:
        lines[node.lineno - 1:node.end_lineno] = [f"{node.name} = _manim_module.{shared[node.name]}\n"]
physics.write_text("".join(lines), encoding="utf-8")
replace(physics, 'radiator = _radiator(np.array([room["x_l"] + 0.55, room["y_f"] + 0.3, 0.0])).scale(0.75)', 'radiator = _radiator(ORIGIN, height=0.26 * room["h"], width=0.52 * room["h"])\n        radiator.move_to(np.array([room["x_l"] + 0.55, room["y_f"] + 0.048 * room["h"] + radiator.height / 2, 0.0]))')
calculation = root / "🔥️heating/🎬️clip/🧮️calculation/🐍️.py"
replace(calculation, 'person = person_glyph(ORIGIN, INT_ORANGE, scale=0.85)', 'person = person_glyph(ORIGIN, INT_ORANGE).set_height(0.7 * int_room["h"])')
replace(calculation, 'person = person_glyph(ORIGIN, PASTEL_ORANGE, scale=0.9)', 'person = person_glyph(ORIGIN, PASTEL_ORANGE).set_height(0.7 * house["w_height"] / 2)')
replace(calculation, 'heater = radiator(np.array([0.82, floor_y + 0.2, 0.0]), QH_C).scale(0.45)', 'heater = radiator(ORIGIN, QH_C, height=0.26 * house["w_height"] / 2, width=0.52 * house["w_height"] / 2)\n        heater.set_x(0.82)')
replace(calculation, 'RAD_X, RAD_Y, PIPE_Y, TANK_X = (-6.12, -1.88), 0.04, -0.48, -4.9', 'room_h, wall = 1.6, 0.35 * 1.6 / 2.5\n    metre = room_h / 2.5\n    RAD_X, PIPE_Y, TANK_X = (-6.12, -1.88), -0.48, -4.9\n    RAD_Y = -0.15 + metre * (0.12 + 0.65 / 2)')
replace(calculation, 'Rectangle(width=4.95, height=1.6', 'Rectangle(width=5.3 - wall, height=room_h')
replace(calculation, 'Rectangle(width=4.95, height=1.3', 'Rectangle(width=5.3 - wall, height=1.3')
replace(calculation, 'slab(x - 0.15, x + 0.15,', 'slab(x - wall / 2, x + wall / 2,')
replace(calculation, '''parts["roof"] = VGroup(
        VMobject(color=PASTEL_WHITE, stroke_width=1.7).set_points_as_corners([P(-7.0, 1.45), P(-4.0, 2.4), P(-1.0, 1.45)]),
        VMobject(color=PASTEL_WHITE, stroke_width=1.3).set_points_as_corners([P(-6.5, 1.42), P(-4.0, 2.26), P(-1.5, 1.42)]),
    )''', '''parts["roof"] = Polygon(P(-7.0, 1.45), P(-4.0, 2.4), P(-1.0, 1.45),
                            P(-1.0, 1.32), P(-4.0, 2.24), P(-7.0, 1.32),
                            color=PASTEL_WHITE, stroke_width=1.2, fill_color=PASTEL_WHITE, fill_opacity=0.9)''')
replace(calculation, 'depth=0.3, color=PASTEL_CYAN)', 'depth=0.3, color=PASTEL_CYAN, wall=wall, side="l" if x < -4 else "r")')
replace(calculation, 'radiator(P(x, RAD_Y), PIPE).scale(0.42)', 'radiator(P(x, RAD_Y), PIPE, height=0.65 * metre, width=1.3 * metre)')
replace(calculation, '    return parts\n', '    parts["floor_y"] = -0.15 + _HOUSE_DY\n    parts["room_height"] = room_h\n    parts["person"] = person_glyph(P(-3.35, -0.15 + 1.75 * metre / 2), PASTEL_ORANGE, scale=1.75 * metre / 0.66)\n    return parts\n')
replace(calculation, 'person = person_glyph(ORIGIN, PASTEL_ORANGE, scale=0.85)\n        person.move_to(np.array([-3.35, floor_y + 0.01 + person.height / 2, 0.0]))', 'person = h["person"]')
power = root / "📊️demand/🎬️clip/⚡️demand-power/🐍️.py"
replace(power, 'rad_scale = 0.26 * storey / 0.5', 'metre = storey / 2.5')
replace(power, 'r = radiator(ORIGIN, color=POWER).scale(rad_scale)', 'r = radiator(ORIGIN, color=POWER, height=0.65 * metre, width=1.3 * metre)')
replace(power, 'fy + r.height / 2 + 0.03', 'fy + r.height / 2 + 0.12 * metre')
primary = root / "📊️demand/🎬️clip/🏭️primary-energy/🐍️.py"
replace(primary, 'heater.scale(0.75 * s)', 'heater.set_height(0.26 * hs["w_height"] / 2)')
comparison = root / "❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py"
replace(comparison, 'person = person_glyph(ORIGIN, scale=1.1)', 'person = person_glyph(ORIGIN).set_height(0.7 * house["w_height"] / 2)')
internal = root / "🔥️heating/🎬️clip/👥️internal-gains/🐍️.py"
replace(internal, 'person = person_glyph(ORIGIN, color=COLOR_PEOPLE, scale=1.6)', 'person = person_glyph(ORIGIN, color=COLOR_PEOPLE).set_height(0.7 * house["w_height"] / 2)')
