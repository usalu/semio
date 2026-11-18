"""⚡️ Authors the nameplate `ratedPowerW` / `ratedPowerBasis` of every power-drawing element into the catalog assets by small text insertions (re-reads each file right before it writes)."""
import glob, json, re, sys
E, F, R, P = "electrical-input", "fuel-input", "rated-output", "passive"
ROOT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
RATINGS = {
    "concrete-mixer-truck": [("engine-bay", 310000, R)],
    "diesel-generator": [("silencer", 320000, R)],
    "excavator": [("engine-compartment", 110000, R)],
    "expansion-valve": [("body", 0, P)],
    "finned-heat-exchanger": [("fins", 0, P)],
    "forklift": [("drive-motor", 15000, R), ("hydraulics", 12000, E)],
    "heat-pump-outdoor-unit": [("compressor-casing", 2700, E), ("coil-guard", 150, E)],
    "oil-pumpjack": [("motor", 22000, R)],
    "scissor-lift": [("motor-pump", 3000, E), ("battery-box", 0, P)],
    "scroll-compressor": [("shell", 2500, E)],
    "solar-thermal-collector": [("absorber", 0, P)],
    "sorption-wheel": [("regen", 1500, E), ("hub", 30, E)],
    "tower-crane": [("hoist-motor", 55000, R), ("slew-drive", 15000, R)],
    "wheel-loader": [("engine-bay", 150000, R)],
    "leaning-person": [("phone-surface", 5, E)],
    "person-cooking": [("pot", 1500, E)],
    "seated-office-worker": [("laptop", 65, E)],
    "standing-phone-user": [("phone", 5, E)],
    "ceiling-diffuser": [("plate", 0, P)],
    "circulation-pump": [("motor", 45, E)],
    "decentralised-hrv-fan": [("hub", 4, E)],
    "desktop-monitor": [("housing", 25, E)],
    "duct-fan": [("hub", 28, E)],
    "electric-fan-heater": [("casing", 2000, E)],
    "electric-kettle": [("water", 2000, E)],
    "electricity-meter": [("plate", 2.4, E)],
    "gas-boiler": [("burner", 20000, F), ("controls", 3, E)],
    "gas-meter": [("pipes", 0, P)],
    "hot-water-cylinder": [("zone-1", 0, P)],
    "kitchen-extractor-hood": [("canopy", 120, E), ("lamps", 30, E)],
    "laptop": [("deck", 65, E)],
    "mvhr-unit": [("impeller", 75, E)],
    "pendant-lamp": [("bulb-glass", 60, E)],
    "room-thermostat": [("casing", 0.7, E)],
    "server-rack": [("servers", 5000, E)],
    "shower-head": [("spray", 0, P)],
    "split-ac-indoor-unit": [("casing", 700, E)],
    "thermostatic-radiator-valve": [("body", 0, P)],
    "toaster": [("casing", 900, E)],
    "tower-server": [("side-panel", 200, E)],
    "vacuum-cleaner": [("body-shell", 700, E)],
    "cargo-bike": [("drive-motor", 250, R)],
    "city-bicycle": [("lamps", 3, E)],
    "city-bus": [("e-axle", 250000, R), ("hvac-unit", 12000, E)],
    "city-car": [("engine-bay", 80000, R)],
    "delivery-van": [("engine-bay", 100000, R)],
    "electric-kick-scooter": [("hub-motor", 250, R), ("bms-controller", 63, E)],
    "motor-scooter": [("engine", 8000, R)],
    "tram": [("traction-motors", 480000, R), ("hvac-unit", 15000, E), ("brake-resistor", 0, P)],
    "vintage-car-1929": [("engine-bay", 45000, R)],
    "height-adjustable-desk": [("drive-left", 100, E), ("drive-right", 100, E), ("monitor-housing", 25, E)],
    "operating-table": [("light-housing", 120, E)],
    "panel-radiator": [("panel-rear", 0, P)],
    "charcoal-grill": [("charcoal-bed", 8000, F)],
    "cooking-pot": [("water", 1200, F)],
    "patio-heater": [("emitter-head", 13000, F)],
    "retractable-bollard": [("hydraulic-unit", 750, E), ("beacon-led", 10, E)],
    "street-lamp": [("lamp-housing", 60, E)],
    "tealight": [("flame", 35, F)],
}
def fmt(v): return str(int(v)) if float(v) == int(v) else str(v)
def objectEnd(s, i):
    depth, inString, k = 0, False, i
    while True:
        c = s[k]
        if inString:
            if c == "\\": k += 1
            elif c == '"': inString = False
        elif c == '"': inString = True
        elif c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return k
        k += 1
def edit(asset, region, watts, basis):
    path = glob.glob(f"{ROOT}/*/{asset}/🔣️.json")[0]
    s = open(path).read()
    pattern = re.compile(r'\n( +)"id": "%s",\n\1"title": \{[^}]*\},\n\1"kind": "[a-z]+",\n\1"model": ' % re.escape(region))
    found = list(pattern.finditer(s))
    assert len(found) == 1, (asset, region, len(found))
    indent = found[0].group(1)
    start = found[0].start()
    close = objectEnd(s, s.rindex("{", 0, start))
    seg = s[start:close]
    seg = re.sub(r'\n%s"ratedPower(W|Basis)": [^\n]*(?=\n)' % indent, "", seg)
    src = seg.index("\n" + indent + '"source":')
    seg = seg[:src] + f'\n{indent}"ratedPowerW": {fmt(watts)},\n{indent}"ratedPowerBasis": "{basis}",' + seg[src:]
    s = s[:start] + seg + s[close:]
    json.loads(s)
    open(path, "w").write(s)
for asset, items in RATINGS.items():
    if len(sys.argv) > 1 and asset not in sys.argv[1:]: continue
    for region, watts, basis in items:
        edit(asset, region, watts, basis)
    print("ok", asset)
