"""🧊 Verify pausing a known-unchanged live group preserves pixels and static caching."""
from contextlib import contextmanager
from manim import Camera, FadeOut, Scene, Square, VGroup
import numpy as np

@contextmanager
def frozen_updaters(mobject):
    """🧊 Restore the same live updaters after a known-unchanged animation interval."""
    saved = tuple(mobject.updaters)
    for updater in saved:
        mobject.remove_updater(updater)
    try:
        yield
    finally:
        for updater in saved:
            mobject.add_updater(updater)

scene = Scene()
live = VGroup(Square(side_length=1, fill_color="#336699", fill_opacity=1))
calls = []
updater = lambda mob: calls.append(1)
live.add_updater(updater)
heavy = VGroup(*[Square(side_length=.04, stroke_width=0, fill_color="#dd6622", fill_opacity=1).shift([i / 40 - 1.25, -1, 0]) for i in range(100)])
rectangle = Square(side_length=3)
scene.add(live, heavy, rectangle)
animation = FadeOut(rectangle)
before = scene.get_moving_mobjects(animation)
camera = Camera(pixel_width=320, pixel_height=180, frame_width=14.222222222222221, frame_height=8, background_opacity=1)
camera.capture_mobjects(scene.mobjects)
reference = np.array(camera.get_image())
with frozen_updaters(live):
    during = scene.get_moving_mobjects(animation)
    camera.reset()
    camera.capture_mobjects(scene.mobjects)
    assert np.array_equal(reference, np.array(camera.get_image()))
    assert not any(member in during for member in heavy.get_family())
    assert live.updaters == []
assert any(member in before for member in heavy.get_family())
assert live.updaters == [updater]
live.update(0)
assert calls == [1]
print({"cairo_pixels": "exact", "moving_heavy_before": sum(member in before for member in heavy.get_family()), "moving_heavy_during": sum(member in during for member in heavy.get_family()), "updater_restored": len(live.updaters)})
