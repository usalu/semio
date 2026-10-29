import importlib, sys
sys.path.insert(0, "C:/git/semio")
d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
r = d.region([[(0, 0), (1, 0), (1, 1), (0, 1)], [(0.2, 0.2), (0.4, 0.2), (0.4, 0.4), (0.2, 0.4)]], "#fff", 0.5)
print("ok", type(r))
