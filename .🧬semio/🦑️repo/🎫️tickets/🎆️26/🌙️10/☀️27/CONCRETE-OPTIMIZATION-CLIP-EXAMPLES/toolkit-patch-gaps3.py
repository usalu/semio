"""[DEBUG] One-off edit: robust labels at every scale, safe animation helpers, plot_axes titles."""
from pathlib import Path

base = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals")
room, main = base / "🏠️room/🐍️.py", base / "🐍️.py"
source = room.read_text(encoding="utf-8")


def swap(old, new, count=1):
    global source
    assert old in source, old[:70]
    source = source.replace(old, new, count)


swap("""def _overlap(first, second, margin=0.05):""", '''def label_size(scale: float) -> int:
    """🔠 Label font size that follows the drawing scale between 11 and 16 points."""
    return min(16, max(11, round(28 * scale + 1)))


def _overlap(first, second, margin=0.05):''')
swap("""        word_demand = self.tr("Heating", "Heizen") if season == "winter" else self.tr("Cooling", "Kühlen")
""", """        word_demand = self.tr("Heating", "Heizen") if season == "winter" else self.tr("Cooling", "Kühlen")
        size = label_size(s)
""")
old = source[source.index("                notes = [(chip([\"d\""):source.index("                group.add(_dimension(P(x_o - 0.35, sill), P(x_o - 0.35, HEAD), color=GLASS))")]
new = '''                notes = [(chip(["d", *parts(d, "m", 1, german)], size=size - 1, color=WARN), HEIGHT + SLAB + 0.45, min(x_tip, x_o)),
                         (chip(["WWR", *parts(design["wwr"] * 100, "%", 0, german)], size=size - 1, color=GLASS), level, x_o - 0.35),
                         (chip(["t", *parts(t * 100, "cm", 0, german)], size=size - 1, color=INSULATION), -0.62, x_o)]
                lowest = math.inf
                for item, y, end in notes:
                    drawn = min(y, lowest - item.height / s - 0.1 / s) if lowest < math.inf else y
                    place(item, P(column, drawn), "right")
                    group.add(item, _leader(item, P(end, y), item[0].get_stroke_color()))
                    placed.append(item)
                    lowest = drawn - item.height / s / 2
'''
source = source.replace(old, new, 1)
swap("""                    group.add(place(text([word_demand], size=15, color=colour), P(DEPTH + MASONRY + 0.5, 3.35), "left"),
                              place(text(parts(found[key], "kWh/a", 0, german), size=15, color=colour), P(DEPTH + MASONRY + 0.5, 2.9), "left"))""",
     """                    head = place(text([word_demand], size=size, color=colour), P(DEPTH + MASONRY + 0.5, 3.35), "left")
                    group.add(head, place(text(parts(found[key], "kWh/a", 0, german), size=size, color=colour),
                                          (head.get_left()[0], head.get_bottom()[1] - 0.04 - size * 0.0075, 0), "left"))""")
swap("""text("6 m", P(DEPTH / 2, -1.58), size=14, color=GRAY))""", """text("6 m", P(DEPTH / 2, -1.3) + np.array([0, -0.12 - size * 0.012, 0]), size=size - 2, color=GRAY))""")
swap("""text("1 m", P(-2.1, -1.58), size=14, color=GRAY))""", """text("1 m", P(-2.1, -1.3) + np.array([0, -0.12 - size * 0.012, 0]), size=size - 2, color=GRAY))""")
swap("""label = text(words, P(*sun_at) + np.array([dx, dy, 0]) * s * 2, size=14, color=SUN)""", """label = text(words, P(*sun_at) + np.array([dx, dy, 0]) * s * 2, size=size - 1, color=SUN)""")

start = source.index("    def facade(")
end = source.index("    def plan(")
facade = source[start:end]
old = facade[facade.index("            group.add(_dimension(P(-.35, 0)"):facade.index("            return group")]
new = '''            size = label_size(s)
            if s >= 0.3:
                group.add(_dimension(P(-.35, 0), P(-.35, HEIGHT)), text("3 m", P(-.35, HEIGHT / 2) + np.array([-0.3 - size * 0.012, 0, 0]), size=size - 2, color=GRAY))
                group.add(_dimension(P(0, -.35), P(WIDTH, -.35)), text("8 m", P(WIDTH / 2, -.35) + np.array([0, -0.2 - size * 0.012, 0]), size=size - 2, color=GRAY))
            else:
                group.add(_dimension(P(-.35, 0), P(-.35, HEIGHT)), _dimension(P(0, -.35), P(WIDTH, -.35)))
            if labels and s >= 0.22:
                full = chip(["WWR", *parts(design["wwr"] * 100, "%", 0, german), "·", *parts(found["window_area"], "m²", 1, german)], size=size - 1, color=GLASS)
                short = chip(["WWR", *parts(design["wwr"] * 100, "%", 0, german)], size=size - 1, color=GLASS)
                depth_chip = chip(["d", *parts(design["overhang"], "m", 1, german)], size=size - 1, color=WARN)
                room_wide = s * WIDTH + 0.3
                top = P(0, HEIGHT + SLAB + 0.5)
                first = full if full.width + depth_chip.width + 0.15 <= room_wide else short
                if first.width + depth_chip.width + 0.15 <= room_wide:
                    group.add(place(first, top, "left"), place(depth_chip, P(WIDTH, HEIGHT + SLAB + 0.5), "right"))
                else:
                    group.add(place(short, (origin[0] + s * WIDTH / 2 + short.width / 2 * 0, top[1] + depth_chip.height + 0.08, 0), "center"),
                              place(depth_chip, (origin[0] + s * WIDTH / 2, top[1], 0), "center"))
'''
facade = facade.replace(old, new, 1)
source = source[:start] + facade + source[end:]

swap("""    def ramp(self, low=None, high=None):""", '''    def reveal(self, *widgets, run_time: float = 0.5):
        """🌅 Fade live widgets in safely: they are refreshed to the current trackers first, so fading in after a fade-out works."""
        for widget in widgets:
            widget.update(0)
        return _engine.AnimationGroup(*[_engine.FadeIn(widget, run_time=run_time) for widget in widgets])

    def dismiss(self, *widgets, run_time: float = 0.5):
        """🌇 Fade widgets out; ``reveal`` brings them back."""
        return _engine.AnimationGroup(*[_engine.FadeOut(widget, run_time=run_time) for widget in widgets])

    def pulse(self, widget, *, color=INK, run_time: float = 1.0, buff: float = 0.12):
        """💓 Emphasize any widget, live or static, by flashing a static outline around its current extent (``Indicate`` on live widgets breaks)."""
        outline = _engine.SurroundingRectangle(widget, color=color, buff=buff, stroke_width=3)
        return _engine.Succession(_engine.FadeIn(outline, run_time=0.2 * run_time), _engine.Indicate(outline, color=color, scale_factor=1.05, run_time=0.6 * run_time),
                                  _engine.FadeOut(outline, run_time=0.2 * run_time))

    def ramp(self, low=None, high=None):''')
room.write_text(source, encoding="utf-8")

text = main.read_text(encoding="utf-8")
old = text[text.index("        y_title = _text(y_label"):text.index("        for (low, high, stride), digits, ticks, horizontal")]
new = """        labels = mn.VGroup()
        if x_label:
            labels.add(_text(x_label, (point[0], point[1] - height / 2 - .6, 0), size=19, width=width))
        if y_label:
            y_title = _text(y_label, (0, point[1] + height / 2 + .4, 0), size=19, width=width * .5)
            labels.add(y_title.shift(mn.RIGHT * (point[0] - width / 2 - .55 - y_title.get_left()[0])))
"""
text = text.replace(old, new, 1)
text = text.replace("axes.c2p(x_range[0], value) + mn.LEFT * .1)", "axes.c2p(x_range[0], value) + mn.LEFT * .2)")
text = text.replace("def plot_axes(self, *, x_range, y_range, x_label, y_label,", "def plot_axes(self, *, x_range, y_range, x_label=None, y_label=None,")
main.write_text(text, encoding="utf-8")
print("[DEBUG] patched")
