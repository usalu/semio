import re
p='🐍️.py'
s=open(p,encoding='utf-8',newline='').read()
crlf='\r\n' in s
s=s.replace('\r\n','\n')
def rep(old,new,count=1):
    global s
    assert s.count(old)>=1,old[:60]
    s=s.replace(old,new) if count==0 else s.replace(old,new,count)

rep("person_glyph = _manim_module.person_glyph\nseated_person_glyph = _manim_module.seated_person_glyph\nlamp_glyph = _manim_module.lamp_glyph\n",
    "staffage = _manim_module.staffage\nstaffage_motion = _manim_module.staffage_motion\n")
rep("Every stage is drawn with the Physical Fundamentals glyphs (house and room\nsections, person, lamp, moon, thermometer)","Every stage is drawn with the Physical Fundamentals sections and moon and thermometer\nglyphs plus handcrafted thermal staffage (person, laptop, monitor, lamp)")

a=s.index("def _laptop(hinge")
b=s.index("def _plume(")
s=s[:a]+'''def _spot(group, fx: float, fy: float):
    """📍 Point at fractions of a figure's bounding box — where its heat leaves."""
    return np.array([group.get_left()[0] + fx * group.width, group.get_bottom()[1] + fy * group.height, 0.0])


def _worker(anchor, unit: float):
    """🧑‍💻 Handcrafted seated office worker typing, in the animated thermal view, with the points heat leaves from."""
    group = staffage_motion("heating-internal-gains-office-worker", unit=unit, anchor=anchor)
    return {"group": group, "chest": _spot(group, 0.45, 0.62), "head": group.get_top(), "lap": _spot(group, 0.5, 0.38)}


def _on_surface(cast: str, x: float, top: float, unit: float):
    """💻 Thermal appliance standing on a surface at height ``top`` with its centre over ``x``."""
    return staffage(cast, unit=unit, anchor=np.array([x, top, 0.0]))


'''+s[b:]

# Beat1 gains
a=s.index("def _house_gains(house):")
b=s.index("def _house_heat(")
s=s[:a]+'''def _house_gains(house):
    """💡 Person, desk laptop and pendant lamp in the house — handcrafted thermal staffage, labelled."""
    floor_y, mid_y = house["bottom_left"][1], house["level_1"].get_center()[1]
    unit = house["h"] / 2 / 2.5
    person = staffage_motion("heating-internal-gains-standing-adult", unit=unit,
                             anchor=np.array([-1.3, floor_y + 0.02, 0.0]))
    deck_top = mid_y + house["level_1"].height / 2
    desk = staffage("heating-internal-gains-desk", unit=unit, anchor=np.array([0.05, deck_top, 0.0]), color=PASTEL_WHITE)
    laptop = _on_surface("heating-internal-gains-laptop", 0.05, desk.get_top()[1], unit)
    lamp = staffage("heating-internal-gains-pendant-lamp", unit=unit, anchor=np.array([1.0, mid_y, 0.0]))
    lamp.shift(UP * (mid_y - lamp.get_top()[1]))
    bulb = _spot(lamp, 0.5, 0.15)
    rays = _light_rays(bulb, floor_y + 0.02, (-0.5, -0.17, 0.17, 0.5))
    labels = VGroup(
        Text("Personen", font_size=LABEL_FONT_SIZE, color=COLOR_PEOPLE).next_to(person, RIGHT, buff=0.45),
        Text("Geräte", font_size=LABEL_FONT_SIZE, color=COLOR_EQUIP).next_to(desk, RIGHT, buff=0.3).set_y(desk.get_top()[1] + 0.2),
        Text("Licht", font_size=LABEL_FONT_SIZE, color=COLOR_LIGHT).next_to(lamp, RIGHT, buff=0.22),
    )
    return {
        "sources": VGroup(person, VGroup(desk, laptop), lamp), "labels": labels, "rays": rays,
        "warm": [_spot(person, 0.5, 0.6), laptop.get_center()], "bulb": bulb,
    }


'''+s[b:]
rep('self.play(gains["lamp"]["bulb"].animate.set_fill(COLOR_LIGHT, opacity=0.8),\n                  LaggedStart(*[Create(r) for r in gains["rays"]], lag_ratio=0.1),',
    'self.play(LaggedStart(*[Create(r) for r in gains["rays"]], lag_ratio=0.1),')

# Beat2
rep('''        seated = seated_person_glyph(np.array([room["x_l"] + 1.9, room["y_f"], 0.0]), color=COLOR_PEOPLE, scale=0.52 * room["h"] / 0.55,
                                     chair_color=PASTEL_WHITE)
        chest = seated["chest"]
        plume = _plume(seated["head"].get_top(), room["y_c"] - 0.08)''','''        seated = _worker(np.array([room["x_l"] + 1.9, room["y_f"], 0.0]), room["h"] / 2.6)
        chest = seated["chest"]
        plume = _plume(seated["head"], room["y_c"] - 0.08)''')

s=s.replace('\n',('\r\n' if crlf else '\n'))
open(p,'w',encoding='utf-8',newline='').write(s)
