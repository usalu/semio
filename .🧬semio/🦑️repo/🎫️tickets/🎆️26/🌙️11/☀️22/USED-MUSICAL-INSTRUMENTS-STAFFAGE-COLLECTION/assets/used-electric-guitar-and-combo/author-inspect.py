"""🔍️ Prints the world extent of every body solid of a scale in the rest pose (debug aid of the author)."""
import importlib.util, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("author", os.path.join(HERE, "author-guitar-combo.py"))
A = importlib.util.module_from_spec(spec); spec.loader.exec_module(A)
scale = int(sys.argv[1]) if len(sys.argv) > 1 else 50
for s in A.body_solids(scale):
    fr = A.REST[s["bone"]]
    w = [A.G.to_world(fr, p) for p in A.G.placed_points(s)]
    lo = [min(p[i] for p in w) for i in range(3)]; hi = [max(p[i] for p in w) for i in range(3)]
    print(f"[DEBUG] {s['id']:28s} x {lo[0]:.3f}..{hi[0]:.3f} y {lo[1]:.3f}..{hi[1]:.3f} z {lo[2]:.3f}..{hi[2]:.3f}")
print("[DEBUG] cradle", A.CRADLE_Y, "rest cable abs", [round(a,2) for a in A.CABLE_REST_ABS])
