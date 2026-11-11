"""🔬 Verify keyed live updaters invalidate later Manim static-frame objects."""
from manim import Scene, Square, VGroup, FadeOut

scene = Scene()
live = VGroup(Square(side_length=.2))
updater = lambda mob: None
live.add_updater(updater)
heavy = VGroup(*[Square(side_length=.01).shift([i / 100, 0, 0]) for i in range(100)])
rectangle = Square()
scene.add(live, heavy, rectangle)
animation = FadeOut(rectangle)
moving = scene.get_moving_mobjects(animation)
moving_heavy = sum(member in moving for member in heavy.get_family())
live.remove_updater(updater)
static_case = scene.get_moving_mobjects(animation)
static_heavy = sum(member in static_case for member in heavy.get_family())
assert moving_heavy == len(heavy.get_family())
assert static_heavy == 0
print({"active_updater_moving_heavy": moving_heavy, "cleared_updater_moving_heavy": static_heavy, "total_heavy_family": len(heavy.get_family())})
