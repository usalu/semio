"""Independent oracle of the DIN EN 16798 scene diagrams: closed forms of EN 16798-1 Annex B.2 (adaptive bands), Tables B.6 to B.9 and NA.9 (ventilation rate, CO2). Reads {"zones":[...]} on stdin, prints {"ok": bool, "errors": [...]}."""
import json
import sys

ADAPTIVE_UPPER_OFFSET = {1: 2.0, 2: 3.0, 3: 4.0}
PER_PERSON = {1: 10.0, 2: 7.0, 3: 4.0, 4: 2.5}
CO2_ABOVE = {1: 550.0, 2: 800.0, 3: 1350.0, 4: 1350.0}
AREA_RATE = {"very_low": (0.5, 0.35, 0.2, 0.15), "low": (1.0, 0.7, 0.4, 0.3), "non_low": (2.0, 1.4, 0.8, 0.6)}
CO2_ABOVE_DE = {1: 350.0, 2: 550.0, 3: 900.0, 4: 1350.0}


def close(a, b):
    return abs(a - b) <= 1e-6 * max(1.0, abs(a), abs(b))


def design_rates(zone, cat):
    annex, method = zone["annex"], zone["ventilation_method"]
    occupants, area = zone["occupants"], zone["area"]
    usage, pollution = zone["usage"], zone["pollution"]
    if annex == "de" and (usage == "residential" or "method_3" in method):
        return None
    if annex == "de" and "method_1" in method and pollution == "very_low":
        return None
    if "method_2" in method:
        ppm = (CO2_ABOVE_DE if annex == "de" else CO2_ABOVE)[cat]
        flow = occupants * 20_000 / ppm / 3.6
    elif usage == "residential":
        if cat == 4:
            return None
        if "method_3" in method:
            flow = occupants * PER_PERSON[cat]
        else:
            person, building = [(3.5, 0.25), (2.5, 0.15), (1.5, 0.1)][cat - 1]
            flow = occupants * person + area * building
    elif "method_3" in method:
        if usage != "office" or pollution != "low":
            return None
        person = [20, 14, 8, 5.5][cat - 1]
        flow = person * max(occupants, area / 10)
    else:
        area_rate = AREA_RATE.get(pollution, AREA_RATE["low"])[cat - 1]
        flow = occupants * PER_PERSON[cat] + area * area_rate
    return max(flow, 0 if usage == "residential" else occupants * 4) * 3.6


def check(zone):
    errors = []
    cat = zone["category"]
    for low, high in zip(zone["lower"], zone["upper"]):
        centre = 0.33 * low[0] + 18.8
        if not (close(low[1], centre - 1.0 - ADAPTIVE_UPPER_OFFSET[cat]) and close(high[1], centre + ADAPTIVE_UPPER_OFFSET[cat])):
            errors.append(f"adaptive band at {low[0]}")
    expected = {c: v for c in range(1, 5) if (v := design_rates(zone, c)) is not None}
    reported = dict(zip(zone["ventilation_categories"], zone["ventilation_totals"]))
    if expected.keys() != reported.keys() or any(not close(v, reported[c]) for c, v in expected.items()):
        errors.append(f"ventilation design flow {reported} vs {expected}")
    co2_deltas = CO2_ABOVE_DE if zone["annex"] == "de" else CO2_ABOVE
    co2_values = [] if zone["annex"] == "de" and zone["usage"] == "residential" else [
        zone["outdoor_co2"] + delta for delta in co2_deltas.values()
    ]
    if len(co2_values) != len(zone["co2_limits"]) or any(not close(a, b) for a, b in zip(co2_values, zone["co2_limits"])):
        errors.append("CO2 category values")
    return errors


def main():
    document = json.load(sys.stdin)
    errors = [e for zone in document["zones"] for e in check(zone)]
    print(json.dumps({"ok": not errors, "errors": errors}))
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
