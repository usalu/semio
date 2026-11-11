"""⏸️ Verify whether keyed one-argument live updates animate narration waits."""
from manim import Scene, Square, Wait

scene = Scene()
live = Square()
live.add_updater(lambda mob: None)
scene.add(live)
scene.animations = [Wait(1)]
assert not scene.should_update_mobjects()
live.add_updater(lambda mob, dt: None)
scene.animations = [Wait(1)]
assert scene.should_update_mobjects()
print({"keyed_one_argument_wait_dynamic": False, "time_based_wait_dynamic": True})
