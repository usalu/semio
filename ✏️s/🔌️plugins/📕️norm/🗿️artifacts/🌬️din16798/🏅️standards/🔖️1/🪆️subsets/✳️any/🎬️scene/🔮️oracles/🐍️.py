"""Independent oracle of the DIN EN 16798 scene diagrams: closed forms of EN 16798-1 Annex B.2 (adaptive bands), Tables B.5 to B.7 (CO2, ventilation rate). Reads {"zones":[...]} on stdin, prints {"ok": bool, "errors": [...]}."""
import json
import sys

HALF_BAND = {1: 2.0, 2: 3.0, 3: 4.0, 4: 5.0}
PER_PERSON = {1: 10.0, 2: 7.0, 3: 4.0, 4: 2.5}
CO2_ABOVE = {1: 550.0, 2: 800.0, 3: 1350.0, 4: 1350.0}
AREA_RATE = {"very_low": (0.5, 0.35, 0.2, 0.15), "low": (1.0, 0.7, 0.4, 0.3), "non_low": (2.0, 1.4, 0.8, 0.6)}
ABSOLUTE = {"de": {"residential": 1200.0, "classroom": 800.0, "other": 900.0}, "en": {"residential": 1500.0, "classroom": 1000.0, "other": 1000.0}}


def close(a, b):
    return abs(a - b) <= 1e-6 * max(1.0, abs(a), abs(b))


def check(zone):
    errors = []
    cat = zone["category"]
    for low, high in zip(zone["lower"], zone["upper"]):
        centre = 0.33 * low[0] + 18.8
        if not (close(low[1], centre - HALF_BAND[cat]) and close(high[1], centre + HALF_BAND[cat])):
            errors.append(f"adaptive band at {low[0]}")
    area = AREA_RATE.get(zone["pollution"], AREA_RATE["low"])
    for c in (1, 2, 3, 4):
        total = (zone["occupants"] * PER_PERSON[c] + zone["area"] * area[c - 1]) * 3.6
        if not close(total, zone["ventilation_totals"][c - 1]):
            errors.append(f"ventilation total of category {c}")
    usage = zone["usage"] if zone["usage"] in ("residential", "classroom") else "other"
    for c in (1, 2, 3, 4):
        above = CO2_ABOVE[c]
        if zone["annex"] == "de":
            above = min(ABSOLUTE["de"][usage] - zone["outdoor_co2"], above)
        if not close(zone["outdoor_co2"] + above, zone["co2_limits"][c - 1]):
            errors.append(f"co2 limit of category {c}")
    return errors


def main():
    document = json.load(sys.stdin)
    errors = [e for zone in document["zones"] for e in check(zone)]
    print(json.dumps({"ok": not errors, "errors": errors}))
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
