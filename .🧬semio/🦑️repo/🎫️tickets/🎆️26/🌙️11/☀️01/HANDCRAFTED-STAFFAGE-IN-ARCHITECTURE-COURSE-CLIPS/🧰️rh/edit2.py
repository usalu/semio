from pathlib import Path
p = Path("🧱️conduction/🐍️.py")
s = p.read_bytes().decode("utf-8")
def rep(a, b):
    global s
    a = a.replace(chr(10), chr(13)+chr(10)); b = b.replace(chr(10), chr(13)+chr(10))
    assert a in s, a
    s = s.replace(a, b)
rep("TRUE_METRE = 0.95\nZOOM_RADIUS = 0.78", "TRUE_METRE = 0.85\nZOOM_RADIUS = 0.7")
rep('''def _zoom_inset(kind: str, fill: float, source_top, color: str):''', '''def _zoom_inset(kind: str, fill: float, source, color: str):''')
rep('''    disc.move_to(source_top + UP * (0.45 + ZOOM_RADIUS))
    item.move_to(disc)
    leader = DashedLine(source_top + UP * 0.06, disc.get_bottom(), color=PASTEL_WHITE, stroke_width=1.4, dash_length=0.06)
    tag = body_text(f"{int(factor)}×", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
    tag.next_to(disc, RIGHT, buff=0.1, aligned_edge=UP)''', '''    disc.move_to(source.get_center() + RIGHT * (1.45 + ZOOM_RADIUS) + UP * (0.55 + ZOOM_RADIUS - source.height / 2))
    item.move_to(disc.get_center() + DOWN * 0.04 * ZOOM_RADIUS, aligned_edge=ORIGIN)
    item.shift(disc.get_center() - item.get_center())
    leader = DashedLine(source.get_right() + RIGHT * 0.05, disc.get_left(), color=PASTEL_WHITE, stroke_width=1.4, dash_length=0.06)
    tag = body_text(f"{int(factor)}×", font_size=LABEL_FONT_SIZE, color=PASTEL_WHITE)
    tag.next_to(disc, DOWN, buff=0.08)''')
rep("parts.append(_zoom_inset(kind, fill, real.get_top(), PASTEL_ORANGE))", "parts.append(_zoom_inset(kind, fill, real, PASTEL_ORANGE))")
rep('''def _emitters(kind''', '''def _ground_line(y: float):
    """📏 Shared ground line every true-scale device stands on."""
    return Line(np.array([-6.3, y, 0.0]), np.array([6.3, y, 0.0]), color=PASTEL_WHITE, stroke_width=1.6, stroke_opacity=0.7)


def _emitters(kind''')
rep("self.play(LaggedStart(*[FadeIn(g, shift=UP * 0.2) for g in intro], lag_ratio=0.25), run_time=1.4)",
    "intro.add(_ground_line(0.0))\n        self.play(LaggedStart(*[FadeIn(g, shift=UP * 0.2) for g in intro], lag_ratio=0.25), run_time=1.4)")
rep("self.play(*[Indicate(h, color=PASTEL_YELLOW) for h in hints], *device_warmth(0.8), run_time=0.8)", "self.play(*[Indicate(h, color=PASTEL_YELLOW) for h in hints], *device_warmth(0.8), run_time=0.8)")
rep("        self.play(*[FadeTransform(sec, dev) for sec, dev in zip(sections, devices)],\n", "        devices.add(_ground_line(y0))\n        self.play(*[FadeTransform(sec, dev) for sec, dev in zip(sections, devices)],\n")
p.write_bytes(s.encode("utf-8"))
