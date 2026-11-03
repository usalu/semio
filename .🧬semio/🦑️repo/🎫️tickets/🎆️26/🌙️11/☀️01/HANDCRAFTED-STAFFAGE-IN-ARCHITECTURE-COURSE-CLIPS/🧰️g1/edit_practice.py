import time
p = "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py"
s = open(p, encoding="utf8").read()
def rep(a, b):
    global s
    assert s.count(a) == 1, a
    s = s.replace(a, b)
rep("PUMP_SPIN = 4.0\n", "VALVE_CLOSE_S = 6.0\nVALVE_OPEN_AT_S = 1.0\n")
i = s.index("def _valve(center, opening, gate):")
j = s.index("def _thermo(")
s = s[:i] + '''def _valve(center, opening):
    """🎚️ Handcrafted thermostatic radiator valve on the riser — its knob follows the ``opening`` tracker (1 wide open … 0 shut) through the close-down clip."""
    group = fitted_staffage("demand-practice-valve", width=0.3, height=0.42, rate=0.0).move_to(np.array(center, dtype=float))
    group.add_updater(lambda m: m.seek(VALVE_OPEN_AT_S + (1.0 - float(np.clip(opening.get_value(), 0.0, 1.0))) * VALVE_CLOSE_S))
    return group


''' + s[j:]
i = s.index("        pump = VGroup(Circle(radius=0.30")
j = s.index("        pump_tag = ")
s = s[:i] + '        pump = fitted_staffage("demand-practice-pump", width=0.6, height=0.6, rate=1.0, loop=True).move_to(pump_c)\n' + s[j:]
rep("        valve_gate = ValueTracker(0.0)\n", "")
rep("valves = [_valve(np.array([xi, -0.28, 0.0]), op, valve_gate) for", "valves = [_valve(np.array([xi, -0.28, 0.0]), op) for")
rep("vl_th[\"column\"], *[v[1] for v in valves])", "vl_th[\"column\"])")
rep("FadeIn(pump), FadeIn(impeller), FadeIn(pump_tag)", "FadeIn(pump), FadeIn(pump_tag)")
rep("self.play(*[FadeIn(v[0]) for v in valves], valve_gate.animate.set_value(1.0), *heat(1.4), run_time=1.4)", "self.play(*[FadeIn(v) for v in valves], *heat(1.4), run_time=1.4)")
rep("gauge_gate.animate.set_value(0.0), valve_gate.animate.set_value(0.0),\n                  *[FadeOut(v[0]) for v in valves]", "gauge_gate.animate.set_value(0.0),\n                  *[FadeOut(v) for v in valves]")
for _ in range(8):
    try:
        open(p, "w", encoding="utf8", newline="").write(s); break
    except OSError:
        time.sleep(1)
print("ok")
