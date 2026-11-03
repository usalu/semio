from pathlib import Path

p = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py")
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)


sub("radiator = _manim_module.radiator\nperson_glyph = _manim_module.person_glyph\n",
    "staffage = _manim_module.staffage\nstaffage_motion = _manim_module.staffage_motion\nstaffage_height = _manim_module.staffage_height\n")
sub("lamp_glyph = _manim_module.lamp_glyph\n", "pendant_lamp = _manim_module.pendant_lamp\n")

a = s.index("def _stand(floor_y, room_h, x, color=P_ORANGE):")
b = s.index("def _stream(")
s = s[:a] + '''def _stand(floor_y, room_h, x):
    """🧍 Breathing thermal occupant at true scale — 70 % of the clear room height, feet on the floor line at ``x``."""
    cast = "demand-practice-person"
    return staffage_motion(cast, unit=0.7 * room_h / staffage_height(cast), anchor=np.array([x, floor_y, 0.0]))


def _talker(floor_y, room_h, x, color):
    """🗣️ Handcrafted standing person in a clip colour — no heat meaning, so a line drawing rather than the thermal view."""
    cast = "demand-practice-talker"
    return staffage(cast, unit=0.7 * room_h / staffage_height(cast), anchor=np.array([x, floor_y, 0.0]), color=color)


def _heater(floor_y, room_h, x):
    """♨️ Thermal panel radiator at its physical size on the floor line of a 2.5 m room."""
    return staffage("demand-practice-radiator", unit=room_h / 2.5, anchor=np.array([x, floor_y, 0.0]))


''' + s[b:]

sub('''        lamp = lamp_glyph(anchor, drop=0.25)
        lamp["group"].scale(1.5, about_point=anchor)
''', '''        lamp = pendant_lamp(anchor, 0.9, on_cast="demand-practice-lamp-on", off_cast="demand-practice-lamp-off")
''')
sub('''        self.play(Create(lamp["group"]), Create(gauge["group"])''', '''        self.play(FadeIn(lamp["group"]), Create(gauge["group"])''')
sub('''lamp["bulb"].animate.set_fill(P_YELLOW, opacity=0.9), *glow(1.6),''', '''lamp["on"].animate.set_opacity(1), lamp["off"].animate.set_opacity(0), *glow(1.6),''')
sub("glow = lambda rt: [ripples([lamp[\"bulb\"].get_center()]", "glow = lambda rt: [ripples([lamp[\"bulb\"].get_center()]")
sub('landlord = _stand(ho_floor, ho_room_h, 2.70)', 'landlord = _talker(ho_floor, ho_room_h, 2.70, P_ORANGE)')
sub('tenant = _stand(ho_floor, ho_room_h, 3.50, color=P_CYAN)', 'tenant = _talker(ho_floor, ho_room_h, 3.50, P_CYAN)')
p.write_text(s, encoding="utf-8", newline="")

m = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json")
t = m.read_text(encoding="utf-8")
anchor = '    "demand-certificate-radiator"'
add = '''    "demand-practice-person": { "asset": "standing-adult", "clip": "idle-breathing", "view": "thermal", "environment": "indoor-heated-winter", "theme": "dark" },
    "demand-practice-talker": { "asset": "standing-adult", "pose": "relaxed", "view": "dark-line", "theme": "dark" },
    "demand-practice-radiator": { "asset": "panel-radiator", "pose": "rated", "view": "thermal", "environment": "indoor-heated-winter", "theme": "dark" },
    "demand-practice-lamp-on": { "asset": "pendant-lamp", "pose": "on", "view": "thermal", "environment": "indoor-heated-winter", "theme": "dark" },
    "demand-practice-lamp-off": { "asset": "pendant-lamp", "pose": "off", "view": "thermal", "environment": "indoor-heated-winter", "theme": "dark" },
'''
assert t.count(anchor) == 1 and "demand-practice-person" not in t
t = t.replace(anchor, add + anchor)
m.write_text(t, encoding="utf-8", newline="")
