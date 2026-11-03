import importlib.util
import numpy as np

path = "🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py"
spec = importlib.util.spec_from_file_location("calc", path)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
metre = 1.6 / 2.5
for mirror in (False, True):
    g = m._cast("shared-radiator-thermostat", 0.71 * metre, motion=True, anchor=np.array([0.0, 0.0, 0.0]), mirror=mirror)
    print(mirror, "left", g.get_left()[0], "right", g.get_right()[0], "bottom", g.get_bottom()[1], "top", g.get_top()[1])
wall = 0.35 * 1.6 / 2.5
print("wall inner offset", wall / 2, "metre", metre)
