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
    BODY_FONT_SIZE, LABEL_FONT_SIZE, FORMULA_FONT_SIZE, body_text,
)
from manim_visuals import (
    P_DEEP_DARK, P_WHITE, P_CYAN, P_TEAL, P_ORANGE, P_YELLOW, P_RED, P_BLUE, P_GREEN,
    SAFE_TOP, SAFE_BOTTOM_FORMULA, CONTENT_TOP,
    chip, note_line, card_grid, stat_card, side_labels, stacked_bar, fit_band, dim_arrow,
    equation_row, formula_panel, highlight_param,
    math_label, math_panel, math_readout, de_num,
    caption_bar, swap_caption, hold_for, subtitle_text,
    set_vo_language, load_vo_timing,
    room_section, radiator, person_glyph, energy_tank, ripples, droplets,
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

FLOW_UNIT = 0.0065
LANE_PITCH = 0.052
PARTICLE_GAP = 0.5
PARTICLE_R = 0.03
FLOW_PACE = 0.9
_FLOW_CLOCK = [0.0]

ROOM_C = np.array([-3.55, 0.97, 0.0])
ROOM_W, ROOM_H = 5.4, 1.6
GROUND_Y = -1.40
PIPE_Y = -0.66
RAD_X, TANK_X, BOILER_X, METER_X = -5.45, -3.9, -1.9, 0.30


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
        edges = self.edges(self.grid)
        lo, hi = edges[i], edges[i + 1]
        if float((hi - lo).max()) < 1e-3:
            mob.clear_points()
            return
        a = self.pts + self.nrm * lo[:, None]
        b = self.pts + self.nrm * hi[:, None]
        mob.set_points_as_corners(np.concatenate([a, b[::-1], a[:1]]))
        mob.set_fill(self.colour(i), opacity=0.5)
        mob.set_stroke(width=0)

    def advance(self, clock: float, dots) -> None:
        """💨 Place every particle for stream time ``clock``; lanes beyond the current width stay dark."""
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
        fade = np.clip(np.minimum(t / 0.05, (1.0 - t) / 0.07), 0.0, 1.0)
        alpha = 0.95 * weight * fade
        colours = [self.colour(i) for i in range(len(self.layers))]
        for dot, p, k, a in zip(dots, pos, layer, alpha):
            dot.move_to(p)
            dot.set_fill(colours[k], opacity=float(a))


def _flow(streams, rt: float):
    """💨 The particles of several ``_Stream``s as one animation; stream time runs on across plays so nothing jumps."""
    dots, meta = VGroup(), []
    for stream in streams:
        group = [Dot(radius=PARTICLE_R, stroke_width=0).set_fill(P_WHITE, opacity=0.0)
                 for _ in range(stream.lanes * stream.per)]
        dots.add(*group)
        meta.append((stream, group))
    base = _FLOW_CLOCK[0]
    _FLOW_CLOCK[0] += rt

    def update(_mob, alpha):
        for stream, group in meta:
            stream.advance(base + alpha * rt, group)

    return UpdateFromAlphaFunc(dots, update, remover=True, rate_func=linear)


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


def _heat_pump(center, color=AMBIENT_GREEN, w: float = 1.0, h: float = 1.0):
    """🌬️ Air-source heat pump — casing, fan on the outdoor side and a compressor ring."""
    c = np.array(center, dtype=float)
    body = RoundedRectangle(corner_radius=0.12, width=w, height=h, color=P_WHITE, stroke_width=2.5).move_to(c)
    fan = _fan(c + LEFT * 0.21 + UP * 0.12, color)
    compressor = Circle(radius=0.12, color=P_YELLOW, stroke_width=2.2).move_to(c + RIGHT * 0.25 + DOWN * 0.22)
    fins = VGroup(*[Line(c + LEFT * 0.42 + DOWN * (0.28 + 0.07 * k) + RIGHT * 0.0, c + LEFT * 0.02 + DOWN * (0.28 + 0.07 * k),
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


def _ratio_row(name: str, symbol: str, color: str, y: float, *, x0: float = 0.95, length: float = 2.0,
               top: float = 1.5, tick_label: bool = False):
    """📊 One factor gauge — track 0 … ``top``, a tick at 1 and the fill driven by a tracker.

    Up to 1 the fill is the useful heat (blue); whatever lies beyond 1 is the stage's extra energy in its own
    colour, so a factor above one reads as an overshoot and the heat pump's 0,26 as a short bar.
    """
    tracker = ValueTracker(0.0)
    label = body_text(name, font_size=LABEL_FONT_SIZE, color=color).move_to(np.array([x0, y, 0.0]), aligned_edge=LEFT)
    sym = math_label(symbol, np.array([x0 + 1.22, y, 0.0]), size=BODY_FONT_SIZE, color=color, edge="left")
    left = x0 + 1.78
    unit = length / top
    track = Rectangle(width=length, height=0.2, color=P_WHITE, stroke_width=1.6, fill_color=P_DEEP_DARK,
                      fill_opacity=1.0).move_to(np.array([left + length / 2, y, 0.0]))
    base = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=NUTZ_BLUE, fill_opacity=0.9)
    over = Rectangle(width=0.01, height=0.14, stroke_width=0, fill_color=color, fill_opacity=0.95)

    def shape(_m=None):
        e = tracker.get_value()
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
    return {"tracker": tracker, "group": group, "label": label, "symbol": sym, "track": track,
            "right": left + length + 0.18, "y": y, "color": color}


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
    person = _stand(room, -1.45) if with_person else None
    tank = energy_tank(np.array([TANK_X, PIPE_Y - 0.14, 0.0]), height=1.0, width=0.8, color=SPEI_YELLOW, level=1.0)
    boiler = _boiler(np.array([BOILER_X, PIPE_Y - 0.14, 0.0]))
    meter = _meter(np.array([METER_X, PIPE_Y, 0.0]))
    chimney = _chimney(-0.30, -0.34, 1.55)

    v = {name: ValueTracker(0.0) for name in
         ("z0n", "z2n", "z2u", "z2v", "z3n", "z3u", "z3v", "z3s", "z4n", "z4u", "z4v", "z4s", "z4e", "z4w", "flue")}
    target = {"z0n": Q_NUTZ, "z2n": Q_NUTZ, "z2u": LOSS[0], "z2v": LOSS[1], "z3n": Q_NUTZ, "z3u": LOSS[0],
              "z3v": LOSS[1], "z3s": LOSS[2], "z4n": Q_NUTZ, "z4u": LOSS[0], "z4v": LOSS[1], "z4s": LOSS[2],
              "z4e": LOSS[3], "z4w": 0.0, "flue": LOSS[3]}
    half = 0.5 * Q_NUTZ * FLOW_UNIT
    cols = (NUTZ_BLUE, UEBER_RED, VERT_ORANGE, SPEI_YELLOW, ERZ_AMBER)
    rad_out = rad_c + RIGHT * 0.50
    z0 = _Stream([rad_out, np.array([-4.2, rad_out[1] + 0.03, 0.0]), np.array([-3.2, rad_out[1] + 0.14, 0.0])],
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
    flue = _Stream([np.array([BOILER_X - 0.1, PIPE_Y + 0.38, 0.0]), np.array([BOILER_X - 0.1, flue_y, 0.0]),
                    np.array([-0.30, flue_y, 0.0]), np.array([-0.30, 1.50, 0.0])],
                   [(v["flue"], P_RED, False)], start=-0.5 * LOSS[3] * FLOW_UNIT, cap=LOSS[3], seed=5, radius=0.3)

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
            "flue": flue, "streams": (z0, z2, z3, z4, flue), "heat": heat, "rad_c": rad_c, "under_slab": under_slab}


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

        q_h = math_readout(lambda: rf"Q_{{h}}={de_num(v['z0n'].get_value())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
                           np.array([ROOM_C[0], 1.42, 0.0]), size=BODY_FONT_SIZE, color=NUTZ_BLUE, edge="center")
        q_e = math_readout(lambda: rf"Q_{{E}}={de_num(P['z4'].total())}\,\mathrm{{kWh/(m^{{2}}\cdot a)}}",
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
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "nutz"))
        self.add(P["z0"].ribbons, q_h)
        active.append(P["z0"])
        self.play(FadeOut(revision_note), *_grow(P, ["z0n"], 1.6), *live(1.6, "rad", "person"), run_time=1.6)
        hold_for(self, self.NARRATION, "nutz", during=lambda rt: live(rt, "rad", "person"))

        # —— Four stages, walked upstream: every node widens the stream by its loss ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "stufen"))
        bar_x, bar_base, unit = 4.9, -1.2, 0.0135
        bar = stacked_bar([("Nutzenergie", Q_NUTZ, NUTZ_BLUE)] + [(n, val, c) for n, val, c in STAGES],
                          base=np.array([bar_x, bar_base, 0.0]), width=1.0, unit=unit)
        legend = VGroup()
        leaders = VGroup()
        rows = {0: -0.20, 1: 0.30, 2: 0.70, 3: 1.10, 4: 1.50}

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
            self.play(Indicate(hit, color=colour, scale_factor=1.12), *grow, *extra, *live(run, *heat_names),
                      run_time=run)
            source = _still(still_of)
            self.add(source)
            self.play(ReplacementTransform(source, bar["bars"][k + 1]), FadeIn(label), Create(leader),
                      *live(1.0, *heat_names), run_time=1.0)

        active.append(P["z2"])
        stage(0, "Übergabe", UEBER_RED, P["radiator"], _grow(P, ["z2n", "z2u"]), ("rad", "person", "ue"),
              still_of=P["z2"].ribbons[1])
        stage(1, "Verteilung", VERT_ORANGE, P["z2"].ribbons, _grow(P, ["z2v"]), ("rad", "person", "ue", "ve"),
              still_of=P["z2"].ribbons[2])
        self.add(P["z3"].ribbons)
        active.append(P["z3"])
        stage(2, "Speicherung", SPEI_YELLOW, P["tank"]["group"],
              [FadeIn(P["tank"]["group"]), FadeIn(P["tank"]["fill"]), *_grow(P, ["z3n", "z3u", "z3v", "z3s"])],
              ("rad", "person", "ue", "ve", "sp"), still_of=P["z3"].ribbons[3])
        self.add(P["z4"].ribbons, P["flue"].ribbons, q_e)
        active.extend([P["z4"], P["flue"]])
        stage(3, "Erzeugung", ERZ_AMBER, P["boiler"],
              [FadeIn(P["boiler"]), FadeIn(P["meter"]), FadeIn(P["chimney"]), FadeIn(gas_tag),
               *_grow(P, ["z4n", "z4u", "z4v", "z4s", "z4e", "flue"])],
              ("rad", "person", "ue", "ve", "sp"), still_of=P["flue"].ribbons[0])
        hold_for(self, self.NARRATION, "stufen", during=lambda rt: live(rt, "rad", "person", "ue", "ve", "sp"))

        # —— Final energy: the stream at the meter against the stream in the room ——
        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "ende"))
        top_n = bar["base"] + UP * Q_NUTZ * unit
        extra_arrow = dim_arrow(top_n + RIGHT * 0.75, bar["top"] + RIGHT * 0.75, color=END_RED)
        extra_label = math_label(rf"\approx +{de_num((Q_E_GAS / Q_NUTZ - 1) * 100)}\,\%",
                                 np.array([bar_x + 0.95, (top_n[1] + bar["top"][1]) / 2, 0.0]),
                                 size=BODY_FONT_SIZE, color=END_RED, edge="left")
        self.play(Create(extra_arrow), FadeIn(extra_label), Indicate(q_e, color=END_RED), *live(1.4, "rad", "person",
                                                                                             "ue", "ve", "sp"),
                  run_time=1.4)
        hold_for(self, self.NARRATION, "ende", during=lambda rt: live(rt, "rad", "person", "ue", "ve", "sp"))

        self.play(FadeOut(caption), run_time=0.6)
        self.wait(0.4)

#endregion
