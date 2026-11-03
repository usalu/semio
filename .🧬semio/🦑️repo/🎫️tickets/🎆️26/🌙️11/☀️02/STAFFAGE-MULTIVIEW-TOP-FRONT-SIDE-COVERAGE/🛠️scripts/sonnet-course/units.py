from pathlib import Path

A = Path("C:/git/semio/🎓️teaching/🏛️architecture")
MAN = '_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")\n'


def edit(path, pairs, constants="", bindings=()):
    p = A / path
    s = p.read_bytes().decode("utf-8")
    for old, new in pairs:
        assert old in s, (path, old)
        s = s.replace(old, new)
    add = "".join(b + "\n" for b in bindings if b not in s) + constants
    if add:
        assert MAN in s, path
        s = s.replace(MAN, MAN + add, 1)
    p.write_bytes(s.encode("utf-8"))


edit("⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py", [
    ("unit=0.68, at=(0.0, -0.95)", "unit=SINGLE_METRE, at=(0.0, -0.95)"),
    ('"laptop", unit=1.5, at=(panel_cx', '"laptop", unit=LAPTOP_GRID_METRE, at=(panel_cx'),
    ("unit=0.47, at=(-2.4", "unit=ROW_METRE, at=(-2.4"),
    ("unit=0.19, at=(x_pos", "unit=GRID_METRE, at=(x_pos"),
    ('"laptop-closed", unit=1.5', '"laptop-closed", unit=LAPTOP_GRID_METRE'),
    ('_figure("standing", unit=0.6, at=(0.0, 0.0))', '_figure("standing", unit=CARD_METRE, at=(0.0, 0.0))'),
    ('_figure("laptop", unit=0.6, at=(0.0, 0.0), moving=False)', '_figure("laptop", unit=CARD_METRE, at=(0.0, 0.0), moving=False)'),
    ('_figure("lamp", unit=1.1, at=(0.0, 0.0), moving=False)', '_figure("lamp", unit=CARD_LAMP_METRE, at=(0.0, 0.0), moving=False)'),
], "SINGLE_METRE, ROW_METRE, GRID_METRE, LAPTOP_GRID_METRE, CARD_METRE, CARD_LAMP_METRE = 0.68, 0.47, 0.19, 1.5, 0.6, 1.1\n")

edit("⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py", [
    ("unit=width / 0.595", 'unit=width / staffage_width("cooling-system-sizing-diffuser")'),
], bindings=("staffage_width = _manim_module.staffage_width",))

edit("⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py", [
    ('"ventilation-volume-blackboard", unit=0.25', '"ventilation-volume-blackboard", unit=_USE_UNIT'),
    ('"ventilation-volume-hood-mirrored", unit=0.31', '"ventilation-volume-hood-mirrored", unit=_USE_UNIT'),
    ('"ventilation-volume-operating-table", unit=0.354', '"ventilation-volume-operating-table", unit=_USE_UNIT'),
])

edit("⚡️energy/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py", [
    ('"shared-lamp-winter", unit=0.9, anchor=feet', '"shared-lamp-winter", unit=B5_ICON_METRE, anchor=feet'),
    ('"shared-person-winter", unit=0.8, anchor=feet', '"shared-person-winter", unit=B5_ICON_METRE, anchor=feet'),
    ('"shared-laptop-winter", unit=0.8, anchor=feet', '"shared-laptop-winter", unit=B5_ICON_METRE, anchor=feet'),
], "B5_ICON_METRE = 0.8\n")

edit("⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py", [
    ('unit=0.7 * hs["w_height"] / 2 / 1.75', 'unit=room_metre(hs["w_height"] / 2)'),
], bindings=("room_metre = _manim_module.room_metre",))

edit("⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py", [
    ('_staffage_motion("physics-fundamentals-vacuum", unit=1.0, rate=VACUUM_RATE)', '_staffage_motion("physics-fundamentals-vacuum", unit=width / _staffage_width("physics-fundamentals-vacuum"), rate=VACUUM_RATE)'),
    ("    return group.scale(width / group.width)\n\n\ndef _shower_head", "    return group\n\n\ndef _shower_head"),
    ('_staffage("shared-radiator-front-winter", unit=1.2, anchor=np.array([5.5, 0.0, 0.0]))', '_staffage("shared-radiator-front-winter", unit=FE_RADIATOR_METRE, anchor=np.array([5.5, 0.0, 0.0]))'),
], "FE_RADIATOR_METRE = 1.2\n_staffage_width = _manim_module.staffage_width\n")

edit("🎯️optimization/🎬️clip/🧪️experiments/🐍️.py", [
    ('"optimization-experiments-occupant", unit=.5,', '"optimization-experiments-occupant", unit=ICON_METRE,'),
    ('"optimization-experiments-window", unit=.5,', '"optimization-experiments-window", unit=ICON_METRE,'),
])
print("ok")
