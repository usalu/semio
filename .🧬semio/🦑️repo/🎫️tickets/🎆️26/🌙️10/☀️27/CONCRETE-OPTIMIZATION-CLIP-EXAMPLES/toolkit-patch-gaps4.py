"""[DEBUG] One-off edit: animation-safe live groups, one-shape labels, cached chip frames and sun states."""
from pathlib import Path

room = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py")
source = room.read_text(encoding="utf-8")


def swap(old, new):
    global source
    assert old in source, old[:80]
    source = source.replace(old, new, 1)


start = source.index("class LiveText(_engine.Text):")
end = source.index("def text(value")
new = '''class LiveText(_engine.Text):
    """🏷️ A label drawn as one shape from cached glyph outlines; it is a ``Text`` so layout checks treat it as text."""

    def __init__(self, points, color, text=""):
        _engine.VMobject.__init__(self)
        shape = _engine.VMobject(fill_color=color, fill_opacity=1, stroke_width=0)
        shape.points = points
        self.add(shape)
        self.text = self.original_text = text


class LiveGroup(_engine.VGroup):
    """♻️ A widget that rebuilds its children every frame; copies are static snapshots, so fades, transforms and ``Indicate`` stay safe while trackers move."""

    def __deepcopy__(self, memo):
        twin = super().__deepcopy__(memo)
        twin.updaters = []
        return twin


def clone(mob):
    """🪞 A cheap structural copy of a static glyph, so redraws never share mutable objects with an animation."""
    twin = _engine.VMobject()
    twin.points = mob.points.copy()
    twin.match_style(mob)
    twin.add(*[clone(sub) for sub in mob.submobjects])
    return twin


@lru_cache(maxsize=8192)
def _label_points(size, key):
    table, gap, advance = _glyphs()
    factor = size / REFERENCE_SIZE
    outlines, cursor, tight = [], 0.0, True
    for live, part in key:
        if live:
            if not tight:
                cursor += gap
            for char in part:
                points, dx, dy, step = table[char]
                outlines.append(points + np.array([cursor + dx, dy, 0.0]))
                cursor += step
        else:
            joined = part.startswith("~")
            word = part[1:] if joined else part
            if not tight and not joined:
                cursor += gap
            shapes, length = _word(word)
            outlines += [points + np.array([cursor, 0.0, 0.0]) for points in shapes]
            cursor += length
        tight = False
    return (np.concatenate(outlines) * factor) if outlines else np.zeros((0, 3))


def row(size, color, *parts):
    """🏷️ One label from static words and live numbers, joined by a space; a leading ``~`` joins a word tightly to the previous part."""
    key = tuple((isinstance(part, Number), str(part)) for part in parts if part)
    return LiveText(_label_points(size, key).copy(), color, " ".join(part for _, part in key))


'''
source = source[:start] + new + source[end:]

swap('''def chip(value, point=(0, 0, 0), *, size=16, color=INK):
    """🔖 Place one label in the shared square chip chrome."""
    return _art.window_chip(text(value, size=size, color=color), color=color).move_to(point)''',
'''@lru_cache(maxsize=2048)
def _frame(width, height, color):
    return _art.window_frame(width, height, color=color)


def chip(value, point=(0, 0, 0), *, size=16, color=INK):
    """🔖 Place one label in the shared square chip chrome (frames are cached by size)."""
    label = text(value, size=size, color=color)
    frame = clone(_frame(round(label.width + 2 * _art.UI_SPACING, 2), round(label.height + 2 * _art.UI_SPACING, 2), color)).move_to(label.get_center())
    return _engine.VGroup(frame, label).move_to(point)''')

# sun state from the model, cached by quarter degrees; arc from the study constants
start = source.index("@lru_cache(maxsize=4)\ndef _sun_track")
end = source.index("def cardinal(")
sun = '''@lru_cache(maxsize=4)
def _sun_track(season: str):
    sun = _model.load_study()["sun"]
    phi, delta = math.radians(sun["latitude"]), math.radians(sun["declination"] if season == "summer" else -sun["declination"])
    track = []
    for step in range(-900, 901):
        hour = math.radians(step / 5)
        altitude = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(hour)))
        if altitude >= sun["minimum_altitude"]:
            track.append(((180 + math.degrees(math.atan2(math.sin(hour), math.cos(hour) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360, altitude))
    return track


def sun_arc(season: str) -> tuple[float, float, float]:
    """☀️ Bearing range and noon altitude of the sun at the solstice."""
    track = _sun_track(season)
    bearings = [bearing for bearing, _ in track]
    return min(bearings), max(bearings), max(altitude for _, altitude in track)


@lru_cache(maxsize=4096)
def _sun(orientation: float, season: str) -> dict:
    return _model.sun_state(orientation, season)


def sun_state(orientation: float, season: str) -> dict:
    """🌞 The model's sun-path point that meets the façade best (cached per quarter degree; do not mutate)."""
    return _sun(round(orientation * 4) / 4, season)


'''
source = source[:start] + sun + source[end:]
swap('''    def _live(self, builder):
        holder = builder()
        holder.add_updater(lambda mob: setattr(mob, "submobjects", builder().submobjects))
        return holder''', '''    def _live(self, builder):
        holder = LiveGroup(*builder().submobjects)
        holder.add_updater(lambda mob: setattr(mob, "submobjects", builder().submobjects))
        return holder''')
room.write_text(source, encoding="utf-8")
print("[DEBUG] patched")
