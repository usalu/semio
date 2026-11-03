from pathlib import Path
p = Path("🧱️conduction/🐍️.py")
s = p.read_text(encoding="utf-8")
a = s.index("def _person_breathing")
b = s.index("def _envelope_house")
new = '''TRUE_METRE = 0.95
ZOOM_RADIUS = 0.78
ZOOM = {"toaster": 3.0, "tealight": 14.0}


def _person_breathing(metre: float):
    """🫁 Standing adult in the animated thermal view at true scale — body heat shows while the clip breathes."""
    return staffage_motion("heating-conduction-person", unit=metre)


def _toaster(color: str = PASTEL_ORANGE, fill: float = 0.0, metre: float = TRUE_METRE):
    """🍞 Handcrafted toaster at true scale whose body is filled to ``fill`` — 0,7 means seventy percent of one toaster's watts."""
    body = staffage_motion("heating-conduction-toaster", unit=metre, rate=TOASTER_RATE)
    level = Rectangle(width=0.72 * body.width, height=max(0.01, 0.5 * fill * body.height), stroke_width=0,
                      fill_color=color, fill_opacity=0.4)
    level.move_to(body.get_bottom() + UP * (0.16 * body.height + level.height / 2))
    return VGroup(level, body)


def _tealight(metre: float = TRUE_METRE):
    """🕯️ Handcrafted tealight at true scale in the animated thermal view — the flame flickers while the cup warms."""
    light = staffage_motion("heating-conduction-tealight", unit=metre, rate=1.0)
    light.loops = True
    return light


def _zoom_inset(kind: str, fill: float, source_top, color: str):
    """🔍 Magnifier disc that shows one device of a true-scale row enlarged, tied to the real item by a leader line."""
    factor = ZOOM[kind]
    item = _toaster(color, fill, TRUE_METRE * factor) if kind == "toaster" else _tealight(TRUE_METRE * factor)
    disc = Circle(radius=ZOOM_RADIUS, stroke_color=PASTEL_WHITE, stroke_width=1.6, fill_color=PASTEL_WHITE, fill_opacity=0.06)
    disc.move_to(source_top + UP * (0.45 + ZOOM_RADIUS))
    item.move_to(disc.get_center() + DOWN * 0.04 * ZOOM_RADIUS, aligned_edge=ORIGIN)
    item.shift(disc.get_center() - item.get_center())
    leader = DashedLine(source_top + UP * 0.06, disc.get_bottom(), color=PASTEL_WHITE, stroke_width=1.4, dash_length=0.06)
    tag = body_text(f"{int(factor)}×", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
    tag.next_to(disc, RIGHT, buff=0.1, aligned_edge=UP)
    return VGroup(leader, disc, item, tag)


def _devices(kind: str, count: int, x: float, ground_y: float, fill: float = 1.0):
    """🔌 Row of everyday power references at one shared true scale standing on a common ground line, with a magnifier for the small ones."""
    if kind == "toaster":
        items = [_toaster(PASTEL_ORANGE, fill)]
    elif kind == "person":
        items = [_person_breathing(TRUE_METRE) for _ in range(count)]
    else:
        items = [_tealight() for _ in range(count)]
    real = VGroup(*items).arrange(RIGHT, buff=0.14 if kind == "tealight" else 0.3, aligned_edge=DOWN)
    real.move_to(np.array([x, ground_y + real.height / 2, 0.0]))
    parts = [real]
    if kind in ZOOM:
        parts.append(_zoom_inset(kind, fill, real.get_top(), PASTEL_ORANGE))
    return VGroup(*parts)


def _emitters(kind: str, group) -> list:
    """🎯 Points a device row gives its heat off from — toaster slots, a person's chest, tealight flames."""
    row = group[0]
    if kind == "toaster":
        top = row[0][1].get_top()
        return [top + RIGHT * dx * row[0][1].width + DOWN * 0.01 for dx in (-0.2, 0.2)]
    if kind == "person":
        return [p.get_center() + UP * p.height * 0.18 for p in row]
    return [t.get_top() for t in row]


'''
s = s[:a] + new + s[b:]
s = s.replace("glyph = _devices(kind, 1).move_to(np.array([x, 0.7, 0.0]))", "glyph = _devices(kind, 1, x, 0.0)")
s = s.replace("glyph = _devices(kind, max(1, int(round(ratio))), fill=min(1.0, ratio)).move_to(np.array([d[\"x\"], y0 + wall_h / 2, 0.0]))",
              "glyph = _devices(kind, max(1, int(round(ratio))), d[\"x\"], y0, fill=min(1.0, ratio))")
p.write_text(s, encoding="utf-8")
