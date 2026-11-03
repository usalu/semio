import re
import time
from pathlib import Path

BASE = "🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/"


def write(path, text):
    for _ in range(40):
        try:
            with open(str(path), "w", encoding="utf8", newline="") as handle:
                handle.write(text)
            return
        except OSError:
            time.sleep(0.5)
    raise OSError(str(path))


def load(series):
    path = Path(BASE + series + "/🐍️.py")
    return path, open(str(path), encoding="utf8", newline="").read()


def swap(text, old, new):
    assert text.count(old) == 1, old[:70]
    return text.replace(old, new)


def between(text, first, last, new):
    start = text.index(first)
    end = text.index(last, start)
    return text[:start] + new + text[end:]


IMPORT = "staffage_height = _manim_module.staffage_height\n"
FITTED = IMPORT + "fitted_staffage = _manim_module.fitted_staffage\n"

path, text = load("⚙️mechanical")
text = between(text, "def _fan(pos, color, radius=0.24):", "def _streams(", '''def _fan(pos, radius=0.24):
    """🌀 Handcrafted duct fan in thermal view, fitted into a square of ``2 * radius``; it stands still until ``_spin`` starts it."""
    return fitted_staffage("ventilation-mechanical-duct-fan", width=2 * radius, height=2 * radius, rate=0.0).move_to(pos)


def _spin(fans, rt: float):
    """🔄 Start the rotor of every ``_fan`` and hold for ``rt`` seconds."""
    for fan in fans:
        fan.rate = 1.0
    return [Wait(rt)]


''')
NEWLINE = chr(10)
text = NEWLINE.join(re.sub(r", P_(?:ORANGE|CYAN|WHITE)", "", line, count=1) if "_fan(" in line and "def " not in line else line for line in text.split(NEWLINE))
text = swap(text, "_spin([fan3], rt / n, sign=-1.0 if out else 1.0)[0]", "_spin([fan3], rt / n)[0]")
text = swap(text, IMPORT, FITTED)

write(path, text)

path, text = load("❓️need")
text = between(text, "def _fan(pos, color, radius: float = 0.24):", "def _streams(", "")
text = swap(text, "return _fan(ORIGIN, color, radius=0.4)", 'return fitted_staffage("ventilation-need-mvhr", width=1.0, height=1.0)')
text = swap(text, IMPORT, FITTED)
write(path, text)

path, text = load("📦️volume")
text = between(text, "def _fan(pos, color, radius: float = 0.18):", "def _streams(", "")
text = swap(text, '''        unit = Rectangle(width=0.5, height=0.24, color=P_GREEN, stroke_width=2).move_to([-0.2, 0.36, 0])''',
            '''        unit = fitted_staffage("ventilation-volume-mvhr", width=0.5, height=0.24).move_to([-0.2, 0.36, 0])''')
text = swap(text, "unit, _fan(unit.get_center(), P_GREEN, radius=0.08), _use_cast(", "unit, _use_cast(")
text = swap(text, IMPORT, FITTED)
write(path, text)
print("ok")
