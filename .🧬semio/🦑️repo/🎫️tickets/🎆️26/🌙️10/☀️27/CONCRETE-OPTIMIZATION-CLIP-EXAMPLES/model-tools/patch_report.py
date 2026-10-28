from pathlib import Path
base = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/model-tools")
p = base / "build_report.py"
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub('add("\n## 5. Pareto (south; WWR {10,25,40,55,70} % × insulation {12,20,30} cm × overhang {0,0.6} m)\n")', 'add("\n## 5. Pareto (south; WWR {" + ", ".join(f(100 * x, 0) for x in study["pareto"]["wwr"]) + "} % × insulation {" + ", ".join(f(100 * x, 0) for x in study["pareto"]["insulation"]) + "} cm × overhang {" + ", ".join(f(x, 1) for x in study["pareto"]["overhang"]) + "} m)\n")')
sub('add("\n## 6. Algorithms (south surface; start WWR 75 %, overhang 0.1 m = 712 kWh/a)\n")', 'add(f"\n## 6. Algorithms (south surface; start WWR {f(100 * g[0][\'design\'][\'wwr\'], 0)} %, overhang {f(g[0][\'design\'][\'overhang\'], 1)} m = {f(g[0][\'electricity\'], 0)} kWh/a)\n")')
sub('; the second iterate overshoots the valley floor (WWR {f(100 * g[1][\'design\'][\'wwr\'], 0)} % < {f(100 * s[\'minimum\'][\'design\'][\'wwr\'], 0)} %) and recovers.")', '; electricity rises once ({\', \'.join(f(x[\'electricity\'], 0) for x in g[3:6])}) before the descent recovers.")')
sub('add("## Test results\n")', '''add("## Geometric shading (since the coordinator correction)\n")
sk = m.sun_samples(180, "summer")
add("The overhang slab sits on the wall top (3.0 m), the window head at 2.7 m (`room.head`), so 0.3 m of wall stay above the window — the same numbers as `🎨️visuals/🏠️room/🐍️.py` (`HEAD = 2.7`, `HEIGHT = 3.0`, window height = min(area / 8 m, HEAD − 0.05), `x_tip = x_o − d`). They agree; only the toolkit's sun position differs (see below).\n")
add("Shaded share of the window height = clamp((d · tan(profile angle) − 0.3 m) / window height, 0, 1) per sun position, with tan(profile) = tan(altitude) / cos(angle across the façade normal); sun positions behind the façade (across ≥ 89°) or below 3° give no direct sun. `shading_winter` / `shading_summer` are the beam-weighted averages (weight = cos(altitude) · cos(across), the beam on a vertical surface) over the hourly solstice sun positions noon ± 4 h that shine on the façade. Solar gain = window · g · irradiance · (1 − direct_share · shading).\n")
add("South, summer solstice sun positions: " + "; ".join(f"{x['hour']:+d} h: alt {f(x['altitude'])}°, across {f(x['across'])}°, profile {f(x['profile'])}°, weight {f(x['weight'], 2)}" for x in sk) + ".\n")
add(f"Baseline (d = 0.6 m, WWR 45 %): shading_winter {f(b['shading_winter'], 2)} (a 0.6 m overhang casts only {f(0.6 * __import__('math').tan(__import__('math').radians(m.sun_state(180, 'winter')['profile'])), 2)} m of shadow at the 19° winter sun, less than the 0.3 m gap plus window), shading_summer {f(b['shading_summer'], 2)}.\n")
add("Toolkit parity: the toolkit's `sun_state` picks the single sun-path point nearest to the façade bearing (noon for south; model `sun_state(orientation, season)` is the same function). The model averages the hourly positions instead, so at south in summer it reports slightly more shading than the noon ray alone (the toolkit already bends the drawn ray so exactly round(7 · shading) rays are blocked).\n")
add("## Test results\n")''')
p.write_text(s, encoding="utf-8", newline="\n")
