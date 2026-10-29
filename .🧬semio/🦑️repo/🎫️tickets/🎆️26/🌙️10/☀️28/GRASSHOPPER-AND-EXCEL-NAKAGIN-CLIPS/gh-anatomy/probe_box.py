import importlib.util, sys
from pathlib import Path
ROOT = Path(r"C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py")
spec = importlib.util.spec_from_file_location("ghm", ROOT); m = importlib.util.module_from_spec(spec); sys.modules["ghm"] = m; spec.loader.exec_module(m)
d = m.canvas("anatomy"); s = m.evaluate(d)
b = s.value("cb", "Box"); print(b, b.volume)
for o in ("w","bl","bh","st","cnt","fl","tg","vl"):
    print(o, d.objects[o].settings, [m.describe(x) for x in s.items(o)][:3] if o in ("fl","tg","vl") else s.value(o))
print(m.panel_rows(s.tree("flp")), m.panel_rows(s.tree("tgp")), m.panel_rows(s.tree("vlp")), m.panel_rows(s.tree("cbwp")), m.panel_rows(s.tree("cbep")))
t = m.aggregate_tree(); br = t.branch((0,0,3,1)); print(br, br[0].tag)
print(d.spec("cb").nickname, d.spec("se").inputs[2])
