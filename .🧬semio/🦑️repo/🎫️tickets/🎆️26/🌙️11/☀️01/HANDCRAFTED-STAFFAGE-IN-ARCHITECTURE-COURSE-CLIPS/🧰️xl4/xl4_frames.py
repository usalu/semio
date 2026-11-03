"""Pixel identity of tower drawings across code versions. Usage: xl4_frames.py save|compare NAME [storeys]

save renders six variants with real Cairo at the default 1080p frame and stores the pixel arrays; compare renders them again and diffs against the stored ones."""
import hashlib
import os
import sys
import time

import numpy as np
from manim import Camera

import xl4_common as C

visuals = C.load()
mode, name = sys.argv[1], sys.argv[2]
storeys = int(sys.argv[3]) if len(sys.argv) > 3 else 2
root = os.path.join(os.environ.get("XL4_OUT", "."), f"frames-{name}-{storeys}.npz")
fit = C.program_fit()
other = {"at": np.array([-1.0, 0.3, 0.0]), "scale": fit["scale"] * 0.8}
draw = visuals.NakaginDrawing()
base = dict(types=False, towers=(0,), where=lambda s, n=storeys: s.tower == 0 and s.storey < n, core=False, podium=False, capital=False)
keys = sorted(draw.tower_axon(**fit, **base).capsules)


def paint(group):
    camera = Camera(background_color=visuals.BACKGROUND)
    camera.capture_mobjects([group])
    return camera.pixel_array.copy()


variants = {
    "plain": lambda: draw.tower_axon(**fit, **base),
    "tint": lambda: draw.tower_axon(**fit, **{**base, "tint": lambda k, ks=keys[::3]: "#c0392b" if k in ks else None}),
    "types": lambda: draw.tower_axon(**fit, **{**base, "types": True}),
    "lit": lambda: draw.tower_axon(**fit, **base, lit=lambda path, ks=keys[::4]: path in ks),
    "other": lambda: draw.tower_axon(**other, **base),
    "again": lambda: draw.tower_axon(**fit, **base),
    "capsules": lambda: visuals._engine.VGroup(*draw.tower_capsules(keys[:6], "#2e86c1", **fit, towers=(0,), where=base["where"], core=False, podium=False, capital=False)),
}
frames = {}
for label, make in variants.items():
    began = time.time()
    frames[label] = paint(make())
    print(f"[frames] {label} {time.time() - began:.1f}s sha={hashlib.sha1(frames[label].tobytes()).hexdigest()[:12]} rss={C.rss() / 1024**2:.0f}MB peak={C.peak() / 1024**2:.0f}MB", flush=True)
if mode == "save":
    np.savez_compressed(root, **frames)
    print(f"[frames] saved {root}", flush=True)
else:
    stored = np.load(root)
    bad = 0
    for label, pixels in frames.items():
        diff = np.abs(pixels.astype(int) - stored[label].astype(int))
        count = int((diff.sum(axis=2) > 0).sum())
        bad += count
        print(f"[frames] {label}: differing pixels={count} max={int(diff.max())} covered={int((stored[label][..., :3] != stored[label][0, 0, :3]).any(axis=2).sum())}", flush=True)
    print(f"[frames] RESULT {'IDENTICAL' if bad == 0 else 'DIFFERENT'} total differing={bad}", flush=True)
