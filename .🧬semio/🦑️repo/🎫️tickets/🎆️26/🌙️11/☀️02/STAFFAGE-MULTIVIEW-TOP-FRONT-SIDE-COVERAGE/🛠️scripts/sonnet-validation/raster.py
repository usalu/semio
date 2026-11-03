import sys
import resvg_py
src, dst, width = sys.argv[1], sys.argv[2], int(sys.argv[3])
try:
    png = resvg_py.svg_to_bytes(svg_path=src, width=width)
except ValueError as e:
    print("with width failed", e)
    png = resvg_py.svg_to_bytes(svg_path=src, dpi=int(width / 370 * 25.4))
open(dst, "wb").write(bytes(png))
