"""🔎 Frame-by-frame collision audit for the Physical Fundamentals beats.

Hooks ``Scene.update_to_time`` so every sampled frame — including mid-animation
frames — is checked for:
  text×text   overlapping text boxes
  text×shape  a visible stroke/fill outline (particle, arrow, bar, ray, line) crossing a text box
  panel×shape a non-panel shape reaching into the formula panel
  caption×shape a shape reaching into the subtitle bar
  offframe    a shape or text leaving the 16:9 frame

Usage: .venv/bin/python checks/collision_check.py Beat1_EnergieImAlltag [Beat2_Leistung ...]
"""
import sys
from collections import OrderedDict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
PF = ROOT / "tutorial" / "energy" / "demand" / "1_physical_fundamentals"
sys.path.insert(0, str(PF))

import numpy as np
from manim import Scene, Text, VMobject, VectorizedPoint, config, tempconfig

import scene_1

EVERY = 2
INSET = 0.03
TT_INSET = 0.05
FRAME_X, FRAME_Y = 7.12, 4.0
_T = np.linspace(0.0, 1.0, 9)
_B = np.stack([(1 - _T) ** 3, 3 * (1 - _T) ** 2 * _T, 3 * (1 - _T) * _T ** 2, _T ** 3], axis=1)


def _samples(mob):
    pts = mob.points
    if len(pts) < 4:
        return np.zeros((0, 3))
    n = (len(pts) // 4) * 4
    cubic = pts[:n].reshape(-1, 4, 3)
    return np.einsum("tk,ckd->ctd", _B, cubic).reshape(-1, 3)


def _box(mob, inset=0.0):
    return (mob.get_left()[0] + inset, mob.get_right()[0] - inset,
            mob.get_bottom()[1] + inset, mob.get_top()[1] - inset)


def _visible(mob):
    stroke = mob.get_stroke_width() > 0.3 and mob.get_stroke_opacity() > 0.08
    fill = mob.get_fill_opacity() > 0.03
    return stroke or fill


def _collect(scene):
    texts, shapes = [], []

    def walk(mob):
        if isinstance(mob, Text):
            if mob.width > 0.01 and mob.get_fill_opacity() > 0.08:
                texts.append(mob)
            return
        if isinstance(mob, VMobject) and not isinstance(mob, VectorizedPoint) and len(mob.points) >= 4 \
                and _visible(mob):
            shapes.append(mob)
        for sub in mob.submobjects:
            walk(sub)

    for top in scene.mobjects:
        walk(top)
    return texts, shapes


def _label(mob):
    if isinstance(mob, Text):
        return repr(getattr(mob, "text", "?"))
    c = mob.get_center()
    col = mob.get_stroke_color() if mob.get_stroke_opacity() > 0.08 else mob.get_fill_color()
    return f"{type(mob).__name__}[{col.to_hex()}]@({c[0]:.1f},{c[1]:.1f}) {mob.width:.2f}x{mob.height:.2f}"


def _zone_box(shapes, zone, shrink):
    boxes = [s for s in shapes if getattr(s, "_layout_zone", "") == zone and s.width > 1.0]
    if not boxes:
        return None, None
    box = boxes[-1]
    return box, _box(box, shrink)


def _inside(pts, box):
    l, r, b, t = box
    return np.any((pts[:, 0] > l) & (pts[:, 0] < r) & (pts[:, 1] > b) & (pts[:, 1] < t))


def audit(scene, t_now, report):
    texts, shapes = _collect(scene)
    samples = [(s, _samples(s)) for s in shapes]
    text_ids = {id(t) for t in texts}

    def hit(kind, a, b):
        key = (kind, _label(a), _label(b) if b is not None else "")
        if key not in report:
            report[key] = t_now

    for i, a in enumerate(texts):
        ab = _box(a, TT_INSET)
        for b in texts[i + 1:]:
            bb = _box(b, TT_INSET)
            if getattr(a, "text", None) == getattr(b, "text", None):
                continue
            if getattr(a, "_layout_zone", "") == "formula" and getattr(b, "_layout_zone", "") == "formula":
                continue
            if min(ab[1], bb[1]) > max(ab[0], bb[0]) and min(ab[3], bb[3]) > max(ab[2], bb[2]):
                hit("text×text", a, b)
    for txt in texts:
        tb = _box(txt, INSET)
        for shape, pts in samples:
            if len(pts) and _inside(pts, tb):
                hit("text×shape", txt, shape)
    panel_box, panel = _zone_box(shapes, "formula_box", -0.2)
    caption_box, caption = _zone_box(shapes, "caption_box", -0.02)
    for txt in texts:
        zone = getattr(txt, "_layout_zone", "")
        tb = _box(txt, INSET)
        for region, own, kind in ((panel, "formula", "panel×text"), (caption, "caption", "caption×text")):
            if region and zone != own and tb[1] > region[0] and tb[0] < region[1] and tb[3] > region[2] \
                    and tb[2] < region[3]:
                hit(kind, txt, None)
    for shape, pts in samples:
        if not len(pts):
            continue
        zone = getattr(shape, "_layout_zone", "")
        if panel and zone not in ("formula", "formula_box", "caption_box") and _inside(pts, panel):
            hit("panel×shape", shape, None)
        if caption and zone not in ("caption", "caption_box", "formula_box") and _inside(pts, caption):
            hit("caption×shape", shape, None)
        if np.any(np.abs(pts[:, 0]) > FRAME_X) or np.any(np.abs(pts[:, 1]) > FRAME_Y):
            hit("offframe", shape, None)
    for txt in texts:
        l, r, b, tp = _box(txt)
        if l < -FRAME_X or r > FRAME_X or b < -FRAME_Y or tp > FRAME_Y:
            hit("offframe", txt, None)


def run(beat_names):
    original = Scene.update_to_time
    reports = OrderedDict()
    state = {"n": 0, "report": None}

    def patched(self, t):
        original(self, t)
        state["n"] += 1
        if state["n"] % EVERY == 0:
            audit(self, float(self.renderer.time) + float(t), state["report"])

    Scene.update_to_time = patched
    original_wait_end = Scene.wait

    def patched_wait(self, *args, **kwargs):
        audit(self, float(self.renderer.time), state["report"])
        return original_wait_end(self, *args, **kwargs)

    Scene.wait = patched_wait
    media = sys.argv[1] if sys.argv[1].startswith("/") else None
    for name in beat_names:
        state["report"] = OrderedDict()
        with tempconfig({"quality": "low_quality", "disable_caching": True, "media_dir": media or "/tmp/pf_cc",
                         "write_to_movie": True, "verbosity": "ERROR", "progress_bar": "none"}):
            getattr(scene_1, name)().render()
        reports[name] = state["report"]
        print(f"\n=== {name}: {len(state['report'])} findings")
        for (kind, a, b), t in state["report"].items():
            print(f"  {t:6.1f}s  {kind:13s} {a}  ×  {b}")
    return reports


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("/")]
    run(args or [cls.__name__ for cls in scene_1.BEATS])
