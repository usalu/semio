"""🧮 Re-derive every substituted ledger row, slope and nudge of Beat6_EvolutionarySearch from the model and test whether the printed arithmetic closes."""

import importlib.util
from pathlib import Path

MODEL = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
spec = importlib.util.spec_from_file_location("optimization_model", MODEL)
model = importlib.util.module_from_spec(spec)
spec.loader.exec_module(model)

LAMP_KW = 9 * 48 / 1000
HOURS = 1500
FAILS = []


def closes(label, computed, printed, tolerance):
    """🧾 Record a printed substitution that does not reproduce from its printed factors within the tolerance."""
    ok = abs(computed - printed) <= tolerance
    if not ok:
        FAILS.append(label)
    print(f"{'OK      ' if ok else 'OPEN    '} {label}: from printed factors {computed:.2f}, printed {printed}")


def ledger(label, design, heat_demand, cool_demand, share, heat_part, cool_part, lamp_part):
    """🧾 One ledger row: heating ÷ COP, cooling ÷ EER and power × hours × share, against the printed parts and the model."""
    found = model.evaluate(design)
    print(f"== {label}: window {design['wwr'] * 100:.2f} %, overhang {design['overhang']:.3f} m, exact lamps {found['lighting']:.2f} kWh/a, share {found['lighting'] / (LAMP_KW * HOURS) * 100:.2f} %")
    closes(f"{label} heating {heat_demand} / 3.5", heat_demand / 3.5, heat_part, 0.5)
    closes(f"{label} cooling {cool_demand} / 3.0", cool_demand / 3.0, cool_part, 0.5)
    closes(f"{label} lamps 0.43 kW x 1,500 h x {share} %", 0.43 * HOURS * share / 100, lamp_part, 0.5)
    print(f"        model heating {found['heating']:.2f}, cooling {found['cooling']:.2f}, lamps {found['lighting']:.2f}, total {found['electricity']:.2f}")


g = model.gradient_path()
ledger("start", g[0]["design"], 122, 1778, 18, 35, 593, 117)
ledger("gradient_4 iterate 1", g[1]["design"], 165, 1237, 24, 47, 413, 153)
ledger("map_2 minimum", model.surface(41)["minimum"]["design"], 140, 820, 28, 40, 273, 182)
ledger("overshoot_2 last iterate", g[-1]["design"], 161, 759, 32, 46, 253, 205)
gradient_points = [(item["design"], item["electricity"]) for item in g] + [(probe["design"], probe["electricity"]) for item in g for probe in item["probes"]]
best_gradient = min(gradient_points, key=lambda point: point[1])[0]
ledger("gradient_result best", best_gradient, 150, 792, 29, 43, 264, 189)
pattern_points = [(item["design"], item["electricity"]) for item in model.pattern_path()] + [(probe["design"], probe["electricity"]) for item in model.pattern_path() for probe in item["probes"]]
best_pattern = min(pattern_points, key=lambda point: point[1])[0]
ledger("pattern_result best", best_pattern, 125, 879, 26, 36, 293, 168)

probes = {probe["direction"]: probe for probe in g[0]["probes"]}
print("== overhang slope (gradient_3, printed probes 743.05 and 746.33 over 0.024 m)")
closes("overhang slope (743.05 - 746.33) / 0.024", (743.05 - 746.33) / 0.024, -136, 0.5)
print(f"   exact central difference from unrounded probes: {(probes['overhang+']['electricity'] - probes['overhang-']['electricity']) / (0.112 - 0.088):.2f} kWh/a per m")
print(f"   windows slope (748.2 - 741.2) / 1.4 = {(748.2 - 741.2) / 1.4:.2f} (printed 5.0)")
print("== nudge (mutation frame, child at 31.1 % printed after parent at 30.3 % printed)")
closes("windows nudge printed +0.9 % against 31.1 - 30.3", 31.1 - 30.3, 0.9, 0.05)
print(f"   exact nudge from the model log: {0.8555:.4f} points")
print(f"SUMMARY {len(FAILS)} printed substitutions do not close at the printed precision")
for item in FAILS:
    print("   -", item)
