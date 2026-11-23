"""🖨️ vegetation/agave, 1:200 drawing at the end of its range (1:150): the rib plate of the outer half of the lowest leaves 6 and 7 showed a sliver of 2.7 mm beside the blade plate (0.198 mm on paper with the pen, below the 0.2 mm that print) in the side and front elevations.

The rib plate of these two leaf parts is drawn 3 mm thicker on both faces (82 mm instead of 76 mm at the knee, the lens of the 1:200 model is 80 mm there), in all three views, so the visible strip prints. `python3 rework-agave-print150.py <asset document json>` (candidate first, then the catalog file; the file is re-read immediately before writing).
"""
import re
import sys

BEFORE = {"leaf-06-a-p2-k1": (0.02584, 0.10189, -0.01557, 0.03951), "leaf-07-a-p2-k1": (0.02071, 0.09688, -0.01594, 0.03921)}
GROW = 0.003


def main():
    """▶️ Widens the span of the two rib plates in every view, as a text edit that leaves every other byte of the file alone."""
    path = sys.argv[1]
    with open(path, encoding="utf8") as f:
        text = f.read()
    for ident, (z0, z1, t0, t1) in BEFORE.items():
        old = re.compile(r'"z0": %s,(\s*)"z1": %s,(\s*)"zTip": \[%s, %s\]' % tuple(re.escape(str(v)) for v in (z0, z1, t0, t1)))
        new = r'"z0": %s,\1"z1": %s,\2"zTip": [%s, %s]' % (round(z0 - GROW, 5), round(z1 + GROW, 5), round(t0 - GROW, 5), round(t1 + GROW, 5))
        text, count = old.subn(new, text)
        if count != 3:
            raise SystemExit(f"{ident}: expected its span in 3 views, found {count} (already applied?)")
    with open(path, "w", encoding="utf8") as f:
        f.write(text)
    print("applied to", path)


if __name__ == "__main__":
    main()
