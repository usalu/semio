import importlib.util
import sys
from pathlib import Path

ROOT = Path(r"C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py")
spec = importlib.util.spec_from_file_location("ghm", ROOT)
m = importlib.util.module_from_spec(spec)
sys.modules["ghm"] = m
spec.loader.exec_module(m)
out = []
for key in m.canvas_keys():
    d = m.canvas(key)
    s = m.evaluate(d)
    out.append(f"== {key}: {len(d.objects)} objects")
    for oid in d.objects:
        st = s.state(oid)
        if st != "ok":
            out.append(f"  {oid} {st} {[x['en'] for x in s.msgs(oid)]}")
        spec_ = d.spec(oid)
        if spec_.kind == "panel":
            t = s.tree(oid)
            out.append(f"  panel {oid}: {t.path_count} paths, {t.count} items, first {[m.describe(i) for i in t.items()[:6]]}")
Path(sys.argv[1]).write_text("\n".join(out), encoding="utf-8")
