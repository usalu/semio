"""Compare the dependency-free frame parser with Manim's SVGMobject import (third-party reference) on every available cast."""

import importlib
import sys

import numpy as np
from manim import SVGMobject, VGroup

sys.path.insert(0, ".")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")


def reference(folder, frame, unit, anchor):
    index = S._index(folder)
    raw = SVGMobject(str(folder / index["frames"][frame]), should_center=False, height=None, width=None)
    centre = float(raw.get_center()[1])
    group = VGroup(*raw.submobjects)
    scale = unit / index["mmPerMetre"]
    ox, oy = index["originMm"]
    out = []
    for member in group.family_members_with_points():
        p = member.points
        placed = np.column_stack((anchor[0] + (p[:, 0] - ox) * scale, anchor[1] + (oy - 2 * centre + p[:, 1]) * scale))
        width = float(np.max(member.get_stroke_width()))
        out.append((placed, member.get_fill_opacity(), member.get_stroke_opacity(), str(member.get_fill_color()), str(member.get_stroke_color()),
                    max(width * scale / S._STROKE_UNIT, S._MIN_STROKE) if width > 0 else 0.0))
    return out


unit, anchor = 0.4, np.array([1.0, -2.0, 0.0])
checked = problems = 0
folders = {}
for key in sorted(S._cache_root().iterdir(), key=lambda k: -k.stat().st_mtime):
    for found in key.glob("*/🔣️.json"):
        folders.setdefault(found.parent.name, found.parent)
for cast, folder in folders.items():
    if sys.argv[1:] and cast not in sys.argv[1:]:
        continue
    index = S._index(folder)
    for frame in sorted({0, len(index["frames"]) // 2, len(index["frames"]) - 1}):
        ref = reference(folder, frame, unit, anchor)
        new = [m for m in S._place(folder, frame, unit, anchor, None, 1.0).family_members_with_points()]
        checked += 1
        if len(ref) != len(new):
            print("COUNT", cast, frame, len(ref), len(new))
            problems += 1
            continue
        for (points, fo, so, fc, sc, width), member in zip(ref, new):
            ok = points.shape == member.points[:, :2].shape and np.allclose(points, member.points[:, :2], atol=1e-9)
            ok = ok and abs(fo - member.get_fill_opacity()) < 5e-3 and abs(float(np.max(member.get_stroke_width())) - width) < 1e-6
            if width > 0:
                ok = ok and abs(so - member.get_stroke_opacity()) < 5e-3 and sc == str(member.get_stroke_color())
            if fo > 0:
                ok = ok and fc == str(member.get_fill_color())
            if not ok:
                problems += 1
                print("DIFF", cast, frame, points.shape, member.points.shape, fo, member.get_fill_opacity(), width, float(np.max(member.get_stroke_width())), fc, str(member.get_fill_color()))
                break
print("checked frames", checked, "problems", problems)
