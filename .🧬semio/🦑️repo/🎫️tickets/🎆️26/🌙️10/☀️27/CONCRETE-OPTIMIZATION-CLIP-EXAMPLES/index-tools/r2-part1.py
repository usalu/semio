"""🎬 Multiple Criteria and a Normalized Index: five façade variants, physical rulers, weights and a ranked index."""

import importlib
import math
import re
import sys
from functools import lru_cache
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
_room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
_art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
mn = importlib.import_module("manim")

INK, MUTED, GOOD, WARN, BAD = _art.P_WHITE, _art.PASTEL_GREY, _art.P_GREEN, _art.P_ORANGE, _art.P_PRIMARY
COLORS = {"electricity": _art.PASTEL_PURPLE, "cost": _art.P_ORANGE, "daylight": _art.P_YELLOW, "carbon": _art.PASTEL_GREY}
GLYPHS = {"electricity": "bulb", "daylight": "sun", "carbon": "factory"}
ORDER = [item["id"] for item in _model.criteria()]
CRITERIA = {item["id"]: item for item in _model.criteria()}
DIGITS = {"electricity": 2, "cost": 0, "daylight": 2, "carbon": 0}
SHORT = {"electricity": ("Electricity", "Strombedarf"), "cost": ("Cost", "Kosten"), "daylight": ("Daylight", "Tageslicht"), "carbon": ("Carbon", "Graue Emissionen")}
TITLES = {"electricity": ("Electricity intensity", "Strombedarf pro m²")}
S0, S1, S2 = 15, _fonts.LABEL_FONT_SIZE - 2, _fonts.BODY_FONT_SIZE
FLOOR_AREA = 48
BUDGET_CHF = _model.brief()["budget_chf"]
ROOM_SCALE, ROOM_LEFT, FLOOR_Y = 0.72, 2.0, -0.7
SECTION_POINT = (ROOM_LEFT + 3 * ROOM_SCALE, FLOOR_Y + 1.5 * ROOM_SCALE, 0)
LEFT, RIGHT = -6.95, -0.2
FRAME_X0, FRAME_X1 = -7.08, 0.1
WIDTH, DEPTH, HEIGHT, SLAB, MASONRY, HEAD = _room.WIDTH, _room.DEPTH, _room.HEIGHT, _room.SLAB, _room.MASONRY, _room.HEAD
LAYER = {"fixed": MUTED, "wall": INK, "insulation": _room.INSULATION, "window": _room.GLASS, "overhang": WARN}
VARIANT_COLOR = {"glass_shaded": _art.P_CYAN, "ribbon": _art.P_GREEN, "punched": _art.PASTEL_PURPLE, "punched_insulated": _art.P_YELLOW, "glass_bare": _art.P_ORANGE, "glass_premium": _art.P_RED}
ENERGY = (("heating", _room.HEAT), ("cooling", _room.COOL), ("lighting", _room.LIGHT))


def source_files() -> tuple[Path, ...]:
    """🧾 Include this scene in the shared render provenance."""
    return (Path(__file__),)


def P(x, y):
    """📍 Screen point of an interior coordinate (metres across the room depth and above the floor) of the room panel."""
    return np.array([ROOM_LEFT + ROOM_SCALE * x, FLOOR_Y + ROOM_SCALE * y, 0.0])


@lru_cache(maxsize=None)
def variants(extra=False):
    """🏘️ The five (or six) compared façade variants of the model."""
    return tuple(_model.variants(extra=extra))


def by_id(identifier):
    """🔎 One variant (including the extra one) by its id."""
    return next(item for item in variants(True) if item["id"] == identifier)


def ids(extra=False):
    """🔖 Ids of the compared variants in model order."""
    return [item["id"] for item in variants(extra)]


@lru_cache(maxsize=None)
def table(weights, extra=False, pool=None):
    """📋 Model index rows by variant id for hashable weights (a tuple in criterion order) and an optional tuple of compared ids."""
    return {row["id"]: row for row in _model.index(dict(zip(ORDER, weights)), list(pool) if pool else ids(extra))}


def raw(identifier, criterion):
    """📏 The model's raw value of one variant and criterion."""
    return by_id(identifier)[CRITERIA[criterion]["key"]]


def best_id(criterion, pool):
    """🏅 The variant with the best raw value of a criterion in a pool."""
    sign = 1 if CRITERIA[criterion]["direction"] == "min" else -1
    return min(pool, key=lambda i: sign * raw(i, criterion))


def winner_of(weights, extra=False, pool=None):
    """🥇 The id with the lowest index under weights."""
    return min(table(weights, extra, pool).values(), key=lambda row: row["score"])["id"]


def text(value, point=(0, 0, 0), *, size=S1, color=INK, width=None):
    """🔤 Static label in the canonical face, shrunk to a width when needed."""
    label = _fonts.body_text(str(value).replace("kWh/(m²·a)", _room.INTENSITY_UNIT), font_size=size, color=color)
    if width is not None and label.width > width:
        label.scale_to_fit_width(width)
    return label.move_to(point)


def lines(value, limit=15):
    """✂️ Break a long name into two balanced lines, at spaces or after hyphens."""
    if len(value) <= limit:
        return value
    tokens = re.findall(r"[^\s-]+-?", value)
    if len(tokens) < 2:
        return value

    def join(parts):
        return "".join(part + ("" if part.endswith("-") else " ") for part in parts).strip()
    cut = min(range(1, len(tokens)), key=lambda k: max(len(join(tokens[:k])), len(join(tokens[k:]))))
    return join(tokens[:cut]) + "\n" + join(tokens[cut:])


def stack(value):
    """📚 A name with one word per line (hyphenated words break after the hyphen)."""
    return re.findall(r"[^\s-]+-?", value)


def block(value, point=(0, 0, 0), *, size=S0, color=INK, width=None):
    """📝 One or two centred lines of text."""
    group = _fonts.centered_body_text(value, font_size=size, color=color)
    if width is not None and group.width > width:
        group.scale_to_fit_width(width)
    return group.move_to(point)


def glyph(criterion, point=(0, 0, 0), width=0.34, color=None):
    """🔣 The criterion's pictogram: bulb, coin, sun or factory."""
    color = color or COLORS[criterion]
    if criterion == "cost":
        return mn.VGroup(mn.Circle(radius=width / 2, color=color, stroke_width=2.2, fill_color=color, fill_opacity=.18),
                         mn.Circle(radius=width / 3.4, color=color, stroke_width=1.4)).move_to(point)
    return _art.object_silhouette(GLYPHS[criterion], color=color, width=width).move_to(point)


def arrow(criterion, point=(0, 0, 0), length=0.3):
    """⬇️ Direction arrow: down where less is better, up where more is better."""
    sign = 1 if CRITERIA[criterion]["direction"] == "max" else -1
    start = np.array(point, dtype=float) - np.array([0, sign * length / 2, 0])
    return mn.Arrow(start, start + np.array([0, sign * length, 0]), buff=0, color=COLORS[criterion], stroke_width=3,
                    tip_length=.11, max_tip_length_to_length_ratio=.5)


def live(builder, key=None):
    """🔁 The toolkit's animation-safe live group: rebuilt every frame, or only when ``key()`` changes; copies made by animations are static."""
    return _room.LiveGroup(builder, key)


def nice_step(span, target):
    """📶 A round tick step giving about ``target`` ticks over a span."""
    raw_step = span / target
    base = 10 ** math.floor(math.log10(raw_step))
    return next(m * base for m in (1, 2, 2.5, 5, 10) if m * base >= raw_step)


def lerp(a, b, t):
    """➗ Linear interpolation."""
    return a + (b - a) * t


def reproduce(function, inputs, target, out_digits, limit=3):
    """🧮 Fewest decimals of the printed inputs for which the printed result reproduces; ``≈`` when no precision does."""
    for digits in range(limit + 1):
        if round(function(*[round(value, digits) for value in inputs]), out_digits) == round(target, out_digits):
            return digits, "="
    return limit, "≈"


def energy_digits(found):
    """⚡ Decimals of the electricity parts for which parts add up to the total and total ÷ floor area reproduces the printed intensity."""
    for digits in (0, 1, 2):
        shown = _room.apportion([found[key] for key, _ in ENERGY], digits)
        if round(round(sum(shown), digits) / FLOOR_AREA, 2) == round(found["intensity"], 2):
            return digits
    return 2


class Retire(mn.Animation):
    """🧊 Fade a live group out through a static twin made when the animation starts, so shared trackers and rebuilds never race the fade."""

    def __init__(self, group, **kwargs):
        super().__init__(group, **kwargs)
        self.scene = self.twin = self.inner = None

    def _setup_scene(self, scene):
        self.scene = scene

    def begin(self):
        self.twin = self.mobject.copy()
        self.scene.remove(self.mobject)
        self.scene.add(self.twin)
        self.inner = mn.FadeOut(self.twin, run_time=self.run_time)
        self.inner._setup_scene(self.scene)
        self.inner.begin()
        self.starting_mobject = self.inner.starting_mobject

    def interpolate(self, alpha):
        self.inner.interpolate(alpha)

    def finish(self):
        self.inner.finish()

    def clean_up_from_scene(self, scene):
        self.inner.clean_up_from_scene(scene)
        scene.remove(self.twin, self.mobject)

    def get_all_mobjects(self):
        return self.inner.get_all_mobjects()

    def update_mobjects(self, dt):
        self.inner.update_mobjects(dt)


def runs_of(items):
    """🏃 Maximal runs of equal consecutive items as (item, first index, last index)."""
    runs, start = [], 0
    for k in range(1, len(items) + 1):
        if k == len(items) or items[k] != items[start]:
            runs.append((items[start], start, k - 1))
            start = k
    return runs


def sorted_index(pool, values, identifier):
    """🔢 Position of an id when the pool is sorted by value (ties by pool order)."""
    ordered = sorted(pool, key=lambda i: (values[i], list(pool).index(i)))
    return ordered.index(identifier)


class Ruler:
    """📏 A criterion ruler: dots at the real values, thumbnails in slots, driven by offset, scale and mirror trackers."""

    def __init__(self, scene, criterion, pool, compared, *, x0=-6.6, length=6.0, line_y=.85, state="physical", name_size=S0, rings=True):
        self.name_size, self.rings = name_size, rings
        self.scene, self.criterion, self.pool, self.compared = scene, criterion, list(pool), list(compared)
        self.x0, self.length, self.line_y, self.center = x0, length, line_y, x0 + length / 2
        self.color = COLORS[criterion]
        self.digits_tag = 3 if criterion == "electricity" else DIGITS[criterion]
        self.sign = 1 if CRITERIA[criterion]["direction"] == "min" else -1
        self.values = {i: raw(i, criterion) for i in self.pool}
        inside = [self.values[i] for i in self.compared]
        self.low, self.high = min(inside), max(inside)
        self.best = self.low if self.sign > 0 else self.high
        every = list(self.values.values())
        pad = (max(every) - min(every)) * .06
        self.step_abs = nice_step(max(every) - min(every) + 2 * pad, 6)
        self.digits_abs = 0 if self.step_abs >= 1 else (1 if self.step_abs >= .1 else 2)
        self.lo = math.floor((min(every) - pad) / self.step_abs) * self.step_abs
        self.hi = math.ceil((max(every) + pad) / self.step_abs) * self.step_abs
        self.k0 = length / (self.hi - self.lo)
        self.step_off = nice_step(self.high - self.low, 4)
        self.digits_off = 0 if self.step_off >= 1 else (1 if self.step_off >= .1 else 2)
        self.xb = mn.ValueTracker(x0 + (self.best - self.lo) * self.k0)
        self.g = mn.ValueTracker(self.k0)
        self.sigma = mn.ValueTracker(1.0)
        self.o_abs, self.o_off, self.o_norm, self.o_val, self.o_ring = (mn.ValueTracker(0.0) for _ in range(5))
        self.focus = mn.ValueTracker(0.0)
        gap = (length - 1.5) / max(len(self.pool) - 1, 1)
        order = sorted(self.pool, key=lambda i: (self.values[i], self.pool.index(i)))
        self.gap = gap
        self.slot_x = {i: x0 + .75 + n * gap for n, i in enumerate(order)}
        self.scan = sorted(self.compared, key=lambda i: (self.sign * self.values[i], self.pool.index(i)))
        norm = {row["id"]: row["normalized"][criterion] for row in _model.index(dict(zip(ORDER, (.25,) * 4)), self.compared)}
        self.norm = norm
        self.members, self.ticks = {}, mn.VGroup()
        self.line = mn.Line([x0 - .15, line_y, 0], [x0 + length + .15, line_y, 0], color=INK, stroke_width=2.4)
        self.ring = mn.Rectangle(width=.95, height=.78, color=INK, stroke_width=2.4)
        self.ring.add_updater(lambda m: self._ring_update(m))
        self._ticks()
        for identifier in self.pool:
            self.members[identifier] = self._member(identifier)
        if state == "normalized":
            self.g.set_value(length / (self.high - self.low))
            self.xb.set_value(x0 if self.sign > 0 else x0 + length)
            self.sigma.set_value(self.sign)
            self.o_norm.set_value(1.0)
            self.o_val.set_value(1.0)

    def x_of(self, value):
        """📍 Screen x of a physical value under the live offset, scale and mirror."""
        pre = self.xb.get_value() + (value - self.best) * self.g.get_value()
        return self.center + self.sigma.get_value() * (pre - self.center)

    def fraction(self, identifier):
        """🔢 Position of a variant along the ruler frame, 0 at the left end and 1 at the right."""
        return (self.x_of(self.values[identifier]) - self.x0) / self.length

    def clamp(self, x):
        """📎 Keep a dot near the ruler frame; off-scale values wait just beyond the right end."""
        return min(max(x, self.x0 - .2), self.x0 + self.length + .3)

    def _slot_x(self, identifier):
        return self.center + self.sigma.get_value() * (self.slot_x[identifier] - self.center)

    def _ticks(self):
        s = self.scene
        specs, value = [], self.lo
        while value <= self.hi + 1e-9:
            specs.append(("abs", value, s.number(value, self.digits_abs)))
            value += self.step_abs
        offset = 0.0
        while offset <= (self.high - self.low) * 1.7 + 1e-9:
            label = "0" if offset == 0 else ("+" if self.sign > 0 else "−") + s.number(offset, self.digits_off) + (" " + CRITERIA[self.criterion]["unit"] if abs(offset - self.step_off) < 1e-9 else "")
            specs.append(("off", offset, label))
            offset += self.step_off
        for fraction in (0, .25, .5, .75, 1):
            specs.append(("norm", fraction, {0: "0", 1: "1"}.get(fraction, s.number(fraction, 2))))
        for kind, value, label in specs:
            tick = mn.VGroup(mn.Line([0, self.line_y - .07, 0], [0, self.line_y + .07, 0], color=INK, stroke_width=2),
                             text(label, [0, self.line_y - .3, 0], size=S0, color=INK))
            tick.kind, tick.value = kind, value
            tick.add_updater(lambda m: self._tick_update(m))
            self.ticks.add(tick)

    def _tick_update(self, tick):
        if tick.kind == "abs":
            x, level = self.x_of(tick.value), self.o_abs.get_value()
        elif tick.kind == "off":
            pre = self.xb.get_value() + self.sign * tick.value * self.g.get_value()
            x, level = self.center + self.sigma.get_value() * (pre - self.center), self.o_off.get_value()
        else:
            x, level = self.x0 + tick.value * self.length, self.o_norm.get_value()
        tick.move_to([x, tick.get_center()[1], 0])
        tick.set_opacity(level if self.x0 - .03 <= x <= self.x0 + self.length + .03 else 0.0)

    def _member(self, identifier):
        s = self.scene
        item = by_id(identifier)
        norm = self.norm.get(identifier)
        ring = (GOOD if norm == 0 else (WARN if norm == 1 else None)) if self.rings else None
        shade = (GOOD if norm == 0 else (WARN if norm == 1 else INK)) if self.rings else INK
        row = sorted_index(self.pool, self.values, identifier) % 2
        ly = self.line_y
        thumb = s.thumbnail(item["design"], (0, ly + .85, 0), .15, ramp=s._ramp, value=s._ramp.low, ring=ring)
        name = block(lines(item["title"][s.language()], 14), (0, ly + 1.5 + .5 * row, 0), size=self.name_size, color=shade, width=1.9)
        parts = [thumb, name]
        if row:
            parts.append(mn.Line([0, ly + 1.18, 0], [0, ly + 1.62, 0], color=MUTED, stroke_width=1.2, stroke_opacity=.6))
        slot = mn.VGroup(*parts)
        slot.identifier = identifier
        slot.add_updater(lambda m: m.move_to([self._slot_x(m.identifier), m.get_center()[1], 0]))
        dot = mn.Dot(radius=.075, color=self.color)
        dot.identifier = identifier
        dot.add_updater(lambda m: m.move_to([self.clamp(self.x_of(self.values[m.identifier])), self.line_y, 0]))
        leader = mn.Line([0, 0, 0], [1, 0, 0], color=self.color, stroke_width=1.6, stroke_opacity=.7)
        leader.identifier = identifier
        leader.add_updater(lambda m: m.put_start_and_end_on([self._slot_x(m.identifier), self.line_y + .5, 0],
                                                              [self.clamp(self.x_of(self.values[m.identifier])), self.line_y + .08, 0]))
        tag = text(s.number(self.values[identifier], self.digits_tag), (0, 0, 0), size=S0, color=shade)
        tag.identifier, tag.row = identifier, row
        tag.add_updater(lambda m: (m.move_to([self.clamp(self.x_of(self.values[m.identifier])), self.line_y - .62 - .3 * m.row, 0]),
                                   m.set_opacity(self.o_val.get_value() * (1 - self.o_norm.get_value()))))
        number = live(lambda: self._norm_tag(identifier))
        for part in (slot, dot, leader, tag):
            part.update()
        return {"slot": slot, "dot": dot, "leader": leader, "tag": tag, "norm": number}

    def _norm_tag(self, identifier):
        fraction = min(max(self.fraction(identifier), 0), 9.99)
        label = _room.text([_room.Number(self.scene.number(fraction, 2))], size=S1, color=INK)
        row = sorted_index(self.pool, self.values, identifier) % 2
        label.move_to([self.clamp(self.x_of(self.values[identifier])), self.line_y - .62 - .3 * row, 0])
        label.set_opacity(self.o_norm.get_value())
        return mn.VGroup(label)

    def _ring_update(self, ring):
        position = min(max(self.focus.get_value(), 0), len(self.scan) - 1)
        low, high = int(math.floor(position)), int(math.ceil(position))
        a, b = self.members[self.scan[low]]["slot"][0].get_center(), self.members[self.scan[high]]["slot"][0].get_center()
        ring.move_to(a + (b - a) * (position - low))
        ring.set_stroke(opacity=self.o_ring.get_value())

    def mobjects(self, only=None):
        """🧩 Every mobject of the ruler (or of the given variants) in one group, ticks and ring included."""
        group = mn.VGroup(self.line, self.ticks) if only is None else mn.VGroup()
        for identifier in (self.compared if only is None else only):
            group.add(*self.members[identifier].values())
        if only is None:
            group.add(self.ring)
        return group

    def current(self):
        """📌 The id the focus ring points at."""
        return self.scan[int(round(min(max(self.focus.get_value(), 0), len(self.scan) - 1)))]

    def tween(self, run_time, **targets):
        """🎚️ Animate named trackers together."""
        return mn.AnimationGroup(*[getattr(self, name).animate(run_time=run_time).set_value(value) for name, value in targets.items()])

    def appear(self, run_time=1.2):
        """✨ Show the physical ruler: ticks, then value tags."""
        return self.tween(run_time, o_abs=1.0, o_val=1.0)

    def pan(self, run_time=2.0):
        """➖ Step one: move the best value to the origin."""
        return self.tween(run_time, xb=self.x0 if self.sign > 0 else self.x0 + self.length, o_abs=0.0, o_off=1.0)

    def stretch(self, run_time=2.0, span=None):
        """➗ Step two: divide by the span so the worst value reaches one."""
        return self.tween(run_time, g=self.length / (span or (self.high - self.low)), o_off=0.0, o_norm=1.0)

    def flip(self, run_time=2.0):
        """🔄 Mirror the ruler so that the best value is on the left."""
        return self.tween(run_time, sigma=-1.0)

    def visit(self, positions, hold=.5, travel=.7, move_room=True):
        """🧭 Move the focus ring (and the room) along scan positions."""
        steps = []
        for position in positions:
            moves = [self.focus.animate(run_time=travel).set_value(position)]
            if move_room:
                moves.append(self.scene.set_focus(self.scan[position], travel))
            steps += [mn.AnimationGroup(*moves), mn.Wait(hold)]
        return mn.Succession(*steps)


@lru_cache(maxsize=None)
def glyph_proto(criterion):
    """🧊 A cached small pictogram to clone from."""
    return glyph(criterion, width=.26)


def head_content(scene, criterion):
    """🏷️ Column head content: pictogram, name (one word per line when long) and unit with its direction arrow, in the criterion colour."""
    name = SHORT[criterion][1 if scene.language() == "de" else 0]
    rows = [_room.text([word], size=13, color=COLORS[criterion]) for word in (name.split() if len(name) > 12 else [name])]
    rows.append(_room.text([*CRITERIA[criterion]["unit"].split(), "~↓" if CRITERIA[criterion]["direction"] == "min" else "~↑"], size=13, color=COLORS[criterion]))
    return mn.VGroup(_room.clone(glyph_proto(criterion)), *rows).arrange(mn.DOWN, buff=.05)


class Board:
    """📊 The weighted table: columns as wide as the weights, cells filled to the normalized value, rows ranked by index."""

    def __init__(self, scene, pool, extra=(), *, y0=1.7, pitch=.76, xl=-4.45, width=4.2, weights=None, order=None, name_size=14):
        self.scene, self.pool, self.extra = scene, list(pool), list(extra)
        self.all = self.pool + self.extra
        self.y0, self.pitch, self.xl, self.width = y0, pitch, xl, width
        start = weights or (.25,) * 4
        self.w = {c: mn.ValueTracker(v) for c, v in zip(ORDER, start)}
        positions = order or self.all
        self.r = {i: mn.ValueTracker(positions.index(i) if i in positions else len(positions)) for i in self.all}
        self.row_o = {i: mn.ValueTracker(1.0 if i in self.pool else 0.0) for i in self.all}
        self.t, self.fill, self.compact, self.numbers, self.scores, self.badges, self.header, self.win, self.old, self.mode, self.cur, self.focus, self.seg, self.seg_o = (mn.ValueTracker(0.0) for _ in range(14))
        self.header.set_value(1.0)
        self.numbers.set_value(1.0)
        self.old_id = None
        self.scan = list(self.pool)
        self.labels = {}
        for identifier in self.all:
            item = by_id(identifier)
            thumb = scene.thumbnail(item["design"], (-6.45, 0, 0), .095, ramp=scene._ramp, value=scene._ramp.low)
            name = block(lines(item["title"][scene.language()], 15), (-5.3, 0, 0), size=name_size, width=1.55)
            label = mn.VGroup(thumb, name)
            label.identifier = identifier
            label.add_updater(lambda m: m.move_to([m.get_center()[0], self.y(m.identifier), 0]))
            label.update()
            self.labels[identifier] = label
        self.live = live(lambda: self._build(), lambda: self._key())

    def _key(self):
        trackers = (*self.w.values(), *self.r.values(), *self.row_o.values(), self.t, self.fill, self.compact, self.numbers, self.scores, self.badges,
                    self.header, self.win, self.old, self.mode, self.cur, self.focus, self.seg, self.seg_o)
        return tuple(tracker.get_value() for tracker in trackers) + (self.old_id,)

    def segments(self, travel=.55, dwell=.35):
        """🔦 Point at the four segments of the focus row one after the other; the room's value strip names the raw value behind each."""
        active = self.scene.active
        steps = [mn.AnimationGroup(self.seg.animate(run_time=.01).set_value(0), active.animate(run_time=.01).set_value(0)),
                 self.seg_o.animate(run_time=.3).set_value(1.0), mn.Wait(dwell)]
        for j in range(1, 4):
            steps += [mn.AnimationGroup(self.seg.animate(run_time=travel).set_value(j), active.animate(run_time=travel).set_value(j)), mn.Wait(dwell)]
        return mn.Succession(*steps, mn.AnimationGroup(self.seg_o.animate(run_time=.3).set_value(0.0), active.animate(run_time=.3).set_value(-1)))

    def cursor(self):
        """🎯 The id the focus row sits on and the screen height of the focus frame; the frame follows its row while rows re-rank."""
        position = min(max(self.focus.get_value(), 0), len(self.scan) - 1)
        low, high = int(math.floor(position)), int(math.ceil(position))
        a, b = self.scan[low], self.scan[high]
        return self.scan[int(round(position))], lerp(self.y(a), self.y(b), position - low)

    def visit(self, positions, travel=.3, move_room=True):
        """🧭 Move the focus row (and the room) along scan positions."""
        steps = []
        for position in positions:
            moves = [self.focus.animate(run_time=travel).set_value(position)]
            if move_room:
                moves.append(self.scene.set_focus(self.scan[position], travel))
            steps.append(mn.AnimationGroup(*moves))
        return mn.Succession(*steps)

    def weights(self):
        """⚖️ The live weights, normalized to sum one, in criterion order."""
        values = [self.w[c].get_value() for c in ORDER]
        total = sum(values) or 1.0
        return tuple(v / total for v in values)

    def y(self, identifier):
        """↕️ Live row height of a variant."""
        return self.y0 - self.r[identifier].get_value() * self.pitch

    def state(self):
        """📋 Live normalized values and scores, blended towards the six-variant normalization by ``t``."""
        key = tuple(round(v, 6) for v in self.weights())
        five, t = table(key, False), self.t.get_value()
        six = table(key, True) if self.extra else None
        out = {}
        for identifier in self.all:
            if identifier in five and (six is None or t <= 0):
                values = [five[identifier]["normalized"][c] for c in ORDER]
            elif identifier in five:
                values = [lerp(five[identifier]["normalized"][c], six[identifier]["normalized"][c], t) for c in ORDER]
            else:
                values = [six[identifier]["normalized"][c] for c in ORDER]
            out[identifier] = {"n": values, "score": sum(w * v for w, v in zip(key, values))}
        return out

    def winner(self):
        """🥇 The id with the lowest live score among visible rows."""
        state = self.state()
        return min((i for i in self.all if self.row_o[i].get_value() > .5), key=lambda i: state[i]["score"])

    def _build(self):
        s, german = self.scene, self.scene.language() == "de"
        group = mn.VGroup()
        ws = self.weights()
        state = self.state()
        W, xl = self.width, self.xl
        lefts = [xl + W * sum(ws[:j]) for j in range(4)]
        compact, fill = self.compact.get_value(), self.fill.get_value()
        header, mode = self.header.get_value(), self.mode.get_value()
        strip_y = self.y0 + .5
        if header > .02:
            for j, c in enumerate(ORDER):
                x = xl + W / 2 + (j - 1.5) * 1.18
                chip = _art.window_chip(head_content(s, c), color=COLORS[c])
                chip.move_to([x, strip_y + .26 + chip.height / 2, 0]).set_opacity(header)
                group.add(chip)
                if mode > .02:
                    split = mn.Rectangle(width=max(ws[j] * W, 1e-3), height=.26, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85 * header * mode)
                    split.move_to([lefts[j] + ws[j] * W / 2, strip_y, 0])
                    link = mn.Line([x, chip.get_bottom()[1], 0], [lefts[j] + ws[j] * W / 2, strip_y + .13, 0], color=COLORS[c], stroke_width=1.4, stroke_opacity=.6 * header * mode)
                    decimals = 1 if any(abs(w * 100 - round(w * 100)) > .05 for w in ws) else 0
                    percent = [_room.Number(s.number(round(ws[j] * 100, decimals), decimals)), "%"]
                    if ws[j] * W < .44:
                        share = _room.place(_room.text(percent, size=14, color=COLORS[c]), (lefts[j] + ws[j] * W + .08, strip_y, 0), "left").set_opacity(header * mode)
                    else:
                        share = _room.text(percent, (lefts[j] + ws[j] * W / 2, strip_y, 0), size=14 if ws[j] * W > .7 else 13, color="#06121a").set_opacity(header * mode)
                    group.add(split, link, share)
            if mode > .02:
                word = s.tr("equal weights", "gleiche Gewichte") if max(ws) - min(ws) < 1e-3 else s.tr("weights", "Gewichte")
                group.add(_room.place(_room.text(word.split(), size=S0, color=MUTED), (xl - .12, strip_y, 0), "right").set_opacity(header * mode))
        numbers = self.numbers.get_value()
        tall, drop = self.pitch - .02 + .12 * numbers, .08 + .08 * numbers
        frame_w, frame_x = FRAME_X1 - FRAME_X0, (FRAME_X1 + FRAME_X0) / 2
        if self.cur.get_value() > .02:
            group.add(mn.Rectangle(width=frame_w, height=tall, color=INK, stroke_width=2.4, stroke_opacity=self.cur.get_value()).move_to([frame_x, self.cursor()[1] - drop, 0]))
        if self.win.get_value() > .02:
            group.add(mn.Rectangle(width=frame_w, height=tall, color=GOOD, stroke_width=3.2, stroke_opacity=self.win.get_value()).move_to([frame_x, self.y0 - drop, 0]))
        if self.old.get_value() > .02 and self.old_id in self.all:
            for edge in (tall / 2, -tall / 2):
                group.add(mn.DashedVMobject(mn.Line([frame_x - frame_w / 2, self.y(self.old_id) - drop + edge, 0], [frame_x + frame_w / 2, self.y(self.old_id) - drop + edge, 0], color=MUTED, stroke_width=2.2,
                                                    stroke_opacity=self.old.get_value()), num_dashes=40))
        if self.seg_o.get_value() > .02:
            ident, j = self.cursor()[0], int(round(min(max(self.seg.get_value(), 0), 3)))
            part = state[ident]["n"]
            start = xl + W * sum(ws[k] * part[k] * fill for k in range(j))
            left, span = lerp(lefts[j], start, compact), max(lerp(ws[j] * W, ws[j] * W * part[j] * fill, compact), .1)
            group.add(mn.Rectangle(width=span + .1, height=.44, color=INK, stroke_width=3.6, stroke_opacity=self.seg_o.get_value()).move_to([left + span / 2, self.y(ident), 0]))
        height = lerp(.34 if len(self.all) <= 5 else .32, .26, numbers)
        for identifier in self.all:
            level = self.row_o[identifier].get_value()
            if level < .02:
                continue
            y = self.y(identifier)
            values = state[identifier]["n"]
            cursor = xl
            for j, c in enumerate(ORDER):
                slot = ws[j] * W
                width = slot * values[j] * fill
                left = lerp(lefts[j], cursor, compact)
                if (1 - compact) * level > .02:
                    group.add(mn.Rectangle(width=max(slot, 1e-3), height=height, color=COLORS[c], stroke_width=1.4, stroke_opacity=.55 * (1 - compact) * level,
                                           fill_opacity=0).move_to([lefts[j] + slot / 2, y, 0]))
                if width > .004:
                    group.add(mn.Rectangle(width=width, height=height, stroke_width=0, fill_color=COLORS[c], fill_opacity=.8 * level).move_to([left + width / 2, y, 0]))
                if (1 - compact) * self.numbers.get_value() * level > .05:
                    shown = (1 - compact) * self.numbers.get_value() * level
                    group.add(_room.text([_room.Number(s.number(values[j], 2))], (lefts[j] + slot / 2, y - .26, 0), size=14, color=INK, width=max(slot + .2, .55)).set_opacity(shown))
                    if slot >= .6:
                        group.add(_room.text([_room.Number(s.number(raw(identifier, c), DIGITS[c]))], (lefts[j] + slot / 2, y - .46, 0), size=13, color=MUTED, width=slot + .2).set_opacity(shown))
                cursor += slot * values[j] * fill
            if self.scores.get_value() * level > .02:
                group.add(_room.place(_room.text([_room.Number(s.number(state[identifier]["score"], 2))], size=S1, color=INK), (xl + W - .1, y, 0), "right")
                          .set_opacity(self.scores.get_value() * level))
            if self.badges.get_value() * level > .02:
                rank = int(round(self.r[identifier].get_value())) + 1
                color = GOOD if rank == 1 else INK
                group.add(mn.Circle(radius=.15, color=color, stroke_width=2, stroke_opacity=self.badges.get_value() * level).move_to([-6.9, y, 0]),
                          _room.text([_room.Number(str(rank))], (-6.9, y, 0), size=S0, color=color).set_opacity(self.badges.get_value() * level))
        return group

    def mobjects(self):
        """🧩 Labels and the live table in one group."""
        return mn.VGroup(*self.labels.values(), self.live)

    def rank_to(self, ranks, run_time=2.0):
        """🔀 Move every row to its rank position."""
        return mn.AnimationGroup(*[self.r[i].animate(run_time=run_time).set_value(rank - 1) for i, rank in ranks.items()])

    def weights_to(self, weights, run_time=2.5):
        """🎚️ Tween the weights together."""
        return mn.AnimationGroup(*[self.w[c].animate(run_time=run_time).set_value(v) for c, v in zip(ORDER, weights)])
