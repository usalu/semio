"""🔩 Print the part ids and bounds of a cast: python parts.py CAST [PREFIX]."""
import importlib
import sys
sys.path.insert(0, "C:/git/semio")

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
cast = sys.argv[1]
parts = V.staffage_parts(cast, sys.argv[2] if len(sys.argv) > 2 else "")
print(cast, "height", V.staffage_height(cast), "width", V.staffage_width(cast))
for name, pts in parts.items():
    print(f"{name:40s} x[{pts[:,0].min():.3f},{pts[:,0].max():.3f}] y[{pts[:,1].min():.3f},{pts[:,1].max():.3f}]")
