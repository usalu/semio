#!/usr/bin/env python3
"""🩹 Patch Beat5 Raffstore glyph and steady shade radiation."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
while not (ROOT / "🎓️teaching").is_dir() and ROOT != ROOT.parent:
    ROOT = ROOT.parent

base = ROOT / "🎓️teaching" / "🏛️architecture" / "⚡️energy"
phys = next(c for c in base.iterdir() if "physics" in c.name)
clip = next(c for c in phys.iterdir() if "clip" in c.name)
fund = next(c for c in clip.iterdir() if "fundamentals" in c.name)
path = fund / "🐍️.py"
text = path.read_text()


def repl(old: str, new: str, label: str) -> None:
    global text
    if old not in text:
        raise SystemExit(f"MISSING {label}")
    text = text.replace(old, new, 1)
    print("OK", label)


repl(
    '''BLIND_CAST = "shared-blind-lowered-mirrored"
BLIND_PORTION = 0.65
REFLECT_LEN = 1.45


def _blind(x: float, y_top: float, y_bot: float):
    """🪟 External venetian blind of the staffage archive fitted between ``y_top`` and ``y_bot`` and centred on ``x``.

    Only the upper ``BLIND_PORTION`` of the blind is drawn (true proportions, the lower part cut away) so the lamellae read at 1/``BLIND_PORTION`` times the scale of the full blind.

    ``slats`` holds the sun-facing face of every lamella as (outer end, inner end), ``ys`` their centres, ``angle`` the lamella angle and ``group`` the drawn cast with explicit lamella strokes on top of the thermal fill.

    @see ../../../../🧍️staffage/🗂️catalog/🧱️components/external-venetian-blind/🔣️.json — the cast the lamellae are read from
    """
    top = float(_staffage_parts(BLIND_CAST, "box")["box"][:, 1].max())
    bottom = float(_staffage_parts(BLIND_CAST, "bottom-rail")["bottom-rail"][:, 1].min())
    bottom = top - BLIND_PORTION * (top - bottom)
    unit = (y_top - y_bot) / (top - bottom)
    anchor = np.array([x + 0.075 * unit, y_bot - bottom * unit, 0.0])
    cast = _staffage(BLIND_CAST, unit=unit, anchor=anchor, window=(-1.0, bottom - 1e-3, 0.0, top + 1e-3))
    cast.set_fill(opacity=0.4)
    parts = {key: polygon for key, polygon in _staffage_parts(BLIND_CAST, "slat-").items() if float(polygon[:, 1].min()) >= bottom - 1e-6}
    slats = [(np.append(anchor[:2] + outer * unit, 0.0), np.append(anchor[:2] + inner * unit, 0.0)) for outer, inner in lamella_faces(parts)]
    faces = VGroup(*[Line(a, b, color=P_TEAL, stroke_width=2.6, stroke_opacity=0.95) for a, b in slats])
    axis = slats[0][1] - slats[0][0]
    return {"ys": np.array([(a[1] + b[1]) / 2 for a, b in slats]), "slats": slats, "angle": float(np.arctan2(axis[1], axis[0])),
            "group": VGroup(cast, faces).set_z_index(4), "faces": faces}
''',
    '''REFLECT_LEN = 1.6
BLIND_SLAT_COUNT = 12
BLIND_SLAT_ANGLE = -0.55
BLIND_SLAT_DEPTH = 0.38
BLIND_SLAT_THICK = 0.045


def _blind(x: float, y_top: float, y_bot: float):
    """🪟 Side-elevation Raffstore between ``y_top`` and ``y_bot``: head rail, guides and tilted orange lamellae.

    ``slats`` are sun-facing faces (outer → inner) for ray hits; ``angle`` is the lamella tilt; ``group`` is the drawable glyph.
    """
    along = np.array([np.cos(BLIND_SLAT_ANGLE), np.sin(BLIND_SLAT_ANGLE), 0.0])
    normal = np.array([-along[1], along[0], 0.0]) * BLIND_SLAT_THICK
    ys = np.linspace(y_top - 0.06, y_bot + 0.06, BLIND_SLAT_COUNT)
    slats, bodies = [], []
    for y in ys:
        center = np.array([x, float(y), 0.0])
        outer = center - along * (BLIND_SLAT_DEPTH * 0.5)
        inner = center + along * (BLIND_SLAT_DEPTH * 0.5)
        slats.append((outer, inner))
        bodies.append(Polygon(outer + normal, outer - normal, inner - normal, inner + normal,
                              stroke_color=P_TEAL, stroke_width=1.8, fill_color=P_ORANGE, fill_opacity=0.92))
    half = BLIND_SLAT_DEPTH * 0.55
    head = Line(np.array([x - half, y_top, 0.0]), np.array([x + half, y_top, 0.0]), color=P_TEAL, stroke_width=4.0)
    rail_l = Line(np.array([x - half, y_top, 0.0]), np.array([x - half, y_bot, 0.0]), color=P_TEAL, stroke_width=2.2)
    rail_r = Line(np.array([x + half, y_top, 0.0]), np.array([x + half, y_bot, 0.0]), color=P_TEAL, stroke_width=2.2)
    foot = Line(np.array([x - half, y_bot, 0.0]), np.array([x + half, y_bot, 0.0]), color=P_TEAL, stroke_width=3.0)
    group = VGroup(head, rail_l, rail_r, foot, *bodies).set_z_index(4)
    return {"ys": ys, "slats": slats, "angle": float(BLIND_SLAT_ANGLE), "group": group}
''',
    "blind glyph",
)

repl(
    '''        self.play(stage.transition("physics.fundamentals.shade.beams-stopped", source=beams_out, target=incoming), shaft.animate.set_fill(opacity=0.03), FadeOut(patch),
                  t_room.animate.set_value(22.0), room["air"].animate.set_fill(P_RED, opacity=0.04),
                  surface_heat["floor"].animate.set_fill(opacity=0),
                  surface_heat["wall_lo"].animate.set_fill(opacity=0),
                  surface_heat["wall_hi"].animate.set_fill(opacity=0), FadeIn(norm2), run_time=1.4)
        self.play(LaggedStart(*[Create(b) for b in bounced], lag_ratio=0.08), run_time=0.9)
        _beam_pulses(self, shade_paths, P_YELLOW, repeats=2, run_time=2.6)
        hold_for(self, N, "summer")
''',
    '''        self.play(stage.transition("physics.fundamentals.shade.beams-stopped", source=beams_out, target=incoming), shaft.animate.set_fill(opacity=0.03), FadeOut(patch),
                  t_room.animate.set_value(22.0), room["air"].animate.set_fill(P_RED, opacity=0.04),
                  surface_heat["floor"].animate.set_fill(opacity=0),
                  surface_heat["wall_lo"].animate.set_fill(opacity=0),
                  surface_heat["wall_hi"].animate.set_fill(opacity=0), FadeIn(norm2), run_time=1.4)
        self.play(LaggedStart(*[Create(b) for b in bounced], lag_ratio=0.05), run_time=0.8)
        flash = VGroup(*[_beam(a, b, opacity=1.0, width=5.0) for path in shade_paths for a, b in zip(path[:-1], path[1:])])
        self.play(ShowPassingFlash(flash, time_width=0.55), run_time=1.8)
        hold_for(self, N, "summer")
''',
    "summer radiation",
)

repl(
    '''        self.play(FadeIn(in_group, shift=DOWN * 0.2), FadeIn(in_lbl),
                  LaggedStart(*[Create(b) for b in in_beams], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(b) for b in in_bounce], lag_ratio=0.08), run_time=0.8)
        warm_slats = interpolate_color(ManimColor(P_TEAL), ManimColor(P_RED), 0.7)
        self.play(in_group.animate.set_stroke(color=warm_slats), shaft.animate.set_fill(opacity=0.0), LaggedStart(
            *[Flash(np.array(pth[1]), color=P_ORANGE, flash_radius=0.12, line_length=0.08) for pth in in_paths],
            lag_ratio=0.1), run_time=1.0)
        _beam_pulses(self, in_paths, P_YELLOW, repeats=1, run_time=2.0, extra=[
                         _ripples(slat_pts, r_max=0.6), t_room.animate.set_value(30.0),
                         in_frac.animate.set_value(1.0), room["air"].animate.set_fill(P_RED, opacity=0.1)])
        hold_for(self, N, "inner")
''',
    '''        self.play(FadeIn(in_group, shift=DOWN * 0.2), FadeIn(in_lbl),
                  LaggedStart(*[Create(b) for b in in_beams], lag_ratio=0.08), run_time=1.4)
        self.play(LaggedStart(*[Create(b) for b in in_bounce], lag_ratio=0.05), run_time=0.7)
        warm_slats = interpolate_color(ManimColor(P_ORANGE), ManimColor(P_RED), 0.65)
        self.play(in_group.animate.set_fill(warm_slats, opacity=0.95).set_stroke(P_RED), shaft.animate.set_fill(opacity=0.0),
                  LaggedStart(*[Flash(np.array(pth[1]), color=P_ORANGE, flash_radius=0.12, line_length=0.08)
                                for pth in in_paths], lag_ratio=0.1), run_time=1.0)
        flash = VGroup(*[_beam(a, b, opacity=1.0, width=5.0) for path in in_paths for a, b in zip(path[:-1], path[1:])])
        self.play(ShowPassingFlash(flash, time_width=0.55),
                  _ripples(slat_pts, r_max=0.6), t_room.animate.set_value(30.0),
                  in_frac.animate.set_value(1.0), room["air"].animate.set_fill(P_RED, opacity=0.1), run_time=1.8)
        hold_for(self, N, "inner")
''',
    "inner radiation",
)

path.write_text(text)
import ast

ast.parse(text)
print("WROTE", path)
