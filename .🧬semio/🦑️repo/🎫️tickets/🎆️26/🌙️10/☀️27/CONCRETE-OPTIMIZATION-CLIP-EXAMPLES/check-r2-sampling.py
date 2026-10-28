"""🔢 Recompute every number shown on the Beat2_GridRandomSampling frames from the optimization model."""

import importlib.util
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
SPEC = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model" / "🐍️.py")
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

ROWS = []
SUN = M.load_study()["sun"]
TARIFF = M.pareto()["tariff"]
SURFACE = M.surface(41)
OPTIMUM = SURFACE["minimum"]
SAMPLES = {kind: M.samples(kind) for kind in ("grid", "random", "lhs")}


def rnd(value, digits=0):
    """🔢 Round half up on the decimal repr, the convention of the clip labels."""
    return str(Decimal(repr(float(value))).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP))


def expect(frame, what, value, shown, digits=0):
    """✅ Record one on-screen number against its recomputed value."""
    computed = rnd(value, digits)
    ROWS.append((frame, what, computed, shown, computed == shown))


def ev(wwr, overhang):
    """⚙️ Evaluate one design with the baseline orientation and insulation."""
    return M.evaluate({"wwr": wwr, "overhang": overhang})


def bar(frame, wwr, overhang, shown_parts, shown_total, shown_cost):
    """📊 Check one stacked electricity bar, its total, its cost and that the shown parts add up."""
    item = ev(wwr, overhang)
    label = f"WWR {wwr * 100:.0f} % d {overhang:.2f} m"
    for key in ("heating", "cooling", "lighting"):
        expect(frame, f"{key} {label}", item[key], shown_parts[key])
    expect(frame, f"total {label}", item["electricity"], shown_total)
    expect(frame, f"cost {label} (electricity x tariff)", item["electricity"] * TARIFF, shown_cost)
    parts = sum(int(shown_parts[key]) for key in ("heating", "cooling", "lighting"))
    ROWS.append((frame, f"shown parts add up to shown total {label}", str(parts), shown_total, parts == int(shown_total)))


def main():
    """▶️ Run every check and print the table, the mismatches and the narrative claims."""
    base = ev(0.45, 0.6)
    expect("parameters", "window area WWR 45 % (m2)", base["window_area"], "10.8", 1)
    expect("run", "winter noon altitude (deg)", SUN["winter_altitude"], "19")
    expect("run", "summer noon altitude (deg)", SUN["summer_altitude"], "66")
    expect("run", "tariff CHF/kWh", TARIFF, "0.25", 2)
    bar("run_2 / run_3", 0.45, 0.6, {"heating": "58", "cooling": "245", "lighting": "219"}, "522", "131")
    expect("grid_2", "best so far 45 % / 0.6 m (kWh/a, 1 dp)", base["electricity"], "522.1", 1)

    grid_tags = {(0.10, 0.0): "808", (0.10, 0.6): "809", (0.10, 1.2): "846", (0.45, 0.0): "637", (0.45, 0.6): "522",
                 (0.45, 1.2): "547", (0.80, 0.0): "775", (0.80, 0.6): "616", (0.80, 1.2): "499"}
    for (wwr, depth), shown in grid_tags.items():
        expect("grid / grid_2 / grid_3", f"grid tag WWR {wwr * 100:.0f} % d {depth:.1f} m", ev(wwr, depth)["electricity"], shown)
    bar("grid_2", 0.45, 0.0, {"heating": "58", "cooling": "408", "lighting": "171"}, "637", "159")
    bar("grid_4 / grid_3", 0.80, 1.2, {"heating": "34", "cooling": "304", "lighting": "161"}, "499", "125")
    expect("grid_4", "worst of nine (ghost bar total)", max(item["electricity"] for item in SAMPLES["grid"]), "846")
    expect("grid_4", "best of nine grid (1 dp)", min(item["electricity"] for item in SAMPLES["grid"]), "499.2", 1)

    levels = M.levels()
    counts = [len(levels[key]) for key in ("wwr", "overhang", "orientation", "insulation")]
    total = 1
    for count in counts:
        total *= count
    expect("growth_2", "simulations 3*3*4*3", total, "108")
    expect("growth_3", "over budget 108 - 9", total - 9, "99")
    expect("growth_3", "times budget 108 / 9", total / 9, "12")

    random_table = [(14, 0.95), (23, 0.30), (33, 0.72), (35, 0.78), (47, 0.12), (58, 0.15), (70, 0.62), (77, 0.30), (18, 0.08)]
    for position, (shown_wwr, shown_depth) in enumerate(random_table):
        sample = SAMPLES["random"][position]
        expect("random (table)", f"draw {position + 1} WWR %", sample["wwr"] * 100, str(shown_wwr))
        expect("random (table)", f"draw {position + 1} d m", sample["overhang"], f"{shown_depth:.2f}", 2)
    bar("random_3", SAMPLES["random"][8]["wwr"], SAMPLES["random"][8]["overhang"],
        {"heating": "129", "cooling": "233", "lighting": "345"}, "707", "177")
    expect("random_3", "best random 561.8 (1 dp)", min(item["electricity"] for item in SAMPLES["random"]), "561.8", 1)

    box = M.sampling_diagnostics("random")["largest_empty_box"]
    expect("gaps", "box WWR low %", box["wwr"][0] * 100, "35")
    expect("gaps", "box WWR high %", box["wwr"][1] * 100, "70")
    expect("gaps", "box d low m", box["overhang"][0], "0.15", 2)
    expect("gaps", "box d high m", box["overhang"][1], "1.20", 2)
    centre_wwr, centre_d = (box["wwr"][0] + box["wwr"][1]) / 2 * 100, (box["overhang"][0] + box["overhang"][1]) / 2
    expect("gaps", "probe centre WWR % (52.5 is a tie; Python format gives 52)", centre_wwr, "52")
    expect("gaps", "probe centre d m", centre_d, "0.68", 2)
    inside = [item for item in SAMPLES["random"] if box["wwr"][0] < item["wwr"] < box["wwr"][1] and box["overhang"][0] < item["overhang"] < box["overhang"][1]]
    ROWS.append(("gaps", "draws strictly inside the empty box", str(len(inside)), "0", len(inside) == 0))
    deep_large = [(rnd(item["wwr"] * 100), rnd(item["overhang"], 2)) for item in SAMPLES["random"] if item["wwr"] >= 0.55 and item["overhang"] >= 0.8]
    ROWS.append(("gaps caption", "draws with WWR >= 55 % and d >= 0.8 m (caption claim, box is not this region)", str(len(deep_large)), "0", len(deep_large) == 0))

    diag = M.sampling_diagnostics("random")
    first, second, gap = diag["closest_pair"][0], diag["closest_pair"][1], diag["closest_delta"]
    ROWS.append(("gaps_2", "closest pair indices (1-based)", f"#{first + 1} / #{second + 1}", "#3 / #4", (first, second) == (2, 3)))
    expect("gaps_2", "delta WWR (pp)", gap["wwr"] * 100, "2")
    expect("gaps_2", "delta d (m)", gap["overhang"], "0.06", 2)
    pair_gap = abs(SAMPLES["random"][first]["electricity"] - SAMPLES["random"][second]["electricity"])
    expect("gaps_2", "delta kWh/a of the pair", pair_gap, "6")
    expect("gaps_2", "electricity of draw #3", SAMPLES["random"][first]["electricity"], "568")
    expect("gaps_2", "electricity of draw #4", SAMPLES["random"][second]["electricity"], "562")
    bar("gaps_2", SAMPLES["random"][second]["wwr"], SAMPLES["random"][second]["overhang"],
        {"heating": "75", "cooling": "202", "lighting": "285"}, "562", "140")

    lhs = M.lhs_strata()
    expect("lhs", "strata per axis", lhs["count"], "9")
    expect("lhs", "stratum width WWR (pp)", lhs["wwr"][1] * 100 - lhs["wwr"][0] * 100, "7.8", 1)
    expect("lhs", "stratum height d (m)", lhs["overhang"][1] - lhs["overhang"][0], "0.13", 2)
    expect("cells", "occupied cells 9 of 9*9", lhs["count"] ** 2, "81")
    column = SAMPLES["lhs"][4]
    ROWS.append(("marginals", "strip WWR low % (edge 4)", rnd(lhs["wwr"][4] * 100, 1), "41.1", rnd(lhs["wwr"][4] * 100, 1) == "41.1"))
    ROWS.append(("marginals", "strip WWR high % (edge 5)", rnd(lhs["wwr"][5] * 100, 1), "48.9", rnd(lhs["wwr"][5] * 100, 1) == "48.9"))
    ROWS.append(("marginals", "chosen LHS draw WWR % / d m", f"{column['wwr'] * 100:.1f} / {column['overhang']:.2f}", "45 / 0.33", abs(column["overhang"] - 0.33) < 0.01))
    lhs_best = min(SAMPLES["lhs"], key=lambda item: item["electricity"])
    expect("compare / limits_2", "best LHS kWh/a (1 dp)", lhs_best["electricity"], "495.9", 1)
    expect("compare", "best LHS WWR %", lhs_best["wwr"] * 100, "68")
    expect("compare", "best LHS d m", lhs_best["overhang"], "1.00", 2)
    last = SAMPLES["lhs"][8]
    expect("evaluate_2", "last LHS draw WWR %", last["wwr"] * 100, "76")
    expect("evaluate_2", "last LHS draw d m", last["overhang"], "0.60", 2)
    bar("evaluate_2", last["wwr"], last["overhang"], {"heating": "34", "cooling": "424", "lighting": "140"}, "599", "150")
    bar("limits_4 (LHS best)", lhs_best["wwr"], lhs_best["overhang"], {"heating": "39", "cooling": "281", "lighting": "177"}, "496", "124")

    flat = [value for column_values in SURFACE["electricity"] for value in column_values]
    expect("reveal", "colour ramp low (surface minimum)", min(flat), "495")
    expect("reveal", "colour ramp high (surface maximum)", max(flat), "846")
    expect("reveal", "dense grid simulations 41 x 41", len(SURFACE["wwr"]) * len(SURFACE["overhang"]), "1681")
    expect("reveal_2 / compare", "optimum WWR %", OPTIMUM["design"]["wwr"] * 100, "66")
    expect("reveal_2 / compare", "optimum d m", OPTIMUM["design"]["overhang"], "0.99", 2)
    expect("reveal_2 / limits_2", "optimum kWh/a (1 dp)", OPTIMUM["electricity"], "494.8", 1)
    bar("reveal_2 (optimum)", OPTIMUM["design"]["wwr"], OPTIMUM["design"]["overhang"],
        {"heating": "40", "cooling": "273", "lighting": "182"}, "495", "124")
    bar("reveal (80 % / 0 m)", 0.80, 0.0, {"heating": "33", "cooling": "630", "lighting": "112"}, "775", "194")

    best = {"grid": min(SAMPLES["grid"], key=lambda item: item["electricity"]),
            "random": min(SAMPLES["random"], key=lambda item: item["electricity"]), "lhs": lhs_best}
    shown_gap = {kind: round(round(best[kind]["electricity"], 1) - round(OPTIMUM["electricity"], 1), 1) for kind in best}
    for kind, shown in (("grid", "4.4"), ("random", "67.0"), ("lhs", "1.1")):
        expect("compare_2", f"delta {kind} kWh/a", shown_gap[kind], shown, 1)
        expect("compare_2", f"delta {kind} %", shown_gap[kind] / round(OPTIMUM["electricity"], 1) * 100, {"grid": "0.9", "random": "13.5", "lhs": "0.2"}[kind], 1)

    expect("limits", "<= +5 % threshold (kWh/a, unrounded optimum)", OPTIMUM["electricity"] * 1.05, "519.6", 1)
    expect("limits", "<= +5 % threshold from the displayed 494.8", 494.8 * 1.05, "519.6", 1)
    expect("limits_2", "1,681 simulations -> optimum", OPTIMUM["electricity"], "494.8", 1)
    winner = min(best, key=lambda kind: best[kind]["electricity"])
    ROWS.append(("limits_2", f"'9 simulations -> best of nine' value is the minimum over {len(best)} sets of 9 = {len(best) * 9} simulations, set {winner}",
                 rnd(best[winner]["electricity"], 1), "495.9", rnd(best[winner]["electricity"], 1) == "495.9"))

    expect("gaps / cells", "sampling_diagnostics LHS largest empty box WWR low %", M.sampling_diagnostics("lhs")["largest_empty_box"]["wwr"][0] * 100, "14")
    expect("gaps / cells", "sampling_diagnostics LHS largest empty box WWR high %", M.sampling_diagnostics("lhs")["largest_empty_box"]["wwr"][1] * 100, "76")

    expect("proposal run_2", "heating demand 203.9 kWh/a (baseline)", base["heating_demand"], "203.9", 1)
    expect("proposal run_2", "heating electricity = demand / COP 3.5", base["heating_demand"] / 3.5, "58.3", 1)
    expect("proposal run_2", "cooling demand 735.6 kWh/a (baseline)", base["cooling_demand"], "735.6", 1)
    expect("proposal run_2", "cooling electricity = demand / EER 3", base["cooling_demand"] / 3, "245.2", 1)
    expect("proposal run_2", "lighting = 9 W/m2 x 48 m2 x 1500 h x (1 - autonomy)", 9 * 48 * 1500 * (1 - base["autonomy"]) / 1000, "218.6", 1)
    expect("proposal run_2", "daylight autonomy (share of lighting hours)", base["autonomy"] * 100, "66")
    expect("proposal run_2", "summer shading share (beam-weighted)", base["shading_summer"] * 100, "89")
    expect("proposal run_2", "window-to-wall ratio shading meter at 0.6 m summer", base["shading_summer"], "0.89", 2)
    expect("proposal run_2", "facade construction cost baseline (CHF)", base["cost"], "20359.2", 1)

    for label, sample in (("evaluate_2 last LHS", last), ("limits_4 LHS best", lhs_best)):
        print(f"[DEBUG]{label}: heating {sample['heating']:.3f} cooling {sample['cooling']:.3f} lighting {sample['lighting']:.3f} total {sample['electricity']:.3f} cost {sample['electricity'] * TARIFF:.3f}")
    lhs_box = M.sampling_diagnostics("lhs")["largest_empty_box"]
    print(f"[DEBUG]LHS largest empty box: WWR {lhs_box['wwr'][0] * 100:.2f} to {lhs_box['wwr'][1] * 100:.2f} %, d {lhs_box['overhang'][0]:.3f} to {lhs_box['overhang'][1]:.3f} m")
    print(f"[DEBUG]gaps_2 cost 562 x 0.25 = {562 * TARIFF:.2f}; unrounded draw cost {SAMPLES['random'][3]['electricity'] * TARIFF:.3f}")
    print(f"[DEBUG]random box centre WWR {(box['wwr'][0] + box['wwr'][1]) / 2 * 100:.4f} %, d {(box['overhang'][0] + box['overhang'][1]) / 2:.6f} m")

    width = max(len(row[0]) for row in ROWS)
    print(f"{'frame':<{width}} | {'check':<70} | computed | shown | ok")
    for frame, what, computed, shown, ok in ROWS:
        print(f"{frame:<{width}} | {what[:70]:<70} | {computed:>8} | {shown:>5} | {'OK' if ok else 'MISMATCH'}")
    mismatches = [row for row in ROWS if not row[4]]
    print(f"\nchecks {len(ROWS)}, mismatches {len(mismatches)}")
    print(f"baseline façade cost (not shown on screen) {rnd(base['cost'], 1)} CHF; tariff cost of baseline {rnd(base['electricity'] * TARIFF, 1)} CHF/a shown as 'Cost 131 CHF/a'")


if __name__ == "__main__":
    main()
