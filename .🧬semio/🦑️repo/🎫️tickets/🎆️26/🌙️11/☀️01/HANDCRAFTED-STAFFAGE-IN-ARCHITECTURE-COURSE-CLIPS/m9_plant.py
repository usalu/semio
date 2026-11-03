from pathlib import Path

import time


def write(path, text):
    for attempt in range(40):
        try:
            with open(str(path), "w", encoding="utf8", newline="") as handle:
                handle.write(text)
            return
        except OSError:
            time.sleep(0.5)
    raise OSError(str(path))


ENERGY = "🎓️teaching/🏛️architecture/⚡️energy/"


def edit(relative, changes):
    path = Path(ENERGY + relative)
    text = open(str(path), encoding="utf8", newline="").read()
    for old, new in changes:
        assert text.count(old) == 1, (relative, old[:60])
        text = text.replace(old, new)
    write(path, text)


def cut(text_start, text_end, text):
    start = text.index(text_start)
    end = text.index(text_end, start)
    return text[:start], text[end:]


# ---- system-losses: boiler and tank ----
path = Path(ENERGY + "📊️demand/🎬️clip/♨️system-losses/🐍️.py")
text = open(str(path), encoding="utf8", newline="").read()
head, tail = cut("def _flame(center, color=P_ORANGE):", "def _heat_pump(", text)
text = head + '''def _boiler(center, w: float = 0.80, h: float = 1.0):
    """♨️ Gas boiler in elevation, fitted into the ``w`` × ``h`` box around ``center``; its heating cycle plays through scene time."""
    return fitted_staffage("demand-system-losses-boiler", width=w, height=h, rate=BOILER_RATE).move_to(np.array(center, dtype=float))


''' + tail
head, tail = cut("def _tank(center", "def _chimney(", text)
text = head + '''def _tank(center, w: float = 0.80, h: float = 1.0, level: float = 1.0):
    """🛢️ Hot-water cylinder whose charge is the heat still stored — ``level`` is a 0–1 tracker that seeks the charge-up clip."""
    tracker = ValueTracker(level)
    group = fitted_staffage("demand-system-losses-tank", width=w, height=h, rate=0.0).move_to(np.array(center, dtype=float))
    group.add_updater(lambda m: m.seek(float(np.clip(tracker.get_value(), 0.0, 1.0)) * TANK_CHARGE_S))
    return {"group": group, "fill": VGroup(), "level": tracker}


''' + tail
for old, new in (
    ("boiler = _boiler(np.array([BOILER_X, PIPE_Y - 0.14, 0.0]))", "boiler = _boiler(np.array([BOILER_X, PIPE_Y - 0.14, 0.0]))"),
    ("boiler = _boiler(at(-3.2, Y), w=0.9, h=1.2)", "boiler = _boiler(at(-3.2, Y), w=0.9, h=1.2)"),
):
    assert text.count(old) == 1, old
assert text.count("_flame(") == 0, "flame still used"
text = text.replace("AMBIENT_GREEN = semio_color(\"success\")\n", "AMBIENT_GREEN = semio_color(\"success\")\nBOILER_RATE = 60.0\nTANK_CHARGE_S = 900.0\n", 1)
write(path, text)

# ---- primary-energy: boiler and pump ----
path = Path(ENERGY + "📊️demand/🎬️clip/🏭️primary-energy/🐍️.py")
text = open(str(path), encoding="utf8", newline="").read()
head, tail = cut("def _pump(center, r=0.17):", "def _tree(", text)
head2, tail2 = cut("def _flame(center, h=0.5", "def _tree(", text)
start = text.index("def _pump(center, r=0.17):")
end = text.index("def _flame(center, h=0.5")
text = text[:start] + '''def _pump(center):
    """🌀 Circulation pump in front view, fitted into a 0.36 × 0.3 box; its rotor turns with scene time."""
    return fitted_staffage("demand-primary-energy-pump", width=0.36, height=0.3, rate=1.0).move_to(_pt(center))


''' + text[end:]
start = text.index("def _boiler(center):")
end = text.index("def _heat_pump(center):")
text = text[:start] + '''def _boiler(center):
    """🔥 Gas boiler in elevation, fitted into a 0.5 × 0.62 box; its heating cycle plays through scene time."""
    return fitted_staffage("demand-primary-energy-boiler", width=0.5, height=0.62, rate=BOILER_RATE).move_to(_pt(center))


''' + text[end:]
print("flame uses", text.count("_flame("), "outline-circle", text.count("Circle("))
text = text.replace("fitted_staffage = _manim_module.fitted_staffage\n", "fitted_staffage = _manim_module.fitted_staffage\nBOILER_RATE = 60.0\n", 1)
write(path, text)

# ---- practice: boiler ----
path = Path(ENERGY + "📊️demand/🎬️clip/🧪️practice/🐍️.py")
text = open(str(path), encoding="utf8", newline="").read()
start = text.index("def _boiler(center):")
end = text.index("def _tile(")
text = text[:start] + '''def _boiler(center):
    """🔥 Wall boiler in elevation, fitted into a 0.42 × 0.64 box — where the gas becomes heat in the house."""
    return fitted_staffage("demand-practice-boiler", width=0.42, height=0.64, rate=BOILER_RATE).move_to(np.array(center, dtype=float))


''' + text[end:]
text = text.replace("staffage_height = _manim_module.staffage_height\n", "staffage_height = _manim_module.staffage_height\nfitted_staffage = _manim_module.fitted_staffage\nBOILER_RATE = 60.0\n", 1)
write(path, text)
print("ok")
