"""🧪️ S3-CONTROLS — authors the 26 committed quintets of the energy field-granular leaves (audit C1).

Every vector starts from the committed `🔢️change-model-version/⛔️refuses` before-snapshot (the default model named
"BESTEST 600", version "1" — exactly the typed base every new leaf test's `scenario` builds), applies the scenario's setup,
and writes `(before, mutation, after, diff, outcome)` in the shape the Rust writer (`fixtures::write_when_requested`)
produces: an applied leaf's diff is the whole regenerated model with the unchanged composed-child handles, a refusal or a
no-op carries the all-null diff and leaves the document untouched. The Rust leaf tests then hold every file to the eight
laws (forward, inverse, canonical, outcome, diff, …) and the case's Python second implementation reproduces them
independently. Run from the repository root: `python3 <this file>`.
"""

import copy
import json
import pathlib

SUB = pathlib.Path("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
BASE = json.loads((SUB / "🔢️change-model-version/⛔️refuses/📸️snapshot/⬅️before/🔣️.json").read_text())
EMPTY_DIFF = json.loads((SUB / "🔢️change-model-version/⛔️refuses/🔺️diff/🔣️.json").read_text())
APPLIED_DIFF = json.loads((SUB / "🔢️change-model-version/✅️bumps/🔺️diff/🔣️.json").read_text())


def period(start_month, start_day, end_month, end_day, year):
    return {"start_month": start_month, "start_day": start_day, "end_month": end_month, "end_day": end_day, "year": year}


def site(field, wire, admissible):
    def apply(model, payload):
        value = payload[wire]
        if not admissible(value):
            return "mutation.invariant"
        if model["site"][field] == value:
            return "mutation.no-op"
        model["site"][field] = value
        return None
    return apply


def ground(series):
    def apply(model, payload):
        value, month = payload["newTemperatureC"], payload.get("month")
        if (series and not 1 <= month <= 12) or value < -273.15:
            return "mutation.invariant"
        slot = model["ground_temperature"]
        current = slot[series][month - 1] if series else slot["deep_c"]
        if current == value:
            return "mutation.no-op"
        if series:
            slot[series][month - 1] = value
        else:
            slot["deep_c"] = value
        return None
    return apply


def is_interval(run):
    year = run["year"]
    leap = (year % 4 == 0 and year % 100 != 0) or year % 400 == 0
    lengths = [31, 29 if leap else 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    exists = lambda month, day: 1 <= month <= 12 and 1 <= day <= lengths[month - 1]
    start, end = (run["start_month"], run["start_day"]), (run["end_month"], run["end_day"])
    return exists(*start) and exists(*end) and start <= end


def run(field, wire, low, high):
    def apply(model, payload):
        value = payload[wire]
        if not low <= value <= high:
            return "mutation.invariant"
        if model["run_period"][field] == value:
            return "mutation.no-op"
        edited = dict(model["run_period"], **{field: value})
        if not is_interval(edited):
            return "mutation.target-mismatch"
        model["run_period"] = edited
        return None
    return apply


# (directory, mutation tag, apply, [(case, setup, payload)])
LEAVES = [
    ("🌍️change-site-latitude", "changeSiteLatitude", site("latitude_deg", "newLatitudeDeg", lambda v: -90.0 <= v <= 90.0),
     [("✅️sets", {}, {"newLatitudeDeg": 39.74}), ("⛔️refuses", {}, {"newLatitudeDeg": 123.0})]),
    ("🌏️change-site-longitude", "changeSiteLongitude", site("longitude_deg", "newLongitudeDeg", lambda v: -180.0 <= v <= 180.0),
     [("✅️sets", {}, {"newLongitudeDeg": -104.99}), ("⛔️refuses", {}, {"newLongitudeDeg": 200.0})]),
    ("⛰️change-site-elevation", "changeSiteElevation", site("elevation_m", "newElevationM", lambda v: -300.0 <= v < 8900.0),
     [("✅️sets", {}, {"newElevationM": 1650.0}), ("⛔️refuses", {}, {"newElevationM": 9000.0})]),
    ("🕰️change-site-time-zone", "changeSiteTimeZone", site("time_zone_hours", "newTimeZoneHours", lambda v: -12.0 <= v <= 14.0),
     [("✅️sets", {}, {"newTimeZoneHours": -7.0}), ("⛔️refuses", {}, {"newTimeZoneHours": 15.0})]),
    ("🔝️change-site-north-axis", "changeSiteNorthAxis", site("north_axis_deg", "newNorthAxisDeg", lambda v: True),
     [("✅️sets", {}, {"newNorthAxisDeg": 30.0}), ("🟰️same", {"site.north_axis_deg": 30.0}, {"newNorthAxisDeg": 30.0})]),
    ("🌡️change-ground-building", "changeGroundBuilding", ground("building_surface_c"),
     [("✅️sets", {}, {"month": 7, "newTemperatureC": 21.5}), ("⛔️refuses", {}, {"month": 13, "newTemperatureC": 20.0})]),
    ("🌱️change-ground-shallow", "changeGroundShallow", ground("shallow_c"),
     [("✅️sets", {}, {"month": 1, "newTemperatureC": 2.5}), ("⛔️refuses", {}, {"month": 1, "newTemperatureC": -300.0})]),
    ("⛏️change-ground-deep", "changeGroundDeep", ground(None),
     [("✅️sets", {}, {"newTemperatureC": 11.0}), ("⛔️refuses", {}, {"newTemperatureC": -300.0})]),
    ("🛫️change-run-start-month", "changeRunStartMonth", run("start_month", "newStartMonth", 1, 12),
     [("✅️sets", {}, {"newStartMonth": 3}), ("⛔️refuses", {"run_period": period(1, 1, 1, 31, 2026)}, {"newStartMonth": 2})]),
    ("▶️change-run-start-day", "changeRunStartDay", run("start_day", "newStartDay", 1, 31),
     [("✅️sets", {}, {"newStartDay": 15}), ("⛔️refuses", {"run_period": period(2, 1, 2, 28, 2026)}, {"newStartDay": 30})]),
    ("🛬️change-run-end-month", "changeRunEndMonth", run("end_month", "newEndMonth", 1, 12),
     [("✅️sets", {}, {"newEndMonth": 7}), ("⛔️refuses", {}, {"newEndMonth": 2})]),
    ("⏹️change-run-end-day", "changeRunEndDay", run("end_day", "newEndDay", 1, 31),
     [("✅️sets", {}, {"newEndDay": 30}), ("⛔️refuses", {"run_period": period(3, 15, 3, 31, 2026)}, {"newEndDay": 10})]),
    ("📅️change-run-year", "changeRunYear", run("year", "newYear", 0, 65535),
     [("✅️sets", {}, {"newYear": 2027}), ("⛔️refuses", {"run_period": period(1, 1, 2, 29, 2028)}, {"newYear": 2027})]),
]


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def main():
    written = 0
    for directory, tag, apply, cases in LEAVES:
        for case, setup, payload in cases:
            before = copy.deepcopy(BASE)
            for key, value in setup.items():
                if key == "run_period":
                    before["model"]["run_period"] = value
                else:
                    section, field = key.split(".")
                    before["model"][section][field] = value
            after = copy.deepcopy(before)
            code = apply(after["model"], payload)
            if code == "mutation.no-op":
                outcome, diff = {"status": "no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}, EMPTY_DIFF
            elif code:
                outcome, diff, after = {"status": "rejected", "code": code, "path": []}, EMPTY_DIFF, copy.deepcopy(before)
            else:
                outcome = {"status": "applied", "messages": []}
                diff = dict(APPLIED_DIFF, model=after["model"], structure=before["structure"], zones=before["zones"])
            root = SUB / directory / case
            write(root / "📸️snapshot/⬅️before/🔣️.json", before)
            write(root / "📸️snapshot/➡️after/🔣️.json", after)
            write(root / "🦠️mutation/🔣️.json", {"mutation": tag, **payload})
            write(root / "🔺️diff/🔣️.json", diff)
            write(root / "🎯️outcome/🔣️.json", outcome)
            written += 5
    print("wrote", written)


if __name__ == "__main__":
    main()
