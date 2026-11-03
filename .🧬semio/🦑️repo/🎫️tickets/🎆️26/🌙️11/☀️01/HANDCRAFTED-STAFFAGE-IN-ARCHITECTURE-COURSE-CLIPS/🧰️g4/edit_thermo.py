import re, pathlib
p = pathlib.Path("🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py")
s = p.read_text(encoding="utf-8")
a = s.index("def _thermo(bottom, height, level, gate):")
b = s.index("class Beat2_HydraulischerAbgleich")
s = s[:a] + s[b:]
s = s.replace("_thermo(np.array([x + 0.84, floor_top + 0.30, 0.0]), 1.0, lv, room_gate)", "thermometer_glyph(np.array([x + 0.84, floor_top + 0.30, 0.0]), height=1.0, tracker=lv, ramp=THERMO_RAMP, gate=room_gate, tick_color=CALC_CYAN)")
s = s.replace("_thermo(np.array([-6.45, -0.92, 0.0]), 1.0, vl, vl_gate)", "thermometer_glyph(np.array([-6.45, -0.92, 0.0]), height=1.0, tracker=vl, ramp=THERMO_RAMP, gate=vl_gate, tick_color=CALC_CYAN)")
s = s.replace('th["frame"]', 'th["group"]').replace('vl_th["frame"]', 'vl_th["group"]')
p.write_text(s, encoding="utf-8", newline="")
