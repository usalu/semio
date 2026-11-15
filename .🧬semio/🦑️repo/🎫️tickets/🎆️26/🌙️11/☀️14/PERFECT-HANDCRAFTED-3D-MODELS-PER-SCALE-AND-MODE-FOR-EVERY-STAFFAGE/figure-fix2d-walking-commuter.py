"""Figure kit 2D repair (inputs kept): walking-commuter plan 'waist' ellipse (coat at hem level) is narrower in x (0.222) than the coat hem drawn in elevation (x -0.158..0.151 = 0.309): widen to the hem, centred on it, at all four scales."""
import re, shutil, os
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/walking-commuter/🔣️.json"
B = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated/figure/backup/walking-commuter.asset.before-2d-fix.json")
if not os.path.exists(B): shutil.copy(A, B)
L = open(A).read().split("\n"); n = 0
for i, l in enumerate(L):
    if '"id": "waist"' in l and '"rx": 0.1105' in L[i-2]:
        L[i-2] = L[i-2].replace("0.1105", "0.1545"); L[i-4] = L[i-4].replace('"cx": 0,', '"cx": -0.0035,'); n += 1
open(A, "w").write("\n".join(L)); print("edited", n)
