import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("        half = np.array(")
b = src.index("        tip = centre + delta * scale")
new = '''        reach = [((end_rect[2] if delta[0] > 0 else end_rect[0]) - centre[0]) / delta[0] if abs(delta[0]) > 1e-9 else math.inf,
                 ((end_rect[3] if delta[1] < 0 else end_rect[1]) - centre[1]) / delta[1] if abs(delta[1]) > 1e-9 else math.inf, 1.0]
        scale = min(reach)
'''
src = src[:a] + new + src[b:]
open(path, "w", encoding="utf-8").write(src)
