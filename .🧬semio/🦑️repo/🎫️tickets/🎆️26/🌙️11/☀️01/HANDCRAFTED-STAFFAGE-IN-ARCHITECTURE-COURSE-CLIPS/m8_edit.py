import re
from pathlib import Path
p = Path("🐍️.py")
s = p.read_text(encoding="utf-8")

s = s.replace('''_SILHOUETTES = json.loads((Path(__file__).parent / "🫟️silhouettes/🔣️.json").read_text(encoding="utf-8"))["outlines"]
''', '''_SILHOUETTES = json.loads((Path(__file__).parent / "🫟️silhouettes/🔣️.json").read_text(encoding="utf-8"))["outlines"]
_ICONS = json.loads((Path(__file__).parent / "🔣️icons/🔣️.json").read_text(encoding="utf-8"))["outlines"]
''')

a = s.index("#region Watt anchors")
b = s.index("def watt_anchor")
s = s[:a] + '''#region Watt anchors
_WATT_DEVICES: dict[str, tuple[str, int]] = {
    "laptop": ("Laptop", 60),
    "monitor": ("Monitor", 30),
    "bulb": ("Glühbirne", 100),
    "toaster": ("Toaster", 1000),
    "vacuum": ("Staubsauger", 1400),
    "heater": ("Heizgerät", 2000),
}
_WATT_CASTS: dict[str, str] = {
    "laptop": "shared-watt-laptop",
    "monitor": "shared-watt-monitor",
    "bulb": "shared-watt-bulb",
    "toaster": "shared-watt-toaster",
    "vacuum": "shared-watt-vacuum",
    "heater": "shared-watt-heater",
}


def fitted_staffage(cast: str, *, width: float | None = None, height: float | None = None, color: str | None = None):
    """📐 Draw a staffage cast at the largest uniform scale that fits ``width`` and ``height`` Manim units, feet on the origin.

    @see ../🧍️staffage/🐍️.py — the cast API this wraps
    """
    probe = staffage(cast, unit=1.0)
    limits = [limit / extent for limit, extent in ((width, probe.width), (height, probe.height)) if limit is not None]
    return staffage(cast, unit=min(limits), color=color)


''' + s[b:]

s = s.replace('''    glyph = object_silhouette(compare if compare in _WATT_DEVICES else "laptop", color=color, width=0.65)''',
'''    glyph = fitted_staffage(_WATT_CASTS[compare if compare in _WATT_DEVICES else "laptop"], width=0.65, height=0.65, color=color)''')

a = s.index("def radiator(")
b = s.index("#endregion", a)
s = s[:a] + '''def radiator(center, color=P_RED, *, height: float | None = None, width: float | None = None):
    """♨️ Handcrafted panel radiator fitted into the nominal ``width`` × ``height`` box, centred on x with its feet at the box bottom.

    @see ../../../../🧍️staffage/🗂️catalog/🧱️components/panel-radiator/🔣️.json — the radiator asset
    """
    height = _ARCHITECTURE_DIMENSIONS["radiator_height"] if height is None else height
    width = _ARCHITECTURE_DIMENSIONS["radiator_width"] if width is None else width
    result = fitted_staffage("shared-radiator", width=width, height=height, color=color)
    centre = np.asarray(center, dtype=float)
    result.shift(np.array([centre[0] - result.get_center()[0], centre[1] - height / 2 - result.get_bottom()[1], 0.0]))
    result.architectural_height = height
    return result
''' + s[b:]

s = s.replace('''    return VGroup(object_silhouette("sun", color=color, width=1.4)).move_to(pos)''', '''    return VGroup(icon_glyph("sun", color=color, width=1.4)).move_to(pos)''')
s = s.replace('''    return VGroup(object_silhouette("moon", color=color, width=0.48)).move_to(pos)''', '''    return VGroup(icon_glyph("moon", color=color, width=0.48)).move_to(pos)''')
s = s.replace('''def person_glyph(pos''', '''def icon_glyph(kind: str, *, color=P_WHITE, width: float = 0.65):
    """🔣 Draw one closed diagram icon (sun, moon, flame, microbe or factory) as a single filled contour.

    @see ./🔣️icons/🔣️.json — the icon outlines
    """
    outline = _filled_outline(_ICONS[kind], color, 1)
    return outline.scale(width / outline.width).move_to([0, 0, 0])


def person_glyph(pos''', 1)
p.write_text(s, encoding="utf-8", newline="")
