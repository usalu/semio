import json
import re
import subprocess
import sys

ROOT = r"C:\git\semio"
REL = "🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py"
hist = json.load(open(sys.argv[1], encoding="utf-8"))
out_path = sys.argv[2]

base = subprocess.run(["git", "show", "HEAD:" + REL], cwd=ROOT, capture_output=True).stdout.decode("utf-8")
s = base.replace("\r\n", "\n")
crlf = "\r\n" in base
print("base lines", s.count("\n"), "crlf", crlf)

edits = {}
for e in hist:
    if e[3] == "Edit" and e[0]:
        edits[e[0]] = e[4]
patch_08 = None


def apply_edit(ts):
    global s
    inp = edits[ts]
    old, new = inp["old_string"].replace("\r\n", "\n"), inp["new_string"].replace("\r\n", "\n")
    n = s.count(old)
    assert n >= 1 if inp.get("replace_all") else n == 1, (ts, n, old[:80])
    s = s.replace(old, new)
    print("edit ok", ts)


def sub_line(pattern, repl):
    global s
    s2, n = re.subn(pattern, repl, s, flags=re.M)
    assert n >= 1, pattern
    s = s2
    print("sed ok", pattern[:50], n)


apply_edit("2026-11-01T00:39:08.199Z")
lines = s.split("\n")
lines[175:181] = ["        louvre = _blinds(hs)"]
s = "\n".join(lines)
sub_line(r"Create\(envelope\), Create\(louvre\)", "Create(envelope), FadeIn(louvre)")
sub_line(r"^person_glyph = _manim_module\.person_glyph$", "staffage = _manim_module.staffage\nstaffage_motion = _manim_module.staffage_motion\nstaffage_height = _manim_module.staffage_height")
sub_line(r"blind = staffage\(cast, unit", "blind = staffage_motion(cast, unit")
sub_line(r"_manim_module\.object_silhouette\(", "_manim_module.icon_glyph(")


def rep(a, b):
    global s
    assert s.count(a) == 1, a[:60]
    s = s.replace(a, b)


rep("staffage_height = _manim_module.staffage_height\n", "staffage_height = _manim_module.staffage_height\nfitted_staffage = _manim_module.fitted_staffage\n")
a = s.index("        wheel = Circle(radius=0.95")
b = s.index("        wheel_tag = Text(")
s = s[:a] + '''        wheel = fitted_staffage("cooling-summer-strategy-wheel", width=1.9, height=1.9, rate=0.0, loop=True).move_to(wheel_c)
''' + s[b:]
a = s.index("        hx = Rectangle(width=1.5")
b = s.index("        hx_tag = Text(")
s = s[:a] + '''        hx = fitted_staffage("cooling-summer-strategy-exchanger", width=1.5, height=1.9).move_to(np.array([-0.3, 0.4, 0.0])).set_z_index(-1)
''' + s[b:]
a = s.index("        def _humidifier(pos, color=P_BLUE):")
b = s.index("        hum_tag = Text(")
s = s[:a] + '''        def _humidifier(pos):
            return fitted_staffage("cooling-summer-strategy-humidifier", width=0.9, height=1.05, rate=1.0, loop=True).move_to(pos)

        hum_sup = _humidifier(np.array([2.0, y_sup, 0.0]))
        hum_exh = _humidifier(np.array([2.0, y_exh, 0.0]))
''' + s[b:]
a = s.index("        heater = Rectangle(width=0.72")
b = s.index("        # Below the regeneration heater")
s = s[:a] + '''        heater = fitted_staffage("cooling-summer-strategy-heater", width=0.72, height=1.05, rate=0.0).move_to(heater_c)

''' + s[b:]
rep("            wheel, wheel_spokes, wheel_tag, hx, hx_diag, hx_tag,\n            hum_sup, hum_exh, hum_tag, heater, coil,", "            wheel, wheel_tag, hx, hx_tag,\n            hum_sup, hum_exh, hum_tag, heater,")
rep("            Create(wheel), Create(wheel_spokes), FadeIn(wheel_tag),\n            Create(hx), Create(hx_diag), FadeIn(hx_tag),", "            FadeIn(wheel), FadeIn(wheel_tag),\n            FadeIn(hx), FadeIn(hx_tag),")
rep("            FadeIn(heater), Create(coil),", "            FadeIn(heater),")
rep('''            return Rotate(wheel_spokes, angle=-TAU * run_time / 5.2, about_point=wheel_c,
                          run_time=run_time, rate_func=linear)''', '''            wheel.rate = 1.0
            return Wait(run_time)''')
rep("            heater.animate.set_fill(P_RED, opacity=0.25),\n", "")
rep("        self.play(FadeIn(sun, scale=0.7), FadeIn(solar_tag), Create(solar_duct), *dec(1.0), run_time=1.0)", "        heater.rate = 3.0\n        self.play(FadeIn(sun, scale=0.7), FadeIn(solar_tag), Create(solar_duct), *dec(1.0), run_time=1.0)")

apply_edit("2026-11-01T09:23:00.082Z")
rep("hx_tag.move_to(np.array([-0.2, -1.05, 0.0]))", "hx_tag.move_to(np.array([-0.6, -1.05, 0.0]))") if "hx_tag.move_to(np.array([-0.2, -1.05, 0.0]))" in s else print("sed 10:33 hx skipped")
if "hum_tag.move_to(np.array([2.25, -1.05, 0.0]))" in s:
    rep("hum_tag.move_to(np.array([2.25, -1.05, 0.0]))", "hum_tag.move_to(np.array([2.6, -1.05, 0.0]))")
else:
    print("sed 10:33 hum skipped")
for ts in ("2026-11-01T13:28:12.996Z", "2026-11-01T13:28:13.598Z", "2026-11-01T15:50:50.194Z", "2026-11-01T15:50:53.341Z", "2026-11-01T15:50:54.177Z",
           "2026-11-01T17:22:42.773Z", "2026-11-01T17:22:44.359Z", "2026-11-01T17:22:44.814Z", "2026-11-01T17:22:45.935Z", "2026-11-01T17:30:22.372Z"):
    apply_edit(ts)

open(out_path, "w", encoding="utf-8", newline="").write(s.replace("\n", "\r\n") if crlf else s)
print("done lines", s.count("\n"))
