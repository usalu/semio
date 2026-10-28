from pathlib import Path
import ast

root = Path("🎓️teaching/🏛️architecture/⚡️energy")

def replace(path, old, new):
    source = path.read_text(encoding="utf-8")
    if old not in source:
        raise RuntimeError(f"Missing anchor: {path}: {old[:90]}")
    path.write_text(source.replace(old, new), encoding="utf-8")

conduction = root / "🔥️heating/🎬️clip/🧱️conduction/🐍️.py"
replace(conduction, 'br, tr, t = h["bottom_right"], h["top_right"], 0.08 * scale', 'br, tr, t = h["bottom_right"], h["top_right"], h["wall"]')
replace(conduction, '''h["wall_right"].put_start_and_end_on(np.array([br[0], door_top, 0.0]), tr)
    h["walls"][5][3].put_start_and_end_on(np.array([br[0] - t, door_top, 0.0]), tr + LEFT * t)
    door = window_glyph(br[0], br[1], door_top, depth=0.5 * scale, color=col["Haustür"])
    door["sash"].remove(door["sash"][1])''', '''h["wall_right"].become(Rectangle(width=t, height=tr[1] - door_top, color=col["Außenwand"],
                                    stroke_width=1.3, fill_color=col["Außenwand"], fill_opacity=0.7
                                    ).move_to(np.array([br[0] - t / 2, (door_top + tr[1]) / 2, 0.0])))
    door = window_glyph(br[0] - t / 2, br[1], door_top, depth=0.5 * scale,
                        color=col["Haustür"], wall=t, inward=-1)
    door["sash"][1].set_opacity(0)''')
replace(conduction, 'VGroup(*h["walls"][:4], h["walls"][5])', 'VGroup(h["walls"][0], h["wall_right"])')
replace(conduction, 'h["roof"].set_stroke(col["Dach"])', 'h["roof"].set_color(col["Dach"])')
replace(conduction, '.set_stroke(col["Außenwand"])', '.set_color(col["Außenwand"])')
replace(conduction, 'hs["roof"][0].get_start(), hs["roof"][0].get_end()', 'hs["roof"][0].get_vertices()[0], hs["roof"][0].get_vertices()[2]')
replace(conduction, 'heater = radiator(np.array([br[0] - 0.55, bl[1] + 0.24, 0.0]), PASTEL_RED).scale(0.55)', 'heater = radiator(ORIGIN, PASTEL_RED, height=0.26 * hs["w_height"] / 2, width=0.52 * hs["w_height"] / 2)\n        heater.move_to(np.array([br[0] - 0.55, bl[1] + 0.048 * hs["w_height"] / 2 + heater.height / 2, 0.0]))')
regulations = root / "📊️demand/🎬️clip/⚖️regulations/🐍️.py"
source=regulations.read_text(encoding="utf-8")
lines=source.splitlines(keepends=True)
node=next(n for n in ast.parse(source).body if isinstance(n,ast.FunctionDef) and n.name=="_north_window")
lines[node.lineno-1:node.end_lineno]=['''def _north_window(hs):
    """🧭 Cut a north-facing aperture through the solid exterior wall."""
    br, tr, t = hs["bottom_right"], hs["top_right"], hs["wall"]
    sill, lintel = hs["windows"][1]["sill"], hs["windows"][1]["lintel"]
    style = dict(color=P_WHITE, stroke_width=1.3, fill_color=P_WHITE, fill_opacity=0.7)
    pieces = VGroup(*[Rectangle(width=t, height=b - a, **style).move_to(
        np.array([br[0] - t / 2, (a + b) / 2, 0.0])) for a, b in ((br[1], sill), (lintel, tr[1]))])
    hs["wall_right"].become(pieces)
    glass = window_glyph(float(br[0] - t / 2), sill, lintel, depth=0.42 * HOUSE_S, wall=t, inward=-1)
    hs["window_group"].add(glass["group"])
    return glass
''']
regulations.write_text("".join(lines),encoding="utf-8")
recap=root/"📊️demand/🎬️clip/🔁️recap/🐍️.py"
replace(recap, 'for i in (0, 1, 2, 3)', 'for i in (0, 1)')
replace(recap, 'return mob.copy().set_color(color).set_stroke(width=6.5, opacity=0.95)', 'return mob.copy().set_color(color).set_fill(opacity=0.85).set_stroke(width=1.5, opacity=0.95)')
humidity=root/"❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py"
replace(humidity, 'VGroup(*h["walls"][:4], h["roof"][0])', 'VGroup(h["walls"][0], h["wall_right"], h["roof"][0])')
fundamentals=root/"🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py"
replace(fundamentals, 'ROOM_X0, WALL_X0, WALL_X1, OUT_X1 = -6.3, -0.7, 0.7, 6.3', 'ROOM_X0, WALL_X0, WALL_X1, OUT_X1 = -6.3, -0.21, 0.21, 6.3')
replace(fundamentals, 'PLASTER = 0.10', 'PLASTER = 0.018')
replace(fundamentals, 'RADIATOR_X, RADIATOR_SCALE = -4.6, 1.1\nRADIATOR = (RADIATOR_X - 0.51, FLOOR_Y, RADIATOR_X + 0.51, FLOOR_Y + 0.63)', 'RADIATOR_X = -4.6\n_ROOM_METRE = (CEIL_Y - FLOOR_Y) / 2.5\nRADIATOR = (RADIATOR_X - 0.65 * _ROOM_METRE, FLOOR_Y + 0.12 * _ROOM_METRE,\n            RADIATOR_X + 0.65 * _ROOM_METRE, FLOOR_Y + 0.77 * _ROOM_METRE)')
replace(fundamentals, 'OCCUPANT_X, OCCUPANT_SCALE = -5.72, 2.2', 'OCCUPANT_X, OCCUPANT_SCALE = -5.72, 1.75 * _ROOM_METRE / 0.66')
replace(fundamentals, 'rad = radiator(ORIGIN, PASTEL_RED).scale(RADIATOR_SCALE)\n    return rad.move_to(np.array([RADIATOR_X, FLOOR_Y + rad.height / 2, 0.0]))', 'rad = radiator(ORIGIN, PASTEL_RED, height=RADIATOR[3] - RADIATOR[1], width=RADIATOR[2] - RADIATOR[0])\n    return rad.move_to(np.array([RADIATOR_X, (RADIATOR[1] + RADIATOR[3]) / 2, 0.0]))')
