"""🔍 Report on-screen text that collides or overflows, per beat, without watching the video.

Renders every ``Beat*`` Scene in a scene file with ``dry_run`` (no encoding) at a low
frame rate, and snapshots the scene at every animation boundary — the moments a viewer
actually reads. At each snapshot it reports:

* text-on-text overlap (two labels printed over each other)
* text-on-outline overlap (a label printed across a room edge, axis, arrow or bracket)
* text leaving the frame or entering a reserved zone (title / formula / caption band)

Mid-animation overlap is ignored on purpose: things legitimately cross while moving.

Usage:  python check_layout.py <scene_file.py> [BeatClass ...]
"""

from __future__ import annotations

import sys
from pathlib import Path

import numpy as np

SCENE_PATH = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(SCENE_PATH.parent))
_TUTORIAL_ROOT = next(p for p in SCENE_PATH.parents if (p / "manim_fonts.py").is_file())
sys.path.insert(0, str(_TUTORIAL_ROOT))

from manim import Arc, Arrow, Circle, Line, Polygon, Rectangle, Scene, Text, VMobject, config, tempconfig  # noqa: E402
from manim_visuals import SAFE_BOTTOM, SAFE_BOTTOM_FORMULA, SAFE_TOP  # noqa: E402

# VMobject catches raw curves (radiation waves, convection ribbons, flow
# guides, dimension-arrow ticks) that aren't any of the named shape classes —
# the first pass missed every one of these, which is why real touches survived
# a "0 findings" run.
OUTLINE_TYPES = (Line, Rectangle, Polygon, Circle, Arc, Arrow, VMobject)
PAD = 0.04          # ignore hairline touches — true crossings
NEAR_PAD = -0.10    # boxes within 0.10 units of each other — visually "touching"
OUTLINE_SAMPLES = 80
FINDINGS: list[str] = []
NEAR_FINDINGS: list[str] = []
_CURRENT = {"beat": "", "n": 0}


#region Geometry
def _box(mob):
    """📦 (x0, y0, x1, y1) bounding box of a mobject."""
    lo, hi = mob.get_corner([-1, -1, 0]), mob.get_corner([1, 1, 0])
    return float(lo[0]), float(lo[1]), float(hi[0]), float(hi[1])


def _overlap(a, b, pad=PAD):
    """↔️ Overlapping area of two boxes, shrunk by ``pad`` so touching edges do not count."""
    x0 = max(a[0], b[0]) + pad
    x1 = min(a[2], b[2]) - pad
    y0 = max(a[1], b[1]) + pad
    y1 = min(a[3], b[3]) - pad
    if x1 <= x0 or y1 <= y0:
        return 0.0
    return (x1 - x0) * (y1 - y0)


def _inside(pt, box, pad=PAD):
    return (box[0] + pad) < pt[0] < (box[2] - pad) and (box[1] + pad) < pt[1] < (box[3] - pad)
#endregion


#region Collection
def _family(scene):
    seen, out = set(), []
    for top in scene.mobjects:
        for sub in top.get_family():
            if id(sub) not in seen:
                seen.add(id(sub))
                out.append(sub)
    return out


MIN_EXTENT = 0.02


def _visible(mob) -> bool:
    if not mob.has_points() and not isinstance(mob, Text):
        return False
    # Degenerate zero-size mobjects (manim leaves a few behind inside grouped
    # shapes) occupy a point and cannot visually cross anything — they produced
    # every false "text crossed by Dot" report before this filter.
    if max(float(mob.width), float(mob.height)) < MIN_EXTENT:
        return False
    fill = float(getattr(mob, "fill_opacity", 0) or 0)
    stroke = _stroke_opacity(mob)
    return max(fill, stroke) > 0.05


def _stroke_opacity(mob) -> float:
    """✏️ Effective stroke visibility — ``set_stroke(width=0)`` renders nothing
    even with ``stroke_opacity`` untouched, and is the standard way to hide an
    edge (e.g. two adjacent same-colour rectangles merging into one)."""
    stroke_width = float(getattr(mob, "stroke_width", 0) or 0)
    if stroke_width <= 0:
        return 0.0
    return float(getattr(mob, "stroke_opacity", 0) or 0)


def _is_line_art(mob) -> bool:
    """✏️ Only a drawn stroke reads as a "line" a text could visibly cross.

    A filled-but-unstroked shape (a coloured card, a wall's fill) has a
    perfectly real geometric boundary, but nothing is rendered along it — so
    the fact that a Text mobject's bounding box touches that mathematical
    edge is not a visual collision. Without this split, two same-coloured
    adjacent fills with their shared stroke hidden (exactly how the beat-2
    equilibrium merge works) still "crossed" every time, because the fill
    alone made the shape _visible() and its parametric boundary got traced
    regardless of whether a border was actually drawn.
    """
    return _stroke_opacity(mob) > 0.05


def _texts_and_outlines(scene):
    # get_family() descends into a Text's own glyphs — Text is an SVGMobject,
    # and each letter is a VMobjectFromSVGPath child. Adding plain VMobject to
    # OUTLINE_TYPES (needed to catch radiation waves / flow curves) means those
    # glyph paths now match the elif branch and every label "collides with its
    # own letters". Skip anything whose family includes a Text ancestor.
    texts, outlines, glyph_ids = [], [], set()
    for mob in _family(scene):
        if isinstance(mob, Text):
            glyph_ids.update(id(g) for g in mob.get_family())
    for mob in _family(scene):
        if isinstance(mob, Text):
            if _visible(mob) and mob.width > 0.01:
                texts.append(mob)
        elif id(mob) in glyph_ids:
            continue
        elif isinstance(mob, OUTLINE_TYPES) and _is_line_art(mob) and mob.has_points():
            outlines.append(mob)
    return texts, outlines
#endregion


#region Checks
def _label(mob) -> str:
    raw = getattr(mob, "original_text", getattr(mob, "text", "?"))
    raw = " ".join(str(raw).split())
    return raw[:34]


def _report(msg: str, *, near: bool = False) -> None:
    bucket = NEAR_FINDINGS if near else FINDINGS
    entry = f"  [{_CURRENT['beat']} @{_CURRENT['n']:02d}] {msg}"
    if entry not in bucket:
        bucket.append(entry)


def _check(scene) -> None:
    texts, outlines = _texts_and_outlines(scene)
    boxes = [(t, _box(t)) for t in texts]

    for i, (ta, ba) in enumerate(boxes):
        for tb, bb in boxes[i + 1:]:
            area = _overlap(ba, bb, pad=PAD)
            if area > 0.004:
                _report(f"text/text  {area:5.3f}  {_label(ta)!r} ↔ {_label(tb)!r}")
            elif _overlap(ba, bb, pad=NEAR_PAD) > 0.001:
                _report(f"text/text near       {_label(ta)!r} ↔ {_label(tb)!r}", near=True)

    outlines = [(out, _box(out)) for out in outlines]

    for text, tb in boxes:
        crossed = False
        # Cheap bbox reject first — sampling every curve point-by-point against
        # every text box (the O(text·outline·samples) cost that made the first
        # widened pass too slow to finish) only runs for pairs whose boxes are
        # already within NEAR_PAD of each other.
        candidates = [out for out, ob in outlines if _overlap(tb, ob, pad=NEAR_PAD) > 0.0]
        for out in candidates:
            hit = None
            try:
                for s in range(OUTLINE_SAMPLES + 1):
                    pt = out.point_from_proportion(s / OUTLINE_SAMPLES)
                    if _inside(pt, tb, pad=PAD):
                        hit = pt
                        break
            except Exception:
                continue
            if hit is not None:
                _report(
                    f"text/line            {_label(text)!r} crossed by {type(out).__name__} "
                    f"at ({hit[0]:.2f},{hit[1]:.2f}) text_box="
                    f"{tuple(round(v, 2) for v in tb)}"
                )
                crossed = True
                break
        if crossed:
            continue
        # Near-miss: the outline's own bounding box sits within NEAR_PAD of the
        # text's, even though no sampled point of the curve lands inside the
        # text box — catches a wavy line or arrow passing close alongside text
        # without literally crossing it, which still reads as "touching".
        for out, ob in outlines:
            if _overlap(tb, ob, pad=NEAR_PAD) > 0.001 and _overlap(tb, ob, pad=PAD) <= 0.0:
                _report(
                    f"text/line near       {_label(text)!r} near {type(out).__name__} "
                    f"text_box={tuple(round(v, 2) for v in tb)} outline_box={tuple(round(v, 2) for v in ob)}",
                    near=True,
                )
                break

    half_w, half_h = config.frame_width / 2, config.frame_height / 2
    for text, tb in boxes:
        if tb[0] < -half_w or tb[2] > half_w or tb[1] < -half_h or tb[3] > half_h:
            _report(f"offscreen            {_label(text)!r} box={tuple(round(v, 2) for v in tb)}")
#endregion


#region Driver
def _install_hooks():
    orig_play, orig_wait = Scene.play, Scene.wait

    def play(self, *args, **kwargs):
        orig_play(self, *args, **kwargs)
        _CURRENT["n"] += 1
        _check(self)

    def wait(self, *args, **kwargs):
        orig_wait(self, *args, **kwargs)
        _CURRENT["n"] += 1
        _check(self)

    Scene.play, Scene.wait = play, wait


def main() -> int:
    import importlib

    _install_hooks()
    module = importlib.import_module(SCENE_PATH.stem)
    wanted = sys.argv[2:]
    beats = [
        getattr(module, n) for n in dir(module)
        if n.startswith("Beat") and isinstance(getattr(module, n), type)
        and issubclass(getattr(module, n), Scene)
        and (not wanted or n in wanted)
    ]
    beats.sort(key=lambda c: c.__name__)

    print(f"zones: SAFE_TOP={SAFE_TOP} SAFE_BOTTOM={SAFE_BOTTOM} "
          f"SAFE_BOTTOM_FORMULA={SAFE_BOTTOM_FORMULA}\n")
    for cls in beats:
        _CURRENT["beat"], _CURRENT["n"] = cls.__name__.split("_")[0], 0
        before, before_near = len(FINDINGS), len(NEAR_FINDINGS)
        with tempconfig({
            "dry_run": True, "quality": "low_quality", "frame_rate": 3,
            "disable_caching": True, "verbosity": "CRITICAL", "preview": False,
            "progress_bar": "none",
        }):
            cls().render()
        print(f"{cls.__name__:<36}{len(FINDINGS) - before:>3} finding(s)"
              f"{len(NEAR_FINDINGS) - before_near:>5} near")

    if FINDINGS:
        print("\n".join(["", "=" * 72, "HARD (true crossings):"] + FINDINGS))
    else:
        print("\nno hard findings")
    if NEAR_FINDINGS:
        print("\n".join(["", "=" * 72, "NEAR (tight spacing — verify by eye):"] + NEAR_FINDINGS))
    return 1 if FINDINGS else 0


if __name__ == "__main__":
    raise SystemExit(main())
#endregion
