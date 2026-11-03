import re
import sys

src, dst = sys.argv[1], sys.argv[2]
s = open(src, encoding="utf-8", newline="").read()


def rep(a, b):
    global s
    assert s.count(a) == 1, a[:70]
    s = s.replace(a, b)


rep('width=1.6, height=1.0, rate=0.0).move_to(np.array([-3.6, -1.9, 0.0]))', 'width=1.6, height=0.75, rate=0.0).move_to(np.array([-3.6, -1.8, 0.0]))')
rep("sun = sun_glyph(np.array([-5.0, -1.55, 0.0]))", "sun = sun_glyph(np.array([-5.0, -1.8, 0.0]))")
rep("np.array([-4.72, -1.62 - 0.1 * k, 0.0]), np.array([-4.35, -1.85 - 0.12 * k, 0.0])", "np.array([-4.72, -1.87 - 0.1 * k, 0.0]), np.array([-4.35, -2.1 - 0.12 * k, 0.0])")
rep("solar_tag.move_to(np.array([-1.0, -2.3, 0.0]))", "solar_tag.move_to(np.array([0.0, -2.3, 0.0]))")
rep("        components = VGroup(\n            wheel, wheel_tag, hx, hx_tag,\n            hum_sup, hum_exh, hum_tag, heater,\n        )\n\n", "")
rep("import sys as _sys\n", "")
open(dst, "w", encoding="utf-8", newline="").write(s)
print("finish ok")
