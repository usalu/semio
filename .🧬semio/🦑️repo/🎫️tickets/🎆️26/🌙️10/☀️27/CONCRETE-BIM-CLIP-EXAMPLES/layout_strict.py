"""🧲 Strict per-keyframe layout check for ticket captures: sees live labels, strokes and fills the shared ``layout_conflicts`` cannot.

Why the shared check says "none": ``LiveText`` is a ``Text`` whose own fill opacity is 0 (the visible shape is its child), so ``_visible_texts``
dropped every live chip, value tag and number, and with them every collision those labels take part in. This module reads opacity from the glyphs.
Identical copies live next to both ``capture.py`` scripts; call ``strict_conflicts(scene)``.
"""
from __future__ import annotations

import numpy as np

MIN_TEXT_PX = 11.0
EDGE_PX = 12.0
OVERLAP_PX = 3.0
TOUCH_PX = 4.0
TOUCH_RUN = 4
THROUGH_INSET = 0.011
CAP_RATIO = 0.72
X_RATIO = 0.5
DESCENT = 0.21
STEP = 0.02
FILL_OPACITY = 0.25
STROKE_OPACITY = 0.15
STROKE_WIDTH = 0.3
GLYPH_OPACITY = 0.05
BACKING_OPACITY = 0.85
CAPTION_TOP = -2.62
GRID = (9, 3)
MAX_PER_LABEL = 3
CHIP_PAD = 0.45
CHIP_AREA = 14.0
BADGE_CHARS = 2
THIN = 0.04
FRAME_ZONES = frozenset({"caption_box", "formula_box"})


def _inked(mob) -> bool:
    fill = mob.get_fill_opacity()
    stroke = mob.get_stroke_opacity() if mob.get_stroke_width() > STROKE_WIDTH else 0.0
    return max(fill, stroke) >= GLYPH_OPACITY


class Label:
    """🏷️ One visible typeset string: glyph anchor points, tight box, zone and draw order."""

    def __init__(self, mob, order):
        self.mob, self.order, self.zone = mob, order, getattr(mob, "_layout_zone", "")
        glyphs = [m for m in mob.get_family() if len(m.points) and _inked(m)]
        self.points = np.concatenate([m.get_anchors() for m in glyphs])[:, :2] if glyphs else np.zeros((0, 2))
        self.box = (*self.points.min(axis=0), *self.points.max(axis=0)) if len(self.points) else None
        self.text = str(getattr(mob, "original_text", None) or getattr(mob, "text", "?"))
        self.words = any(c.isalnum() for c in self.text) and "\n" not in self.text
        tall = any(c.isdigit() or c.isupper() or c in "bdfhijklt%/()[]" for c in self.text)
        drop = any(c in "gjpqy()[]" for c in self.text)
        height = 0.0 if self.box is None else self.box[3] - self.box[1]
        self.cap_height = height * CAP_RATIO / ((CAP_RATIO if tall else X_RATIO) + (DESCENT if drop else 0.0))
        self.grid = None if self.box is None else np.array([
            [self.box[0] + (self.box[2] - self.box[0]) * (a + .5) / GRID[0], self.box[1] + (self.box[3] - self.box[1]) * (b + .5) / GRID[1]]
            for a in range(GRID[0]) for b in range(GRID[1])])


class Shape:
    """🔷 One drawn non-glyph shape: stroke and fill strength, box, draw order, lazily sampled outline."""

    def __init__(self, mob, order, stroke, fill, raster=False):
        self.mob, self.order, self.stroke, self.fill, self.raster = mob, order, stroke, fill, raster
        self.zone = getattr(mob, "_layout_zone", "")
        pts = mob.points[:, :2]
        self.box = (*pts.min(axis=0), *pts.max(axis=0))
        self.thin = min(self.box[2] - self.box[0], self.box[3] - self.box[1]) <= THIN
        self._samples = None

    @property
    def samples(self) -> np.ndarray:
        if self._samples is None:
            self._samples = _curve_samples(self.mob.points)
        return self._samples

    def covers(self, points: np.ndarray) -> np.ndarray:
        if self.raster:
            return _in_box(points, self.box)
        return _inside(points, self.mob.points)


def _curves(pts: np.ndarray) -> np.ndarray:
    count = len(pts) // 4
    return pts[: count * 4, :2].reshape(count, 4, 2)


def _bezier(c: np.ndarray, t: np.ndarray) -> np.ndarray:
    return (1 - t) ** 3 * c[..., 0, :] + 3 * (1 - t) ** 2 * t * c[..., 1, :] + 3 * (1 - t) * t ** 2 * c[..., 2, :] + t ** 3 * c[..., 3, :]


def _curve_samples(pts: np.ndarray) -> np.ndarray:
    curves = _curves(pts)
    if not len(curves):
        return np.zeros((0, 2))
    reach = np.linalg.norm(np.diff(curves, axis=1), axis=2).sum(axis=1)
    per = np.maximum(2, np.ceil(reach / STEP).astype(int) + 1)
    start = np.cumsum(per) - per
    t = ((np.arange(per.sum()) - np.repeat(start, per)) / np.repeat(per - 1, per))[:, None]
    return _bezier(curves[np.repeat(np.arange(len(curves)), per)], t)


def _inside(points: np.ndarray, pts: np.ndarray) -> np.ndarray:
    curves = _curves(pts)
    if not len(curves):
        return np.zeros(len(points), bool)
    poly = _bezier(curves[:, None], np.linspace(0.0, 1.0, 9)[None, :, None])
    a, b = poly[:, :-1].reshape(-1, 2), poly[:, 1:].reshape(-1, 2)
    px, py = points[:, 0:1], points[:, 1:2]
    straddle = (a[:, 1] > py) != (b[:, 1] > py)
    slope = (b[:, 0] - a[:, 0]) / np.where(b[:, 1] == a[:, 1], 1.0, b[:, 1] - a[:, 1])
    return ((straddle & (a[:, 0] + (py - a[:, 1]) * slope > px)).sum(axis=1) % 2) == 1


def _depth(a, b) -> tuple[float, float]:
    return min(a[2], b[2]) - max(a[0], b[0]), min(a[3], b[3]) - max(a[1], b[1])


def _contains(outer, inner) -> bool:
    return outer[0] <= inner[0] and outer[1] <= inner[1] and outer[2] >= inner[2] and outer[3] >= inner[3]


def _in_box(points: np.ndarray, box, grow=0.0) -> np.ndarray:
    return (points[:, 0] > box[0] - grow) & (points[:, 0] < box[2] + grow) & (points[:, 1] > box[1] - grow) & (points[:, 1] < box[3] + grow)


def _grown(box, grow):
    return (box[0] - grow, box[1] - grow, box[2] + grow, box[3] + grow)


def _nearby(boxes: np.ndarray, box) -> np.ndarray:
    return np.nonzero((boxes[:, 2] >= box[0]) & (boxes[:, 0] <= box[2]) & (boxes[:, 3] >= box[1]) & (boxes[:, 1] <= box[3]))[0]


def _collect(scene):
    from manim import ImageMobject, Text, VMobject

    flat: list = []
    for mob in scene.mobjects:
        flat.extend(mob.get_family())
    flat = [m for _, m in sorted(enumerate(flat), key=lambda p: (getattr(p[1], "z_index", 0), p[0]))]
    order = {id(m): i for i, m in enumerate(flat)}
    nested = {id(sub) for m in flat if isinstance(m, Text) for sub in m.get_family()[1:]}
    labels, text_ids = [], set()
    for mob in flat:
        if isinstance(mob, Text) and id(mob) not in nested:
            label = Label(mob, order[id(mob)])
            if label.box is not None:
                labels.append(label)
            text_ids.update(id(m) for m in mob.get_family())
    shapes = []
    for mob in flat:
        if id(mob) in text_ids or id(mob) in nested:
            continue
        if isinstance(mob, ImageMobject):
            opacity = float(getattr(mob, "fill_opacity", 1.0) or 0.0)
            if opacity >= FILL_OPACITY and len(mob.points):
                shapes.append(Shape(mob, order[id(mob)], 0.0, opacity, raster=True))
        elif isinstance(mob, VMobject) and len(mob.points) >= 4:
            stroke = mob.get_stroke_opacity() if mob.get_stroke_width() > STROKE_WIDTH else 0.0
            fill = mob.get_fill_opacity()
            if stroke >= STROKE_OPACITY or fill >= FILL_OPACITY:
                shapes.append(Shape(mob, order[id(mob)], stroke, fill))
    return labels, shapes


class Frame:
    """📐 Pixel conversion for the active 16:9 render configuration."""

    def __init__(self):
        from manim import config

        self.width, self.height = float(config["frame_width"]), float(config["frame_height"])
        self.ppu = float(config["pixel_width"]) / self.width

    def span(self, box) -> str:
        left, top = (box[0] + self.width / 2) * self.ppu, (self.height / 2 - box[3]) * self.ppu
        right, bottom = (box[2] + self.width / 2) * self.ppu, (self.height / 2 - box[1]) * self.ppu
        return f"px({left:.0f}-{right:.0f},{top:.0f}-{bottom:.0f})"

    def name(self, shape) -> str:
        return f"{type(shape.mob).__name__}@{self.span(shape.box)}"

    def margin(self, box) -> float:
        return min(box[0] + self.width / 2, self.width / 2 - box[2], box[1] + self.height / 2, self.height / 2 - box[3]) * self.ppu


def _listing(verb, label, frame, hits, box=None):
    extra = f" +{len(hits) - MAX_PER_LABEL} more" if len(hits) > MAX_PER_LABEL else ""
    return f"{verb} {label.text!r} {frame.span(box or label.box)} x {'; '.join(hits[:MAX_PER_LABEL])}{extra}"


def _frame_issues(labels, frame) -> list[str]:
    issues, small = [], []
    for label in labels:
        where = f"{label.text!r} {frame.span(label.box)}"
        margin = frame.margin(label.box)
        if margin < 0:
            issues.append(f"outside frame {where}")
        elif margin < EDGE_PX:
            issues.append(f"near frame edge ({margin:.0f}px) {where}")
        height = label.cap_height * frame.ppu
        if height < MIN_TEXT_PX and label.words:
            small.append((height, label))
        if label.zone != "caption" and label.box[1] < CAPTION_TOP:
            issues.append(f"in caption band {where}")
    if small:
        sample = "; ".join(f"{l.text!r} {h:.1f}px" for h, l in sorted(small, key=lambda p: p[0])[:MAX_PER_LABEL + 2])
        issues.append(f"unreadable text (<{MIN_TEXT_PX:.0f}px tall) x{len(small)}: {sample}")
    return issues


def _text_issues(labels, frame) -> list[str]:
    issues = []
    for i, first in enumerate(labels):
        for second in labels[i + 1:]:
            if first.zone == second.zone == "caption":
                continue
            dx, dy = (d * frame.ppu for d in _depth(first.box, second.box))
            if dx >= OVERLAP_PX and dy >= OVERLAP_PX:
                issues.append(f"text over text {first.text!r} {frame.span(first.box)} x {second.text!r} {frame.span(second.box)} by {dx:.0f}x{dy:.0f}px")
    return issues


def _backings(label, shapes, near):
    return [shapes[k] for k in near if shapes[k].fill >= BACKING_OPACITY and shapes[k].order < label.order
            and not shapes[k].thin and _contains(shapes[k].box, label.box) and shapes[k].covers(label.grid).all()]


def _hidden(shape, points, backings) -> bool:
    above = [b for b in backings if b.order > shape.order]
    return bool(above) and len(points) > 0 and all(b.covers(points).all() for b in above)


def _fill_hit(shape, label, backings, frame):
    depth = _depth(shape.box, label.box)
    if shape.fill < FILL_OPACITY or depth[0] <= 0 or depth[1] <= 0 or _hidden(shape, label.grid, backings):
        return None
    covered = shape.covers(label.grid)
    if covered.any() and not covered.all():
        return "fill", f"{frame.name(shape)} covers {covered.mean() * 100:.0f}%"
    if not covered.any() and shape.stroke < STROKE_OPACITY and _in_box(shape.samples, _grown(label.box, -THROUGH_INSET)).any():
        return "line", f"{frame.name(shape)} (thin fill)"
    return None


def _chip_fit(label, box) -> bool:
    pad = (label.box[0] - box[0], label.box[1] - box[1], box[2] - label.box[2], box[3] - label.box[3])
    own = (label.box[2] - label.box[0]) * (label.box[3] - label.box[1])
    return min(pad) >= -1e-6 and max(pad) <= CHIP_PAD and (box[2] - box[0]) * (box[3] - box[1]) <= CHIP_AREA * own


def _lopsided(label, box, touch) -> bool:
    for near, far in ((label.box[0] - box[0], box[2] - label.box[2]), (label.box[1] - box[1], box[3] - label.box[3])):
        if min(near, far) < touch and abs(near - far) > touch / 2:
            return True
    return False


def _stroke_hit(shape, label, backings, frame):
    pts = shape.samples
    through = _in_box(pts, _grown(label.box, -THROUGH_INSET))
    close = _in_box(pts, _grown(label.box, TOUCH_PX / frame.ppu))
    enclosing = not shape.thin and _contains(shape.box, label.box) and not _chip_fit(label, shape.box) and _lopsided(label, shape.box, TOUCH_PX / frame.ppu)
    if not through.any() and not (enclosing and close.sum() >= TOUCH_RUN):
        return None
    if _hidden(shape, pts[through] if through.any() else pts[close], backings):
        return None
    return "line", frame.name(shape) + ("" if through.any() else f" (frame within {TOUCH_PX:.0f}px)")


def _shape_hits(label, shapes, near, frame):
    backings = _backings(label, shapes, near)
    lines, fills = [], []
    for k in near:
        shape = shapes[k]
        if shape.zone in FRAME_ZONES or shape in backings:
            continue
        hit = _fill_hit(shape, label, backings, frame) or (_stroke_hit(shape, label, backings, frame) if shape.stroke >= STROKE_OPACITY else None)
        if hit:
            (fills if hit[0] == "fill" else lines).append(hit[1])
    return lines, fills


def _chip_of(label, shapes, boxes):
    area = (boxes[:, 2] - boxes[:, 0]) * (boxes[:, 3] - boxes[:, 1])
    found = [k for k in _nearby(boxes, label.box) if not shapes[k].raster and not shapes[k].thin and shapes[k].zone not in FRAME_ZONES
             and _chip_fit(label, shapes[k].box)]
    return shapes[min(found, key=lambda k: area[k])] if found else None


def _passes(points: np.ndarray, box) -> bool:
    inside = np.nonzero(_in_box(points, box))[0]
    return bool(len(inside)) and not _in_box(points[:1], box).any() and not _in_box(points[-1:], box).any()


def _chip_issues(labels, shapes, boxes, frame) -> list[str]:
    issues, chips = [], []
    for label in labels:
        chip = _chip_of(label, shapes, boxes)
        if chip is not None and len(label.text.strip()) > BADGE_CHARS:
            chips.append((label, chip))
    owned = {id(chip) for _, chip in chips}
    for label, chip in chips:
        inner = _grown(chip.box, -0.005)
        crossing = [f"{o.text!r} {frame.span(o.box)}" for o in labels if o is not label and all(d * frame.ppu >= OVERLAP_PX for d in _depth(chip.box, o.box))
                    and not _contains(chip.box, o.box) and not _contains(o.box, chip.box)]
        if crossing:
            issues.append(_listing("text crosses chip border of", label, frame, crossing, chip.box))
        hits = [frame.name(shapes[k]) for k in _nearby(boxes, chip.box)
                if shapes[k] is not chip and id(shapes[k]) not in owned and shapes[k].stroke >= STROKE_OPACITY and not _contains(chip.box, shapes[k].box)
                and not (_contains(shapes[k].box, chip.box) and shapes[k].fill >= FILL_OPACITY) and _passes(shapes[k].samples, inner)]
        if hits:
            issues.append(_listing("stroke through chip of", label, frame, hits, chip.box))
    for i, (first, a) in enumerate(chips):
        for second, b in chips[i + 1:]:
            dx, dy = (d * frame.ppu for d in _depth(a.box, b.box))
            if a is not b and dx >= OVERLAP_PX and dy >= OVERLAP_PX and not _contains(a.box, b.box) and not _contains(b.box, a.box):
                issues.append(f"chips overlap {first.text!r} {frame.span(a.box)} x {second.text!r} {frame.span(b.box)} by {dx:.0f}x{dy:.0f}px")
    return issues


def strict_conflicts(scene) -> list[str]:
    """🚧 Text-on-text, strokes through text or chips, fills partly under text, frame, caption-band and size violations of the current frame."""
    frame = Frame()
    labels, shapes = _collect(scene)
    issues = _frame_issues(labels, frame) + _text_issues(labels, frame)
    if len([l for l in labels if l.zone == "caption"]) > 2:
        issues.append("caption has more than 2 lines")
    if not shapes:
        return issues
    boxes = np.array([s.box for s in shapes])
    issues += _chip_issues([l for l in labels if l.zone != "caption"], shapes, boxes, frame)
    reach = TOUCH_PX / frame.ppu
    for label in labels:
        lines, fills = _shape_hits(label, shapes, _nearby(boxes, _grown(label.box, reach)), frame)
        for verb, hits in (("stroke through", lines), ("filled shape under", fills)):
            if hits:
                issues.append(_listing(verb, label, frame, hits))
    return issues


def strict_section(conflicts: list[str]) -> str:
    """📝 Markdown block appended to a keyframe's report section."""
    return "\n### strict\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n"
