from pathlib import Path

p = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py")
s = p.read_text(encoding="utf-8")
s = s.replace('_SILHOUETTES = json.loads((Path(__file__).parent / "🫟️silhouettes/🔣️.json").read_text(encoding="utf-8"))["outlines"]\n', "")
a = s.index("def object_silhouette(")
b = s.index("def icon_glyph(")
s = s[:a] + s[b:]
a = s.index("def person_glyph(")
b = s.index("def cloud_glyph(")
s = s[:a] + s[b:]
a = s.index("def lamp_glyph(")
b = s.index("def droplets(")
new = '''def pendant_lamp(top, height: float, *, on_cast: str, off_cast: str):
    """💡 Handcrafted thermal pendant lamp hanging ``height`` long from ``top`` in an ``off`` and an ``on`` cast.

    ``group`` holds both casts: ``off`` is shown and ``on`` is transparent. Fade ``on`` in with ``set_opacity`` to switch the lamp on.
    ``bulb`` is a point on the bulb for heat ripples.

    @see ./🧍️staffage/🐍️.py — the cast API this wraps
    """
    from manim import UP, Point, VGroup

    unit = height / staffage_height(off_cast)
    off, on = staffage(off_cast, unit=unit), staffage(on_cast, unit=unit)
    on.set_opacity(0)
    group = VGroup(off, on)
    group.shift(np.asarray(top, dtype=float) - group.get_top())
    return {"group": group, "off": off, "on": on, "bulb": Point(off.get_bottom() + UP * 0.2 * height)}


'''
s = s[:a] + new + s[b:]
p.write_text(s, encoding="utf-8", newline="")
