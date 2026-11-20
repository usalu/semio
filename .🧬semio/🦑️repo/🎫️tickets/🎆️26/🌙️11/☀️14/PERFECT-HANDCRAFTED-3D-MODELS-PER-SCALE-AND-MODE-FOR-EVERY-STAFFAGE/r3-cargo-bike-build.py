"""🛺 Round 3 build of vehicles/cargo-bike: runs the hand-stated scale files and writes the volumes source `volumes/vehicles/cargo-bike.json` and the three drawings of every authored scale with their decisions to `🗑️generated/r3-cargo-bike/drawings.json` (read by r3-cargo-bike-roots.py).

Usage: python3 r3-cargo-bike-build.py [50] [100] [200] [500]   (no argument: every scale that has its file)"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, "volumes", "vehicles", "cargo-bike.json")
OUT = os.path.join(T, "🗑️generated", "r3-cargo-bike")


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


if __name__ == "__main__":
    wanted = [a for a in sys.argv[1:] if a.isdigit()] or [d for d in ("50", "100", "200", "500") if os.path.exists(os.path.join(T, f"r3-cargo-bike-s{d}.py"))]
    source = json.load(open(SRC, encoding="utf8"))
    os.makedirs(OUT, exist_ok=True)
    path2d = os.path.join(OUT, "drawings.json")
    drawings = json.load(open(path2d, encoding="utf8")) if os.path.exists(path2d) else {}
    for d in wanted:
        m = load(f"r3-cargo-bike-s{d}")
        S = m.build()
        source["volumes"][d] = S.volume(m.DECISIONS)
        g = S.drawing()
        if hasattr(m, "DRAWN"):
            drawings[d] = {"geometry": g, "decisions": m.DRAWN, "pens": S.pens}
        print(d, "body", len(S.body), "thermal", len(S.thermal), "structure", len(S.structure), "shapes", {k: len(v) for k, v in g.items()})
    with open(SRC, "w", encoding="utf8") as fh:
        json.dump(source, fh, ensure_ascii=False)
        fh.write("\n")
    with open(path2d, "w", encoding="utf8") as fh:
        json.dump(drawings, fh, ensure_ascii=False)
        fh.write("\n")
