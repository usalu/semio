p = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/check-extrema-r2-tables.py"
s = open(p, encoding="utf-8").read()
marker = 'print("[DEBUG] peak"'
add = '''scene = importlib.import_module(CLIP + ".📉️extrema.🐍️")
balance = scene.Beat1_LocalGlobalExtrema._eta
for theta in (90, 180, 282, 315, 360):
    found = at(theta)
    for season, part, hours_key in (("winter", "heating", "heating_degree_hours"), ("summer", "cooling", "cooling_degree_hours")):
        winter = season == "winter"
        constants = {name: item["value"] for section in ("climate", "envelope") for name, item in model.load_study()["parameters"][section].items()}
        conductance = found["u_wall"] * found["wall_area"] + constants["u_window"] * found["window_area"] + constants["h_other"]
        envelope = conductance * constants[hours_key] / 1000
        gains = found["solar_" + season] + constants["internal_gains_" + season]
        eta = balance(gains / envelope if winter else envelope / gains, constants["utilization_exponent_" + ("heating" if winter else "cooling")])
        demand = envelope - eta * gains if winter else gains - eta * envelope
        check(f"{part} demand balance at {theta}", abs(demand - found[part + "_demand"]) < 1e-6, f"{envelope if winter else gains:.0f} - {eta:.2f} x {gains if winter else envelope:.0f} = {demand:.1f}")
kilowatt = 9 * 8 * 6 / 1000
for theta in (90, 180, 360):
    found = at(theta)
    on = round(100 * (1 - found["autonomy"]), 1)
    check(f"lamps line at {theta}", round(kilowatt * 1500 * on / 100) == round(found["lighting"]), f"{kilowatt:.3f} kW x 1,500 h/a x {on} % = {kilowatt * 1500 * on / 100:.1f}")

'''
open(p, "w", encoding="utf-8").write(s.replace(marker, add + marker, 1))
