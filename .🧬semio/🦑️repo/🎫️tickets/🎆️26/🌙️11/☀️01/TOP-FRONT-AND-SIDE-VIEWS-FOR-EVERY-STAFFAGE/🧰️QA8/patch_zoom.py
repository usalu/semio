import os

p = os.path.join(os.path.dirname(os.path.abspath(__file__)), "zoom.py")
s = open(p, encoding="utf-8").read()
s = s.replace(
    "        parts.append(im.crop((x0 + max(0, xx.min() - 8), y0 + max(0, yy.min() - 8), x0 + xx.max() + 8, y0 + yy.max() + 8)))",
    "        mm = dpi / 25.4\n        xmax = min(xx.max(), xx.min() + (WM * 1000 / scale + 3) * mm)\n        ymax = min(yy.max(), yy.min() + (HM * 1000 / scale + 5) * mm)\n        parts.append(im.crop((x0 + max(0, xx.min() - 8), y0 + max(0, yy.min() - 8), x0 + xmax + 8, y0 + ymax + 8)))",
)
s = s.replace("per = int(sys.argv[6])", "per = int(sys.argv[6])\nWM = float(sys.argv[8]) if len(sys.argv) > 8 else 9.0")
open(p, "w", encoding="utf-8").write(s)
print(s.count("WM"))
