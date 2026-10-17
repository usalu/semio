import numpy as np
from manim import *

from pathlib import Path as _Path
import sys as _sys

_TUTORIAL_ROOT = next(
    p for p in _Path(__file__).resolve().parents
    if (p / "manim_fonts.py").is_file()
)
if str(_TUTORIAL_ROOT) not in _sys.path:
    _sys.path.insert(0, str(_TUTORIAL_ROOT))

from manim_fonts import (
    apply_scene_style, scene_title, play_scene_title, TITLE_RUN_TIME,
    beat_subtitle, BEAT_SUBTITLE_FADE,
    BODY_FONT_SIZE, LABEL_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE,
    note_line, stacked_bar, dim_arrow,
    math_label, math_panel, de_num,
    caption_bar, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    room_section, radiator, person_glyph, ripples, droplets,
)

set_vo_language("de")
_VO_TIMING = _Path(__file__).resolve().parent / "vo_timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)

TITLE_DE = "Von der Nutzenergie zur Endenergie"

NUTZ_BLUE = "#38BDF8"
UEBER_RED = "#EF4444"
VERT_ORANGE = "#FB923C"
SPEI_YELLOW = "#FACC15"
ERZ_AMBER = "#F59E0B"
END_RED = "#DC2626"
AMBIENT_GREEN = "#22C55E"

# The one worked example the rest of the series keeps quoting: 100 kWh/(m²·a)
# of useful heat, delivered once by a gas boiler and once by a heat pump. Every
# number below is a schematic teaching example, not a measured or normed value.
Q_NUTZ = 100.0

# —— The Anlagenaufwandszahl chain (Beats 2–4) — one set of factors used
# everywhere, so the multiplication a viewer reproduces by hand always matches
# what is on screen. Erzeugung is kept separate for the plain and the
# condensing boiler, since Beat 3 shows both and they must not share a label.
E_UEBERGABE = 1.04
E_VERTEILUNG = 1.08
E_SPEICHERUNG = 1.06
E_ERZEUGUNG_GAS = 1.18            # Gaskessel: 118 kWh Erdgas → 100 kWh Wärme
E_ERZEUGUNG_GAS_BRENNWERT = 1.07  # nach Kondensation: 107 kWh → 100 kWh
E_ERZEUGUNG_WP = 0.26             # Wärmepumpe: 26 kWh Strom → 100 kWh Wärme

E_P_GAS = E_UEBERGABE * E_VERTEILUNG * E_SPEICHERUNG * E_ERZEUGUNG_GAS  # ≈ 1.40
E_P_WP = E_UEBERGABE * E_VERTEILUNG * E_SPEICHERUNG * E_ERZEUGUNG_WP    # ≈ 0.31
Q_E_GAS = Q_NUTZ * E_P_GAS  # ≈ 140 kWh/(m²·a)
Q_E_WP = Q_NUTZ * E_P_WP    # ≈ 31 kWh/(m²·a)


def _cascade(factors) -> tuple[float, ...]:
    """🪜 Energy each stage adds when the useful heat is multiplied up the chain."""
    added, level = [], Q_NUTZ
    for factor in factors:
        added.append(level * factor - level)
        level *= factor
    return tuple(added)


# Beat 1's staircase is the same multiplication Beat 4 shows as e_p, so its
# segments are derived from the factors rather than typed — typed values once
# summed to 135 while Beat 4 printed 140 for the very same boiler.
STAGES = tuple(zip(
    ("Übergabe", "Verteilung", "Speicherung", "Erzeugung"),
    _cascade((E_UEBERGABE, E_VERTEILUNG, E_SPEICHERUNG, E_ERZEUGUNG_GAS)),
    (UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER),
))  # ≈ +4, +8, +7, +21 kWh → Q_E_GAS

LOSS = tuple(value for _name, value, _colour in STAGES)
LEVELS = tuple(Q_NUTZ + sum(LOSS[:k]) for k in range(5))  # 100, 104, 112, 119, 140 kWh


#region DIN citation
def _din_ref(text: str):
    """📖 Standards citation for the beat, pinned to the empty top-right corner."""
    ref = body_text(text, font_size=LABEL_FONT_SIZE - 3, color=P_TEAL)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
#endregion


#region Shared visual motifs

FLOW_UNIT = 0.0050
LANE_PITCH = 0.042
PARTICLE_GAP = 0.5
PARTICLE_R = 0.034
_KAPPA = 0.5523
_UNIT_CIRCLE = np.array([
    [1, 0, 0], [1, _KAPPA, 0], [_KAPPA, 1, 0], [0, 1, 0], [0, 1, 0], [-_KAPPA, 1, 0], [-1, _KAPPA, 0], [-1, 0, 0],
    [-1, 0, 0], [-1, -_KAPPA, 0], [-_KAPPA, -1, 0], [0, -1, 0], [0, -1, 0], [_KAPPA, -1, 0], [1, -_KAPPA, 0], [1, 0, 0],
], dtype=float)
FLOW_PACE = 0.9
_FLOW_CLOCK = [0.0]

ROOM_C = np.array([-3.55, 0.97, 0.0])
ROOM_W, ROOM_H = 5.4, 1.6
GROUND_Y = -1.40
PIPE_Y = -0.58
RAD_X, TANK_X, BOILER_X, METER_X, PERSON_X = -5.45, -3.9, -1.9, 0.30, -1.10
WP_STROM = LEVELS[3] * E_ERZEUGUNG_WP
WP_UMWELT = LEVELS[3] - WP_STROM


def _rounded_polyline(points, radius: float = 0.7, per_corner: int = 9) -> np.ndarray:
    """📐 Dense polyline through ``points`` whose corners are rounded by quadratic Béziers."""
    pts = [np.array(p, dtype=float) for p in points]
    out = [pts[0]]
    for a, b, c in zip(pts, pts[1:], pts[2:]):
        u, v = a - b, c - b
        lu, lv = np.linalg.norm(u), np.linalg.norm(v)
        r = min(radius, 0.48 * lu, 0.48 * lv)
        p0, p2 = b + u / lu * r, b + v / lv * r
        out.extend((1 - t) ** 2 * p0 + 2 * (1 - t) * t * b + t ** 2 * p2 for t in np.linspace(0.0, 1.0, per_corner))
    out.append(pts[-1])
    return np.array(out)


def _resample(points: np.ndarray, n: int = 110):
    """📏 Points at equal arc length with unit normals (the left of the travel direction) and the path length."""
    seg = np.linalg.norm(np.diff(points, axis=0), axis=1)
    arc = np.concatenate([[0.0], np.cumsum(seg)])
    samples = np.linspace(0.0, arc[-1], n)
    pts = np.stack([np.interp(samples, arc, points[:, k]) for k in range(3)], axis=1)
    tangent = np.gradient(pts, axis=0)
    tangent /= np.linalg.norm(tangent, axis=1)[:, None]
    normals = np.stack([-tangent[:, 1], tangent[:, 0], np.zeros(n)], axis=1)
    return pts, normals, float(arc[-1])


class _Stream:
    """🌊 Energy stream — layered ribbon whose width per layer is kWh, carrying particles of the same width.

    ``layers`` are ``(tracker, colour, taper)``; a layer's width is its tracker value times ``FLOW_UNIT`` and a
    tapered layer shrinks linearly to nothing along the path. Layers stack on the left of the travel direction
    from offset ``start``, so every ribbon in a chain keeps one common base line.
    """

    def __init__(self, points, layers, *, start: float, cap: float, seed: int, radius: float = 0.7):
        self.pts, self.nrm, self.length = _resample(_rounded_polyline(points, radius))
        self.layers, self.start = layers, start
        self.grid = np.linspace(0.0, 1.0, len(self.pts))
        self.lanes = int(np.ceil(cap * FLOW_UNIT / LANE_PITCH)) + 1
        self.per = max(2, int(round(self.length / PARTICLE_GAP)))
        rng = np.random.default_rng(seed)
        self.phase = (np.arange(self.per)[None, :] + rng.uniform(0.0, 1.0, (self.lanes, 1))) / self.per
        self.offset = start + (np.repeat(np.arange(self.lanes), self.per) + 0.5) * LANE_PITCH
        self._seen = [None] * len(layers)
        self.ribbons = VGroup(*[VMobject(stroke_width=0) for _ in layers])
        for i, ribbon in enumerate(self.ribbons):
            ribbon.add_updater(lambda m, i=i: self._shape(i, m))
            self._shape(i, ribbon)

    def edges(self, s: np.ndarray) -> np.ndarray:
        """📊 Cumulative layer edges at path fractions ``s`` — row ``i`` is the lower edge of layer ``i``."""
        widths = [max(tr.get_value(), 0.0) * FLOW_UNIT * ((1.0 - s) if taper else np.ones_like(s))
                  for tr, _c, taper in self.layers]
        return self.start + np.vstack([np.zeros_like(s), np.cumsum(widths, axis=0)])

    def total(self) -> float:
        """🧮 Stream width in kWh at its start."""
        return float(sum(max(tr.get_value(), 0.0) for tr, _c, _t in self.layers))

    def colour(self, i: int):
        """🎨 Colour of layer ``i`` — layers may follow a tracker through a callable."""
        c = self.layers[i][1]
        return c() if callable(c) else c

    def _shape(self, i: int, mob) -> None:
        signature = (tuple(round(tr.get_value(), 3) for tr, _c, _t in self.layers), self.colour(i))
        if self._seen[i] == signature:
            return
        self._seen[i] = signature
        edges = self.edges(self.grid)
        lo, hi = edges[i], edges[i + 1]
        if float((hi - lo).max()) < 1e-3:
            mob.clear_points()
            return
        a = self.pts + self.nrm * lo[:, None]
        b = self.pts + self.nrm * hi[:, None]
        mob.set_points_as_corners(np.concatenate([a, b[::-1], a[:1]]))
        mob.set_fill(self.colour(i), opacity=0.45)
        mob.set_stroke(width=0)

    def advance(self, clock: float, holders) -> None:
        """💨 Place every particle for stream time ``clock``; lanes beyond the current width shrink to nothing."""
        t = ((self.phase + FLOW_PACE * clock / self.length) % 1.0).ravel()
        edges = self.edges(t)
        weight = np.clip((edges[-1] - self.offset) / LANE_PITCH + 0.5, 0.0, 1.0)
        layer = np.minimum((self.offset[None, :] >= edges[1:-1]).sum(axis=0), len(self.layers) - 1)
        f = t * (len(self.pts) - 1)
        i0 = np.minimum(f.astype(int), len(self.pts) - 2)
        u = (f - i0)[:, None]
        centre = self.pts[i0] * (1 - u) + self.pts[i0 + 1] * u
        normal = self.nrm[i0] * (1 - u) + self.nrm[i0 + 1] * u
        pos = centre + normal * self.offset[:, None]
        size = weight * np.clip(np.minimum(t / 0.05, (1.0 - t) / 0.07), 0.0, 1.0)
        for i, mob in enumerate(holders):
            pick = (layer == i) & (size > 0.05)
            if not pick.any():
                mob.clear_points()
                continue
            mob.set_points((pos[pick][:, None, :] + (PARTICLE_R * np.sqrt(size[pick]))[:, None, None]
                            * _UNIT_CIRCLE[None]).reshape(-1, 3))
            mob.set_fill(self.colour(i), opacity=0.95)
            mob.set_stroke(width=0)


def _flow(streams, rt: float):
    """💨 The particles of several ``_Stream``s as one animation; stream time runs on across plays so nothing jumps."""
    holders = [(stream, [VMobject(stroke_width=0) for _ in stream.layers]) for stream in streams]
    group = VGroup(*[mob for _stream, mobs in holders for mob in mobs])
    base = _FLOW_CLOCK[0]
    _FLOW_CLOCK[0] += rt

    def update(_mob, alpha):
        for stream, mobs in holders:
            stream.advance(base + alpha * rt, mobs)

    return UpdateFromAlphaFunc(group, update, remover=True, rate_func=linear)


def _readout(fn, at, *, size: float | None = None, color: str = P_WHITE, edge: str = "left"):
    """📟 Live typeset label that is rebuilt only when its text changes — ``math_readout`` redraws every frame."""
    holder = VGroup()
    state = {"text": None}

    def refresh(mob):
        text = fn()
        if text == state["text"]:
            return
        state["text"] = text
        place = np.array(at() if callable(at) else at, dtype=float)
        mob.remove(*list(mob.submobjects))
        if text is not None:
            mob.add(math_label(text, place, size=size, color=color, edge=edge))

    holder.add_updater(refresh)
    refresh(holder)
    return holder


def _still(mob):
    """🧊 Frozen copy of a ribbon's current shape — the piece that lifts off a stream and morphs into a bar."""
    frozen = VMobject(stroke_width=0)
    frozen.set_points(mob.points.copy())
    frozen.set_fill(mob.get_fill_color(), opacity=mob.get_fill_opacity())
    return frozen


def _stand(room, x: float, color=P_ORANGE):
    """🧍 Occupant at true scale — 70 % of the clear room height, feet on the floor slab's top edge."""
    height = 0.7 * room["h"]
    return person_glyph(np.array([x, room["y_f"] + height / 2, 0.0]), color=color, scale=height / 0.66)


def _flame(center, color=P_ORANGE):
    """🔥 Single burner flame."""
    c = np.array(center, dtype=float)
    pts = [(0.0, -0.20), (0.12, -0.07), (0.08, 0.09), (0.0, 0.26), (-0.08, 0.10), (-0.12, -0.06), (0.0, -0.20)]
    flame = VMobject(color=color, stroke_width=1.5)
    flame.set_points_smoothly([c + np.array([x, y, 0.0]) for x, y in pts])
    return flame.set_fill(color, opacity=0.85)


def _boiler(center, color=ERZ_AMBER, w: float = 0.80, h: float = 1.0):
    """♨️ Gas boiler in elevation — casing, burner plate and flame."""
    c = np.array(center, dtype=float)
    body = RoundedRectangle(corner_radius=0.12, width=w, height=h, color=P_WHITE, stroke_width=2.5).move_to(c)
    plate = Line(c + LEFT * 0.22 + DOWN * 0.26, c + RIGHT * 0.22 + DOWN * 0.26, color=P_WHITE, stroke_width=1.8)
    return VGroup(body, plate, _flame(c + UP * 0.02, color))


def _fan(center, color, radius: float = 0.26):
    """🌀 Fan ring with three blades."""
    c = np.array(center, dtype=float)
    ring = Circle(radius=radius, color=color, stroke_width=2.2).move_to(c)
    blades = VGroup(*[Line(c, c + radius * 0.78 * np.array([np.cos(a), np.sin(a), 0.0]), color=color, stroke_width=2.2)
                      for a in (PI / 2, PI / 2 + TAU / 3, PI / 2 + 2 * TAU / 3)])
    return VGroup(ring, blades)


def _heat_pump(center, color=AMBIENT_GREEN, w: float = 1.0, h: float = 1.0, flip: bool = False):
    """🌬️ Air-source heat pump — casing, fan on the outdoor side and a compressor ring; ``flip`` puts the outdoor side right."""
    c = np.array(center, dtype=float)
    d = -1.0 if flip else 1.0
    body = RoundedRectangle(corner_radius=0.12, width=w, height=h, color=P_WHITE, stroke_width=2.5).move_to(c)
    fan = _fan(c + np.array([-0.21 * d, 0.12, 0.0]), color)
    compressor = Circle(radius=0.12, color=P_YELLOW, stroke_width=2.2).move_to(c + np.array([0.25 * d, -0.22, 0.0]))
    fins = VGroup(*[Line(c + np.array([-0.42 * d, -(0.28 + 0.07 * k), 0.0]), c + np.array([-0.02 * d, -(0.28 + 0.07 * k), 0.0]),
                         color=color, stroke_width=1.2) for k in range(3)])
    return VGroup(body, fan, compressor, fins)


def _meter(center, kind: str = "gas", color=ERZ_AMBER):
    """🔢 Utility meter — dial face with a needle; the bolt marks the electricity meter."""
    c = np.array(center, dtype=float)
    face = Circle(radius=0.27, color=P_WHITE, stroke_width=2.4).move_to(c)
    ticks = VGroup(*[Line(c + 0.2 * np.array([np.cos(a), np.sin(a), 0.0]), c + 0.25 * np.array([np.cos(a), np.sin(a), 0.0]),
                          color=P_WHITE, stroke_width=1.4) for a in np.linspace(PI * 0.15, PI * 0.85, 5)])
    needle = Line(c + DOWN * 0.04, c + 0.17 * np.array([np.cos(PI * 0.62), np.sin(PI * 0.62), 0.0]), color=color,
                  stroke_width=2.6)
    glyph = VGroup(face, ticks, needle)
    if kind == "power":
        bolt = Polygon(*[c + np.array([x, y, 0.0]) for x, y in
                         [(0.02, -0.02), (-0.06, -0.15), (0.0, -0.15), (-0.03, -0.24), (0.07, -0.11), (0.01, -0.11)]],
                       color=color, stroke_width=1.2).set_fill(color, opacity=0.9)
        glyph.add(bolt)
    return glyph


def _tank(center, color=SPEI_YELLOW, w: float = 0.80, h: float = 1.0, level: float = 1.0):
    """🛢️ Storage tank whose translucent fill is the heat still stored — ``level`` is a 0–1 tracker."""
    c = np.array(center, dtype=float)
    frame = RoundedRectangle(corner_radius=0.14, width=w, height=h, color=P_WHITE, stroke_width=2.5).move_to(c)
    cap = RoundedRectangle(corner_radius=0.04, width=w * 0.4, height=0.1, color=P_WHITE, stroke_width=2.2,
                           fill_color=P_WHITE, fill_opacity=0.6).next_to(frame, UP, buff=0)
    tracker = ValueTracker(level)
    fill = Rectangle(width=w - 0.12, height=0.01, stroke_width=0, fill_color=color, fill_opacity=0.38)

    def shape(_m=None):
        height = max(0.01, (h - 0.12) * float(np.clip(tracker.get_value(), 0.0, 1.0)))
        fill.stretch_to_fit_height(height).move_to(c + DOWN * (h / 2 - 0.06) + UP * height / 2)

    shape()
    fill.add_updater(shape)
    return {"group": VGroup(frame, cap), "fill": fill, "level": tracker, "frame": frame}


def _chimney(x: float, y0: float, y1: float):
    """🏭 Flue stack outside the right wall."""
    return VGroup(Line(np.array([x - 0.13, y0, 0.0]), np.array([x - 0.13, y1, 0.0]), color=P_WHITE, stroke_width=2),
                  Line(np.array([x + 0.13, y0, 0.0]), np.array([x + 0.13, y1, 0.0]), color=P_WHITE, stroke_width=2))


def _floor_loop(x0: float, x1: float, y: float, color=P_CYAN):
    """➰ Underfloor heating loop embedded in the floor slab."""
    xs = np.linspace(x0, x1, 44)
    loop = VMobject(color=color, stroke_width=3.2)
    loop.set_points_smoothly([np.array([x, y + 0.045 * np.sin((x - x0) * 9.0), 0.0]) for x in xs])
    return loop


def _ratio_row(name: str, symbol: str, color: str, y: float, *, out: float, value, shown=lambda: True,
               x0: float = 0.75, length: float = 2.2, top: float = 1.45, name_w: float = 1.55,
               tick_label: bool = False):
    """📊 One factor gauge — track 0 … ``top``, a tick at 1 and a fill that follows ``value()``.

    Up to 1 the fill is the useful heat (blue, one unit comes out); whatever lies beyond 1 is the extra energy
    the stage swallows, in its own colour — so a factor above one reads as an overshoot and the heat pump's 0,26
    as a short bar. The readout counts ``in / out = factor`` from the live value, ``out`` being the kWh that leave
    the stage.
    """
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=color).move_to(np.array([x0, y, 0.0]), aligned_edge=LEFT)
    sym = math_label(symbol, np.array([x0 + name_w, y, 0.0]), size=BODY_FONT_SIZE, color=color, edge="left")
    left = x0 + name_w + 0.58
    unit = length / top
    track = Rectangle(width=length, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK,
                      fill_opacity=1.0).move_to(np.array([left + length / 2, y, 0.0]))
    base = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=NUTZ_BLUE, fill_opacity=0.9)
    over = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=color, fill_opacity=0.95)

    def shape(_m=None):
        e = value()
        b, o = min(e, 1.0) * unit, max(e - 1.0, 0.0) * unit
        base.stretch_to_fit_width(max(b, 1e-3)).move_to(np.array([left + b / 2, y, 0.0]))
        over.stretch_to_fit_width(max(o, 1e-3)).move_to(np.array([left + unit + o / 2, y, 0.0]))
        over.set_fill(opacity=0.95 if o > 1e-3 else 0.0)

    shape()
    base.add_updater(shape)
    tick = Line(np.array([left + unit, y - 0.17, 0.0]), np.array([left + unit, y + 0.17, 0.0]), color=P_WHITE,
                stroke_width=1.8)
    group = VGroup(label, sym, track, base, over, tick)
    if tick_label:
        group.add(math_label("1", np.array([left + unit, y + 0.33, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE))
    readout = _readout(
        lambda: (rf"=\frac{{{de_num(round(value() * out))}}}{{{de_num(round(out))}}}={de_num(value(), 2)}"
                 if shown() else None),
        np.array([left + length + 0.2, y, 0.0]), size=LABEL_FONT_SIZE, color=color, edge="left")
    return {"group": group, "label": label, "symbol": sym, "track": track, "readout": readout, "y": y,
            "color": color, "tick_x": left + unit, "value": value}


def _stage_row(name: str, symbol: str, color: str, y: float, loss, out: float, **kwargs):
    """🎚️ ``_ratio_row`` for one stage: the ``unit`` tracker lifts the useful unit to 1, the stage's loss tracker adds the rest."""
    unit = ValueTracker(0.0)
    row = _ratio_row(name, symbol, color, y, out=out, value=lambda: unit.get_value() + loss.get_value() / out,
                     shown=lambda: unit.get_value() > 0.999 and loss.get_value() > 0.05, **kwargs)
    row["unit"] = unit
    return row


def _product_row(name: str, symbol: str, y: float, factors, *, lift=lambda: 1.0, out: float = Q_NUTZ,
                 x0: float = 0.75, length: float = 2.2, top: float = 1.45, name_w: float = 1.55):
    """✖️ Gauge of the product of several stage factors — each stage's share of the overshoot stacks in its own colour.

    ``factors`` is a list of ``(value(), colour)``; ``lift()`` scales the useful unit from 0 to 1 first. The bar's
    end is the running product, so a factor below one cuts the stack back down and a product below one leaves only
    the blue useful part.
    """
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=P_WHITE).move_to(np.array([x0, y, 0.0]), aligned_edge=LEFT)
    sym = math_label(symbol, np.array([x0 + name_w, y, 0.0]), size=BODY_FONT_SIZE, color=P_WHITE, edge="left")
    left = x0 + name_w + 0.58
    unit = length / top
    track = Rectangle(width=length, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK,
                      fill_opacity=1.0).move_to(np.array([left + length / 2, y, 0.0]))
    base = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=NUTZ_BLUE, fill_opacity=0.9)
    parts = VGroup(*[Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=c, fill_opacity=0.95)
                     for _v, c in factors])

    def product() -> float:
        return float(lift() * np.prod([f() for f, _c in factors]))

    def shape(_m=None):
        b = min(product(), 1.0) * unit
        base.stretch_to_fit_width(max(b, 1e-3)).move_to(np.array([left + b / 2, y, 0.0]))
        level = lift()
        for part, (f, _c) in zip(parts, factors):
            nxt = level * f()
            lo, hi = max(1.0, min(level, nxt)), max(1.0, max(level, nxt))
            width = (hi - lo) * unit
            part.stretch_to_fit_width(max(width, 1e-3)).move_to(np.array([left + unit * (lo + hi) / 2, y, 0.0]))
            part.set_fill(opacity=0.95 if width > 1e-3 else 0.0)
            level = nxt

    shape()
    base.add_updater(shape)
    tick = Line(np.array([left + unit, y - 0.17, 0.0]), np.array([left + unit, y + 0.17, 0.0]), color=P_WHITE,
                stroke_width=1.8)
    readout = _readout(
        lambda: rf"=\frac{{{de_num(round(product() * out))}}}{{{de_num(round(out))}}}={de_num(product(), 2)}",
        np.array([left + length + 0.2, y, 0.0]), size=LABEL_FONT_SIZE, color=P_WHITE, edge="left")
    return {"group": VGroup(label, sym, track, base, parts, tick), "label": label, "symbol": sym, "readout": readout,
            "product": product, "y": y, "tick_x": left + unit}


def _swap(scene, old, text_de: str, live, run_time: float = 0.35):
    """💬 ``swap_caption`` that keeps particles and heat ripples running through the two fades."""
    new = caption_bar(text_de)
    half = run_time / 2
    scene.play(FadeOut(old), *live(half), run_time=half)
    scene.play(FadeIn(new), *live(half), run_time=half)
    scene._vo_caption_t0 = float(getattr(scene.renderer, "time", 0.0) or 0.0)
    return new


def _plant(*, with_person: bool = True):
    """🏠 The building section every beat of the chapter draws — room over cellar, plant parts and their streams.

    The heat path runs gas meter → boiler → tank → distribution → radiator → room; each segment is a
    ``_Stream`` whose layers are the useful heat plus every loss still to come, so the ribbon widens by
    exactly one stage's loss at each node walking upstream.
    """
    room = room_section(ROOM_C, w=ROOM_W, h=ROOM_H)
    style = dict(color=P_WHITE, stroke_width=2, fill_color=P_WHITE, fill_opacity=0.14)
    under_slab = room["y_f"] - room["slab"]
    cellar_h = under_slab - GROUND_Y
    cellar_walls = VGroup(*[Rectangle(width=0.2, height=cellar_h, **style).move_to(
        np.array([x, under_slab - cellar_h / 2, 0.0])) for x in (room["x_l"] - 0.1, room["x_r"] + 0.1)])
    cellar_air = Rectangle(width=ROOM_W, height=cellar_h, stroke_width=0, fill_color=P_BLUE, fill_opacity=0.05).move_to(
        np.array([ROOM_C[0], under_slab - cellar_h / 2, 0.0]))
    ground = Line(np.array([room["x_l"] - 0.5, GROUND_Y, 0.0]), np.array([1.3, GROUND_Y, 0.0]), color=P_TEAL,
                  stroke_width=2.2)
    hatches = VGroup(*[Line(ground.point_from_proportion(u), ground.point_from_proportion(u) + np.array([-0.07, -0.1, 0.0]),
                            color=P_TEAL, stroke_width=1.1) for u in np.linspace(0.03, 0.97, 24)])
    shell = VGroup(room["group"], cellar_air, cellar_walls, ground, hatches)

    rad_c = np.array([RAD_X, room["y_f"] + 0.285, 0.0])
    rad = radiator(rad_c, color=UEBER_RED)
    person = _stand(room, PERSON_X) if with_person else None
    tank = _tank(np.array([TANK_X, PIPE_Y - 0.14, 0.0]))
    boiler = _boiler(np.array([BOILER_X, PIPE_Y - 0.14, 0.0]))
    meter = _meter(np.array([METER_X, PIPE_Y, 0.0]))
    chimney = _chimney(-0.30, -0.34, 1.55)

    v = {name: ValueTracker(0.0) for name in
         ("z0n", "z2n", "z2u", "z2v", "z3n", "z3u", "z3v", "z3s", "z4n", "z4u", "z4v", "z4s", "z4e", "z4w", "flue", "env")}
    target = {"z0n": Q_NUTZ, "z2n": Q_NUTZ, "z2u": LOSS[0], "z2v": LOSS[1], "z3n": Q_NUTZ, "z3u": LOSS[0],
              "z3v": LOSS[1], "z3s": LOSS[2], "z4n": Q_NUTZ, "z4u": LOSS[0], "z4v": LOSS[1], "z4s": LOSS[2],
              "z4e": LOSS[3], "z4w": WP_STROM, "flue": LOSS[3], "env": WP_UMWELT}
    half = 0.5 * Q_NUTZ * FLOW_UNIT
    cols = (NUTZ_BLUE, UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER)
    rad_out = rad_c + RIGHT * 0.50
    z0 = _Stream([rad_out, np.array([-4.2, rad_out[1], 0.0]), np.array([-3.1, rad_out[1], 0.0])],
                 [(v["z0n"], cols[0], False)], start=-half, cap=Q_NUTZ, seed=1)
    z2 = _Stream([np.array([TANK_X - 0.40, PIPE_Y, 0.0]), np.array([RAD_X, PIPE_Y, 0.0]),
                  np.array([RAD_X, room["y_f"] + 0.07, 0.0])],
                 [(v["z2n"], cols[0], False), (v["z2u"], cols[1], False), (v["z2v"], cols[2], True)],
                 start=-half, cap=LEVELS[2], seed=2)
    z3 = _Stream([np.array([BOILER_X - 0.40, PIPE_Y, 0.0]), np.array([TANK_X + 0.40, PIPE_Y, 0.0])],
                 [(v["z3n"], cols[0], False), (v["z3u"], cols[1], False), (v["z3v"], cols[2], False),
                  (v["z3s"], cols[3], False)], start=-half, cap=LEVELS[3], seed=3)
    z4 = _Stream([np.array([METER_X - 0.27, PIPE_Y, 0.0]), np.array([BOILER_X + 0.40, PIPE_Y, 0.0])],
                 [(v["z4n"], cols[0], False), (v["z4u"], cols[1], False), (v["z4v"], cols[2], False),
                  (v["z4s"], cols[3], False), (v["z4e"], cols[4], False), (v["z4w"], P_YELLOW, False)],
                 start=-half, cap=LEVELS[4], seed=4)
    flue_y = under_slab - 0.12
    flue = _Stream([np.array([BOILER_X + 0.15, PIPE_Y - 0.14 + 0.46, 0.0]), np.array([BOILER_X + 0.15, flue_y, 0.0]),
                    np.array([-0.30, flue_y, 0.0]), np.array([-0.30, 1.50, 0.0])],
                   [(v["flue"], P_RED, False)], start=-0.5 * LOSS[3] * FLOW_UNIT, cap=LOSS[3], seed=5, radius=0.3)

    env = _Stream([np.array([room["x_r"] + 0.30, PIPE_Y - 0.27, 0.0]), np.array([BOILER_X + 0.40, PIPE_Y - 0.27, 0.0])],
                  [(v["env"], AMBIENT_GREEN, False)], start=-0.5 * WP_UMWELT * FLOW_UNIT, cap=WP_UMWELT, seed=6)
    heat_pump = _heat_pump(np.array([BOILER_X, PIPE_Y - 0.14, 0.0]), flip=True)
    power_meter = _meter(np.array([METER_X, PIPE_Y, 0.0]), kind="power", color=P_YELLOW)

    def heat(name: str, rt: float, r_max: float = 0.4):
        """🌡️ Ripples for the named warm place, cycling for ``rt`` seconds."""
        cyc = max(1.0, rt / 1.4)
        top = rad_c + UP * 0.30
        spots = {
            "rad": ripples([top], r_max=r_max, color=P_ORANGE, cycles=cyc),
            "ue": ripples([rad_c + LEFT * 0.47], facing=PI, r_max=0.32, color=UEBER_RED, cycles=cyc),
            "ve": ripples([np.array([-4.75, PIPE_Y - 0.43, 0.0])], down=True, r_max=0.30, color=VERT_ORANGE, cycles=cyc),
            "gain": ripples([np.array([RAD_X + 0.42, room["y_f"] + 0.02, 0.0])], facing=PI / 2, r_max=0.22,
                            color=VERT_ORANGE, cycles=cyc),
            "sp": ripples([np.array([TANK_X, PIPE_Y - 0.14 + 0.5, 0.0])], r_max=0.30, color=SPEI_YELLOW, cycles=cyc),
        }
        if person is not None:
            spots["person"] = ripples([person.get_top() + UP * 0.05], r_max=0.36, color=P_ORANGE, cycles=cyc)
        return spots[name]

    return {"room": room, "shell": shell, "radiator": rad, "person": person, "tank": tank, "boiler": boiler,
            "meter": meter, "chimney": chimney, "v": v, "target": target, "z0": z0, "z2": z2, "z3": z3, "z4": z4,
            "flue": flue, "env": env, "heat_pump": heat_pump, "power_meter": power_meter,
            "streams": (z0, z2, z3, z4, flue, env), "heat": heat, "rad_c": rad_c, "under_slab": under_slab}


def _grow(plant, names, run_time: float = 1.4, rate=smooth):
    """📈 Animations that swell the named stream layers from zero to their kWh values together."""
    return [plant["v"][n].animate(run_time=run_time, rate_func=rate).set_value(plant["target"][n]) for n in names]
#endregion


#region Beat 1 – Die Verlustkette vom Raum zum Zähler

class Beat1_Verlustkette(Scene):
    NARRATION = [
        ("intro",
         "So far every kilowatt-hour has been heat inside the room. Nobody sells that — it has to be produced.",
         "Bisher war jede Kilowattstunde Wärme im Raum. Die verkauft niemand — sie muss erst erzeugt werden."),
        ("nutz",
         "Start from the demand we calculated: a hundred kilowatt-hours per square metre and year of useful heat.",
         "Wir starten beim berechneten Bedarf: hundert Kilowattstunden je Quadratmeter und Jahr Nutzenergie."),
        ("stufen",
         "Between that room and the gas meter sit four stages, each needing extra energy — how it is counted depends on where it happens.",
         "Zwischen Raum und Gaszähler liegen vier Stufen, jede braucht zusätzliche Energie — wie sie zählt, hängt vom Ort des Verlusts ab."),
        ("ende",
         "The energy billed at the building boundary is counted as final energy — here roughly forty percent more than the room ever received.",
         "Die am Gebäude bezogene Energie wird als Endenergie bilanziert — hier rund vierzig Prozent mehr, als der Raum je bekommen hat."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        play_scene_title(self, title)
        subtitle = beat_subtitle("Die Verlustkette vom Raum zum Zähler", title)
        din = _din_ref("DIN V 18599-5:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        revision_note = note_line(
            "Seit 2025 als DIN/TS 18599-5:2025-10 fortgeschrieben — GEG nutzt weiterhin 2018",
            color=P_TEAL,
        )
        self.play(FadeIn(caption), FadeIn(revision_note), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        active: list = []

        def live(rt, *heat):
            return [_flow(active, rt), *[P["heat"](n, rt) for n in heat]]

        q_h = _readout(lambda: rf"Q_{{h}}={de_num(v['z0n'].get_value())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                           np.array([ROOM_C[0], 1.42, 0.0]), size=BODY_FONT_SIZE, color=NUTZ_BLUE, edge="center")
        q_e = _readout(lambda: rf"Q_{{E}}={de_num(P['z4'].total())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                           np.array([0.78, -1.10, 0.0]), size=BODY_FONT_SIZE, color=END_RED, edge="left")
        gas_tag = body_text("Gaszähler", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(
            np.array([METER_X + 0.36, -0.22, 0.0]))

        # —— Room first: the heat everybody already knows ——
        self.play(Create(P["shell"]), run_time=1.4)
        self.play(FadeIn(P["radiator"]), FadeIn(P["person"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(1.0, "rad", "person"), run_time=1.0)
        hold_for(self, self.NARRATION, "intro", used=TITLE_RUN_TIME + BEAT_SUBTITLE_FADE + 0.3 + 1.4 + 1.0,
                 during=lambda rt: live(rt, "rad", "person"))

        # —— The useful heat as the first and slimmest stream ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "nutz"), lambda rt: live(rt, "rad", "person"))
        self.add(P["z0"].ribbons, q_h)
        active.append(P["z0"])
        self.play(FadeOut(revision_note), *_grow(P, ["z0n"], 1.6), *live(1.6, "rad", "person"), run_time=1.6)
        hold_for(self, self.NARRATION, "nutz", during=lambda rt: live(rt, "rad", "person"))

        # —— Four stages, walked upstream: every node widens the stream by its loss ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "stufen"), lambda rt: live(rt, "rad", "person"))
        bar_x, bar_base, unit = 4.9, -1.2, 0.0165
        bar = stacked_bar([("Nutzenergie", Q_NUTZ, NUTZ_BLUE)] + [(n, val, c) for n, val, c in STAGES],
                          base=np.array([bar_x, bar_base, 0.0]), width=1.0, unit=unit)
        legend = VGroup()
        leaders = VGroup()
        rows = {0: -0.35, 1: 0.35, 2: 0.78, 3: 1.21, 4: 1.64}

        def legend_row(k, text, colour):
            label = body_text(text, font_size=LABEL_FONT_SIZE - 2, color=colour)
            label.move_to(np.array([bar_x - 0.78, rows[k], 0.0]), aligned_edge=RIGHT)
            leader = Line(label.get_right() + RIGHT * 0.08, np.array([bar_x - 0.5, bar["anchors"][k][1], 0.0]),
                          color=colour, stroke_width=1.4, stroke_opacity=0.6)
            legend.add(label)
            leaders.add(leader)
            return label, leader

        label, leader = legend_row(0, f"Nutzenergie  {round(Q_NUTZ)} kWh", NUTZ_BLUE)
        self.play(GrowFromEdge(bar["bars"][0], DOWN), FadeIn(label), Create(leader), *live(1.0, "rad", "person"),
                  run_time=1.0)

        def stage(k, name, colour, hit, grow, heat_names, extra=(), still_of=None, run=1.8):
            label, leader = legend_row(k + 1, f"{name}  + {round(LOSS[k])} kWh", colour)
            self.play(Circumscribe(hit, color=colour, buff=0.1, stroke_width=3, fade_out=True), *grow, *extra, *live(run, *heat_names),
                      run_time=run)
            source = _still(still_of)
            self.add(source)
            self.play(ReplacementTransform(source, bar["bars"][k + 1]), FadeIn(label), Create(leader),
                      *live(1.0, *heat_names), run_time=1.0)

        self.add(P["z2"].ribbons)
        active.append(P["z2"])
        stage(0, "Übergabe", UEBER_RED, P["radiator"], _grow(P, ["z2n", "z2u"]), ("rad", "person", "ue"),
              still_of=P["z2"].ribbons[1])
        stage(1, "Verteilung", VERT_ORANGE, P["z2"].ribbons, _grow(P, ["z2v"]), ("rad", "person", "ue", "ve"),
              still_of=P["z2"].ribbons[2])
        self.add(P["z3"].ribbons, P["tank"]["fill"])
        active.append(P["z3"])
        stage(2, "Speicherung", SPEI_YELLOW, P["tank"]["group"],
              [FadeIn(P["tank"]["group"]), *_grow(P, ["z3n", "z3u", "z3v", "z3s"])],
              ("rad", "person", "ue", "ve", "sp"), still_of=P["z3"].ribbons[3])
        self.add(P["z4"].ribbons, P["flue"].ribbons, q_e)
        active.extend([P["z4"], P["flue"]])
        stage(3, "Erzeugung", ERZ_AMBER, P["boiler"],
              [FadeIn(P["boiler"]), FadeIn(P["meter"]), FadeIn(P["chimney"]), FadeIn(gas_tag),
               *_grow(P, ["z4n", "z4u", "z4v", "z4s", "z4e", "flue"])],
              ("rad", "person", "ue", "ve", "sp"), still_of=P["flue"].ribbons[0])
        hold_for(self, self.NARRATION, "stufen", during=lambda rt: live(rt, "rad", "person", "ue", "ve", "sp"))

        # —— Final energy: the stream at the meter against the stream in the room ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "ende"),
                        lambda rt: live(rt, "rad", "person", "ue", "ve", "sp"))
        top_n = bar["base"] + UP * Q_NUTZ * unit
        extra_arrow = dim_arrow(top_n + RIGHT * 0.75, bar["top"] + RIGHT * 0.75, color=END_RED)
        extra_label = math_label(rf"\approx +{de_num((Q_E_GAS / Q_NUTZ - 1) * 100)}\,\%",
                                 np.array([bar_x + 0.95, (top_n[1] + bar["top"][1]) / 2, 0.0]),
                                 size=BODY_FONT_SIZE, color=END_RED, edge="left")
        self.play(Create(extra_arrow), FadeIn(extra_label), Indicate(P["meter"], color=END_RED), *live(1.4, "rad", "person",
                                                                                             "ue", "ve", "sp"),
                  run_time=1.4)
        hold_for(self, self.NARRATION, "ende", during=lambda rt: live(rt, "rad", "person", "ue", "ve", "sp"))

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 2 – Übergabe, Verteilung, Speicherung

class Beat2_UebergabeVerteilungSpeicherung(Scene):
    NARRATION = [
        ("intro",
         "Walk the chain backwards through a real building, starting in the room itself.",
         "Gehen wir die Kette rückwärts durch ein echtes Gebäude — beginnend im Raum selbst."),
        ("uebergabe",
         "Emission: how the surface hands heat to the air. A radiator needs hot water, underfloor heating needs far less.",
         "Übergabe: wie die Fläche Wärme an die Luft abgibt. Ein Heizkörper braucht heißes Wasser, eine Fußbodenheizung viel weniger."),
        ("verteilung",
         "Distribution: every pipe metre loses heat. Outside the envelope it is lost for good — inside it partly comes back as a gain.",
         "Verteilung: Jeder Rohrmeter verliert Wärme. Außerhalb der Hülle ist sie weg — innerhalb kommt sie teilweise als Gewinn zurück."),
        ("speicher",
         "Storage: a tank kept warm around the clock loses heat even without any draw-off, for as long as a whole week.",
         "Speicherung: Ein warm gehaltener Speicher verliert auch ohne Zapfung Wärme — selbst eine ganze Woche lang."),
        ("kennzahl",
         "Each stage gets one expense figure: how much has to go in for one unit to come out.",
         "Jede Stufe bekommt eine Aufwandszahl: wie viel hinein muss, damit eine Einheit herauskommt."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Übergabe, Verteilung, Speicherung", title)
        din = _din_ref("DIN V 18599-5:2018-09 · -6")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        active: list = []

        def live(rt, *heat):
            return [_flow(active, rt), *[P["heat"](n, rt) for n in heat]]

        envelope = DashedVMobject(
            Rectangle(width=ROOM_W + 0.64, height=ROOM_H + 0.26 + 0.12 + 0.26, color=P_TEAL, stroke_width=2.4).move_to(
                np.array([ROOM_C[0], P["under_slab"] + (ROOM_H + 0.64) / 2, 0.0])), num_dashes=70)
        shell_tag = body_text("beheizte Hülle", font_size=LABEL_FONT_SIZE - 3, color=P_TEAL).move_to(
            np.array([0.45, 1.80, 0.0]), aligned_edge=LEFT)
        cellar_tag = body_text("unbeheizter Keller", font_size=LABEL_FONT_SIZE - 3, color=P_WHITE).move_to(
            np.array([ROOM_C[0], GROUND_Y - 0.30, 0.0]))
        cellar_tag.set_opacity(0.7)

        rows = {
            "ue": _stage_row("Übergabe", r"e_{ü}", UEBER_RED, 1.35, v["z2u"], LEVELS[0], tick_label=True),
            "ve": _stage_row("Verteilung", r"e_{d}", VERT_ORANGE, 0.55, v["z2v"], LEVELS[1]),
            "sp": _stage_row("Speicherung", r"e_{s}", SPEI_YELLOW, -0.35, v["z3s"], LEVELS[2]),
        }

        floor_heat = lambda rt: ripples([np.array([x, room["y_f"] + 0.02, 0.0]) for x in (-2.7, -2.35, -2.0)],
                                        r_max=0.30, color=P_CYAN, cycles=max(1.0, rt / 1.4))

        def reveal(name, rt):
            row = rows[name]
            return [row["unit"].animate.set_value(1.0)]

        # —— The building, heated storey over the unheated cellar ——
        self.play(Create(P["shell"]), Create(envelope), FadeIn(shell_tag), FadeIn(cellar_tag), run_time=1.5)
        self.play(FadeIn(P["person"]), FadeIn(P["radiator"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(0.8, "person"), run_time=0.8)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.5 + 0.8,
                 during=lambda rt: live(rt, "person"))

        # —— Emission: the radiator hands one unit to the room and takes 1,04 in ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "uebergabe"), lambda rt: live(rt, "person"))
        rad_tag = math_label(r"\text{Heizkörper}\,\cdot\,55\,\mathrm{°C}", np.array([-4.95, 1.52, 0.0]),
                             size=LABEL_FONT_SIZE, color=UEBER_RED)
        loop = _floor_loop(-2.95, -1.75, room["y_f"] - 0.07)
        loop_tag = math_label(r"\text{Fußbodenheizung}\,\cdot\,35\,\mathrm{°C}", np.array([-3.00, 1.10, 0.0]),
                              size=LABEL_FONT_SIZE, color=P_CYAN)
        flow_note = note_line("niedrige Vorlauftemperatur → bessere Erzeugereffizienz", color=P_CYAN)
        self.add(P["z0"].ribbons, P["z2"].ribbons, rows["ue"]["readout"], rows["ve"]["readout"], rows["sp"]["readout"])
        active.extend([P["z0"], P["z2"]])
        self.play(FadeIn(rows["ue"]["group"]), FadeIn(rad_tag), *_grow(P, ["z0n"], 1.2), *reveal("ue", 1.2),
                  *live(1.2, "person", "rad"), run_time=1.2)
        self.play(Circumscribe(P["radiator"], color=UEBER_RED, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z2n", "z2u"], 1.6), *live(1.6, "person", "rad", "ue"), run_time=1.6)
        self.play(Create(loop), FadeIn(loop_tag), FadeIn(flow_note), *live(1.2, "person", "rad", "ue"), floor_heat(1.2),
                  run_time=1.2)
        hold_for(self, self.NARRATION, "uebergabe",
                 during=lambda rt: [*live(rt, "person", "rad", "ue"), floor_heat(rt)])

        # —— Distribution: the pipe narrows by 8 kWh on its way up ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "verteilung"),
                        lambda rt: [*live(rt, "person", "rad", "ue"), floor_heat(rt)])
        inside_note = note_line("Verluste innerhalb der Hülle werden als Gewinn gutgeschrieben", color=VERT_ORANGE)
        self.play(FadeOut(flow_note), FadeOut(rad_tag), FadeOut(loop_tag), FadeOut(loop), run_time=0.3)
        self.play(FadeIn(rows["ve"]["group"]), FadeIn(inside_note), *reveal("ve", 1.0), *live(1.0, "person", "rad", "ue"),
                  run_time=1.0)
        self.play(Circumscribe(P["z2"].ribbons, color=VERT_ORANGE, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z2v"], 2.0), *live(2.0, "person", "rad", "ue", "ve", "gain"), run_time=2.0)
        hold_for(self, self.NARRATION, "verteilung",
                 during=lambda rt: live(rt, "person", "rad", "ue", "ve", "gain"))

        # —— Storage: the tank drains while it waits ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "speicher"),
                        lambda rt: live(rt, "person", "rad", "ue", "ve", "gain"))
        tank_note = body_text("7 Tage ohne Zapfung", font_size=LABEL_FONT_SIZE - 3, color=SPEI_YELLOW).move_to(
            np.array([TANK_X, GROUND_Y - 0.62, 0.0]))
        self.play(FadeOut(inside_note), FadeOut(cellar_tag), run_time=0.3)
        self.add(P["z3"].ribbons, P["tank"]["fill"])
        active.append(P["z3"])
        self.play(FadeIn(rows["sp"]["group"]), FadeIn(P["tank"]["group"]), FadeIn(P["boiler"]), FadeIn(tank_note),
                  *reveal("sp", 1.0), *live(1.0, "person", "rad", "ue", "ve"), run_time=1.0)
        self.play(Circumscribe(P["tank"]["group"], color=SPEI_YELLOW, buff=0.1, stroke_width=3, fade_out=True),
                  *_grow(P, ["z3n", "z3u", "z3v", "z3s"], 1.8), *live(1.8, "person", "rad", "ue", "ve", "sp"),
                  run_time=1.8)
        hold_for(self, self.NARRATION, "speicher",
                 during=lambda rt: [*live(rt, "person", "rad", "ue", "ve", "sp"),
                                    P["tank"]["level"].animate.set_value(0.72)])

        # —— One expense figure per stage ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "kennzahl"),
                        lambda rt: live(rt, "person", "rad", "ue", "ve", "sp"))
        gt_note = note_line("alle Aufwandszahlen > 1", color=P_TEAL)
        eq, eq_box, items = math_panel([
            ("eu", r"e_{ü}\approx 1{,}04", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"e_{d}\approx 1{,}08", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"e_{s}\approx 1{,}06", SPEI_YELLOW),
        ])
        self.play(FadeOut(tank_note), FadeIn(gt_note), run_time=0.3)
        self.play(Create(eq_box), *[ReplacementTransform(rows[k]["symbol"].copy(), items[i])
                                    for k, i in (("ue", "eu"), ("ve", "ed"), ("sp", "es"))],
                  FadeIn(VGroup(*[m for m in eq if all(m is not items[i] for i in ("eu", "ed", "es"))])),
                  *live(1.4, "person", "rad", "ue", "ve", "sp"), run_time=1.4)
        hold_for(self, self.NARRATION, "kennzahl",
                 during=lambda rt: live(rt, "person", "rad", "ue", "ve", "sp"))

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 3 – Erzeugung: Wirkungsgrad oder Arbeitszahl

class Beat3_Erzeugung(Scene):
    NARRATION = [
        ("intro",
         "The last stage is the biggest — and this is where a combustion boiler and a heat pump differ fundamentally.",
         "Die letzte Stufe ist die größte — hier unterscheiden sich Verbrennungskessel und Wärmepumpe grundlegend."),
        ("kessel",
         "A boiler burns fuel. Part of the heat leaves through the flue, so more always goes in than comes out.",
         "Ein Kessel verbrennt Brennstoff. Ein Teil der Wärme geht durch den Schornstein — es muss immer mehr hinein als heraus."),
        ("brennwert",
         "A condensing boiler cools the flue gas until its water vapour condenses — the latent heat from the fundamentals video becomes usable.",
         "Ein Brennwertkessel kühlt das Abgas, bis Wasserdampf darin kondensiert — die latente Wärme aus dem Grundlagen-Video wird nutzbar."),
        ("waermepumpe",
         "A heat pump burns nothing. It lifts ambient heat, so three quarters of what it delivers is not purchased final energy.",
         "Eine Wärmepumpe verbrennt nichts. Sie hebt Umweltwärme an — drei Viertel der Lieferung sind keine zugekaufte Endenergie."),
        ("jaz",
         "Its seasonal performance factor is the ratio of heat delivered over the year to the electricity that drove it: here a hundred to twenty-six.",
         "Ihre Jahresarbeitszahl ist das Verhältnis von Wärme zu eingesetztem Strom im Jahr: hier hundert zu sechsundzwanzig."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Erzeugung: Wirkungsgrad oder Arbeitszahl", title)
        din = _din_ref("DIN V 18599-5 · VDI 4650-1")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        t = {k: ValueTracker(0.0) for k in ("heat", "gas", "loss", "recov", "el", "env", "out_env", "out_el", "split")}
        Y, half = 0.55, 0.5 * Q_NUTZ * FLOW_UNIT
        boiler = _boiler(np.array([-3.2, Y, 0.0]), w=0.9, h=1.2)
        pump = _heat_pump(np.array([3.7, Y, 0.0]), w=1.0, h=1.2)
        divider = DashedLine(np.array([0.0, -1.0, 0.0]), np.array([0.0, 2.05, 0.0]), color=P_WHITE, stroke_width=1.6,
                             stroke_opacity=0.3)
        out_env = lambda: interpolate_color(ManimColor(NUTZ_BLUE), ManimColor(AMBIENT_GREEN), t["split"].get_value())
        out_el = lambda: interpolate_color(ManimColor(NUTZ_BLUE), ManimColor(P_YELLOW), t["split"].get_value())

        gas_in = _Stream([np.array([-6.3, Y, 0.0]), np.array([-3.65, Y, 0.0])],
                         [(t["gas"], ERZ_AMBER, False), (t["loss"], P_RED, False)], start=-half, cap=118, seed=11)
        heat_l = _Stream([np.array([-2.75, Y, 0.0]), np.array([-0.60, Y, 0.0])], [(t["heat"], NUTZ_BLUE, False)],
                         start=-half, cap=100, seed=12)
        flue = _Stream([np.array([-3.40, Y + 0.58, 0.0]), np.array([-3.40, 1.95, 0.0])], [(t["loss"], P_RED, False)],
                       start=-0.5 * 18 * FLOW_UNIT, cap=18, seed=13)
        recov = _Stream([np.array([-3.40, 1.62, 0.0]), np.array([-2.85, 1.62, 0.0]), np.array([-2.85, Y + 0.58, 0.0])],
                        [(t["recov"], P_CYAN, False)], start=-0.5 * 11 * FLOW_UNIT, cap=11, seed=14, radius=0.25)
        env_in = _Stream([np.array([0.9, Y - 0.25, 0.0]), np.array([3.15, Y - 0.25, 0.0])],
                         [(t["env"], AMBIENT_GREEN, False)], start=-0.5 * 74 * FLOW_UNIT, cap=74, seed=15)
        el_in = _Stream([np.array([0.9, Y + 0.42, 0.0]), np.array([3.15, Y + 0.42, 0.0])],
                        [(t["el"], P_YELLOW, False)], start=-0.5 * 26 * FLOW_UNIT, cap=26, seed=16)
        hp_out = _Stream([np.array([4.25, Y, 0.0]), np.array([6.0, Y, 0.0])],
                         [(t["out_env"], out_env, False), (t["out_el"], out_el, False)], start=-half, cap=100, seed=17)
        active: list = []

        def live(rt):
            fan = Rotate(pump[1][1], angle=-TAU * rt / 0.9, about_point=pump[1][0].get_center(), rate_func=linear)
            return [_flow(active, rt), fan]

        def label(src, at, color, edge="left"):
            return _readout(src, np.array(at, dtype=float), size=LABEL_FONT_SIZE, color=color, edge=edge)

        gas_lbl = label(lambda: rf"\text{{Erdgas}}\;{de_num(t['gas'].get_value() + t['loss'].get_value())}\,\mathrm{{kWh}}",
                        (-6.3, Y + 0.75, 0.0), ERZ_AMBER)
        heat_lbl = label(lambda: rf"\text{{Wärme}}\;{de_num(t['heat'].get_value())}\,\mathrm{{kWh}}",
                         (-0.60, Y + 0.50, 0.0), NUTZ_BLUE, "right")
        pump_lbl = label(lambda: rf"{de_num(t['out_env'].get_value() + t['out_el'].get_value())}\,\mathrm{{kWh}}",
                         (6.0, Y + 0.50, 0.0), NUTZ_BLUE, "right")
        el_lbl = label(lambda: rf"\text{{Strom}}\;{de_num(t['el'].get_value())}\,\mathrm{{kWh}}",
                       (0.9, Y + 0.75, 0.0), P_YELLOW)
        env_lbl = label(lambda: rf"\text{{Umwelt}}\;{de_num(t['env'].get_value())}\,\mathrm{{kWh}}",
                        (0.9, Y - 0.82, 0.0), AMBIENT_GREEN)

        gas_row = _ratio_row("Gaskessel", r"e_{g}", ERZ_AMBER, -0.65, out=Q_NUTZ,
                             value=lambda: (t["gas"].get_value() + t["loss"].get_value()) / Q_NUTZ,
                             shown=lambda: t["gas"].get_value() > 99.0, x0=-6.95, name_w=2.15, length=2.0)
        wp_row = _ratio_row("Wärmepumpe", r"e_{g}", AMBIENT_GREEN, -0.65, out=Q_NUTZ,
                            value=lambda: t["el"].get_value() / Q_NUTZ,
                            shown=lambda: t["el"].get_value() > 25.0, x0=0.35, name_w=1.85, length=2.0)
        cond_label = body_text("Brennwertkessel", font_size=LABEL_FONT_SIZE, color=P_CYAN).move_to(
            gas_row["label"].get_left(), aligned_edge=LEFT)
        flue_lbl = body_text("Abgas", font_size=LABEL_FONT_SIZE - 3, color=P_RED).move_to(np.array([-3.0, 1.95, 0.0]))
        drops = droplets(np.array([-3.12, 1.40, 0.0]), n=4, spread=(0.10, 0.08), color=P_CYAN)

        self.play(Create(divider), FadeIn(boiler), FadeIn(pump), run_time=0.8)
        self.add(heat_l.ribbons, hp_out.ribbons, heat_lbl, pump_lbl)
        active.extend([heat_l, hp_out])
        self.play(t["heat"].animate.set_value(Q_NUTZ), t["out_env"].animate.set_value(74.0),
                  t["out_el"].animate.set_value(26.0), *live(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 0.8 + 1.2, during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "kessel"), live)
        self.add(gas_in.ribbons, flue.ribbons, gas_lbl, gas_row["readout"])
        active.extend([gas_in, flue])
        self.play(FadeIn(gas_row["group"]), FadeIn(flue_lbl), *live(0.8), run_time=0.8)
        self.play(t["gas"].animate.set_value(Q_NUTZ), t["loss"].animate.set_value(18.0),
                  Circumscribe(boiler, color=ERZ_AMBER, buff=0.1, stroke_width=3, fade_out=True),
                  *live(2.0), run_time=2.0)
        hold_for(self, self.NARRATION, "kessel", during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "brennwert"), live)
        cond_note = note_line("Brennwert: Wasserdampf im Abgas kondensiert → latente Wärme wird nutzbar", color=P_CYAN)
        self.add(recov.ribbons)
        active.append(recov)
        self.play(FadeIn(cond_note), FadeIn(drops), ReplacementTransform(gas_row["label"], cond_label),
                  t["loss"].animate.set_value(7.0), t["recov"].animate.set_value(11.0), *live(2.2), run_time=2.2)
        hold_for(self, self.NARRATION, "brennwert", during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "waermepumpe"), live)
        bought = body_text("zugekauft", font_size=LABEL_FONT_SIZE - 2, color=P_YELLOW).move_to(np.array([4.95, 1.50, 0.0]))
        free = body_text("nicht zugekauft", font_size=LABEL_FONT_SIZE - 2, color=AMBIENT_GREEN).move_to(
            np.array([5.2, 0.02, 0.0]))
        bought_leader = Line(bought.get_bottom() + DOWN * 0.04, np.array([4.95, Y + 0.185, 0.0]), color=P_YELLOW,
                             stroke_width=1.4, stroke_opacity=0.6)
        free_leader = Line(np.array([4.95, free.get_top()[1] + 0.04, 0.0]), np.array([4.95, Y - 0.065, 0.0]), color=AMBIENT_GREEN,
                           stroke_width=1.4, stroke_opacity=0.6)
        self.add(env_in.ribbons, el_in.ribbons, el_lbl, env_lbl, wp_row["readout"])
        active.extend([env_in, el_in])
        self.play(FadeOut(cond_note), FadeIn(wp_row["group"]), *live(0.3), run_time=0.3)
        self.play(t["env"].animate.set_value(74.0), t["el"].animate.set_value(26.0), t["split"].animate.set_value(1.0),
                  Circumscribe(pump, color=AMBIENT_GREEN, buff=0.1, stroke_width=3, fade_out=True),
                  *live(2.2), run_time=2.2)
        self.play(FadeIn(bought), FadeIn(free), Create(bought_leader), Create(free_leader), *live(0.8), run_time=0.8)
        hold_for(self, self.NARRATION, "waermepumpe", during=live)

        caption = _swap(self, caption, subtitle_text(self.NARRATION, "jaz"), live)
        eq, eq_box, items = math_panel([
            ("jaz", r"\mathrm{JAZ}", AMBIENT_GREEN), (None, "=", P_WHITE),
            ("frac", rf"\frac{{\textcolor{{{NUTZ_BLUE}}}{{\text{{Wärme ab}}}}}}{{\textcolor{{{P_YELLOW}}}{{\text{{Strom zu}}}}}}",
             P_WHITE),
            ("num", r"=\frac{100}{26}\approx 3{,}8", P_WHITE),
            ("eg", r"\;\Rightarrow\;e_{g}=\frac{1}{\mathrm{JAZ}}", AMBIENT_GREEN),
        ])
        pump_lbl.clear_updaters()
        el_lbl.clear_updaters()
        sources = VGroup(pump_lbl[0].copy(), el_lbl[0].copy())
        self.play(Create(eq_box), FadeIn(items["jaz"]), FadeIn(eq[1]),
                  ReplacementTransform(sources, items["frac"]),
                  Circumscribe(hp_out.ribbons, color=NUTZ_BLUE, buff=0.06, stroke_width=3, fade_out=True),
                  Circumscribe(el_in.ribbons, color=P_YELLOW, buff=0.06, stroke_width=3, fade_out=True),
                  *live(2.0), run_time=2.0)
        self.play(FadeIn(items["num"]), *live(1.0), run_time=1.0)
        self.play(FadeIn(items["eg"]), Circumscribe(wp_row["group"], color=AMBIENT_GREEN, buff=0.08, stroke_width=3,
                                                    fade_out=True), *live(1.0), run_time=1.0)
        hold_for(self, self.NARRATION, "jaz", during=live)

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion


#region Beat 4 – Die Anlagenaufwandszahl e_p

class Beat4_Anlagenaufwandszahl(Scene):
    NARRATION = [
        ("intro",
         "Multiply the four stages together and the whole plant collapses into a single number.",
         "Multipliziert man die vier Stufen, schrumpft die ganze Anlage auf eine einzige Zahl."),
        ("kette",
         "That is the plant expense figure: how much final energy is required per unit of useful heat.",
         "Das ist die Anlagenaufwandszahl: wie viel Endenergie je Einheit Nutzenergie erforderlich ist."),
        ("gas",
         "For the gas boiler it lands at one point four — a hundred becomes about a hundred and forty.",
         "Beim Gaskessel liegt sie bei eins Komma vier — aus hundert werden etwa hundertvierzig."),
        ("wp",
         "For the heat pump it drops below one, because three quarters of the delivered heat came from the environment.",
         "Bei der Wärmepumpe fällt sie unter eins — drei Viertel der gelieferten Wärme kamen aus der Umwelt."),
        ("brueck",
         "But thirty-one kilowatt-hours of electricity and a hundred and forty of gas are still not comparable. That is the next chapter.",
         "Doch einunddreißig Kilowattstunden Strom und hundertvierzig Gas sind noch nicht vergleichbar. Das ist das nächste Kapitel."),
    ]

    def construct(self):
        apply_scene_style(self)

        title = scene_title(TITLE_DE)
        self.add(title)
        subtitle = beat_subtitle("Die Anlagenaufwandszahl e_p", title)
        din = _din_ref("DIN V 18599-5:2018-09")
        self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)

        caption = caption_bar(subtitle_text(self.NARRATION, "intro"))
        self.play(FadeIn(caption), run_time=0.3)

        P = _plant()
        room, v = P["room"], P["v"]
        mix, lift, grow_p = ValueTracker(0.0), ValueTracker(0.0), ValueTracker(0.0)
        active: list = []
        heat_names = ("rad", "person", "ue", "ve", "sp")

        def live(rt):
            return [_flow(active, rt), *[P["heat"](n, rt) for n in heat_names]]

        q_h = _readout(lambda: rf"Q_{{h}}={de_num(v['z0n'].get_value())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                       np.array([ROOM_C[0], 1.42, 0.0]), size=BODY_FONT_SIZE, color=NUTZ_BLUE, edge="center")
        q_e = _readout(lambda: rf"Q_{{E}}={de_num(P['z4'].total())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                       np.array([1.55, -1.22, 0.0]), size=BODY_FONT_SIZE, color=END_RED, edge="left")

        rows = {
            "ue": _stage_row("Übergabe", r"e_{ü}", UEBER_RED, 1.80, v["z2u"], LEVELS[0], name_w=1.85),
            "ve": _stage_row("Verteilung", r"e_{d}", VERT_ORANGE, 1.22, v["z2v"], LEVELS[1], name_w=1.85),
            "sp": _stage_row("Speicherung", r"e_{s}", SPEI_YELLOW, 0.64, v["z3s"], LEVELS[2], name_w=1.85),
        }
        eg_unit = ValueTracker(0.0)

        def eg_value():
            m = mix.get_value()
            return ((1 - m) * (eg_unit.get_value() + v["z4e"].get_value() / LEVELS[3])
                    + m * v["z4w"].get_value() / LEVELS[3])

        rows["eg"] = _ratio_row("Erzeugung", r"e_{g}", ERZ_AMBER, 0.06, out=LEVELS[3], value=eg_value, name_w=1.85,
                                shown=lambda: eg_unit.get_value() > 0.999 and (v["z4e"].get_value() > 0.05
                                                                              or v["z4w"].get_value() > 0.05))
        keys = ("ue", "ve", "sp", "eg")
        colours = (UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER)
        units = {"ue": rows["ue"]["unit"], "ve": rows["ve"]["unit"], "sp": rows["sp"]["unit"], "eg": eg_unit}
        product = _product_row(
            "Anlage", r"e_{p}", -0.75, name_w=1.85, factors=
            [((lambda f=rows[k]["value"]: 1.0 + grow_p.get_value() * (f() - 1.0)), c) for k, c in zip(keys, colours)],
            lift=lambda: lift.get_value())

        # —— The whole plant, drawn once: every stream at its final width ——
        self.play(Create(P["shell"]), run_time=1.2)
        self.add(P["z0"].ribbons, P["z2"].ribbons, P["z3"].ribbons, P["z4"].ribbons, P["flue"].ribbons,
                 P["env"].ribbons, P["tank"]["fill"], q_h, q_e)
        active.extend([P["z0"], P["z2"], P["z3"], P["z4"], P["flue"]])
        self.play(FadeIn(P["person"]), FadeIn(P["radiator"]), FadeIn(P["tank"]["group"]), FadeIn(P["boiler"]),
                  FadeIn(P["meter"]), FadeIn(P["chimney"]),
                  room["air"].animate.set_fill(P_ORANGE, opacity=0.10), *live(0.8), run_time=0.8)
        self.play(*_grow(P, ["z0n", "z2n", "z2u", "z2v", "z3n", "z3u", "z3v", "z3s", "z4n", "z4u", "z4v", "z4s",
                             "z4e", "flue"], 2.4), *live(2.4), run_time=2.4)
        hold_for(self, self.NARRATION, "intro", used=BEAT_SUBTITLE_FADE + 0.3 + 1.2 + 0.8 + 2.4, during=live)

        # —— Four gauges, then their product as one bar ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "kette"), live)
        self.add(*[rows[k]["readout"] for k in keys], product["readout"])
        for k, c, node in zip(keys, colours, (P["radiator"], P["z2"].ribbons, P["tank"]["group"], P["boiler"])):
            self.play(FadeIn(rows[k]["group"]), units[k].animate.set_value(1.0),
                      Circumscribe(node, color=c, buff=0.1, stroke_width=3, fade_out=True), *live(0.8), run_time=0.8)
        eq, eq_box, items = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, "=", P_WHITE),
            ("eu", r"e_{ü}", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"e_{d}", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"e_{s}", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"e_{g}", ERZ_AMBER),
        ])
        schematic_note = note_line("Schematische Darstellung — DIN/TS 18599-5 rechnet deutlich detaillierter",
                                   color=P_TEAL)
        self.play(FadeIn(product["group"]), FadeIn(schematic_note), lift.animate.set_value(1.0), *live(0.8),
                  run_time=0.8)
        self.play(grow_p.animate.set_value(1.0), *live(2.0), run_time=2.0)
        self.play(Create(eq_box),
                  ReplacementTransform(product["symbol"].copy(), items["ep"]),
                  *[ReplacementTransform(rows[k]["symbol"].copy(), items[i])
                    for k, i in zip(keys, ("eu", "ed", "es", "eg"))],
                  FadeIn(VGroup(*[m for m in eq if all(m is not items[i] for i in ("ep", "eu", "ed", "es", "eg"))])),
                  *live(1.4), run_time=1.4)
        hold_for(self, self.NARRATION, "kette", during=live)

        # —— Gas: 100 become 140 ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "gas"), live)
        gas_label = body_text("Gaskessel", font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(
            rows["eg"]["label"].get_left(), aligned_edge=LEFT)
        eq2, eq2_box, items2 = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, r"\approx", P_WHITE),
            ("eu", r"1{,}04", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"1{,}08", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"1{,}06", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"1{,}18", ERZ_AMBER), (None, r"\approx 1{,}40", P_WHITE),
        ])
        self.play(FadeOut(schematic_note), ReplacementTransform(rows["eg"]["label"], gas_label),
                  ReplacementTransform(eq_box, eq2_box), ReplacementTransform(eq, eq2),
                  Indicate(P["meter"], color=END_RED), *live(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "gas", during=live)

        # —— Heat pump: the same chain with a generator that takes 0,26 ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "wp"), live)
        pump_label = body_text("Wärmepumpe", font_size=LABEL_FONT_SIZE, color=AMBIENT_GREEN).move_to(
            gas_label.get_left(), aligned_edge=LEFT)
        env_lbl = _readout(lambda: rf"\text{{Umwelt}}\;{de_num(v['env'].get_value())}\,\mathrm{{kWh}}",
                           np.array([-0.45, PIPE_Y - 0.62, 0.0]), size=LABEL_FONT_SIZE, color=AMBIENT_GREEN, edge="left")
        eq3, eq3_box, items3 = math_panel([
            ("ep", r"e_{p}", P_WHITE), (None, r"\approx", P_WHITE),
            ("eu", r"1{,}04", UEBER_RED), (None, r"\cdot", P_WHITE),
            ("ed", r"1{,}08", VERT_ORANGE), (None, r"\cdot", P_WHITE),
            ("es", r"1{,}06", SPEI_YELLOW), (None, r"\cdot", P_WHITE),
            ("eg", r"0{,}26", AMBIENT_GREEN), (None, r"\approx 0{,}31", P_WHITE),
        ])
        why = note_line("e_p < 1 nur, weil Umweltwärme ohne zugekaufte Endenergie dazukommt", color=AMBIENT_GREEN)
        self.add(env_lbl)
        active.append(P["env"])
        self.play(
            ReplacementTransform(P["boiler"], P["heat_pump"]), ReplacementTransform(P["meter"], P["power_meter"]),
            FadeOut(P["chimney"]), ReplacementTransform(gas_label, pump_label),
            *[v[n].animate.set_value(0.0) for n in ("z4n", "z4u", "z4v", "z4s", "z4e", "flue")],
            v["z4w"].animate.set_value(WP_STROM), v["env"].animate.set_value(WP_UMWELT), mix.animate.set_value(1.0),
            ReplacementTransform(eq2_box, eq3_box), ReplacementTransform(eq2, eq3), FadeIn(why),
            *live(3.0), run_time=3.0)
        hold_for(self, self.NARRATION, "wp", during=live)

        # —— Both numbers side by side — not yet comparable ——
        caption = _swap(self, caption, subtitle_text(self.NARRATION, "brueck"), live)
        bridge, bridge_box, bitems = math_panel([
            ("qe", r"Q_{E}", END_RED), (None, "=", P_WHITE),
            ("qh", r"Q_{h}", NUTZ_BLUE), (None, r"\cdot", P_WHITE),
            ("ep", r"e_{p}", P_WHITE),
        ])
        gas_meter = _meter(np.array([2.1, 1.35, 0.0]), kind="gas", color=ERZ_AMBER).scale(1.5)
        power_meter = _meter(np.array([5.3, 1.35, 0.0]), kind="power", color=P_YELLOW).scale(1.5)
        gas_val = math_label(rf"{de_num(round(Q_E_GAS))}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}", np.array([2.1, 0.55, 0.0]),
                             size=BODY_FONT_SIZE, color=END_RED)
        power_val = math_label(rf"{de_num(round(Q_E_WP))}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}", np.array([5.3, 0.55, 0.0]),
                               size=BODY_FONT_SIZE, color=P_YELLOW)
        not_equal = VGroup(*[Line(np.array([3.55 - 0.16, 1.35 + dy, 0.0]), np.array([3.55 + 0.16, 1.35 + dy, 0.0]),
                                  color=P_WHITE, stroke_width=3) for dy in (0.07, -0.07)],
                           Line(np.array([3.55 - 0.1, 1.35 - 0.2, 0.0]), np.array([3.55 + 0.1, 1.35 + 0.2, 0.0]),
                                color=P_WHITE, stroke_width=3))
        gas_name = body_text("Gas", font_size=LABEL_FONT_SIZE, color=ERZ_AMBER).move_to(np.array([2.1, 1.95, 0.0]))
        power_name = body_text("Strom", font_size=LABEL_FONT_SIZE, color=P_YELLOW).move_to(np.array([5.3, 1.95, 0.0]))
        bridge_note = note_line("1 kWh Strom ≠ 1 kWh Gas", color=P_TEAL)
        everything = VGroup(*[rows[k]["group"] for k in keys], product["group"])
        self.remove(*[rows[k]["readout"] for k in keys], product["readout"])
        self.play(FadeOut(why), FadeOut(everything), FadeOut(pump_label),
                  ReplacementTransform(eq3_box, bridge_box), ReplacementTransform(eq3, bridge),
                  FadeIn(gas_meter), FadeIn(power_meter), FadeIn(gas_val), FadeIn(power_val), FadeIn(not_equal),
                  FadeIn(gas_name), FadeIn(power_name), FadeIn(bridge_note), *live(1.6), run_time=1.6)
        hold_for(self, self.NARRATION, "brueck", during=live)

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion
