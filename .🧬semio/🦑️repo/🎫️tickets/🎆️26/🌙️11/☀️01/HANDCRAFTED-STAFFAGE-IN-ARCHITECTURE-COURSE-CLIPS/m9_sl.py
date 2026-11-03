from pathlib import Path

path = Path("🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py")
text = open(str(path), encoding="utf8", newline="").read()

start = text.index("def _fan(center, color, radius")
end = text.index("def _meter(")
new = '''def _heat_pump(center, w: float = 1.0, h: float = 1.0):
    """🌬️ Air-source heat pump outdoor unit in winter air, fitted into the ``w`` × ``h`` box around ``center``; its clip plays through scene time at ``rate``."""
    return fitted_staffage("demand-system-losses-heat-pump", width=w, height=h, rate=1.0).move_to(np.array(center, dtype=float))


'''
text = text[:start] + new + text[end:]
text = text.replace('    heat_pump = _heat_pump(np.array([BOILER_X, PIPE_Y - 0.14, 0.0]), flip=True)\n', '')
text = text.replace('"env": env, "heat_pump": heat_pump, "power_meter"', '"env": env, "power_meter"')
text = text.replace('pump = _heat_pump(at(3.7, Y), w=1.0, h=1.2)', 'pump = _heat_pump(at(3.7, Y), w=1.0, h=1.2)')
text = text.replace('''            fan = [Rotate(pump[1][1], angle=-TAU * rt / 0.9, about_point=pump[1][0].get_center(), rate_func=linear)]
            return [_flow(active, rt), *(fan if pump_on[0] else [])]''', '''            pump.rate = 1.0 if pump_on[0] else 0.0
            return [_flow(active, rt)]''')
text = text.replace("staffage_height = _manim_module.staffage_height\n", "staffage_height = _manim_module.staffage_height\nfitted_staffage = _manim_module.fitted_staffage\n", 1)
open(str(path), "w", encoding="utf8", newline="").write(text)
print("ok")
