"""🔎 Recompute every number the Pareto clip shows on screen from the model and print the mismatches."""

import importlib.util
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
SPEC = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model" / "🐍️.py")
MODEL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODEL)
DATA = MODEL.pareto()
DESIGNS = {item["id"]: item for item in DATA["designs"]}
PROBLEMS = []


def ident(design):
    """🏷️ Front-grid id of a design dictionary as the model names it."""
    return f"w{round(design['wwr'] * 100)}-t{round(design['insulation'] * 100)}-d{round(design['overhang'] * 100)}"


def check(label, shown, actual, tolerance=0.5):
    """✅ Compare one on-screen value with the model value and remember mismatches."""
    ok = abs(float(shown) - float(actual)) <= tolerance
    print(f"{'ok  ' if ok else 'MISS'} {label}: screen {shown} / model {float(actual):.3f}")
    if not ok:
        PROBLEMS.append(label)


def show(label, value):
    """📝 Print a model value that the screen does not state directly."""
    print(f"info {label}: {value}")


def bar_check(identifier, total, lighting, cooling, heating):
    """📊 Stacked bar: displayed parts and total must add up to the same integer."""
    design = DESIGNS[identifier]
    parts = (round(design["lighting"]), round(design["cooling"]), round(design["heating"]))
    print(f"bar {identifier}: model L/C/H = {design['lighting']:.2f}/{design['cooling']:.2f}/{design['heating']:.2f} total {design['electricity']:.2f}")
    check(f"bar {identifier} lighting", lighting, design["lighting"])
    check(f"bar {identifier} cooling", cooling, design["cooling"])
    check(f"bar {identifier} heating", heating, design["heating"])
    check(f"bar {identifier} total", total, design["electricity"])
    if sum(parts) != round(design["electricity"]):
        PROBLEMS.append(f"bar sum {identifier}: rounded parts {sum(parts)} vs total {round(design['electricity'])}")
        print(f"MISS bar sum {identifier}: rounded parts {sum(parts)} != rounded total {round(design['electricity'])}")
    if sum((lighting, cooling, heating)) != total:
        print(f"note screen parts {lighting}+{cooling}+{heating}={lighting + cooling + heating} vs screen total {total} for {identifier}")


def main():
    """🚀 Run every recomputation for the Pareto scene and summarise."""
    print("== grid (36 designs)")
    grid = DATA["designs"]
    show("designs", len(grid))
    show("wwr levels", sorted({round(item["design"]["wwr"] * 100) for item in grid}))
    show("insulation levels cm", sorted({round(item["design"]["insulation"] * 100) for item in grid}))
    show("overhang levels m", sorted({item["design"]["overhang"] for item in grid}))
    electricity = [item["electricity"] for item in grid]
    cost = [item["cost"] for item in grid]
    daylight = [item["daylight"] for item in grid]
    show("electricity min/max over 36", (round(min(electricity), 1), round(max(electricity), 1)))
    show("cost min/max over 36", (round(min(cost), 1), round(max(cost), 1)))
    show("daylight min/max over 36 %", (round(min(daylight), 2), round(max(daylight), 2)))
    check("front_2 ramp electricity low (shown 495)", 495, min(electricity), tolerance=0.5)
    check("front_2 ramp electricity high (shown 846)", 846, max(electricity), tolerance=0.5)
    check("third ramp daylight low (shown 1 %)", 1, min(daylight), tolerance=0.5)
    check("third ramp daylight high (shown 7 %)", 7, max(daylight), tolerance=0.5)
    print("== front")
    front = DATA["front"]
    show("2D front", [(identifier, round(DESIGNS[identifier]["cost"]), round(DESIGNS[identifier]["electricity"], 1)) for identifier in front])
    check("front count (shown 8)", 8, len(front), tolerance=0)
    check("3D front count (shown 30)", 30, len(DATA["front3"]), tolerance=0)
    check("dashed front count step total (shown 7 steps)", 7, len(DATA["steps"]), tolerance=0)
    print("== steps")
    previous = None
    for step in DATA["steps"]:
        show(f"{step['from']} -> {step['to']}", f"+{step['cost']:.1f} CHF, {step['electricity']:.1f} kWh/a, saved {step['saved']:.2f}, ratio {step['ratio']:.2f}, payback {step['payback']:.1f} a @0.25, {step['payback_hi']:.1f} a @0.40")
        if previous is not None and step["saved"] > previous:
            print(f"MISS narration 'each step saves less': saved {step['saved']:.2f} after {previous:.2f} at {step['from']} -> {step['to']}")
            PROBLEMS.append(f"monotone saving violated at {step['from']} -> {step['to']}")
        previous = step["saved"]
    first = DATA["steps"][0]
    check("walk_2 cost step (shown 192)", 192, first["cost"])
    check("walk_2 saving (shown 65)", 65, first["saved"])
    check("walk_3 ratio CHF per kWh/a (shown 3.0)", 3.0, first["ratio"], tolerance=0.05)
    check("payback step 1 @0.25 (shown 12)", 12, first["payback"])
    check("payback step 1 @0.40 (shown 7)", 7, first["payback_hi"])
    check("payback step 2 @0.25 (shown 34)", 34, DATA["steps"][1]["payback"])
    check("payback step 2 @0.40 (shown 21)", 21, DATA["steps"][1]["payback_hi"])
    print("== segment deltas (walk / knee_3 / knee)")
    for start_id, end_id, shown in (("w30-t12-d0", "w30-t12-d60", (57, -122, 0)),
                                    ("w30-t12-d60", "w40-t12-d60", (-57, 31, -25)),
                                    ("w40-t30-d60", "w50-t30-d60", (-41, 51, -10))):
        start = DESIGNS[start_id]
        end = DESIGNS[end_id]
        deltas = (round(end["lighting"] - start["lighting"]), round(end["cooling"] - start["cooling"]), round(end["heating"] - start["heating"]))
        print(f"delta {start_id} -> {end_id}: lighting {end['lighting'] - start['lighting']:+.2f} cooling {end['cooling'] - start['cooling']:+.2f} heating {end['heating'] - start['heating']:+.2f} total {end['electricity'] - start['electricity']:+.2f}")
        for label, screen, actual in zip(("lighting", "cooling", "heating"), shown, deltas):
            check(f"delta {start_id}->{end_id} {label}", screen, actual, tolerance=0.5)
    print("== knee")
    show("knee @0.25", DATA["knee"]["tariff"])
    show("knee @0.40", DATA["knee"]["tariff_hi"])
    check("knee tariff is the 30 % / 0.6 m / 12 cm design", 1, DATA["knee"]["tariff"] == "w30-t12-d60", tolerance=0)
    check("knee high tariff is 40 % / 0.6 m / 12 cm", 1, DATA["knee"]["tariff_hi"] == "w40-t12-d60", tolerance=0)
    print("== dominated pair")
    show("dominated_example raw", DATA["dominated_example"])
    winner = DESIGNS["w30-t12-d0"]
    loser = DESIGNS["w70-t30-d0"]
    show("dominated pair", (ident(loser["design"]), round(loser["cost"]), round(loser["electricity"], 1), ident(winner["design"]), round(winner["cost"]), round(winner["electricity"], 1)))
    check("pair cost difference (shown 2,486)", 2486, loser["cost"] - winner["cost"], tolerance=0.5)
    check("pair electricity difference (shown 75)", 75, loser["electricity"] - winner["electricity"], tolerance=0.5)
    check("pair loser cost (shown 21,696)", 21696, loser["cost"])
    check("pair loser electricity (shown 724)", 724, loser["electricity"])
    bar_check("w70-t30-d0", 724, 121, 568, 34)
    bar_check("w30-t12-d0", 649, 241, 310, 98)
    print("== extremes")
    cheapest = min(grid, key=lambda item: item["cost"])
    frugal = min(grid, key=lambda item: item["electricity"])
    show("cheapest", (cheapest["id"], round(cheapest["cost"]), round(cheapest["electricity"], 1)))
    show("most frugal", (frugal["id"], round(frugal["cost"]), round(frugal["electricity"], 1)))
    check("gap cost (shown +2,342)", 2342, frugal["cost"] - cheapest["cost"], tolerance=0.5)
    check("gap electricity (shown -129)", -129, frugal["electricity"] - cheapest["electricity"], tolerance=0.5)
    bar_check("w50-t30-d60", 520, 200, 275, 45)
    print("== third (daylight)")
    example = DATA["third_example"]
    dominated = DESIGNS[example["dominated_2d"]]
    dominator = DESIGNS[example["dominator"]]
    show("third example", (example, round(dominated["cost"]), round(dominated["electricity"], 1), round(dominated["daylight"], 2), round(dominator["cost"]), round(dominator["electricity"], 1), round(dominator["daylight"], 2)))
    check("third loser cost (shown 21,346)", 21346, dominated["cost"])
    check("third loser electricity (shown 776)", 776, dominated["electricity"])
    check("third loser daylight (shown 6.4)", 6.4, dominated["daylight"], tolerance=0.05)
    check("third dominator daylight (shown 2.4)", 2.4, dominator["daylight"], tolerance=0.05)
    check("third lamps on share at loser (shown 17 %)", 17, 100 * (1 - MODEL.evaluate(dominated["design"])["autonomy"]))
    check("third lamps on share at dominator (shown 37 %)", 37, 100 * (1 - MODEL.evaluate(dominator["design"])["autonomy"]))
    bar_check("w80-t12-d0", 776, 112, 629, 34)
    print("== refinement")
    refinement = DATA["refinement"]
    refined = MODEL.evaluate(refinement["design"])
    show("refinement design", {key: round(value, 4) if isinstance(value, float) else value for key, value in refinement["design"].items()})
    check("refinement cost (shown 21,260)", 21260, refined["cost"])
    check("refinement electricity (shown 495)", 495, refined["electricity"])
    check("refinement WWR % (shown 66)", 66, refinement["design"]["wwr"] * 100, tolerance=0.5)
    check("refinement overhang m (shown 1.0)", 1.0, refinement["design"]["overhang"], tolerance=0.05)
    show("refinement dominates", refinement["dominates"])
    check("refinement beats 2 front facades (shown 2)", 2, len(refinement["dominates"]), tolerance=0)
    beaten = DESIGNS["w50-t30-d60"]
    check("check row cost gap (shown -292)", -292, refined["cost"] - beaten["cost"])
    check("check row electricity gap (shown -25)", -25, refined["electricity"] - beaten["electricity"])
    check("member cost (shown 21,552)", 21552, beaten["cost"])
    check("member electricity (shown 520)", 520, beaten["electricity"])
    bar_check("w50-t30-d60", 520, 200, 275, 45)
    print("== surface and room numbers")
    surface_minimum = MODEL.surface()["minimum"]
    show("surface minimum", surface_minimum)
    show("sun summer noon", MODEL.sun_state(180, "summer"))
    show("sun winter noon", MODEL.sun_state(180, "winter"))
    print("== weighted picks")
    for weight, expect in ((0.0, "w30-t12-d0"), (0.5, "w40-t12-d60"), (0.98, "w50-t30-d60")):
        pick = MODEL.weighted_pick(weight)
        show(f"pick at electricity weight {weight}", (pick, round(DESIGNS[pick]["cost"]), round(DESIGNS[pick]["electricity"])))
        check(f"pick {weight} equals {expect}", 1, pick == expect, tolerance=0)
    print("== room bars and costs shown")
    bar_check("w40-t12-d60", 533, 241, 219, 73)
    bar_check("w30-t12-d60", 584, 298, 188, 98)
    bar_check("w80-t20-d0", 775, 112, 631, 32)
    bar_check("w30-t12-d0", 649, 241, 310, 98)
    bar_check("w80-t12-d0", 776, 112, 629, 34)
    check("intensity 533 kWh/a on 48 m2 (shown 11)", 11, DESIGNS["w40-t12-d60"]["electricity"] / 48, tolerance=0.5)
    check("intensity 776 kWh/a on 48 m2 (shown 16)", 16, DESIGNS["w80-t12-d0"]["electricity"] / 48, tolerance=0.5)
    print(f"== {len(PROBLEMS)} mismatches")
    for item in PROBLEMS:
        print("  -", item)


if __name__ == "__main__":
    main()
