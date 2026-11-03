import time
from pathlib import Path

ENERGY = "🎓️teaching/🏛️architecture/⚡️energy/"


def write(path, text):
    for _ in range(60):
        try:
            with open(str(path), "w", encoding="utf8", newline="") as handle:
                handle.write(text)
            return
        except OSError:
            time.sleep(0.5)
    raise OSError(str(path))


def edit(relative, steps):
    path = Path(ENERGY + relative)
    text = open(str(path), encoding="utf8", newline="").read()
    for step in steps:
        if step[0] == "swap":
            assert text.count(step[1]) == 1, step[1][:70]
            text = text.replace(step[1], step[2])
        else:
            start = text.index(step[1])
            end = text.index(step[2], start)
            text = text[:start] + step[3] + text[end:]
    write(path, text)


edit("🔥️heating/🎬️clip/🧱️conduction/🐍️.py", [
    ("swap", "staffage_height = _manim_module.staffage_height\n",
     "staffage_height = _manim_module.staffage_height\nfitted_staffage = _manim_module.fitted_staffage\n"),
    ("cut", "def _tealight(color: str = PASTEL_ORANGE):", "def _devices(", '''def _tealight():
    """🕯️ Handcrafted tealight in the animated thermal view — the flame flickers while the cup warms."""
    return fitted_staffage("heating-conduction-tealight", width=0.6, height=0.62, rate=1.0, loop=True)


'''),
    ("swap", "_tealight(PASTEL_ORANGE) for _ in range(count)", "_tealight() for _ in range(count)"),
    ("swap", "return [t[2].get_top() for t in row]", "return [t.get_top() for t in row]"),
])

edit("🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py", [
    ("cut", "def _shower_head(anchor, color=P_WHITE):", "def _thermometer(", '''def _shower_head(anchor):
    """🚿 Handcrafted shower rail and head in the animated thermal view — the head warms as the water runs; ``anchor`` is the spot the water leaves."""
    return _staffage_motion("physics-fundamentals-shower", unit=SHOWER_UNIT, rate=SHOWER_RATE,
                            anchor=np.array(anchor, dtype=float) + np.array([-0.15, -0.6, 0.0]))


'''),
    ("swap", "_staffage_height = _manim_module.staffage_height\n",
     "_staffage_height = _manim_module.staffage_height\nSHOWER_UNIT = 0.74\nSHOWER_RATE = 40.0\n"),
])
print("ok")
