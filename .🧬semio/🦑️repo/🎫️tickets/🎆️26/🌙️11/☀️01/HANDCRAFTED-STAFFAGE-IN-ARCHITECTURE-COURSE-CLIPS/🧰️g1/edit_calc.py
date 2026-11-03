import time

p = "🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py"
s = open(p, encoding="utf8").read()


def rep(a, b):
    global s
    assert s.count(a) == 1, a
    s = s.replace(a, b)


rep("energy_tank = _manim_module.energy_tank\n", "fitted_staffage = _manim_module.fitted_staffage\n")
rep('P_DEEP_GREY = semio_color("dark-5-7")\n', "BOILER_RATE = 60.0\nTANK_CHARGE_S = 900.0\n")
rep(
    '''    """🏠 Physical Fundamentals section through the example house: heated storey, roof, unheated cellar with boiler,
    energy-tank store and pipes."""''',
    '''    """🏠 Physical Fundamentals section through the example house: heated storey, roof, unheated cellar with gas boiler,
    hot-water cylinder store, room thermostat and pipes — all handcrafted staffage."""''',
)
i = s.index('    parts["thermostat"] = Circle(')
j = s.index("    flue_bottom, flue_top")
new = '''    parts["thermostat"] = _cast("heating-calculation-thermostat", 0.17, motion=True, rate=0.0, anchor=P(-5.83, RAD_Y + 0.08))
    tank_h, boiler_h, boiler_w = 1.5 * metre, 1.0 * metre, 0.6 * metre
    level = ValueTracker(0.9)
    store = fitted_staffage("heating-calculation-tank", width=0.65 * metre, height=tank_h, rate=0.0).move_to(P(TANK_X, -1.6 + tank_h / 2))
    store.add_updater(lambda m: m.seek(float(np.clip(level.get_value(), 0.0, 1.0)) * TANK_CHARGE_S))
    tank = {"group": store, "fill": VGroup(), "frame": store, "cap": store, "level": level}
    parts["tank"] = tank
    parts["store"] = VGroup(store)
    parts["boiler"] = fitted_staffage("heating-calculation-boiler", width=boiler_w, height=boiler_h, rate=BOILER_RATE
                                      ).move_to(P(-2.85, -1.6 + boiler_h / 2))
'''
s = s[:i] + new + s[j:]
for attempt in range(8):
    try:
        with open(p, "w", encoding="utf8", newline="") as handle:
            handle.write(s)
        break
    except OSError:
        time.sleep(1)
else:
    raise SystemExit("write failed")
print("ok")
